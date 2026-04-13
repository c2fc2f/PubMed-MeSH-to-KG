//! Very basic exemple of usage

use clap::Parser;
use futures::{StreamExt, stream};
use pubmed::{PubMed, PubMedBuilder, chunks::models::PubmedArticle};
use std::{
    error::Error,
    num::NonZero,
    ops::AddAssign,
    sync::{Arc, Mutex},
};

#[global_allocator]
/// Custom allocator to use less memory and speed up the process
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

/// Simple program to test the library
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = Args::parse();

    let mut pubmed: PubMedBuilder = PubMed::builder();
    if args.no_cache {
        pubmed = pubmed.cache(None);
    }
    let pubmed: PubMed = pubmed.build();
    println!("Number of chunks: {}", pubmed.fetch_chunks_count().await?);

    let parallelism: NonZero<usize> = args.parallel.unwrap_or_else(|| {
        std::thread::available_parallelism()
            .unwrap_or(NonZero::new(1).expect("1 is not 0"))
    });

    println!("Parallelism: {parallelism}");

    let nb: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

    stream::iter(pubmed.chunks().await?.processor({
        let nb: Arc<Mutex<usize>> = Arc::clone(&nb);
        move |_: PubmedArticle| {
            nb.lock().unwrap().add_assign(1);
        }
    }))
    .enumerate()
    .skip(args.skip)
    .for_each_concurrent(parallelism.get(), |(idx, fut)| async move {
        if let Err(e) = fut.await {
            eprintln!("error on {idx}\n{e:?}");
        }
    })
    .await;

    println!("Articles total: {}", nb.lock().unwrap());

    Ok(())
}
