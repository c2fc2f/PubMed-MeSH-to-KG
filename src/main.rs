//! CLI tool that converts the PubMed dataset into a CSV-based Knowledge Graph
//! representation (Neo4J)

mod saver;

use std::{
    num::NonZero,
    path::PathBuf,
    process::ExitCode,
    sync::{Arc, Mutex},
};

use clap::Parser;
use dirs::cache_dir;
use futures::{StreamExt, stream};
use mesh::{
    MeSH, descriptor::models::DescriptorRecord,
    qualifier::models::QualifierRecord,
};
use pubmed::{
    PubMed,
    chunks::{Chunks, models::PubmedArticle},
};

use crate::saver::{mesh::SaverMeSH, pubmed::SaverPubMed};

#[global_allocator]
/// Custom allocator to use less memory and speed up the process
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

/// CLI tool that converts the PubMed dataset into a CSV-based Knowledge Graph
/// representation (Neo4J)
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of parallel task
    #[arg(short, long)]
    parallel: Option<NonZero<usize>>,

    /// Skip how many chunk
    #[arg(short, long, default_value_t = 0)]
    skip: usize,

    /// Enable caching
    #[arg(short, long)]
    no_cache: bool,

    /// Output dir
    #[arg(short, long, default_value = ".")]
    output: PathBuf,
}

#[tokio::main]
async fn main() -> ExitCode {
    let args: Args = Args::parse();

    let pubmed: PubMed = PubMed::builder()
        .cache(if args.no_cache {
            None
        } else {
            cache_dir().map(|d| d.join("pm2kg/pubmed"))
        })
        .build();

    let parallelism: NonZero<usize> = args.parallel.unwrap_or_else(|| {
        std::thread::available_parallelism()
            .unwrap_or(NonZero::new(1).expect("1 is not 0"))
    });

    let chunks: Chunks = match pubmed.chunks().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error during fetching chunks:\n{:?}", e);
            return ExitCode::FAILURE;
        }
    };

    let saver: Arc<Mutex<SaverPubMed>> = match SaverPubMed::new(&args.output) {
        Ok(s) => Arc::new(Mutex::new(s)),
        Err(e) => {
            eprintln!("Error during creation of the CSV files:\n{:?}", e);
            return ExitCode::FAILURE;
        }
    };

    stream::iter(chunks.processor({
        let saver: Arc<Mutex<SaverPubMed>> = Arc::clone(&saver);
        move |article: PubmedArticle| {
            saver.lock().unwrap().add_article(&article).unwrap();
        }
    }))
    .enumerate()
    .skip(args.skip)
    .for_each_concurrent(parallelism.get(), |(idx, fut)| async move {
        fut.await.map_err(|e| {
            panic!("Error on during the deserialization of {idx}\n{e:?}")
        });
    })
    .await;

    match saver.lock().unwrap().flush() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error during flushing of the CSV files:\n{:?}", e);
            return ExitCode::FAILURE;
        }
    };

    drop(pubmed);

    let saver: Arc<Mutex<SaverMeSH>> = match SaverMeSH::new(&args.output) {
        Ok(s) => Arc::new(Mutex::new(s)),
        Err(e) => {
            eprintln!("Error during creation of the CSV files:\n{:?}", e);
            return ExitCode::FAILURE;
        }
    };

    let mesh: MeSH = MeSH::builder()
        .cache(if args.no_cache {
            None
        } else {
            cache_dir().map(|d| d.join("pm2kg/mesh"))
        })
        .build();

    if let Err(e) = mesh
        .descriptor({
            let saver: Arc<Mutex<SaverMeSH>> = Arc::clone(&saver);
            move |desc: DescriptorRecord| {
                saver.lock().unwrap().add_descriptor(&desc).unwrap();
            }
        })
        .await
    {
        eprintln!("Error during writing of the CSV files:\n{:?}", e);
        return ExitCode::FAILURE;
    };

    if let Err(e) = mesh
        .qualifier({
            let saver: Arc<Mutex<SaverMeSH>> = Arc::clone(&saver);
            move |qual: QualifierRecord| {
                saver.lock().unwrap().add_qualifier(&qual).unwrap();
            }
        })
        .await
    {
        eprintln!("Error during writing of the CSV files:\n{:?}", e);
        return ExitCode::FAILURE;
    };

    match saver.lock().unwrap().flush() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error during flushing of the CSV files:\n{:?}", e);
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}
