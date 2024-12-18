use std::{io, ops::Range};

use derive_rustc_index::Idx;
use ra_ap_rustc_index::{Idx, IndexVec};

use crate::encoding::{Decodable, Decoder, Encodable};

#[derive(Clone, Copy, Debug, Eq, Hash, Idx, PartialEq)]
pub struct SourceFile(u16);

#[derive(Clone, Copy, Debug, Eq, Hash, Idx, PartialEq)]
struct FilenameLoc(u32);

#[derive(Clone, Copy, Debug, Eq, Hash, Idx, PartialEq)]
pub struct SourceLoc(u32);

impl Encodable for SourceLoc {
    fn encode(&self, data: &mut Vec<u8>) {
        self.0.encode(data);
    }
}

impl Decodable for SourceLoc {
    fn decode(decoder: &mut Decoder) -> Self {
        Self(u32::decode(decoder))
    }
}

pub struct Source {
    filenames: String,
    code: String,
    files: IndexVec<SourceFile, (FilenameLoc, SourceLoc)>,
}

impl Source {
    pub fn new() -> Self {
        Self {
            filenames: String::new(),
            code: String::new(),
            files: IndexVec::new(),
        }
    }

    pub fn read(&mut self, name: &str, mut code: impl io::Read) -> io::Result<SourceFile> {
        let filename_loc = FilenameLoc::new(self.filenames.len());
        self.filenames.push_str(name);
        let source_loc = SourceLoc::new(self.code.len());
        code.read_to_string(&mut self.code)?;
        Ok(self.files.push((filename_loc, source_loc)))
    }

    pub fn name(&self, file: SourceFile) -> &str {
        let i = self.files[file].0.index();
        match self.files.get(file.plus(1)) {
            Some(&(j, _)) => &self.filenames[i..j.index()],
            None => &self.filenames[i..],
        }
    }

    pub fn range(&self, file: SourceFile) -> Range<SourceLoc> {
        let (_, i) = self.files[file];
        match self.files.get(file.plus(1)) {
            Some(&(_, j)) => i..j,
            None => i..SourceLoc::new(self.code.len()),
        }
    }

    pub fn code(&self, range: Range<SourceLoc>) -> &str {
        &self.code[range.start.index()..range.end.index()]
    }
}
