use compiler_module::lexer::{
    Lexer as LexerTrait,
    lex_manager::LexManager,
};
use crate::token::{Token, TokenType::{self, *}};
use crate::util::char::*;
use crate::generated::{
    keywords::KEYWORDS,
    escaped_chars::ESCAPED_CHARS,
};
use super::context::Context;


pub struct Lexer<'a> {
    manager: LexManager<'a>,
    context: Context,
}

impl<'a> LexerTrait for Lexer<'a> {
    type Token = Token;
    type TokenType = TokenType;

    fn lex(&mut self, valid_tokens: Vec<TokenType>) -> Option<Token> {
        let mut has_newline = false;
        let mut indent_len: u16 = 0;

        if valid_tokens.contains(&BEGIN) && self.context.indents.is_empty() {
            self.context.indents.push(0);
            return Some(Token::Symbol(BEGIN));
        }

        loop {
            if self.manager.check('\n') {
                self.manager.advance();
                has_newline = true;
            }
            else if self.manager.check(' ') {
                self.manager.advance();
                if has_newline { indent_len += 1; }
            }
            else if self.manager.check('\t') {
                self.manager.advance();
                if has_newline { indent_len += 8; }
            }
            else {
                break;
            }
        }

        if has_newline {
            if valid_tokens.contains(&NEWLINE) {
                return Some(Token::Symbol(NEWLINE));
            }

            if indent_len > *self.context.indents.last().unwrap()
               && valid_tokens.contains(&BEGIN)
            {
                self.context.indents.push(indent_len);
                return Some(Token::Symbol(BEGIN))
            }

            if indent_len < *self.context.indents.last().unwrap()
               && valid_tokens.contains(&END)
            {
                // FIXME: Check for indent_len to be equal to at least one indent
                self.context.indents.pop();
                return Some(Token::Symbol(END))
            }
        }

        if self.manager.check_if(is_alpha) {
            return self.name();
        }
        else if self.manager.check_if(is_digit) {
            return self.number();
        }
        else if self.manager.check('"') {
            println!("String found on: {:?}", valid_tokens);
            return self.string();
        }
        else if self.manager.check('\'') {
            return self.character();
        }
        else if self.manager.check('=') {
            self.manager.advance();
            return Some(Token::Symbol(EQ));
        }

        if valid_tokens.contains(&END) && self.context.indents.len() == 1 {
            return Some(Token::Symbol(END));
        }

        None
    }
}

impl<'a> Lexer<'a> {
    pub fn new(source_str: &'a str) -> Lexer<'a> {
        let mut source = source_str.chars();

        Lexer {
            manager: LexManager {
                lookahead: source.next(),
                source: source,
            },
            context: Context {
                indents: Vec::new(),
            },
        }
    }

    fn name(&mut self) -> Option<Token> {
        let rec = self.manager.record(|this, recorder| {
            recorder.advance(this);

            while this.check_if(is_alnum)
            { recorder.advance(this); }

            while this.check('\'')
            { recorder.advance(this); }
        });

        if let Some(tt) = KEYWORDS.get(rec.as_str()).cloned() {
            println!("Keyword found: {}", tt);
            Some(Token::Symbol(tt))
        } else {
            println!("Name found: {}", rec);
            Some(Token::Name(rec))
        }
    }

    // TODO: Support for negative and positive numbers
    fn number(&mut self) -> Option<Token> {
        let rec = self.manager.record(|this, recorder| {
            recorder.advance(this);

            while this.check_if(is_digit)
                { recorder.advance(this); }
        });

        Some(Token::Number(rec.parse().unwrap()))
    }

    fn string(&mut self) -> Option<Token> {
        let rec = self.manager.record(|this, recorder| {
            this.advance();

            while {
                recorder.advance(this);

                !this.check('"')
            } { }
        });

        self.manager.advance();

        println!("String found: {}", rec);

        Some(Token::String(rec))
    }

    fn character(&mut self) -> Option<Token> {
        let mut rec = self.manager.record(|this, recorder| {
            this.advance();

            if this.check('\\') {
                recorder.advance(this);

                if let Some(lookahead) = this.lookahead {
                    if let Some(c) = ESCAPED_CHARS.get(&lookahead) {
                        recorder.push(*c);
                        this.advance();
                    } else {
                        // TODO: Better error message
                        panic!("Unexpected character: {}", lookahead)
                    }
                } else {
                    panic!("LexError: Unterminated character literal")
                }
            } else {
                recorder.advance(this);
            }

            if this.check('\'') {
                this.advance();
            } else {
                panic!("Character literal must contain only one character")
            }
        });

        Some(Token::Char(rec.pop().unwrap()))
    }
}
