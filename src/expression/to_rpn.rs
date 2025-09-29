use super::{operator::{Operator}, Token, Expression, stack::Stack};

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

impl Expression {
    pub fn to_rpn(&self) -> Result<Expression, String> {
        let mut operators = Stack::<StackItem>::new();
        let mut tokens: Vec<Token> = Vec::new();

        for token in &self.items {
            match token {
                Token::Number(n) => tokens.push(Token::Number(*n)),

                Token::Op(op) => {
                    while let Some(StackItem::Op(top_op)) = operators.peek() {
                        if top_op.precedence() >= op.precedence() {
                            tokens.push(Token::Op(operators.pop().unwrap().as_op().unwrap()));
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
                                tokens.push(Token::Op(operators.pop().unwrap().as_op().unwrap()));
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
                StackItem::Op(op) => tokens.push(Token::Op(op)),
                StackItem::LParen => return Err("Несбалансированные скобки".into()),
            }
        }

        Ok(Expression { items: tokens, is_rpn: true })
    }
}
