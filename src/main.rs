use std::{thread, thread::{JoinHandle}, io};

const MIN_SIZE: i32 = 0;

// понимаю что подход с паникой плохой, если вы против `expect`
// то дайте пожалуйста знать
fn main() {
    let mut input= String::new();

    // читаем из `stdin`
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    // валидируем `input` из stdin и проводим к типу i32
    let number: usize = input.trim().parse().expect("input not an integer");
    if number as i32 <= MIN_SIZE {
        println!("number should be greater {}", MIN_SIZE);
        return;
    }

    // создаем вектора для результата квадратов числа
    // создаем вектор JoinHandle для того чтобы дождаться всех тредов
    let mut results: Vec<usize> = (1..=number).collect();
    let handlers: Vec<JoinHandle<usize>> = (1..=results.len()).map(|i| {
        thread::spawn(move || {
            let multiply = i * i;
            println!(" {} * {} = {}",i, i, multiply);
            multiply
        })
    }).collect();


    // ждем все треды и записываем результат в вектор
    for (index, handler) in handlers.into_iter().enumerate() {
        let multiply = handler.join().unwrap();
        results[index] = multiply
    }
}