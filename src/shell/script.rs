use super::types::{Arguments, EnvVar};
use super::command::Command;
use super::env::Environment;
use super::LOGGER;

use core::fmt::Write;

/*
TODO: Implement ENV
TODO: Expand tokens & parsing
TODO: Maybe add if/else
TODO: Maybe add while
TODO: Maybe add functions (fn)
TODO: All feature must be on/off configurable
*/

#[derive(Eq, PartialEq)]
pub enum Token<'a> {
    Word(&'a str),
    Dollar(&'a str),
    Semicolon,

    And,
    Or,
}

// TODO: Idea: pass Tokenizer as Arguments to each command handler as an iterator
//             and if EOF or ';' is encountered, return None, signifying the end of input
pub struct TokenizedIterator<'a> {
    input: &'a str,
    index: usize,
    start: usize,
}

impl<'a> TokenizedIterator<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            index: 0,
            start: 0,
        }
    }

    fn current(&self) -> Option<u8> {
        self.input.as_bytes().get(self.index).map(|v| *v)
    }

    fn is_eof(&self) -> bool {
        self.index >= self.input.len()
    }

    fn is_whitespace(&self) -> bool {
        matches!(self.current(), Some(b' ') | Some(b'\t'))
    }

    fn is_special(&self) -> bool {
        matches!(self.current(), Some(b';') | Some(b'$') | Some(b'&') | Some(b'|'))
    }

    fn tokenize_next_word(&mut self) -> Option<&'a str> {
        self.start = self.index;

        while !self.is_eof() && !self.is_whitespace() && !self.is_special() {
            self.index += 1;
        }

        if self.start == self.index {
            return None;
        }

        let word = unsafe { str::from_utf8_unchecked(&self.input.as_bytes()[self.start..self.index]) };

        Some(word)
    }
}

impl<'a> Iterator for TokenizedIterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_eof() {
            return None;
        }

        while self.is_whitespace() {
            self.index += 1;

            if self.is_eof() {
                return None;
            }
        }

        if matches!(self.current(), Some(b';')) {
            self.index += 1;
            return Some(Token::Semicolon);
        }

        if matches!(self.current(), Some(b'$')) {
            self.index += 1;
            return self.tokenize_next_word().map(|w| Token::Dollar(w));
        }

        if matches!(self.current(), Some(b'&')) {
            self.index += 1;
            return Some(Token::And);
        }

        if matches!(self.current(), Some(b'|')) {
            self.index += 1;
            return Some(Token::Or);
        }

        self.tokenize_next_word().map(|w| Token::Word(w))
    }
}

pub struct Runtime {
    pub commands: &'static [Command],
    pub env:      Environment,
}

impl Runtime {
    pub fn new(commands: &'static [Command]) -> Self {
        Self { commands, env: Environment::new() }
    }

    fn run_command(&mut self, args: &Arguments) -> Option<i8> {
        if args.is_empty() {
            return None;
        }

        let mut result = None;

        for cmd in self.commands {
            if cmd.name == args[0] {
                result = Some((cmd.handler)(self, &args.as_slice()[1..]));
                break;
            }
        }

        if matches!(result, None) {
            crate::warn!("Unknown command: {}", args[0]);
        }

        result
    }

    pub fn run(&mut self, src: &str) {
        let mut tokens = TokenizedIterator::new(src);

        let mut condition = None;

        while !tokens.is_eof() {
            let mut args = Arguments::new();

            while let Some(token) = tokens.next() {
                match token {
                    Token::Word(word) => {
                        let _ = args.push(word);
                    }
                    Token::Dollar(name) => {
                        let val = self.env.get(name).unwrap_or("");

                        // FIXME: Nasty workaround to make `self.env` possible to pass to command
                        //        handlers. Only danger this presents, is that if command handler
                        //        blindly writes to `env` and then reads `args`, which might
                        //        contain a variable from `env` - the args become invalid
                        let _ = args.push(unsafe { &*(val as *const _) });
                    }
                    Token::Semicolon => {
                        break;
                    }
                    Token::And => {
                        condition = Some(Token::And);
                        break;
                    }
                    Token::Or => {
                        condition = Some(Token::Or);
                        break;
                    }
                }
            }

            if let Some(res) = self.run_command(&args) {
                let mut s = EnvVar::new();
                let _ = write!(&mut s, "{}", res);
                let _ = self.env.set("?", s.as_str());

                match condition {
                    Some(Token::And) => {
                        if res != 0 {
                            break;
                        }
                    }
                    Some(Token::Or) => {
                        if res == 0 {
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

