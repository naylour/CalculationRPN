mod calculate_rpn;
mod operator;
mod stack;
mod take_input;
mod take_tokens;
mod to_rpn;

use take_input::take_input;
use take_tokens::take_tokens;
use to_rpn::to_rpn;

use crate::calculate_rpn::calculate_rpn;

fn main() {
    let input = take_input();

    let tokens = match take_tokens(input.as_str()) {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Ошибка токенизации: {}", e);
            return;
        }
    };

    let rpn_tokens = match to_rpn(&tokens) {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Ошибка перевода в польскую нотацию: {}", e);
            return;
        }
    };

    let result = calculate_rpn(&rpn_tokens);

    println!("{:?}", result);
}
