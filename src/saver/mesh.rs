//! Module of a SaverPubMeSH to CSV

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    path::Path,
    sync::Arc,
};

use csv::Writer;
use mesh::{
    descriptor::models::DescriptorRecord, qualifier::models::QualifierRecord,
};

use crate::writer;

/// Struct that regroup CSV Files
pub struct SaverMeSH {
    /// CSV Writer for MeSHDescriptor Nodes
    mesh_descriptors: Writer<File>,

    /// CSV Writer for MeSHQualifier Nodes
    mesh_qualifiers: Writer<File>,

    /// CSV writer for NARROWER_THAN Relation for Descriptor Node
    narrower_than: Writer<File>,

    /// MeSH path to his DUI
    path_to_id: HashMap<Arc<str>, Arc<str>>,

    /// MeSH DUI to his path
    id_to_path: HashMap<Arc<str>, Vec<Arc<str>>>,
}

impl SaverMeSH {
    /// Init a saver which creates CSV file and writes header
    pub fn new(dir: &Path) -> std::io::Result<Self> {
        Ok(Self {
            mesh_descriptors: writer!(
                dir,
                "MeSHDescriptor",
                ["dui:ID(MeSH)", "name", "treePath:string[]"]
            ),
            mesh_qualifiers: writer!(
                dir,
                "MeSHQualifier",
                ["dui:ID(MeSH)", "name", "treePath:string[]"]
            ),
            narrower_than: writer!(
                dir,
                "NARROWER_THAN",
                [":START_ID(MeSH)", ":END_ID(MeSH)"]
            ),
            path_to_id: HashMap::new(),
            id_to_path: HashMap::new(),
        })
    }

    /// Flush every CSV file
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.mesh_descriptors.flush()?;

        for (id, paths) in self.id_to_path.iter() {
            let mut parents: HashSet<&str> = HashSet::new();
            for path in paths {
                if let Some(parent) = remove_last_extension(path) {
                    let parent_id: &str =
                        self.path_to_id.get(parent).unwrap().as_ref();
                    if !parents.contains(parent_id) {
                        self.narrower_than
                            .write_record([parent_id, id.as_ref()])?;

                        parents.insert(parent_id);
                    }
                }
            }
        }

        self.narrower_than.flush()?;

        Ok(())
    }

    /// Save one descriptor
    pub fn add_descriptor(
        &mut self,
        desc: &DescriptorRecord,
    ) -> std::io::Result<()> {
        self.mesh_descriptors.write_record([
            desc.ui.as_str(),
            desc.name.value.as_str(),
            desc.tree_numbers
                .as_ref()
                .map(|t| {
                    let ui: Arc<str> = desc.ui.clone().into();
                    let paths: Vec<Arc<str>> = t
                        .items
                        .iter()
                        .map(|item| {
                            let item: Arc<str> = item.clone().into();

                            self.path_to_id
                                .insert(Arc::clone(&item), Arc::clone(&ui));

                            item
                        })
                        .collect();

                    self.id_to_path.insert(ui, paths);

                    t.items.join(";")
                })
                .unwrap_or_default()
                .as_str(),
        ])?;
        Ok(())
    }

    /// Save one qualifier
    pub fn add_qualifier(
        &mut self,
        qual: &QualifierRecord,
    ) -> std::io::Result<()> {
        self.mesh_qualifiers.write_record([
            qual.ui.as_str(),
            qual.name.value.as_str(),
            qual.tree_numbers
                .as_ref()
                .map(|t| {
                    let ui: Arc<str> = qual.ui.clone().into();
                    let paths: Vec<Arc<str>> = t
                        .items
                        .iter()
                        .map(|item| {
                            let item: Arc<str> = item.clone().into();

                            self.path_to_id
                                .insert(Arc::clone(&item), Arc::clone(&ui));

                            item
                        })
                        .collect();

                    self.id_to_path.insert(ui, paths);

                    t.items.join(";")
                })
                .unwrap_or_default()
                .as_str(),
        ])?;
        Ok(())
    }
}

/// Remove the last .<something> of `s` and None if `s` don't contains this.
fn remove_last_extension(s: &str) -> Option<&str> {
    let dot_pos: usize = s.rfind('.')?;
    Some(&s[..dot_pos])
}
