use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("----ADIVINA EL NUMERO!----");

    let secret_number = rand::rng().random_range(1..=100);

    println!("secret number: {}", secret_number);

    loop {
        println!("----------------------------------------");
        println!("porfavor introduce tu valor a adivinar: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("no se pudo leer el valor");

        println!("tu valor adivinado es: {}", guess);

        let guess: i32 = guess.trim().parse().expect("Error");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("el numero es pequeño, sigue adivinando!"),
            Ordering::Greater => println!("el numero es grande, sigue adivinando!"),
            Ordering::Equal => {
                println!("Ganastes has adivinado el numero!!!");
                break;
            }
        }
    }
}
