use std::io;

fn main() {
    let mut entrada = String::new();
    println!("Digite um numero:");
    io::stdin().read_line(&mut entrada).unwrap();

    let numero: i32 = entrada.trim().parse().unwrap();

    for i in 1..=10 {
        println!("{} x {} = {}", numero, i, numero * i);
    }
}
