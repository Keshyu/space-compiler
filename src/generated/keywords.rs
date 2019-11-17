use crate::token::TokenType::{self, *};

pub static KEYWORDS: phf::Map<&'static str, TokenType> =::phf::Map {
    key: 3213172566270843353,
    disps: ::phf::Slice::Static(&[
        (0, 0),
    ]),
    entries: ::phf::Slice::Static(&[
        ("let", LET),
    ]),
};
