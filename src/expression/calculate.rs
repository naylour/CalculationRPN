use super::{
    Expression, Token,
    operator::{HighPriority, LowPriority, Operator},
    stack::Stack,
};

impl Expression {
    pub fn calculate(&mut self) -> Result<f64, String> {
        let mut stack: Stack<f64> = Stack::new();

        if !self.is_rpn {
            match self.to_rpn() {
                Ok(expr) => {
                    self.items = expr.items;
                }

                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        for token in &self.items {
            match token {
                Token::Number(n) => stack.push(*n),
                Token::Op(op) => {
                    if stack.size() < 2 {
                        return Err("Недостаточно операндов для оператора".into());
                    }

                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();

                    let result = match op {
                        Operator::Low(LowPriority::Add) => a + b,
                        Operator::Low(LowPriority::Sub) => a - b,
                        Operator::High(HighPriority::Mul) => a * b,
                        Operator::High(HighPriority::Div) => {
                            if b == 0.0 {
                                return Err("Деление на ноль".into());
                            }

                            a / b
                        }
                    };

                    stack.push(result);
                }
                _ => return Err("Недопустимый токен RPN".into()),
            }
        }

        if stack.size() != 1 {
            return Err("Некорректное выражение RPN".into());
        }

        Ok(stack.pop().unwrap())
    }
}
