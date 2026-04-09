//! Very basic exemple of usage

use clap::Parser;
use futures::{StreamExt, stream};
use pubmed::{PubMed, PubMedBuilder};
use std::{error::Error, num::NonZero};

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

    let parallelism: usize = args
        .parallel
        .map(|p| p.into())
        .unwrap_or(std::thread::available_parallelism()?.get());

    println!("Parallelism: {parallelism}");

    let total: usize = stream::iter(pubmed.chunks().await?)
        .enumerate()
        .skip(args.skip)
        .map(|(idx, fut)| async move {
            match fut.await {
                Err(e) => {
                    eprintln!("error on {idx}\n{e:?}");
                    0
                }
                Ok(chunk) => {
                    let count: usize =
                        chunk.articles.len() + chunk.book_articles.len();
                    println!("{idx:>4} - number of articles: {count}");
                    drop(chunk);
                    count
                }
            }
        })
        .buffer_unordered(parallelism)
        .fold(0, |acc, n| async move { acc + n })
        .await;

    println!("Articles total: {total}");

    Ok(())
}
