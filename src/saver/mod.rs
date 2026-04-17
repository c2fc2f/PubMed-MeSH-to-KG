//! Module which regroup all Saver

pub mod mesh;
pub mod pubmed;

/// Alias for CSV writer used by saver
type Writer = std::sync::Mutex<csv::Writer<std::io::BufWriter<std::fs::File>>>;

/// Macro to initialize CSV Writer
#[macro_export]
macro_rules! writer {
    ($dir:expr, $name:literal, [$($field:literal),* $(,)?]) => {{
        let mut w = csv::Writer::from_writer(
            std::io::BufWriter::with_capacity(
                1024 * 1024,
                std::fs::File::create($dir.join(concat!($name, ".csv")))?
            )
        );
        w.write_record([$($field),*])?;
        std::sync::Mutex::new(w)
    }};
}
