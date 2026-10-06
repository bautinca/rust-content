// De la biblioteca estandard de Rust (std) usamos la biblioteca IO para obtener la entrada del usuario y luego imprimir el resultado en la salida
use std::io;
// POr defecto Rust tiene un conjunto de elementos definidos en la biblioteca estandard (std) que trae al alcance de cada programa, este conjunto se les llama
// PRELUDE

// Si una biblioteca no esta en PRELUDE tenemos que traer ese tipo al alcance explicitamente con una declaracion use

// Funcion main pto de entrada de la app
fn main() {
    // Imprimimos en la terminal el nombre del juego
    println!("Guess the number!");
    // Le pedimos que ingrese su adivinanza
    println!("Please input your guess");
    // Aca declaramos una variable mutable (mut, tiene que ser mutable asi logicamente mas adelante definimos la string) donde la iniciamos como una string vacia
    // Y mas adelante tomara el valor segun la adivinanza que ingrese el usuario. Instanciamos un String que viene de la std de Rust.
    // La sintaxis :: indica que el new() es una funcion (como si fuera un metodo) asociado a la clase String. La funcion new() crea una string vacia
    let mut guess = String::new();
    // Ahora de la terminal leemos lo que ingreso el usuario (STDIN) y con eso definimos la variable anterior
    // Recordar que en este caso stdin() es una funcion de io .
    //  Si no hubieramos hecho use std::io entonces era lo mismo que hace std::io::stdin() ya que io viene de la biblioteca estandard de Rust (std)
    // Luego con read_line() obtenemos la entrada del usuario.
    // Aca es importante que la variable 'guess' sea mutable para poder definirle el valor ya que antes era una string vacia
    // Algo importante es que el & indica una REFERENCIA. Gracias a esta referencia nos permite que varias partes del codigo accedan a una parte de memoria
    // sin necesidad de copiar esos datos en la memoria varias veces.
    // Las referencias por defecto son inmutables, es por ello que debemos escribir &mut guess en lugar de &guess
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    // El io::stdin().read_line() devuelve un resultado (Result) que es basicamente lo que ingresa el usuario en STDIN. El Result este es una ENUMERACION (enum) (osea una string
    // es algo iterable)

    // Las variantes del Result son o Ok o Err. La variante Ok indica que la operacion fue exitosa y dentro del Ok() esta el valor generado con exito. En cambio
    // la variante Err significa que la operacion fallo y el Err contiene informacion sobre como o porque la operacion fallo

    // Los valores de tipo Result (como String, io, etc.) tienen metodos/funciones definidos en ellos. Una instancia de Result tiene el metodo .expect() que podemos llamar.
    // Si el Result es un Err entonces lo que el expect hara que el programa se bloquee y muestre el mensaje que pasamos como argumento al expect(). Si en cambio la instancia
    // de Result es un Ok entonces expect hara tomara el valor de retorno que Ok esta sosteniendo y devolvera solo ese valor para que lo podamos usar. En este caso ese valor
    // es el numero de bytes en la entrada del usuario

    // Si no usamos el expect() el programa se compilara pero aun asi el compilador de Rust (Cargo) nos lanzara una advertencia de que nos recomienda usarlo
    
    // Imprimimos la adivinanza del usuario
    println!("Your guessed: {guess}");
}

// En resumidas cuentas lo que hace el programa hasta ahora es simplemente por la terminal nos pide un valor y luego imprime dicho valor
