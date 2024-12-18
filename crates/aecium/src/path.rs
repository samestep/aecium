use std::collections::HashMap;

use derive_rustc_index::Idx;
use ra_ap_rustc_index::{Idx, IndexVec};

use crate::name::Name;

#[derive(Clone, Copy, Debug, Eq, Hash, Idx, PartialEq)]
pub struct Path(u32);

struct PathData {
    stem: Name,
    parent: Path,
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

    fn stem(&self, path: Path) -> Option<Name> {
        let data = &self.data[path];
        if data.parent == path {
            None
        } else {
            Some(data.stem)
        }
    }

    fn parent(&self, path: Path) -> Option<Path> {
        let data = &self.data[path];
        if data.parent == path {
            None
        } else {
            Some(data.parent)
        }
    }

    pub fn child(&mut self, _: Path, _: Name) -> Path {
        todo!()
    }
}
