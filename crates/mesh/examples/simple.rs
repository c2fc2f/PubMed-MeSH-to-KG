//! Very basic exemple of usage

use clap::Parser;
use mesh::{MeSH, MeSHBuilder};
use std::{
    error::Error,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

/// Simple program to test the library
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Enable caching
    #[arg(short, long)]
    no_cache: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = Args::parse();

    let mut mesh: MeSHBuilder = MeSH::builder();
    if args.no_cache {
        mesh = mesh.cache(None);
    }
    let mesh: MeSH = mesh.build();

    let nb_desc: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let nb_qual: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let nb_supp: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

    mesh.descriptor({
        let nb: Arc<AtomicUsize> = Arc::clone(&nb_desc);

        move |_| {
            nb.fetch_add(1, Ordering::SeqCst);
        }
    })
    .await?;

    mesh.qualifier({
        let nb: Arc<AtomicUsize> = Arc::clone(&nb_qual);

        move |_| {
            nb.fetch_add(1, Ordering::SeqCst);
        }
    })
    .await?;

    mesh.supplemental({
        let nb: Arc<AtomicUsize> = Arc::clone(&nb_supp);

        move |_| {
            nb.fetch_add(1, Ordering::SeqCst);
        }
    })
    .await?;

    println!("Descriptor count: {}", nb_desc.load(Ordering::SeqCst));
    println!("Qualifier count: {}", nb_qual.load(Ordering::SeqCst));
    println!("Supplemental count: {}", nb_supp.load(Ordering::SeqCst));

    Ok(())
}
