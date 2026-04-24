use dashmap::DashMap;
use dashmap::mapref::entry::Entry;
use std::path::PathBuf;

#[salsa::db]
#[derive(Clone, Default)]
pub struct Database {
    storage: salsa::Storage<Self>,
    files: DashMap<PathBuf, File>,
}

#[salsa::input]
pub struct File {
    path: PathBuf,
    #[returns(ref)]
    pub contents: String,
}

#[salsa::db]
pub trait SourceDatabase: salsa::Database {
    fn file(&self, path: PathBuf) -> Option<File>;
}

#[salsa::db]
impl salsa::Database for Database {}

#[salsa::db]
impl SourceDatabase for Database {
    fn file(&self, path: PathBuf) -> Option<File> {
        let path = path.canonicalize().unwrap();
        Some(match self.files.entry(path.clone()) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let contents = std::fs::read_to_string(&path).unwrap();
                *entry.insert(File::new(self, path, contents))
            }
        })
    }
}
