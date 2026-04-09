# pubmed

A high-performance, asynchronous Rust client for managing and processing the PubMed baseline dataset. It provides a structured interface for discovering, downloading, and iterating through NCBI baseline XML chunks, with optional disk-based caching to minimize redundant network requests.

## Features

  - **Asynchronous**: Built on [`reqwest`](https://docs.rs/reqwest) and [`tokio`](https://docs.rs/tokio) for non-blocking network I/O.
  - **Automated discovery**: Parses the NCBI baseline index page to identify all available `.xml.gz` chunks.
  - **Configurable caching**: Downloads can be cached locally to avoid hitting the NCBI servers on repeated runs.
  - **Builder pattern**: Flexible configuration supporting custom mirrors, HTTP clients, and cache paths.
  - **Optimized Parsing**: High-speed XML deserialization with optional path-tracking for debugging.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
pubmed = "0.1"
tokio = { version = "1", features = ["full"] }
```

### Optional Features

| Feature | Description | Default |
|---|---|---|
| `debug_path` | Enables precise error reporting by tracking the XML path during parsing. | Disabled |

To enable path tracking:

```toml
pubmed = { version = "0.1", features = ["debug_path"] }
```

## Usage

### Default configuration

```rust
use pubmed::PubMed;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = PubMed::new();

    let count = api.fetch_chunks_count().await?;
    println!("Found {} baseline chunks.", count);

    let mut chunks = api.chunks().await?;
    // iterate over chunks...

    Ok(())
}
```

By default, `PubMed::new()` points to the official NCBI FTP server and caches files under the system cache directory (e.g. `~/.cache/pubmed-ftp` on Linux).

### Custom configuration

Use `PubMed::builder()` to override any default:

```rust
use pubmed::PubMed;
use std::path::PathBuf;
use reqwest::Url;

#[tokio::main]
async fn main() {
    let api = PubMed::builder()
        .base_url(Url::parse("https://my-mirror.example.com/pubmed/").unwrap())
        .cache(Some(PathBuf::from("./local_cache")))
        .build();
}
```

## Performance & Debugging

This library is designed for high-throughput processing. When parsing large volumes of XML data:

1.  **Production Mode**: Ensure the `debug_path` feature is **disabled**. This allows the parser to run at full CPU speed without the overhead of tracking the current XML element position.
2.  **Debug Mode**: If you encounter a `Parsing` error that is difficult to locate, enable the `debug_path` feature. The `PubMedError::Parsing` variant will then include the exact XML path (e.g., `PubmedArticleSet -> PubmedArticle[42] -> MedlineCitation`) where the failure occurred.

## API overview

### `PubMed`

| Method | Description |
|---|---|
| `PubMed::new()` | Creates an instance with default settings. |
| `PubMed::builder()` | Returns a `PubMedBuilder` for custom configuration. |
| `fetch_chunks_count()` | Returns the number of `.xml.gz` chunks available at the base URL. |
| `chunks()` | Returns a `Chunks` handle for streaming and parsing the baseline XML files. |

### `PubMedBuilder`

| Method | Description |
|---|---|
| `.client(reqwest::Client)` | Use a custom HTTP client. |
| `.base_url(Url)` | Override the base URL (e.g. a local mirror). |
| `.cache(Option<PathBuf>)` | Set or disable the cache directory. |
| `.build()` | Consumes the builder and returns a `PubMed` instance. |

## Caching behaviour

When a cache directory is configured, downloaded baseline files are written to disk and reused on subsequent runs. This is strongly recommended when processing the full baseline, which consists of hundreds of files totalling tens of gigabytes. If caching is disabled (`.cache(None)`), all data is kept in memory only.

The default cache path resolves to a `pubmed-ftp` subdirectory inside the OS standard cache directory, as returned by [`dirs::cache_dir`](https://docs.rs/dirs).

## Error handling

All fallible async methods return `Result<_, PubMedError>`. The `PubMedError` type covers network failures, unexpected HTTP status codes, and response body decoding errors.

If the `debug_path` feature is enabled, parsing errors will wrap `serde_path_to_error::Error`, providing detailed context about the malformed XML structure.

## License

Licensed under [MIT license](../../LICENSE).
