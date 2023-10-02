use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    let max_tryes: i32 = 5;

    println!("Escolha um número de 1 à 10");
    println!("Você tem {} tentativas", max_tryes);

    let secret_number = rand::thread_rng().gen_range(1..=10);
    let mut tryes: i32 = 0;

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Falha ao ler a linha");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        tryes += 1;

        if max_tryes == tryes {
            println!("Você perdeu!");
            break;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                println!("Tentativas {} de {} tentativas", tryes, max_tryes);
                break;
            },
        }
    }

}
