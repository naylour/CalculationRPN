mod token;
mod operator;
mod take_tokens;
mod to_rpn;
mod stack;
mod calculate;

use token::Token;

#[derive(Debug, Clone)]
pub struct Expression {
    pub items: Vec<Token>,
    pub is_rpn: bool
}


impl Expression {
    pub fn new() -> Self {
        Expression { items: Vec::new(), is_rpn: true }
    }

    pub fn from(items: Vec<Token>, is_rpn: bool) -> Expression {
        Expression { items, is_rpn }
    }
}
