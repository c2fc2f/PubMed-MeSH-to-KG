//! CLI tool that converts the PubMed dataset into a CSV-based Knowledge Graph
//! representation (Neo4J)

use clap::Parser;
use csv_async::AsyncWriter;
use futures::{StreamExt, stream};
use pubmed::{
    PubMed, PubMedBuilder,
    chunks::{
        Chunks,
        models::{DateYMD, PubmedArticle, PubmedArticleSet},
    },
};
use std::{
    num::NonZero,
    path::{Path, PathBuf},
    process::ExitCode,
    sync::Arc,
};
use tokio::{fs::File, sync::Mutex};

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

/// Struct that regroup CSV Files
struct Saver {
    /// CSV Writer for Article Nodes
    articles: AsyncWriter<File>,
}

impl Saver {
    /// Init a saver which creates CSV file and writes header
    pub async fn new(dir: &Path) -> std::io::Result<Self> {
        let file: File = File::create(dir.join("Article.csv")).await?;
        let mut writer: AsyncWriter<File> = AsyncWriter::from_writer(file);

        writer
            .write_record(&[
                ":ID(Article)",
                "pmid:int",
                "title",
                "abstract",
                "date_completed:DATE",
                "date_revised:DATE",
                ":LABEL",
            ])
            .await?;

        Ok(Self { articles: writer })
    }

    /// Shuts down the output stream.
    pub async fn shutdown(&mut self) -> Result<(), std::io::Error> {
        self.articles.flush().await
    }

    /// Save one article
    pub async fn add_article(
        &mut self,
        article: &PubmedArticle,
    ) -> std::io::Result<()> {
        let pmid: String = article.medline_citation.pmid.value.to_string();

        let title: &str = &article.medline_citation.article.title.content;

        let abstract_text: String = article
            .medline_citation
            .article
            .abstract_
            .as_ref()
            .map(|a| {
                a.texts
                    .iter()
                    .map(|t| t.content.as_str())
                    .collect::<Vec<_>>()
                    .join("\n\n")
            })
            .unwrap_or_default();

        let date_to_string = |d: Option<&DateYMD>| {
            d.map(|d| format!("{:04}-{:02}-{:02}", d.year, d.month, d.day))
                .unwrap_or_default()
        };

        let date_completed: String =
            date_to_string(article.medline_citation.date_completed.as_ref());
        let date_revised: String =
            date_to_string(article.medline_citation.date_revised.as_ref());

        self.articles
            .write_record(&[
                &pmid,
                &pmid,
                title,
                &abstract_text,
                &date_completed,
                &date_revised,
                "Article",
            ])
            .await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let args: Args = Args::parse();

    let mut pubmed: PubMedBuilder = PubMed::builder();
    if args.no_cache {
        pubmed = pubmed.cache(None);
    }
    let pubmed: PubMed = pubmed.build();

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

    let saver: Arc<Mutex<Saver>> = match Saver::new(&args.output).await {
        Ok(s) => Arc::new(Mutex::new(s)),
        Err(e) => {
            eprintln!("Error during creation of the CSV files:\n{:?}", e);
            return ExitCode::FAILURE;
        }
    };

    stream::iter(chunks)
        .enumerate()
        .skip(args.skip)
        .for_each_concurrent(parallelism.get(), |(_idx, fut)| {
            let saver: Arc<Mutex<Saver>> = Arc::clone(&saver);
            async move {
                let chunk: PubmedArticleSet =
                    fut.await.expect("Error on {idx}");
                for article in chunk.articles {
                    saver
                        .lock()
                        .await
                        .add_article(&article)
                        .await
                        .expect("Error during saving of article")
                }
            }
        })
        .await;

    match saver.lock().await.shutdown().await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error during closing of the CSV files:\n{:?}", e);
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}
