use std::ops::Range;

use logos::Logos;

use crate::{
    token::TokenKind,
    util::{u32_to_usize, Id},
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ByteIndex(pub u32);

impl Id for ByteIndex {
    fn from_usize(n: usize) -> Option<Self> {
        match n.try_into() {
            Ok(k) => Some(Self(k)),
            Err(_) => None,
        }
    }

    fn to_usize(self) -> usize {
        u32_to_usize(self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ByteLen(pub u16);

impl Id for ByteLen {
    fn from_usize(n: usize) -> Option<Self> {
        match n.try_into() {
            Ok(k) => Some(Self(k)),
            Err(_) => None,
        }
    }

    fn to_usize(self) -> usize {
        self.0.into()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub start: ByteIndex,
    pub len: ByteLen,
    pub kind: TokenKind,
}

impl Token {
    pub fn byte_range(&self) -> Range<usize> {
        let start = self.start.to_usize();
        start..(start + self.len.to_usize())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TokenId(pub u32);

impl Id for TokenId {
    fn from_usize(n: usize) -> Option<Self> {
        match n.try_into() {
            Ok(k) => Some(Self(k)),
            Err(_) => None,
        }
    }

    fn to_usize(self) -> usize {
        u32_to_usize(self.0)
    }
}

#[derive(Debug)]
pub struct Tokens {
    tokens: Vec<Token>,
}

impl Tokens {
    pub fn iter(&self) -> impl Iterator<Item = Token> + '_ {
        self.tokens.iter().copied()
    }
}

#[derive(Debug)]
pub enum LexError {
    SourceTooLong,
    TokenTooLong { start: ByteIndex, end: ByteIndex },
    InvalidToken { start: ByteIndex, len: ByteLen },
}

impl LexError {
    pub fn byte_range(&self) -> Range<usize> {
        match *self {
            LexError::SourceTooLong => {
                let max = ByteIndex(u32::MAX).to_usize();
                max..max
            }
            LexError::TokenTooLong { start, end } => start.to_usize()..end.to_usize(),
            LexError::InvalidToken { start, len } => {
                let start = start.to_usize();
                start..(start + len.to_usize())
            }
        }
    }

    pub fn message(&self) -> &str {
        match self {
            LexError::SourceTooLong { .. } => "file size exceeds 4 GiB limit",
            LexError::TokenTooLong { .. } => "token size exceeds 64 KiB limit",
            LexError::InvalidToken { .. } => "invalid token",
        }
    }
}

pub fn lex(source: &str) -> Result<Tokens, LexError> {
    let eof = match u32::try_from(source.len()) {
        Ok(len) => Token {
            start: ByteIndex(len),
            len: ByteLen(0),
            kind: TokenKind::Eof,
        },
        Err(_) => return Err(LexError::SourceTooLong),
    };
    let mut tokens = Vec::new();
    for (result, range) in TokenKind::lexer(source).spanned() {
        let start = ByteIndex::from_usize(range.start)
            .expect("file size limit should ensure all token starts are in range");
        let end = ByteIndex::from_usize(range.end)
            .expect("file size limit should ensure all token ends are in range");
        let len = ByteLen(
            (end.0 - start.0)
                .try_into()
                .map_err(|_| LexError::TokenTooLong { start, end })?,
        );
        let kind = result.map_err(|_| LexError::InvalidToken { start, len })?;
        tokens.push(Token { start, len, kind });
    }
    tokens.push(eof);
    Ok(Tokens { tokens })
}
