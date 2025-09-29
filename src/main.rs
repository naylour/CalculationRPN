mod expression;
mod take_input;

use take_input::take_input;

use expression::Expression;

fn main() {
    let input = take_input();

    let mut expression = Expression::new();

    match expression.take_tokens(input.as_str()) {
        Ok(tokens) => {
            print!("Токенизация:");
            for token in &tokens.items {
                print!(" {}", token);
            }
            println!(";");
        }
        Err(e) => {
            eprintln!("Ошибка токенизации: {}", e);
            return;
        }
    }

    let mut rpn_tokens = match expression.to_rpn() {
        Ok(tokens) => {
            print!("Перевод в польскую нотацию:");
            for token in &tokens.items {
                print!(" {}", token);
            }
            println!(";");

            tokens
        }
        Err(e) => {
            eprintln!("Ошибка перевода в польскую нотацию: {}", e);
            return;
        }
    };

    let result = match rpn_tokens.calculate() {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Ошибка при вычислении: {}", e);
            return;
        }
    };

    println!("Ответ: {};", result);
}
