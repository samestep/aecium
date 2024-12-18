use ra_ap_parser::SyntaxKind;

pub trait Encodable {
    fn encode(&self, data: &mut Vec<u8>);
}

impl Encodable for u16 {
    fn encode(&self, data: &mut Vec<u8>) {
        data.extend_from_slice(&self.to_le_bytes());
    }
}

impl Encodable for u32 {
    fn encode(&self, data: &mut Vec<u8>) {
        data.extend_from_slice(&self.to_le_bytes());
    }
}

impl Encodable for SyntaxKind {
    fn encode(&self, data: &mut Vec<u8>) {
        u16::from(*self).encode(data);
    }
}

pub struct Decoder<'a> {
    data: &'a [u8],
    index: usize,
}

impl<'a> Decoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, index: 0 }
    }

    pub fn data(&self) -> &'a [u8] {
        &self.data[self.index..]
    }

    pub fn increment(&mut self, n: usize) {
        self.index += n;
    }
}

pub trait Decodable {
    fn decode(decoder: &mut Decoder) -> Self;
}

impl Decodable for u16 {
    fn decode(decoder: &mut Decoder) -> Self {
        let result = u16::from_le_bytes(decoder.data()[..2].try_into().unwrap());
        decoder.increment(2);
        result
    }
}

impl Decodable for u32 {
    fn decode(decoder: &mut Decoder) -> Self {
        let result = u32::from_le_bytes(decoder.data()[..4].try_into().unwrap());
        decoder.increment(4);
        result
    }
}

impl Decodable for SyntaxKind {
    fn decode(decoder: &mut Decoder) -> Self {
        u16::decode(decoder).into()
    }
}
