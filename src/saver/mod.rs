//! Module which regroup all Saver

pub mod mesh;
pub mod pubmed;

/// Macro to initialize CSV Writer
#[macro_export]
macro_rules! writer {
    ($dir:expr, $name:literal, [$($field:literal),* $(,)?]) => {{
        let mut w = Writer::from_writer(
            File::create($dir.join(concat!($name, ".csv")))?
        );
        w.write_record([$($field),*])?;
        w
    }};
}
