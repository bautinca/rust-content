/*
A continuacion en la parte 2 de este juego necesitamos generar un num. secreto que el usuario intentara adivinar. El numero secreto debe ser
diferente cada vez para que el juego sea divertido de jugar mas de una vez. Usamos num aleatorio entre 1 y 100. 

Rust no incluye la funcionalidad de numeros aleatorios en su biblioteca estandar sin embargo EXISTE UN CRATE EN crates.io CON DICHA FUNCIONALIDAD!
El crate se llama rand

Antes de escribir codigo que use rand necesitamos modificar el archivo Cargo.toml para incluir el crate rand como una dependencia del proyecto.
Entonces vamos a agregar dicha dependencia en Cargo.toml donde especificamos el nombre del crate y la version que usaremos del crate. En este caso usamos el crate 'rand'
de version 0.8.5

Entonces ahora usaremos rand para generar un numero aleatorio entre 1 y 100
*/

use std::io;
// Usamos la biblioteca rnd donde el trait Rng define un conjunto de metodos que los generadores de numeros aleatorios implementan y ese trait
// debe estar en el alcance para que podamos usar esos metodos (mas adelante hablaremos sobre traits)
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Generamos el numero aleatorio de la biblioteca rand el metodo thread_rng(), este metodo nos permite generar un generador
    // de numeros aleatorios y luego llamamos al metodo
    // gen_range() que es para generar un nuemro aleatorio segun el rango dado donde este metodo esta definido en el trait Rng que traemos
    // al alcance con la declaracion use rand::Rng;. El metodo gen_range toma una expresion de rango como argumento y gneera un numero aleatorio en el rango
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Imprimimos el numero secreto
    println!("The secret number is: {secret_number}");

    println!("Please input your guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("Your guessed: {guess}");
}