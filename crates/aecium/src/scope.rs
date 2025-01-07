use derive_rustc_index::Idx;
use ra_ap_rustc_index::IndexVec;

use crate::{path::Path, syntax::Node};

#[derive(Clone, Copy, Debug, Eq, Hash, Idx, PartialEq)]
pub struct Scope(u32);

struct ScopeData {
    module: Path,
    parent: Scope,
    node: Node,
}

pub struct Scopes {
    data: IndexVec<Scope, ScopeData>,
}

impl Scopes {
    pub fn new() -> Self {
        Self {
            data: IndexVec::new(),
        }
    }

    pub fn push(&mut self, module: Path, parent: Option<Scope>, node: Node) -> Scope {
        let scope = self.data.next_index();
        self.data.push(ScopeData {
            module,
            parent: parent.unwrap_or(scope),
            node,
        });
        scope
    }

    pub fn module(&self, scope: Scope) -> Path {
        self.data[scope].module
    }

    pub fn parent(&self, scope: Scope) -> Option<Scope> {
        let parent = self.data[scope].parent;
        if parent == scope {
            None
        } else {
            Some(parent)
        }
    }

    pub fn node(&self, scope: Scope) -> Node {
        self.data[scope].node
    }
}
