use crate::operator::{Operator, Token};

use crate::stack::Stack;

#[derive(Debug, Clone, Copy)]
enum StackItem {
    Op(Operator),
    LParen,
}

impl StackItem {
    fn as_op(&self) -> Option<Operator> {
        match self {
            StackItem::Op(op) => Some(*op),
            StackItem::LParen => None,
        }
    }
}

pub fn to_rpn(input: &[Token]) -> Result<Vec<Token>, String> {
    let mut operators = Stack::<StackItem>::new();
    let mut output: Vec<Token> = Vec::new();

    for token in input {
        match token {
            Token::Number(n) => output.push(Token::Number(*n)),

            Token::Op(op) => {
                while let Some(StackItem::Op(top_op)) = operators.peek() {
                    if top_op.precedence() >= op.precedence() {
                        output.push(Token::Op(operators.pop().unwrap().as_op().unwrap()));
                    } else {
                        break;
                    }
                }
                operators.push(StackItem::Op(*op));
            }

            Token::LParen => operators.push(StackItem::LParen),

            Token::RParen => {
                // выталкиваем операторы до '('
                while let Some(top) = operators.peek() {
                    match top {
                        StackItem::Op(_) => {
                            output.push(Token::Op(operators.pop().unwrap().as_op().unwrap()));
                        }
                        StackItem::LParen => {
                            operators.pop(); // снимаем '('
                            break;
                        }
                    }
                }
            }
        }
    }

    // выталкиваем оставшиеся операторы
    while let Some(item) = operators.pop() {
        match item {
            StackItem::Op(op) => output.push(Token::Op(op)),
            StackItem::LParen => return Err("Несбалансированные скобки".into()),
        }
    }

    Ok(output)
}
