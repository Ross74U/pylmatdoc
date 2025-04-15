use super::tokenizer::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Format {
    pub italic: bool,
    pub bold: bool,
    pub underline: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Delimiter {
    Bullet,
    SubBullet,
    Centered,
    New,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Paragraph<'a> {
    pub delim: Delimiter,
    pub runs: Vec<Run<'a>>,
}

impl Paragraph<'_> {
    fn new(delim: Delimiter) -> Self {
        Paragraph {
            delim,
            runs: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Run<'a> {
    pub format: Format,
    pub text: &'a str,
}
impl<'a> Run<'a> {
    fn new(format: Format, text: &'a str) -> Self {
        Run { format, text }
    }
}

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser { tokens }
    }

    pub fn parse(&mut self) -> Vec<Paragraph<'a>> {
        let mut tree: Vec<Paragraph> = Vec::new();
        let mut buf = Format {
            bold: false,
            italic: false,
            underline: false,
        };
        let mut pg = Paragraph::new(Delimiter::New);

        for token in &self.tokens {
            match token {
                Token::Text(s) => pg.runs.push(Run::new(buf, s)),
                Token::NewParagraph => {
                    tree.push(pg.clone());
                    pg = Paragraph::new(Delimiter::New);
                }
                Token::NewCentered => {
                    tree.push(pg.clone());
                    pg = Paragraph::new(Delimiter::Centered);
                }
                Token::NewBullet => {
                    tree.push(pg.clone());
                    pg = Paragraph::new(Delimiter::Bullet);
                }
                Token::NewSubBullet => {
                    tree.push(pg.clone());
                    pg = Paragraph::new(Delimiter::SubBullet);
                }
                Token::BegUnderline => buf.underline = true,
                Token::BegBold => buf.bold = true,
                Token::BegItalics => buf.italic = true,
                Token::BegBoldItalics => {
                    buf.italic = true;
                    buf.bold = true;
                }
                Token::EndUnderline => buf.underline = false,
                Token::EndBold => buf.bold = false,
                Token::EndItalics => buf.italic = false,
                Token::EndBoldItalics => {
                    buf.italic = false;
                    buf.bold = false;
                }
            }
        }
        tree.push(pg.clone());
        tree
    }
}
