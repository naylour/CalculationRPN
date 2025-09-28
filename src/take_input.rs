use std::io;

pub fn take_input() -> String {
    let mut input = String::new();

    println!("Введите выражение: ");

    io::stdin().read_line(&mut input).expect("Ошибка при вводе");

    input.trim().to_string()
}
