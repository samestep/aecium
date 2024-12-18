use std::hash::{DefaultHasher, Hash, Hasher};

use derive_rustc_index::Idx;
use indexmap::{
    map::{raw_entry_v1::RawEntryMut, RawEntryApiV1},
    IndexMap,
};
use ra_ap_rustc_index::Idx;

use crate::encoding::{Decodable, Decoder, Encodable};

#[derive(Clone, Copy, Debug, Eq, Hash, Idx, PartialEq)]
pub struct Name(u32);

impl Encodable for Name {
    fn encode(&self, data: &mut Vec<u8>) {
        self.0.encode(data);
    }
}

impl Decodable for Name {
    fn decode(decoder: &mut Decoder) -> Self {
        Self(u32::decode(decoder))
    }
}

#[derive(Clone, Copy)]
struct NameLoc(u32);

pub struct Names {
    data: String,
    names: IndexMap<(NameLoc, NameLoc), ()>,
}

impl Names {
    pub fn new() -> Self {
        Self {
            data: String::new(),
            names: IndexMap::new(),
        }
    }

    pub fn get(&self, id: Name) -> &str {
        let (&(i, j), _) = self.names.get_index(id.index()).unwrap();
        &self.data[i.0.index()..j.0.index()]
    }

    pub fn make(&mut self, name: &str) -> Name {
        let mut state = DefaultHasher::new();
        name.hash(&mut state);
        let hash = state.finish();
        let entry = self
            .names
            .raw_entry_mut_v1()
            .from_hash(hash, |&(i, j)| name == &self.data[i.0.index()..j.0.index()]);
        let id = Name::new(entry.index());
        if let RawEntryMut::Vacant(vacant) = entry {
            let i = NameLoc(self.data.len().try_into().unwrap());
            self.data.push_str(name);
            let j = NameLoc(self.data.len().try_into().unwrap());
            vacant.insert_hashed_nocheck(hash, (i, j), ());
        }
        id
    }
}

#[cfg(test)]
mod tests {
    use crate::name::Names;

    #[test]
    fn test_same() {
        let mut names = Names::new();
        let a = names.make("foo");
        let b = names.make("foo");
        assert_eq!(a, b);
    }

    #[test]
    fn test_different() {
        let mut names = Names::new();
        let a = names.make("foo");
        let b = names.make("bar");
        assert_ne!(a, b);
    }
}
