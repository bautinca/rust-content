Este concepto de los **Lifetimes** es un concepto UNICO de Rust que no existe ni en C ni en Python por ejemplo.

Porque existen los lifetimes? Que problemas resuelven? Consideremos la sig. funcion:

```rust
// Basicamente la funcion recibe 2 referencias a strings y devuelve una referencia a un string
fn mas_larga(x: &str, y: &str) -> &str {
  if x.len() > y.len() {x} else {y}
}
```

Si bien en esta ultima funcion parece que esta todo bien en realidad no, y si intentamos compilar el codigo NO COMPILARIA. ¿Porque? Pues porque la funcion devuelve una referencia a un string, **pero el compilador no sabe de cual de los 2 parametros viene esa referencia**, y eso importa porque cada referencia tiene un tiempo de vida distinto, si por ejemplo devolvemos una referencia a algo que ya se libero, tenemos el tipico problema de C que es el **dangling pointer**, osea que devolvemos una referencia a algo que ya no existe y eso es un error de memoria.

Los lifetimes son muy utiles ya que le dan esa informacion al compilador en tiempo de compilacion y le permite saber que referencia es valida y cual no, y asi evitar errores de memoria en tiempo de ejecucion.

Algo que nos puede surgir es, ¿Pero en el if no especificamos que si se cumple entonces devolver la referencia a x y si no se cumple devolver la referencia a y? Entonces porque el compilador no sabe cual de las 2 referencias es valida? Pues porque el compilador no sabe cuanto tiempo de vida tiene cada referencia, y eso es lo que los lifetimes le dan al compilador, informacion sobre cuanto tiempo de vida tiene cada referencia. ¿A que nos referimos con cuanto tiempo de vida tiene cada referencia? Pues a cuanto tiempo de vida tiene el objeto al que apunta la referencia, osea cuanto tiempo de vida tiene el string al que apunta la referencia. Por ejemplo si tenemos un string que se crea dentro de una funcion y devolvemos una referencia a ese string, esa referencia no es valida porque el string se destruye cuando termina la funcion y por lo tanto la referencia apunta a algo que ya no existe. Entonces con los lifetimes le decimos al compilador que la referencia que devolvemos tiene un tiempo de vida igual al de los parametros de entrada, **osea que la referencia que devolvemos es valida mientras las referencias de entrada sean validas.**

Ahora tema sintaxis, como se indica el lifetime? Un lifetime se escribe con un apostrofo seguido de un nombre, por concencion es letra minuscula:

```rust
&i32 // Una referencia a un i32 tiene un lifetime implicito, el compilador lo infiere

&'a i32 // Una referencia a un i32 con un lifetime explicito llamado 'a

&'a mut i32 // Una referencia mutable a un i32 con un lifetime explicito llamado 'a
```

El lifetime en sí no cambia cuanto vive una variable, **solo describe la relacion entre los tiempos de vida de varias referencias**. Entonces la funcion anterior que no compilaba ahora corregida con los lifetimes seria:

```rust
// Indicamos que la referencia que retorna la funcion tiene el mismo lifetime (tiempo de vida) que las referencias de entrada, osea que la referencia que devolvemos es valida mientras las referencias de entrada sean validas. Si llega a suceder que una string a la que apunta una referencia de entrada se destruye, entonces la referencia que devuelve la funcion tambien se destruye y no es valida.
fn mas_larga<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {x} else {y}
}
```

En otras palabras la anotacion *'a* le dice al compilador: *"La referencia que devuelvo vivira al menos tanto como la mas corta entre x e y"*, es una promesa que el compilador verifica

Veamos mas casos concretos de lifetimes:

```rust
fn main() {
  let string1 = String::from("cadena larga");

  {
    let string2 = String::from("xyz");
    let resultado = mas_larga(string1.as_str(), string2.as_str());
    println!("La cadena mas larga es {}", resultado);
  }
}
```

Esto ultimo compilaria sin problemas porque la referencia que devuelve la funcion *mas_larga()* tiene un lifetime igual al de las referencias de entrada, y en este caso las referencias de entrada son validas mientras se ejecuta el bloque donde se llama a la funcion, entonces la referencia que devuelve la funcion tambien es valida mientras se ejecuta ese bloque.

Sin embargo si hacemos lo siguiente:

```rust
fn main() {
  let string1 = String::from("cadena larga");
  let resultado;
  {
    let string2 = String::from("xyz");
    resultado = mas_larga(string1.as_str(), string2.as_str());
  } // Salimos del scope, por ende string2 se libera y la referencia que apunta a string2 quedaria invalida, su lifetime termina aqui, por lo tanto la referencia que devuelva la funcion tambien QUEDARIA INVALIDA y es aca donde el compilador nos arroja un error de compilacion

  // El resultado podria ser una referencia que apunte a string2 pero string2 ya se habia liberado
  println!("La cadena mas larga es {}", resultado);
}
```

En el segundo caso el compilador nos arroja un error de compilacion porque la referencia que devuelve la funcion *mas_larga()* podria apuntar a *string2* que ya se habia liberado, y eso es un error de memoria. Por eso los lifetimes son tan importantes, porque le dan al compilador la informacion necesaria para saber si una referencia es valida o no. Es asi como el lifetime nos ayuda a evitar errores de memoria en tiempo de ejecucion, y es un concepto unico de Rust que no existe en otros lenguajes como C o Python.

Tambien podemos colocar **lifetimes en structs**, si un struct contiene referencias necesitamos anotar sus lifetimes osea indicar explicitamente en que momento de vida son validas esas referencias, por ejemplo:

```rust
struct Extracto<'a> { // Indicamos que el struct Extracto tiene un lifetime 'a, osea que las referencias que contiene el struct son validas mientras dure el lifetime 'a
    parte: &'a str, // Indicamos que el atributo parte es una referencia a un str con un lifetime 'a, osea que la referencia es valida mientras dure el lifetime 'a
}

fn main() {
  let novela = String::from("Habia una vez...")
  let primera_oracion = novela.split('.').next().unwrap(); // Obtenemos la primera oracion de la novela, que es una referencia a un str
  
  let extracto = Extracto { parte: primera_oracion }; // Le pasamos una referencia a la primera oracion de la novela al struct Extracto.

} // Al salir del scope la variable novela se libera por lo tanto la referencia 'primera_oracion' que apunta a la primera oracion de la novela ya no es valida, y por ende la referencia 'parte' del struct Extracto tampoco es valida, esto lo sabe el compilador en tiempo de compilacion gracias a los lifetimes que indicamos explicitamente en el struct Extracto y en el atributo parte.
```

Tambien podemos indicar lifetimes en metodos de structs, por ejemplo:

```rust
struct Extracto<'a> {
    parte: &'a str,
}

impl<'a> Extracto<'a> {
  fn nivel(&self) -> i32 {
    3 // Devuelve i32, no referencia, por tanto no necesita lifetime
  }

  fn anunciar(&self) -> &'a str { // Indicamos que la referencia que devuelve el metodo anunciar() tiene un lifetime 'a, osea que la referencia que devuelve es valida mientras dure el lifetime 'a
    self.parte // Devuelve la referencia al atributo parte del struct Extracto, que tiene un lifetime 'a, osea que la referencia que devuelve el metodo anunciar() es valida mientras dure el lifetime 'a
}
```

Hay una referencia muy particular que es **'static**, que es un lifetime especial que indica que la referencia es valida durante toda la ejecucion del programa, osea que nunca se libera. Por ejemplo:

```rust
let s: &'static str = "texto literal"; // La referencia a un string literal tiene un lifetime 'static, osea que es valida durante toda la ejecucion del programa
```

Los strings literals siempre son referencias con lifetime 'static porque estan hardcodeados en el binario del programa. Cuando el compilador nos sugiere agregar 'static como solucion a un error de lifetime, generalmente **no es la solucion correcta**, porque generalmente no queremos que la referencia dure toda la ejecucion del programa, sino que queremos que dure lo mismo que las referencias de entrada. Por eso es importante entender bien los lifetimes y no simplemente agregar 'static como solucion a un error de lifetime ya que seria como 'hacer la facil'

## Ejemplo todo junto
A continuacion vamos a ver un ejemplo de una funcion que implementa todo junto, es decir datos genericos + traits + lifetimes

```rust
use std::fmt::Display;

// La funcion recibe 2 referencias a strings y un tipo de dato generico y retorna una referencia a un string. Como recibe referencias y retorna referencia necesitmos indicar lifetimes explicitos entonces:
fn mas_larga_con_aviso<'a, T>(x: &'a str, y: &'a str, aviso: T) -> &'a str
where
    T: Display, // Indicamos que el tipo de dato generico T debe implementar el trait Display para poder usarlo en la funcion
{
    println!("Aviso: {}", aviso); // Imprimimos el aviso que es de tipo generico T que implementa Display
    
    if x.len() > y.len() {x} else {y} // Retornamos la referencia al string mas largo, que tiene un lifetime 'a, osea que es valida mientras duren las referencias de entrada x e y
}
```