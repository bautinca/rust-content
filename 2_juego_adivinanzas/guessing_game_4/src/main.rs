/*
Ahora agregamos bucle para que en cada adivinanza incorrecta el usuario pueda reintentar
y ademas si el usuario adivina correctamente entonces el juego finaliza
*/

use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    // Aca colocamos el bucle
    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Modificamos tambien aca, si el usuario ingresa una entrada no valida no hacemos
        // que corte la ejecucion del programa sino que hacemos que el usuario reintente ingresando una nueva entrada en caso
        // de error
        // Pasamos de una llamada expect a una expresion match para pasar de bloquear el programa en un error a manejar el mismo error.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // Es caso de que devuelva el tipo de Result Ok entonces devolvemos el numero
            Err(_) => continue, // Si el usuario ingresa una entrada no valida intentamos un nuevo ciclo
        };

        println!("Your guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // El programa finaliza cuando el usuario adivino el numero
                break;
            }
        }
    }
}

// Y LISTO YA TENEMOS EL JUEGO DE ADIVINANZAS!!