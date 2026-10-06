/*
Vamos a programar algo simple un hola mundo
*/

fn main() {
  println!("Hola a todos!!");
}

/*
Luego lo que tenemos que hacer es igual que en C, es decir, primero lo compilamos
con RUSTC que en C seria como el GCC (por esto decimos que Rust tiene su propio compilador
que es RUSTC):

  > rustc hola_mundo.rs

Al compilarlo es logico que se nos genera un ejecutable/binario
y como sabemos para ejecutar todo ejecutable se hace con:

  > ./hola_mundo

Recordemos que la funcion main() es especial ya que siempre es el primer codigo que se ejecuta en cada programa
ejecutable en Rust.

Algo importante es que para imprimir en terminal usamos la funcion println!() esto es porque
en realidad no es una funcion comun y corriente sino que es una MACRO (toda funcion MACRO termina con un !)

Para llamar a la funcion println! comun y corriente simplemente tendriamos que haber hecho println() sin el '!'

Entonces en resumen:

  - Sin '!': Funcion normal
  - Con '!': Macro

Como vemos Rust NO ES UN LENGUAJE DINAMICO como Python, Ruby o JavaScript por lo que requiere de una compilacion
y luego ejecutar el binario como pasos separados.

Rust es un LENGUAJE COMPILADO DE ANTEMANO, lo que significa que podemos compilar un programa y luego darle el ejecutable generado
a otra persona al igual que en C. La otra persona puede ejecutar el ejecutable sin siquiera tener Rust instalado luego.

En cambio en Python, Ruby, etc. al estar todo integrado si o si la persona que ejecuta un archivo en Python por ejemplo si o si debe tener Python instalado. Sin embargo
en esos otros lenguajes solo necesitamos un comando para compilar+ejecutar nuestra app
  */
