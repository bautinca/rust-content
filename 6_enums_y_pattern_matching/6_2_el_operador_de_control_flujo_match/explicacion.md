# El operador de contro de flujo match
Rust tiene una construcción de flujo de control extremadamente poderosa llamada match que te permite comparar un valor contra una serie de patrones y luego ejecutar código basado en qué patrón coincide

Los patrones pueden estar compuestos de valores literales, nombres de variables, comodines y muchas otras cosas. En el primer patrón en el que el valor “se ajusta”, el valor cae en el bloque de código asociado para ser utilizado durante la ejecución.

Podemos escribir una función que tome una moneda desconocida de los Estados Unidos y, de una manera similar a la máquina de conteo, determine qué moneda es y devuelva su valor en centavos

```rust
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}

value_in_cents(Coin::Dime); // 10
```

Esto parece muy similar a una expresión condicional utilizada con if, pero hay una gran diferencia: con if, la condición debe evaluar a un valor Booleano, pero aquí puede ser cualquier tipo.

A continuación, dentro de las llaves de match, hay un número de Opciones. Una Opción tiene dos partes: un patrón y algún código. La primera Opción aquí tiene un patrón que es el valor Coin::Penny y luego el operador => que separa el patrón y el código a ejecutar. El código en este caso es solo el valor 1. Cada Opción está separado del siguiente con una coma.

Si desea ejecutar varias líneas de código en una Opción de match, debe usar llaves, y la coma que sigue a la Opción es opcional. Por ejemplo, el siguiente código imprime “¡Moneda de la suerte!” cada vez que el método se llama con un Coin::Penny, pero aún devuelve el último valor del bloque, 1:

```rust
fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => { // Aca el bloque de codigo se abre y cierra con llaves, si la moneda es un Penny se imprime el mensaje y se devuelve 1
      println!("¡Moneda de la suerte!");
      1
    },
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}
```

## Patrones que vinculan valores
Otra característica útil de las Opciones de match es que pueden vincularse a las partes del valor que coinciden con el patrón. Esto es cómo podemos extraer valores de las variantes de enum.

Como ejemplo, podemos cambiar el código de la función value_in_cents para que, en lugar de devolver un valor, imprima el valor que tiene. Esto nos permite ver qué moneda tenemos y cuánto vale. Para hacer esto, necesitamos convertir el código de cada Opción en una expresión, y luego usar una expresión println! en lugar de un valor de retorno. También necesitamos cambiar el tipo de value_in_cents a (), ya que no estamos devolviendo un valor entero, sino que estamos ejecutando código

```rust
#[derive(Debug)] // Lo usamos para poder imprimir el estado en el que se encuentra la moneda Quarter
enum UsState {
  Alabama,
  Alaska,
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) {
  match coin {
    Coin::Penny => println!("¡Moneda de la suerte!"),
    Coin::Nickel => println!("5 centavos"),
    Coin::Dime => println!("10 centavos"),
    Coin::Quarter(state) => println!("25 centavos de {:?}", state), // El patrón de Coin::Quarter(state) vincula el valor de la variante Quarter a la variable state, que luego podemos usar en el bloque de código para imprimir el estado del que proviene la moneda
  }
}
```
Si llamáramos a value_in_cents(Coin::Quarter(UsState::Alaska)), coin sería Coin::Quarter(UsState::Alaska). Cuando comparamos ese valor con cada una de las Opciones de match, ninguno coincide hasta que llegamos a Coin::Quarter(state). En ese punto, el enlace para state será el valor UsState::Alaska. Luego podemos usar ese enlace en la expresión println!, obteniendo así el valor del estado interno de la variante de Coin para Quarter

## Match con Option\<T>
En la seccion anterior manejamos el enum Option usando **match** para atender el caso del valor presente o cuando no hay valor. Antes manejamos el match con el enum Coin, pero el Option al ser tambien un enum podemos manejar las distintas variantes de Option de la misma manera asi por ejemplo:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

let five = Some(5);
let six = plus_one(five); // six es Some(6)
let none = plus_one(None); // none es None
```

Combinando match y enums es útil en muchas situaciones. Verás este patrón mucho en el código Rust: match contra un enum, vincula una variable a los datos internos y luego ejecuta el código en función de él. Es un poco complicado al principio, pero una vez que te acostumbras, desearás tenerlo en todos los lenguajes. Es consistentemente un favorito de los usuarios.

## Los match son exhaustivos
Hay otro aspecto de match que debemos discutir: los patrones de las Opciones deben cubrir **todas las posibilidades**, veeamos el siguiente codigo donde no compilaria justamente porque no cubrimos todas las posiblidades en el match, veamos por ejemplo con el enum Option donde sabemos que tiene 2 variantes Some y None:

```rust
fn use_option(x: Option<i32>) {
  match x {
    Some(value) => println!("El valor es: {}", value),
  }
}
```

Esto utimo tiraria un error de compilacion porque no cubrimos el caso de None, afortunadamente este error salta en tiempo de compilacion ya que lo detecta el compilador de Rust

Rust sabe que no cubrimos todos los casos posibles por lo que **los matches en Rust son exhaustivos**, debemos agotar todas las posibilidades para que el codigo sea valido

## Patrones de captura y el Placeholder
Usando enums, también podemos tomar acciones especiales para algunos valores particulares, pero para todos los demás valores, tomar una acción predeterminada. Imagina que estamos implementando un juego donde, si sacas un 3 en un lanzamiento de dados, tu jugador no se mueve, sino que obtiene un nuevo sombrero elegante. Si sacas un 7, tu jugador pierde un sombrero elegante. Para todos los demás valores, tu jugador se mueve esa cantidad de espacios en el tablero de juego. Aquí hay un match que implementa esa lógica, con el resultado del lanzamiento de dados codificado en lugar de un valor aleatorio, y toda la lógica representada por funciones sin cuerpos porque implementarlas realmente está fuera del alcance de este ejemplo:

```rust
fn roll_dice(dice_roll: u8) {
  match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other), // El patrón other es un placeholder que captura cualquier valor que no coincida con los patrones anteriores. En este caso, cualquier valor que no sea 3 o 7 se capturará en la variable other y se pasará a la función move_player.
  }
}
```

Este patrón de captura cumple con el requisito de que match debe ser exhaustivo. Ten en cuenta que tenemos que poner la Opción de captura al final porque los patrones se evalúan en orden. Si ponemos la Opción de captura antes, las otras Opciones nunca se ejecutarían.

Rust también tiene un patrón que podemos usar cuando queremos un catch-all, pero no queremos usar el valor en el patrón catch-all: _ es un patrón especial que coincide con cualquier valor y no se vincula a ese valor. Esto le dice a Rust que no vamos a usar el valor, por lo que Rust no nos advertirá sobre una variable no utilizada.

Vamos a cambiar las reglas del juego. Ahora, si sacas un número diferente de un 3 o un 7 debes tirar de nuevo. Ya no necesitamos usar el valor general, por lo que puede cambiar nuestro código para usar _ en lugar de la variable llamada other:

```rust
fn roll_dice(dice_roll: u8) {
  match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(), // El patrón _ es un catch-all que coincide con cualquier valor que no coincida con los patrones anteriores. En este caso, cualquier valor que no sea 3 o 7 hará que se ejecute la función reroll.
  }
}
```

Finalmente, cambiaremos las reglas del juego una vez más para que nada más ocurra en tu turno si sacas algo que no sea un 3 o un 7. Se haria asi:

```rust
fn roll_dice(dice_roll: u8) {
  match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (), // El patrón _ es un catch-all que coincide con cualquier valor que no coincida con los patrones anteriores. En este caso, cualquier valor que no sea 3 o un 7 hará que no ocurra nada en tu turno. El bloque de código asociado con el patrón _ es (), que es la unidad, lo que significa que no se hace nada.
  }
}
```

Aquí, le decimos a Rust explícitamente que no vamos a usar ningún otro valor que no coincida con un patrón en una Opción anterior, y no queremos ejecutar ningún código en este caso.
