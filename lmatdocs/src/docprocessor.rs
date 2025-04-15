use super::parser::*;
use super::tokenizer::*;

pub struct DocProcessor<'input> {
    input: &'input str,
}

impl<'input> DocProcessor<'input> {
    pub fn new(input: &'input str) -> Self {
        DocProcessor { input }
    }

    pub fn process(&mut self) -> Result<Vec<Paragraph<'input>>, ()> {
        let mut tokenizer = Tokenizer::new(self.input);
        let tokens = tokenizer.tokenize().expect("fatal err in tokenizer");
        let mut parser = Parser::new(tokens.clone());
        Ok(parser.parse())
    }
}
