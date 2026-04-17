# mesh

A streaming Rust client for the [NLM MeSH XML dataset](https://www.nlm.nih.gov/mesh/meshhome.html).

MeSH (Medical Subject Headings) is the NLM's controlled vocabulary for indexing biomedical literature. NLM publishes the full dataset as a set of large XML files. This crate fetches those files over HTTP, parses them with [`quick-xml`](https://docs.rs/quick-xml) and [`serde`](https://docs.rs/serde), and forwards each record to a caller-supplied callback — without ever allocating the entire dataset in memory.

## Dataset overview

The MeSH dataset is split into three record types, each stored in a dedicated XML file on the NLM server:

| Method | File pattern | Record type |
|---|---|---|
| `MeSH::descriptor` | `descXXXX.xml` | `DescriptorRecord` |
| `MeSH::qualifier` | `qualXXXX.xml` | `QualifierRecord` |
| `MeSH::supplemental` | `suppXXXX.xml` | `SupplementalRecord` |
| `MeSH::pharmacological_action` | `paXXXX.xml` | `PharmacologicalActionRecord` |


## Installation

```toml
[dependencies]
mesh = { git = "..." }
tokio = { version = "1", features = ["full"] }
```

## Quick start

```rust
use mesh::MeSH;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MeSH::new(); // uses the default NLM URL and system cache dir

    client.descriptor(|record| {
        println!("{} — {}", record.ui, record.name.value);
    }).await?;

    Ok(())
}
```

## Configuration

Use `MeSH::builder()` to customise the HTTP client, the base URL, or the cache directory:

```rust
use mesh::MeSH;
use std::path::PathBuf;

let client = MeSH::builder()
    .cache(Some(PathBuf::from("/tmp/mesh-cache")))
    .build();
```

Set the cache to `None` to disable on-disk caching and stream XML directly into the parser:

```rust
let client = mesh::MeSH::builder()
    .cache(None)
    .build();
```

You can also supply a pre-configured `reqwest::Client` — useful for setting custom headers, timeouts, or proxies:

```rust
use mesh::MeSH;
use reqwest::Client;
use std::time::Duration;

let http = Client::builder()
    .timeout(Duration::from_secs(120))
    .build()?;

let client = MeSH::builder()
    .client(http)
    .build();
```

## Caching

When a cache directory is configured (the default when using `MeSH::default()`), the downloaded XML files are written to disk on the first call and reused on subsequent calls. Files are written atomically via a `.tmp` rename to avoid leaving partially-written data.

The default cache location is a `mesh-ftp` subdirectory inside the platform's standard cache directory (resolved via the [`dirs`](https://docs.rs/dirs) crate):

| Platform | Default path |
|---|---|
| Linux | `~/.cache/mesh-ftp/` |
| macOS | `~/Library/Caches/mesh-ftp/` |
| Windows | `%LOCALAPPDATA%\mesh-ftp\` |

## Streaming design

Each XML file can be hundreds of megabytes. The parsing pipeline avoids collecting records into a `Vec`:

1. The HTTP response body is handed to a `BufReader` inside a `spawn_blocking` task (or read from the cache on disk).
2. A custom `serde::de::DeserializeSeed` implementation walks the XML stream element-by-element via `quick-xml`'s streaming deserializer.
3. Each fully-parsed record is forwarded immediately to the callback.

The three macros `streaming_set_seed!`, `streaming_processor!`, and `streaming_fetch_method!` encapsulate this pipeline and are re-exported so that downstream crates can reuse the same pattern for custom record types.

## Feature flags

| Feature | Description |
|---|---|
| `debug_path` | Wraps XML parse errors with [`serde_path_to_error`](https://docs.rs/serde_path_to_error), adding the JSON/XML path to error messages. Useful during development; adds a dependency. |

Enable it in `Cargo.toml`:

```toml
[dependencies]
mesh = { git = "...", features = ["debug_path"] }
```

## Error handling

All three methods return `Result<(), MeSHError>`. The `MeSHError` enum covers:

- `Request` — network or HTTP errors from `reqwest`.
- `Parsing` — malformed XML or schema mismatches from `quick-xml`.
- `Cache` — filesystem I/O errors when reading from or writing to the cache.
- `MissingFile` — the expected XML file was not found on the NLM server.

## Running the example

```bash
# With caching (default)
cargo run --example simple

# Without caching
cargo run --example simple -- --no-cache
```

## License

MIT
