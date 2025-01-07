use std::collections::HashMap;

use derive_rustc_index::Idx;
use ra_ap_rustc_index::{Idx, IndexVec};

use crate::name::Name;

#[derive(Clone, Copy, Debug, Eq, Hash, Idx, PartialEq)]
pub struct Path(u32);

struct PathData {
    parent: Path,
    stem: Name,
}

pub struct Paths {
    data: IndexVec<Path, PathData>,
    children: HashMap<(Path, Name), Path>,
}

impl Paths {
    pub fn new() -> Self {
        let mut data = IndexVec::new();
        data.push(PathData {
            stem: Name::new(0), // dummy value, shouldn't be used
            parent: Path(0),
        });
        let children = HashMap::new();
        Self { data, children }
    }

    pub fn root(&self) -> Path {
        Path(0)
    }

    pub fn stem(&self, path: Path) -> Option<Name> {
        let data = &self.data[path];
        if data.parent == path {
            None
        } else {
            Some(data.stem)
        }
    }

    pub fn parent(&self, path: Path) -> Option<Path> {
        let data = &self.data[path];
        if data.parent == path {
            None
        } else {
            Some(data.parent)
        }
    }

    pub fn child(&mut self, parent: Path, stem: Name) -> Path {
        *self
            .children
            .entry((parent, stem))
            .or_insert_with(|| self.data.push(PathData { parent, stem }))
    }
}
