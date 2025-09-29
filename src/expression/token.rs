use crate::expression::operator::Operator;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),
    Op(Operator),

    LParen,
    RParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => {
                // число будет выводиться как целое, если дробной части нет
                if n.fract() == 0.0 {
                    write!(f, "{:.0}", n)
                } else {
                    write!(f, "{}", n)
                }
            }
            Token::Op(op) => write!(f, "{}", op),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
        }
    }
}
