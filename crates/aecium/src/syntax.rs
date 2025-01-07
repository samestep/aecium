use std::{collections::HashMap, fs, io, iter::Peekable, mem, path::PathBuf};

use derive_rustc_index::Idx;
use ra_ap_parser::{Edition, LexedStr, Step, SyntaxKind, TopEntryPoint};
use ra_ap_rustc_index::Idx;

use crate::{
    encoding::{Decodable, Decoder, Encodable},
    name::{Name, Names},
    path::{Path, Paths},
    scope::{Scope, Scopes},
    source::{Source, SourceFile, SourceLoc},
};

fn is_token(kind: SyntaxKind) -> bool {
    let ws = u16::from(SyntaxKind::WHITESPACE);
    assert_eq!(ws + 1, u16::from(SyntaxKind::ABI));
    u16::from(kind) <= ws
}

#[derive(Clone, Copy, Debug, Eq, Hash, Idx, PartialEq)]
pub struct Node(u32);

impl Encodable for Node {
    fn push(self, data: &mut Vec<u8>) {
        self.0.push(data);
    }

    fn write(self, data: &mut [u8]) {
        self.0.write(data);
    }
}

impl Decodable for Node {
    fn decode(decoder: &mut Decoder) -> Self {
        Self(u32::decode(decoder))
    }
}

struct Nodes {
    data: Vec<u8>,
}

impl Nodes {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn next_index(&self) -> Node {
        Node::new(self.data.len())
    }

    fn push(&mut self, x: impl Encodable) -> Node {
        let i = self.data.len();
        x.push(&mut self.data);
        Node::new(i)
    }

    fn write(&mut self, node: Node, x: impl Encodable) {
        x.write(&mut self.data[node.index()..]);
    }
}

struct PendingMod {
    path: Path,
    node: Node,
}

enum Task {
    PendingMods(Vec<PendingMod>),
}

/// Data other than the actual source, to allow a mutable reference while parsing.
struct TreeData {
    names: Names,
    paths: Paths,
    nodes: Nodes,
    scopes: Scopes,
    modules: HashMap<Path, Scope>,
    pending_mods: Vec<PendingMod>,
    pending_macro_calls: Vec<Node>,
}

pub struct Tree<'a> {
    edition: Edition,
    root: &'a str,
    src: Source,
    tree: TreeData,
}

impl<'a> Tree<'a> {
    pub fn new(edition: Edition, root: &'a str) -> io::Result<Self> {
        let mut tree = Self {
            edition,
            root,
            src: Source::new(),
            tree: TreeData {
                names: Names::new(),
                paths: Paths::new(),
                nodes: Nodes::new(),
                scopes: Scopes::new(),
                modules: HashMap::new(),
                pending_mods: Vec::new(),
                pending_macro_calls: Vec::new(),
            },
        };
        tree.file_mod(root, tree.tree.paths.root())?;
        Ok(tree)
    }

    fn file_mod(&mut self, name: &str, path: Path) -> io::Result<Node> {
        let source_file = self.src.read(name, fs::File::open(name)?)?;
        let node = self.tree.nodes.next_index();
        let scope = self.tree.scopes.push(path, None, node);
        self.parse_source(scope, source_file);
        self.tree.modules.insert(path, scope);
        Ok(node)
    }

    fn parse_source(&mut self, scope: Scope, source_file: SourceFile) {
        let range = self.src.range(source_file);
        let text = self.src.code(range.clone());
        let lexed = LexedStr::new(self.edition, text);
        let input = lexed.to_input(self.edition);
        let output = TopEntryPoint::SourceFile.parse(&input, self.edition);
        Parser {
            tree: &mut self.tree,
            lexed,
            start: range.start,
            offset: 0,
            stack: Vec::new(),
            scope,
            nesting: Vec::new(),
            iterator: output.iter().peekable(),
        }
        .entrypoint();
    }

    pub fn expand(&mut self) -> io::Result<()> {
        while let Some(task) = self.next_task() {
            match task {
                Task::PendingMods(pending_mods) => {
                    for pending_mod in pending_mods {
                        let name = self.path_buf(pending_mod.path);
                        let node = self.file_mod(name.to_str().unwrap(), pending_mod.path)?;
                        self.tree.nodes.write(pending_mod.node.plus(2), node);
                    }
                }
            }
        }
        Ok(())
    }

    fn next_task(&mut self) -> Option<Task> {
        let pending_mods = mem::take(&mut self.tree.pending_mods);
        if !pending_mods.is_empty() {
            return Some(Task::PendingMods(pending_mods));
        }
        None
    }

    fn path_buf(&self, mut path: Path) -> PathBuf {
        let mut components = Vec::new();
        while let Some(stem) = self.tree.paths.stem(path) {
            components.push(self.tree.names.get(stem));
            path = self.tree.paths.parent(path).unwrap();
        }
        let mut buf = PathBuf::from(self.root);
        buf.pop();
        for component in components.into_iter().rev() {
            buf.push(component);
        }
        buf.set_extension("rs");
        buf
    }

    pub fn print(&self) {
        let mut decoder = Decoder::new(&self.tree.nodes.data);
        let mut d: usize = 0;
        while !decoder.data().is_empty() {
            let tag = u16::decode(&mut decoder);
            if tag == u16::MAX {
                d -= 1;
            }
            for _ in 0..d {
                print!("  ");
            }
            if tag < u16::MAX {
                let kind = SyntaxKind::from(tag);
                print!("{kind:?}");
                if is_token(kind) {
                    SourceLoc::decode(&mut decoder);
                    if let SyntaxKind::IDENT = kind {
                        let name = Name::decode(&mut decoder);
                        print!(" {:?}", self.tree.names.get(name));
                    }
                } else {
                    if let SyntaxKind::MODULE = kind {
                        Node::decode(&mut decoder);
                    }
                    print!(" {{");
                    d += 1;
                }
                println!();
            }
            if tag == u16::MAX {
                println!("}}");
            }
        }
    }
}

/// Helper struct to process parser output into data structures used for macroexpansion.
struct Parser<'a, 'b, I: Iterator<Item = Step<'a>>> {
    tree: &'b mut TreeData,
    lexed: LexedStr<'a>,
    start: SourceLoc,
    offset: usize,
    stack: Vec<SyntaxKind>,
    scope: Scope,
    nesting: Vec<Scope>,
    iterator: Peekable<I>,
}

impl<'a, I: Iterator<Item = Step<'a>>> Parser<'a, '_, I> {
    fn entrypoint(&mut self) {
        while let Some(()) = self.node() {}
        assert!(self.iterator.next().is_none());
        assert!(self.stack.is_empty());
    }

    /// Handle a [`Step::Token`].
    fn token(&mut self, kind: SyntaxKind, n_input_tokens: u8) -> Option<Name> {
        assert!(is_token(kind));
        while self.lexed.kind(self.offset).is_trivia() {
            self.offset += 1;
        }
        let n = usize::from(n_input_tokens);
        self.tree.nodes.push(kind);
        self.tree.nodes.push(self.start.plus(self.offset));
        match kind {
            SyntaxKind::IDENT => {
                let name = self
                    .tree
                    .names
                    .make(self.lexed.range_text(self.offset..self.offset + n));
                self.tree.nodes.push(name);
                self.offset += n;
                Some(name)
            }
            _ => {
                self.offset += n;
                None
            }
        }
    }

    /// Handle a [`Step::Enter`].
    fn enter(&mut self, kind: SyntaxKind) -> Node {
        assert!(!is_token(kind));
        self.stack.push(kind);
        self.tree.nodes.push(kind)
    }

    /// Handle a [`Step::Exit`].
    fn exit(&mut self) {
        self.stack.pop();
        self.tree.nodes.push(u16::MAX);
    }

    /// Attempt to process an entire node of the tree, returning `Some(())` if successful.
    ///
    /// If the next event is [`Step::Exit`] or the iterator is exhausted, returns `None` instead of
    /// processing anything.
    fn node(&mut self) -> Option<()> {
        if let Some(Step::Exit) | None = self.iterator.peek() {
            return None;
        }
        let height = self.stack.len();
        loop {
            match self.iterator.next().unwrap() {
                Step::Token {
                    kind,
                    n_input_tokens,
                } => {
                    self.token(kind, n_input_tokens);
                }
                Step::FloatSplit { .. } => todo!(),
                Step::Enter { kind } => {
                    let node = self.enter(kind);
                    match kind {
                        SyntaxKind::BLOCK_EXPR => self.block_expr(node),
                        SyntaxKind::MACRO_CALL => self.macro_call(node),
                        SyntaxKind::MODULE => self.module_decl(node),
                        _ => {}
                    }
                }
                Step::Exit => self.exit(),
                Step::Error { .. } => todo!(),
            }
            if self.stack.len() == height {
                return Some(());
            }
        }
    }

    /// Process a [`Step::Enter`].
    fn descend(&mut self, kind: SyntaxKind) {
        match self.iterator.next() {
            Some(Step::Enter { kind: k }) => {
                assert_eq!(k, kind);
                self.enter(kind);
            }
            _ => panic!(),
        }
    }

    /// Process all nodes at the current level of the tree, then process a [`Step::Exit`].
    fn ascend(&mut self) {
        while let Some(()) = self.node() {}
        match self.iterator.next() {
            Some(Step::Exit) => self.exit(),
            _ => panic!(),
        }
    }

    fn scope_node(&mut self, scope: Scope) {
        self.nesting.push(mem::replace(&mut self.scope, scope));
        self.node().unwrap();
        self.scope = self.nesting.pop().unwrap();
    }

    fn scope_ascend(&mut self, scope: Scope) {
        self.nesting.push(mem::replace(&mut self.scope, scope));
        self.ascend();
        self.scope = self.nesting.pop().unwrap();
    }

    /// Process nodes one at a time until one is found with the given `start` kind.
    ///
    /// Returns `None` if [`Self::node`] returns `None` before finding a node with the given kind.
    fn find(&mut self, start: SyntaxKind) -> Option<()> {
        loop {
            match self.iterator.peek() {
                Some(&Step::Enter { kind }) if kind == start => return Some(()),
                _ => self.node()?,
            }
        }
    }

    fn ident(&mut self) -> Name {
        match self.iterator.next().unwrap() {
            Step::Token {
                kind,
                n_input_tokens,
            } => self.token(kind, n_input_tokens).unwrap(),
            _ => panic!(),
        }
    }

    fn name(&mut self) -> Option<Name> {
        self.find(SyntaxKind::NAME)?;
        self.descend(SyntaxKind::NAME);
        let name = self.ident();
        self.ascend();
        Some(name)
    }

    /// Process part of a [`SyntaxKind::BLOCK_EXPR`] node's interior after its [`Step::Enter`].
    fn block_expr(&mut self, start: Node) {
        let module = self.tree.scopes.module(self.scope);
        let scope = self.tree.scopes.push(module, Some(self.scope), start);
        self.scope_ascend(scope);
    }

    /// Process part of a [`SyntaxKind::MACRO_CALL`] node's interior after its [`Step::Enter`].
    fn macro_call(&mut self, start: Node) {
        self.tree.pending_macro_calls.push(start);
    }

    /// Process part of a [`SyntaxKind::MODULE`] node's interior after its [`Step::Enter`].
    fn module_decl(&mut self, start: Node) {
        let body_pointer = self.tree.nodes.push(start);
        let Some(name) = self.name() else { return };
        let parent = self.tree.scopes.module(self.scope);
        let path = self.tree.paths.child(parent, name);
        match self.find(SyntaxKind::ITEM_LIST) {
            Some(()) => {
                let body = self.tree.nodes.next_index();
                self.tree.nodes.write(body_pointer, body);
                let scope = self.tree.scopes.push(path, None, body);
                self.scope_node(scope);
            }
            None => self
                .tree
                .pending_mods
                .push(PendingMod { path, node: start }),
        }
    }
}
