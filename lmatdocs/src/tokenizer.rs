#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    BegBoldItalics,
    EndBoldItalics,
    BegUnderline,
    EndUnderline,
    BegBold,
    EndBold,
    BegItalics,
    EndItalics,
    NewCentered,
    NewParagraph,
    NewBullet,
    NewSubBullet,
    Text(&'a str),
}

pub struct Tokenizer<'a> {
    chars: &'a str,
    pos: usize, // the position of the next utf8-char to be read
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            chars: input,
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token<'a>>, String> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::with_capacity(3);
        let mut back_pos: usize = 0;

        while self.pos < self.chars.len() {
            let ch: char = self._next_ch().expect("no next char");
            if ch == '#' {
                /* centered paragraph */
                if self.pos > 1 {
                    self._push_buffer(&mut tokens, back_pos, self.pos - 1);
                }
                back_pos = self.pos;
                if self._peek_ch() == Some(' ') {
                    back_pos += 1;
                }
                if let Some(Token::NewParagraph) = tokens.last() {
                    tokens.pop();
                }
                tokens.push(Token::NewCentered);
            } else if ch == '~' {
                /* bullet points */
                let len: usize = self._count_consec('~');
                back_pos += len;
                if self.pos > len + 1 {
                    self._push_buffer(&mut tokens, back_pos, self.pos - len - 1);
                }
                back_pos = self.pos;
                if let Some(Token::NewParagraph) = tokens.last() {
                    tokens.pop();
                }
                match len {
                    0 => tokens.push(Token::NewBullet),
                    1 => tokens.push(Token::NewSubBullet),
                    _ => {
                        return Err("Syntax Error: too many ~".to_string());
                    }
                }
            } else if ch == '\n' {
                /* default paragraph */
                if self.pos > 1 {
                    self._push_buffer(&mut tokens, back_pos, self.pos - 1);
                }
                back_pos = self.pos;
                tokens.push(Token::NewParagraph);
            } else if ch == '*' {
                /* bold and italic */
                let len: usize = self._count_consec('*');
                if self.pos > len + 1 {
                    self._push_buffer(&mut tokens, back_pos, self.pos - len - 1);
                }
                back_pos = self.pos;

                /* check for opening tokens in the stack */
                if let Some(Token::BegItalics) = stack.last() {
                    if len != 0 {
                        return Err("Syntax Error: Expected *".to_string());
                    }
                    tokens.push(Token::EndItalics);
                    stack.pop();
                } else if let Some(Token::BegBold) = stack.last() {
                    if len != 1 {
                        return Err("Syntax Error: Expected **".to_string());
                    }
                    tokens.push(Token::EndBold);
                    stack.pop();
                } else if let Some(Token::BegBoldItalics) = stack.last() {
                    if len != 2 {
                        return Err("Syntax Error: Expected ***".to_string());
                    }
                    tokens.push(Token::EndBoldItalics);
                    stack.pop();
                } else {
                    match len {
                        0 => {
                            stack.push(Token::BegItalics);
                            tokens.push(Token::BegItalics);
                        }
                        1 => {
                            stack.push(Token::BegBold);
                            tokens.push(Token::BegBold);
                        }
                        2 => {
                            stack.push(Token::BegBoldItalics);
                            tokens.push(Token::BegBoldItalics);
                        }
                        _ => {
                            return Err("Syntax Error: too many *".to_string());
                        }
                    }
                }
            } else if ch == '_' {
                /* underlined */
                if self.pos > 1 {
                    self._push_buffer(&mut tokens, back_pos, self.pos - 1);
                }
                back_pos = self.pos;

                if let Some(Token::BegUnderline) = stack.last() {
                    tokens.push(Token::EndUnderline);
                    stack.pop();
                } else {
                    stack.push(Token::BegUnderline);
                    tokens.push(Token::BegUnderline);
                }
            }
        }
        if back_pos < self.pos - 1 {
            tokens.push(Token::Text(&self.chars[back_pos..]));
        }
        Ok(tokens)
    }

    fn _push_buffer(&self, tokens: &mut Vec<Token<'a>>, back_pos: usize, pos: usize) {
        if back_pos < pos - 1 {
            tokens.push(Token::Text(&self.chars[back_pos..pos]));
        }
    }

    fn _count_consec(&mut self, character: char) -> usize {
        let mut count = 0;
        let mut current_pos = self.pos;

        while current_pos < self.chars.len() {
            let remainder = &self.chars[current_pos..];
            if let Some(ch) = remainder.chars().next() {
                if ch == character {
                    current_pos += ch.len_utf8();
                    count += 1;
                    continue;
                }
            }
            break;
        }
        self.pos = current_pos;
        count
    }

    fn _next_ch(&mut self) -> Option<char> {
        let ch = self.chars[self.pos..].chars().next().unwrap();
        self.pos += ch.len_utf8(); // string slicing occur on byte level
        Some(ch)
    }

    fn _peek_ch(&self) -> Option<char> {
        self.chars[self.pos..].chars().next()
    }
}
