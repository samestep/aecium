use std::{collections::HashMap, fs, io};

use derive_rustc_index::Idx;
use ra_ap_parser::{Edition, LexedStr, Step, SyntaxKind, TopEntryPoint};
use ra_ap_rustc_index::Idx;

use crate::{
    encoding::{Decodable, Decoder, Encodable},
    name::{Name, Names},
    path::{Path, Paths},
    source::{Source, SourceLoc},
};

fn is_token(kind: SyntaxKind) -> bool {
    let ws = u16::from(SyntaxKind::WHITESPACE);
    assert_eq!(ws + 1, u16::from(SyntaxKind::ABI));
    u16::from(kind) <= ws
}

#[derive(Clone, Copy, Debug, Eq, Hash, Idx, PartialEq)]
pub struct Node(u32);

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
        x.encode(&mut self.data);
        Node::new(i)
    }
}

pub struct Tree {
    edition: Edition,
    src: Source,
    names: Names,
    nodes: Nodes,
    paths: Paths,
    modules: HashMap<Path, Node>,
}

impl Tree {
    pub fn new(edition: Edition) -> Self {
        Self {
            edition,
            src: Source::new(),
            nodes: Nodes::new(),
            names: Names::new(),
            paths: Paths::new(),
            modules: HashMap::new(),
        }
    }

    pub fn root(&mut self, name: &str) -> io::Result<()> {
        let node = self.file(name)?;
        self.modules.insert(self.paths.root(), node);
        Ok(())
    }

    fn file(&mut self, name: &str) -> io::Result<Node> {
        let source_file = self.src.read(name, fs::File::open(name)?)?;
        let range = self.src.range(source_file);
        let text = self.src.code(range.clone());
        let lexed = LexedStr::new(self.edition, text);
        let input = lexed.to_input(self.edition);
        let output = TopEntryPoint::SourceFile.parse(&input, self.edition);
        let node = self.nodes.next_index();
        let mut i: usize = 0;
        for step in output.iter() {
            match step {
                Step::Token {
                    kind,
                    n_input_tokens,
                } => {
                    assert!(is_token(kind));
                    while lexed.kind(i).is_trivia() {
                        i += 1;
                    }
                    let n = usize::from(n_input_tokens);
                    self.nodes.push(kind);
                    self.nodes.push(range.start.plus(i));
                    if let SyntaxKind::IDENT = kind {
                        let name = self.names.make(lexed.range_text(i..i + n));
                        self.nodes.push(name);
                    }
                    i += n;
                }
                Step::FloatSplit { .. } => todo!(),
                Step::Enter { kind } => {
                    assert!(!is_token(kind));
                    self.nodes.push(kind);
                }
                Step::Exit => {
                    self.nodes.push(u16::MAX);
                }
                Step::Error { .. } => todo!(),
            }
        }
        Ok(node)
    }

    pub fn expand(&mut self) -> io::Result<()> {
        // TODO: actually expand
        let node = self.modules.get(&self.paths.root()).unwrap();
        let mut decoder = Decoder::new(&self.nodes.data[node.index()..]);
        let mut d: usize = 0;
        loop {
            let tag = u16::decode(&mut decoder);
            if tag == u16::MAX {
                d -= 1;
                if d == 0 {
                    return Ok(());
                }
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
                        print!(" {:?}", self.names.get(name));
                    }
                } else {
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
