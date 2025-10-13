use super::command::Command;
use super::LOGGER;

#[derive(Eq, PartialEq)]
pub enum Token<'a> {
    Word(&'a str),
    Semicolon,
    Dollar,
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
        matches!(self.current(), Some(b';') | Some(b'$'))
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
            return Some(Token::Dollar);
        }

        self.start = self.index;

        while !self.is_eof() && !self.is_whitespace() && !self.is_special() {
            self.index += 1;
        }

        let word = unsafe { str::from_utf8_unchecked(&self.input.as_bytes()[self.start..self.index]) };

        Some(Token::Word(word))
    }
}

pub struct Environment {
    commands: &'static [Command]
    // ... - env
}

impl Environment {
    pub fn new(commands: &'static [Command]) -> Self {
        Self { commands }
    }

    fn run_command(&self, args: &super::types::Arguments) -> Option<i8> {
        if args.is_empty() {
            return None;
        }

        if args[0] == "help" {
            for cmd in self.commands {
                crate::info!("{} - {}", cmd.name, cmd.help);
            }
            return Some(0);
        }

        let mut result = None;

        for cmd in self.commands {
            if cmd.name == args[0] {
                result = Some((cmd.handler)(&args.as_slice()[1..]));
                break;
            }
        }

        if matches!(result, None) {
            crate::warn!("Unknown command: {}", args[0]);
        }

        result
    }

    pub fn run(&self, src: &str) {
        let mut tokens = TokenizedIterator::new(src);

        while !tokens.is_eof() {
            let mut args = super::types::Arguments::new();

            while let Some(token) = tokens.next() {
                match token {
                    Token::Word(word) => {
                        args.push(word).unwrap();
                    }
                    Token::Semicolon => {
                        break;
                    }
                    Token::Dollar => {
                        todo!()
                    }
                }
            }

            self.run_command(&args);
        }
    }
}

