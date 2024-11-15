use logos::Logos;

#[derive(Clone, Copy, Debug, Logos)]
#[logos(skip r"\s+")]
pub enum TokenKind {
    Eof,

    #[regex("//[^\n]*")]
    Comment,

    #[token("abstract")]
    Abstract,

    #[token("as")]
    As,

    #[token("async")]
    Async,

    #[token("await")]
    Await,

    #[token("become")]
    Become,

    #[token("box")]
    Box,

    #[token("break")]
    Break,

    #[token("const")]
    Const,

    #[token("continue")]
    Continue,

    #[token("crate")]
    Crate,

    #[token("do")]
    Do,

    #[token("dyn")]
    Dyn,

    #[token("else")]
    Else,

    #[token("enum")]
    Enum,

    #[token("extern")]
    Extern,

    #[token("false")]
    False,

    #[token("final")]
    Final,

    #[token("fn")]
    Fn,

    #[token("for")]
    For,

    #[token("if")]
    If,

    #[token("impl")]
    Impl,

    #[token("in")]
    In,

    #[token("let")]
    Let,

    #[token("loop")]
    Loop,

    #[token("macro")]
    Macro,

    #[token("match")]
    Match,

    #[token("mod")]
    Mod,

    #[token("move")]
    Move,

    #[token("mut")]
    Mut,

    #[token("override")]
    Override,

    #[token("pub")]
    Pub,

    #[token("priv")]
    Priv,

    #[token("ref")]
    Ref,

    #[token("return")]
    Return,

    #[token("self")]
    SelfValue,

    #[token("Self")]
    SelfType,

    #[token("static")]
    Static,

    #[token("struct")]
    Struct,

    #[token("super")]
    Super,

    #[token("trait")]
    Trait,

    #[token("true")]
    True,

    #[token("try")]
    Try,

    #[token("type")]
    Type,

    #[token("typeof")]
    Typeof,

    #[token("unsafe")]
    Unsafe,

    #[token("unsized")]
    Unsized,

    #[token("use")]
    Use,

    #[token("virtual")]
    Virtual,

    #[token("where")]
    Where,

    #[token("while")]
    While,

    #[token("yield")]
    Yield,

    #[regex(r"[A-Za-z]\w*")]
    Identifier,

    #[regex(r#""[^"]*""#)]
    String,

    #[regex(r"\d+")]
    Integer,

    #[regex(r"'\w+")]
    Lifetime,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("%")]
    Percent,

    #[token("^")]
    Caret,

    #[token("!")]
    Not,

    #[token("&")]
    And,

    #[token("|")]
    Or,

    #[token("&&")]
    AndAnd,

    #[token("||")]
    OrOr,

    #[token("<<")]
    Shl,

    #[token(">>")]
    Shr,

    #[token("+=")]
    PlusEq,

    #[token("-=")]
    MinusEq,

    #[token("*=")]
    StarEq,

    #[token("/=")]
    SlashEq,

    #[token("%=")]
    PercentEq,

    #[token("^=")]
    CaretEq,

    #[token("|=")]
    OrEq,

    #[token("<<=")]
    ShlEq,

    #[token(">>=")]
    ShrEq,

    #[token("=")]
    Eq,

    #[token("==")]
    EqEq,

    #[token("!=")]
    Ne,

    #[token(">")]
    Gt,

    #[token("<")]
    Lt,

    #[token(">=")]
    Ge,

    #[token("<=")]
    Le,

    #[token("@")]
    At,

    #[token("_")]
    Underscore,

    #[token(".")]
    Dot,

    #[token("..")]
    DotDot,

    #[token("...")]
    DotDotDot,

    #[token("..=")]
    DotDotEq,

    #[token(",")]
    Comma,

    #[token(";")]
    Semi,

    #[token(":")]
    Colon,

    #[token("::")]
    PathSep,

    #[token("->")]
    RArrow,

    #[token("=>")]
    FatArrow,

    #[token("<-")]
    LArrow,

    #[token("#")]
    Pound,

    #[token("$")]
    Dollar,

    #[token("?")]
    Question,

    #[token("~")]
    Tilde,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,
}
