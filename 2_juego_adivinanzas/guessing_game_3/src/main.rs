/*
Ya tenemos el numero aleatorio generado y la entrada del usuario, ahora ambos numeros los podemos comparar
para ver si la adivinanza que dijo el usuario es mas alta, mas baja o es equivalente
*/

// Usamos de la biblioteca estandar (std) y nos traemos cmp::Ordering. El tipo Ordering es otro enum (asi como la String)
// que tiene las variantes Less, Greater y Equal. Estos son los 3 resultados posbiles cuando compara los valores
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Please input your guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    // El problema aca es que estariamos comparando en el match un string (guess)
    // con un entero (secret_number) cosa que no se puede comprar una cadena con un entero
    // por lo tanto lo tenemos que parsear lo que ingrese el usuario a un entero (ya damos por sentado que el usuario ingresara como string un entero)
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    // Este concepto de Rust se llama 'Shadowing' que es cuando nos permite redefinir el valor anterior
    // de guess con uno nuevo. Nos permite volver usar el nombre de la variable guess en lugar
    // de obligarnos a crear 2 variables unicas como guess_str y guess_int
    // entonces el trim() elimina cualquier espacio en blanco de la string, por ejemplo un "3   " lo pasa a "3"
    // O por ejemplo si el usuario en la terminal ingresa "3" en realidad seria un "3\n" ya que el usuario presiona enter y eso
    // genera un salto de linea en la terminal por lo tanto el trim() elimina el \n y lo deja en "3" tambien
    // Luego el metodo de String llamado parse() pasa una string a otro tipo de dato donde en este caso
    // seria un u32 (entero sin signo de 32 bits)

    // El metodo parse() logicamente funcionara en caracteres que se pueden convertir logicamente en numeros
    // como "3", "392", "323223", etc. por lo tanto es obvio que puede causar facilmente errores por ejemplo si el usuario ingresa "pepito"
    // En ese caso no habria manera de pasarlo a un numero por lo que podria lanzar un error.
    // Como ya es de suponer el metodo parse() devuelve un tipo de dato Result (igual que en read_line()) que es Ok o Err 
    // Si parse() entonces devuelve la variante Err de tipo Result es porque no pudo crear un numero a partir de la cadena entonces corresponde
    // colocar un expect() para hacer que el juego se bloquee y muestre e mensaje que le damos al usuario.
    // Si en cambio parse() puede convertir exitosamente una string a un entero entonces devolvera un Result de tipo Ok y expect() devolvera
    // el numero que queremos del valor Ok

    println!("Your guessed: {guess}");

    // Ya definidas las variables secret_number (inmutable) y guess (mutable) ahora las comparamos entre ambos para ver si lo q
    // dijo el usuario es mayor, igual o menor
    // el numero guess lo vamos a comparar con secret_number usando el metodo cmp() para comparar 2 numeros

    // Una expresion match esta compuesta por brazos. Un brazo consta de un patron para coincidir y el codigo que se debe ejecutar si el valor dado a match
    // se ejecuta al patron del brazo. Rust toma el valor dado en match y busca cada patron
    // de brazo en orden. Gracias a esto podemos expresar una variedad de situaciones que el codigo puede
    // encontrar y se aseguran de que los maneje todos
    match guess.cmp(&secret_number) {
        // El cmp() devuelve una variante del enum Ordering.
        // Digamos que el usuario ha adivinado 50 y el numero secreto generado aleatoriamente es 38
        // cuando el codigo compara 50 con 38 el metodo cmp() devolvera Ordering::Greater porque 50 es mayor que 38
        // Entonces la expresion match obtendra el valor Ordering::Greater y comienza a verificar el patron de cada brazo.
        // Suponiendo que el primer brazo es Ordering::Less y ve que el valor es Ordering::Greater no coincide asi que ignora todo el codigo
        // de ese brazo y se mueve al sig. brazo. Si el siguiente brazo es el caso Ordering::Greater entonces se ejecutaria el codigo de ese brazo 
        // donde podriamos mostrar el mensaje 'Too big!' en la terminal al cliente. EL PATRON MATCH TERMINA DESPUES DE LA PRIMER COINCIDENCIA EXITOSA
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}