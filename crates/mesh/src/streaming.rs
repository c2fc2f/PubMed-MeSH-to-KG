//! Module of the macro [`streaming_set_seed`], [`streaming_processor`]
//! and [`streaming_fetch_method`]

/// Generates a streaming [`DeserializeSeed`] implementation for an XML
/// wrapper element that contains a sequence of elements, without ever
/// allocating a Vec.
///
/// # Usage
///
/// ```rust
/// streaming_set_seed!(
///     seed   = DescriptorRecordSetSeed,    // Name of the public seed struct
///     record = DescriptorRecord,           // The item type to deserialize
///     set    = "DescriptorRecordSet",      // XML tag name of the wrapper
///                                          // element
///     item   = "DescriptorRecord",         // XML tag name of each record
///                                          // element
/// );
/// ```
///
/// This expands to:
/// - `{seed}<'a, F>` — the public `DeserializeSeed` entry point
/// - `{seed}Visitor<'a, F>` — map visitor for the wrapper element
/// - `{seed}SeqSeed<'a, F>` — `DeserializeSeed` for the inner sequence
/// - `{seed}SeqVisitor<'a, F>` — sequence visitor that fires the callback
#[macro_export]
macro_rules! streaming_set_seed {
    (
        seed   = $seed:ident,
        record = $record:ty,
        set    = $set_tag:literal,
        item   = $item_tag:literal $(,)?
    ) => {
        ::paste::paste! {
            // ----------------------------------------------------------------
            // Public seed
            // ----------------------------------------------------------------

            /// A [`DeserializeSeed`] that parses a
            #[doc = concat!("`<", $set_tag, ">`")]
            /// and forwards each
            #[doc = concat!("`<", $item_tag, ">`")]
            /// to `processor` without ever collecting them into a [`Vec`].
            pub(crate) struct $seed<'a, F>
            where
                F: Fn($record) + 'a,
            {
                /// Callback invoked for every record found in the XML stream.
                pub(crate) processor: &'a F,
            }

            impl<'de, 'a, F> ::serde::de::DeserializeSeed<'de> for $seed<'a, F>
            where
                F: Fn($record) + 'a,
            {
                type Value = ();

                fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    deserializer.deserialize_struct(
                        $set_tag,
                        &[$item_tag],
                        [<$seed Visitor>] {
                            processor: self.processor,
                        },
                    )
                }
            }

            // ----------------------------------------------------------------
            // Map visitor — top-level wrapper element
            // ----------------------------------------------------------------

            struct [<$seed Visitor>]<'a, F>
            where
                F: Fn($record) + 'a,
            {
                processor: &'a F,
            }

            impl<'de, 'a, F> ::serde::de::Visitor<'de> for [<$seed Visitor>]<'a, F>
            where
                F: Fn($record) + 'a,
            {
                type Value = ();

                fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(concat!("a ", $set_tag))
                }

                fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: ::serde::de::MapAccess<'de>,
                {
                    while let Some(key) = map.next_key::<String>()? {
                        if key == $item_tag {
                            map.next_value_seed([<$seed SeqSeed>] {
                                processor: self.processor,
                            })?;
                        } else {
                            let _: ::serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                    Ok(())
                }
            }

            // ----------------------------------------------------------------
            // Sequence seed
            // ----------------------------------------------------------------

            struct [<$seed SeqSeed>]<'a, F>
            where
                F: Fn($record) + 'a,
            {
                processor: &'a F,
            }

            impl<'de, 'a, F> ::serde::de::DeserializeSeed<'de> for [<$seed SeqSeed>]<'a, F>
            where
                F: Fn($record) + 'a,
            {
                type Value = ();

                fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    deserializer.deserialize_seq([<$seed SeqVisitor>] {
                        processor: self.processor,
                    })
                }
            }

            // ----------------------------------------------------------------
            // Sequence visitor — fires the callback per element
            // ----------------------------------------------------------------

            struct [<$seed SeqVisitor>]<'a, F>
            where
                F: Fn($record) + 'a,
            {
                processor: &'a F,
            }

            impl<'de, 'a, F> ::serde::de::Visitor<'de> for [<$seed SeqVisitor>]<'a, F>
            where
                F: Fn($record) + 'a,
            {
                type Value = ();

                fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(concat!("a sequence of ", $item_tag))
                }

                fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: ::serde::de::SeqAccess<'de>,
                {
                    while let Some(record) = seq.next_element::<$record>()? {
                        (self.processor)(record);
                    }
                    Ok(())
                }
            }
        }
    };
}

/// Generates a pair of functions:
///
/// * `$deserialize_fn` — runs the `quick-xml` deserializer on a `BufReader`,
///   forwarding each record to a callback as soon as it is parsed.
/// * `$process_fn` — downloads (and optionally caches) one `.xml` file, then
///   calls `$deserialize_fn` inside a `spawn_blocking` task.
///
/// # Parameters
///
/// | Name             | Description                                              |
/// |------------------|----------------------------------------------------------|
/// | `deserialize_fn` | Name of the low-level sync deserializer function         |
/// | `process_fn`     | Name of the async download-and-parse function            |
/// | `seed`           | The `DeserializeSeed` struct (e.g. `DescriptorRecordSetSeed`) |
/// | `record`         | The record type yielded by the seed (e.g. `DescriptorRecord`) |
/// | `error`          | The crate error enum (e.g. `MeSHError`)                  |
/// | `err_parsing`    | Variant that wraps a parse error (e.g. `MeSHError::Parsing`) |
/// | `err_cache`      | Variant that wraps an I/O error   (e.g. `MeSHError::Cache`)   |
/// | `err_request`    | Variant that wraps a reqwest error (e.g. `MeSHError::Request`) |
///
/// # Example
///
/// ```rust
/// streaming_processor!(
///     deserialize_fn   = deserialize_descriptor,
///     process_fn       = process_descriptor,
///     seed             = DescriptorRecordSetSeed,
///     record           = DescriptorRecord,
///     error            = MeSHError,
///     err_parsing      = MeSHError::Parsing,
///     err_cache        = MeSHError::Cache,
///     err_request      = MeSHError::Request,
/// );
/// ```
#[macro_export]
macro_rules! streaming_processor {
    (
        deserialize_fn    = $deserialize_fn:ident,
        process_fn        = $process_fn:ident,
        seed              = $seed:ident,
        record            = $record:ty,
        error             = $error:ty,
        err_parsing       = $err_parsing:path,
        err_cache         = $err_cache:path,
        err_request       = $err_request:path,
    ) => {
        // --------------------------------------------------------------------
        // Sync deserializer
        // --------------------------------------------------------------------

        /// Runs the [`quick-xml`] deserializer on `reader`. Each record is
        /// forwarded to `processor` as soon as it is parsed.
        ///
        /// # Errors
        ///
        /// Returns a parsing error if `quick-xml` fails to deserialize the
        /// document.
        fn $deserialize_fn<R, F>(
            reader: ::std::io::BufReader<R>,
            processor: &F,
        ) -> Result<(), $error>
        where
            R: ::std::io::Read,
            F: Fn($record),
        {
            use ::serde::de::DeserializeSeed as _;

            let seed = $seed { processor };

            #[cfg(feature = "debug_path")]
            {
                let mut de = ::quick_xml::de::Deserializer::from_reader(reader);
                let mut track = ::serde_path_to_error::Track::new();
                let tracked = ::serde_path_to_error::Deserializer::new(
                    &mut de, &mut track,
                );
                seed.deserialize(tracked).map_err(|e| {
                    let path = track.path().clone();
                    $err_parsing(::serde_path_to_error::Error::new(path, e))
                })
            }

            #[cfg(not(feature = "debug_path"))]
            {
                let mut de: ::quick_xml::de::Deserializer<_> =
                    ::quick_xml::de::Deserializer::from_reader(reader);
                seed.deserialize(&mut de).map_err($err_parsing)
            }
        }

        // --------------------------------------------------------------------
        // Async download-and-parse
        // --------------------------------------------------------------------

        /// Downloads (and optionally caches to disk) one `.xml` file, then
        /// runs the deserializer inside a `spawn_blocking` task.
        ///
        /// # Errors
        ///
        /// Returns a cache error on filesystem I/O failures and a request
        /// error on network failures.
        ///
        /// # Panics
        ///
        /// Panics if the file URL cannot be constructed.
        pub async fn $process_fn<F>(
            client: ::reqwest::Client,
            base_url: ::reqwest::Url,
            cache: Option<::std::path::PathBuf>,
            file: String,
            processor: ::std::sync::Arc<F>,
        ) -> Result<(), $error>
        where
            F: Fn($record) + Send + Sync + 'static,
        {
            use ::futures::StreamExt as _;
            use ::tokio::io::AsyncWriteExt as _;
            use ::tokio_util::io::StreamReader;
            use ::tokio_util::io::SyncIoBridge;

            let url: ::reqwest::Url =
                base_url.join(&file).expect("Invalid URL construction");

            if let Some(dir) = &cache {
                let file_path = dir.join(&file);

                if !file_path.exists() {
                    let tmp_path = file_path.with_added_extension("tmp");

                    ::tokio::fs::create_dir_all(dir)
                        .await
                        .map_err($err_cache)?;

                    let response = client
                        .get(url)
                        .send()
                        .await
                        .and_then(|r| r.error_for_status())
                        .map_err($err_request)?;

                    let mut dest = ::tokio::fs::File::create(&tmp_path)
                        .await
                        .map_err($err_cache)?;

                    let mut stream = response.bytes_stream();
                    while let Some(chunk) = stream.next().await {
                        dest.write_all(chunk.map_err($err_request)?.as_ref())
                            .await
                            .map_err($err_cache)?;
                    }
                    dest.flush().await.map_err($err_cache)?;

                    ::tokio::fs::rename(&tmp_path, &file_path)
                        .await
                        .map_err($err_cache)?;
                }

                ::tokio::task::spawn_blocking(move || {
                    let file =
                        ::std::fs::File::open(file_path).map_err($err_cache)?;
                    let buf = ::std::io::BufReader::new(file);
                    $deserialize_fn(buf, processor.as_ref())
                })
                .await
                .expect("Tokio blocking task panicked")
            } else {
                let response = client
                    .get(url)
                    .send()
                    .await
                    .and_then(|r| r.error_for_status())
                    .map_err($err_request)?;

                let byte_stream = response.bytes_stream().map(|res| {
                    res.map_err(|e| ::std::io::Error::other($err_request(e)))
                });

                let buf = ::std::io::BufReader::new(SyncIoBridge::new(
                    StreamReader::new(byte_stream),
                ));

                ::tokio::task::spawn_blocking(move || {
                    $deserialize_fn(buf, processor.as_ref())
                })
                .await
                .expect("Tokio blocking task panicked")
            }
        }
    };
}

/// Generates an async method on `Self` that:
/// 1. Fetches the NLM index page,
/// 2. Extracts the target `.xml` filename via a regex,
/// 3. Delegates to a `$process_fn` generated by [`streaming_processor!`].
///
/// `Self` is expected to expose `self.client`, `self.base_url`, and
/// `self.cache`.
///
/// # Parameters
///
/// | Name          | Description                                                    |
/// |---------------|----------------------------------------------------------------|
/// | `method`      | Name of the generated async method                            |
/// | `process_fn`  | The async process function to delegate to (from `streaming_processor!`) |
/// | `record`      | The record type forwarded to the callback                      |
/// | `regex`       | Literal regex string to extract the filename from the index page |
/// | `label`       | Human-readable label used in the `MissingFile` error variant   |
/// | `error`       | The crate error enum                                           |
/// | `err_missing` | Variant wrapping a missing-file error (takes a `String`)       |
///
/// # Example
///
/// ```rust
/// impl MeSHClient {
///     streaming_fetch_method!(
///         method       = descriptor,
///         process_fn   = process_descriptor,
///         record       = DescriptorRecord,
///         regex        = r#"href="(desc[^"]+\.xml)""#,
///         label        = "Descriptor",
///         error        = MeSHError,
///         err_missing  = MeSHError::MissingFile,
///     );
/// }
/// ```
#[macro_export]
macro_rules! streaming_fetch_method {
    (
        method      = $method:ident,
        process_fn  = $process_fn:ident,
        record      = $record:ty,
        regex       = $regex:literal,
        label       = $label:literal,
        error       = $error:ty,
        err_missing = $err_missing:path $(,)?
    ) => {
        /// Fetches and processes the latest records from the NLM repository.
        ///
        /// # Errors
        ///
        /// Returns an error if:
        /// * The NLM index page is unreachable or does not contain a valid
        ///   file link.
        /// * Network issues occur during the download of the XML file.
        /// * Filesystem errors occur while reading from or writing to the
        ///   cache.
        /// * The XML content is malformed and cannot be parsed into the
        ///   expected models.
        ///
        /// # Panics
        ///
        /// * Panics if the internal regex for identifying the target file
        ///   fails to compile.
        /// * Panics if the URL for the specific file cannot be constructed.
        /// * Panics if the underlying `tokio::task::spawn_blocking` fails.
        pub async fn $method<F>(&self, processor: F) -> Result<(), $error>
        where
            F: Fn($record) + Send + Sync + 'static,
        {
            let pattern: ::regex::Regex =
                ::regex::Regex::new($regex).expect(concat!(
                    "Invalid regex pattern for `",
                    stringify!($method),
                    "`"
                ));

            let content: String = self
                .client
                .get(self.base_url.as_str())
                .send()
                .await?
                .error_for_status()?
                .text()
                .await?;

            let file: String = pattern
                .captures(&content)
                .map(|cap| cap[1].to_string())
                .ok_or_else(|| $err_missing(concat!($label).to_string()))?;

            let client = self.client.clone();
            let base_url = self.base_url.clone();
            let cache = self.cache.clone();

            $process_fn(
                client,
                base_url,
                cache,
                file,
                ::std::sync::Arc::new(processor),
            )
            .await
        }
    };
}
