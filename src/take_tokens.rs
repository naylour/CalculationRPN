use std::char;

use crate::operator::{Operator, Token, char_to_operator, operator_to_char};

pub fn take_tokens(expr: &str) -> Result<Vec<Token>, String> {
    let expr = expr.trim();
    let chars: Vec<char> = expr.chars().collect();

    let mut tokens: Vec<Token> = Vec::new();

    let mut i = 0;
    let mut expect_number = true;
    let mut prev_was_operator = false;
    let mut unary_pending: Option<Operator> = None;

    while i < chars.len() {
        let c = chars[i];

        match c {
            '(' => {
                tokens.push(Token::LParen);
                expect_number = true;
                i += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                expect_number = false;
                i += 1;
            }
            _ => {
                if c.is_whitespace() {
                    i += 1;
                    continue;
                }

                if c.is_digit(10) || c == '.' {
                    prev_was_operator = false;

                    if !expect_number && unary_pending.is_none() {
                        return Err(format!("Два числа подряд недопустимы на позиции {}", i));
                    }

                    let mut num_str = String::new();
                    if let Some(u) = unary_pending.take() {
                        num_str.push(operator_to_char(u));
                    }

                    while i < chars.len() && (chars[i].is_digit(10) || chars[i] == '.') {
                        num_str.push(chars[i]);
                        i += 1;
                    }

                    let num: f64 = num_str
                        .parse()
                        .map_err(|_| format!("Некорректное число: {}", num_str))?;

                    tokens.push(Token::Number(num));
                    expect_number = false;
                    continue;
                }

                if "+-*/".contains(c) {
                    if expect_number {
                        if c == '-' || c == '+' {
                            if unary_pending.is_some() {
                                return Err(format!(
                                    "Два унарных подряд недопустимы на позиции {}",
                                    i
                                ));
                            }
                            unary_pending = Some(char_to_operator(c).unwrap());
                        } else {
                            return Err(format!(
                                "Бинарный оператор '{}' не может стоять на месте числа",
                                c
                            ));
                        }
                    } else {
                        if unary_pending.is_some() {
                            return Err(format!(
                                "Унарный оператор перед бинарным недопустим на позиции {}",
                                i
                            ));
                        }
                        if prev_was_operator {
                            return Err(format!(
                                "Два бинарных оператора подряд недопустимы на позиции {}",
                                i
                            ));
                        }
                        tokens.push(Token::Op(char_to_operator(c).unwrap()));
                        prev_was_operator = true;
                        expect_number = true;
                    }
                    i += 1;
                    continue;
                }

                return Err(format!("Неподдерживаемый символ '{}' на позиции {}", c, i));
            }
        }
    }

    if expect_number && unary_pending.is_some() {
        return Err("Выражение не может заканчиваться унарным оператором".into());
    }

    Ok(tokens)
}
