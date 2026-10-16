Los pattern (patrones) ya los veniamos usando en muchos lugares sin darnos cuenta. No son exclusivos de `match` sino que aparece en varios contextos del lenguaje.

## match
Sin embargo en el lugar mas obvio donde se notan es en `match`, veamos un ejemplo:

```rust
match x {
  None => None,
  Some(i) => Some(i + 1),
}
```

El `match` ya lo conocemos, toma un valor (en este ultimo caso `x`) y lo compara con los patrones que le pasamos, en el primero que coincida ejecuta la expresion correspondiente. En este caso `x` es un `Option<i32>`, y los patrones que le pasamos son `None` y `Some(i)`.

El requisito clave de `match` es que tiene que ser **EXHAUSTIVO**, osea cubrir todas las posibilidades. Por eso siempre es altamente recomendable que termine con `_` cuando no queremos manejar si o si todos los casos:

```rust
match valor {
  1 => println!("Uno"),
  2 => println!("Dos"),
  _ => println!("Otro"), // Captura todo lo demas
}
```

## if let
Por otro lado ademas del `match` esta el `if let`. Donde basicamente es un `match` pero de un solo caso. Podemos mezclar tambien `if let` con `else if let` y `else` libremente:

```rust
let favourite_color: Option<&str> = None;
let is_tuesday = false;
let age: Result<u8, _> = "34".parse();

if let Some(color) = favourite_color {
  println!("Your favourite color is {}", color);
} else if is_tuesday {
  println!("Tuesday is green day!");
} else if let Ok(age) = age {
  if age > 30 {
    println!("Using purple!");
  } else {
    println!("Using orange!");
  }
} else {
  println!("Using blue!");
}
```

La diferencia con `match` es que `if let` no es exhaustivo. El compilador no nos avisa si nos olvidamos de un caso. **Es mas flexible pero mas propenso a cometer errores**.

## while let
Lo vimos en el capitulo de async/await pero es aplicable en general. Corre el loop mientras el patron coincida:

```rust
// Nos creamos un canal entre hilos
let (tx, rx) = std::sync::mpsc::channel();

// Creamos un hilo que enviara 3 enteros
std::thread::spawn(move || {
  for val in [1, 2, 3] {
    tx.send(val).unwrap();
  }
});

// Hacemos un loop mientras el patron coincida.
while let Ok(val) = rx.recv() {
  println!("Got: {}", val);
}
```

Entonces cuando el `rx.recv()` deja de devolver `Ok(val)` y devuelve `Err(_)` entonces el loop termina. Esto es muy util para leer de canales, iteradores, etc.

## for
En un bucle `for` lo que va despues del `for` es un patron. El caso mas simple es una variable sola, pero podemos poner patrones mas complejos, por ejemplo:

```rust
let v =  vec!['a', 'b', 'c'];

// El patron (indice, valor) es una tupla, el for itera sobre el enumerador de v
for (index, value) in v.iter().enumerate() {
  println!("{} is at index {}", value, index);
}
```

## let
El clasico `let` para declarar variables tambien es un patron!. Cada vez que escribimos `let` estamos usando un patron. Por ejemplo:

```rust
let x = 5; // El patron es `x`
```

Formalmente esto ultimo Rust lo lee asi:

```
let PATRON = EXPRESION;
```

`x` es el patron mas simple de todos ya que vincula todo lo que tenga a la variable x, pero podemos usar patrones mas complejos, por ejemplo:

```rust
let (x, y, z) = (1, 2, 3); // El patron es una tupla (x, y, z)
```

Si el patron no coincide con el tipo entonces le compilador de Rust nos avisara:

```rust
let (x, y) = (1, 2, 3); // Error en compilacion
```

## Parametros de funciones
Los parametros en las funciones tambien son patrones. La forma mas simple es una variable sola:

```rust
fn foo(x: i32) {
  // x es un patron que vincula el valor pasado a la variable x
}
```

Pero podemos desestructura directamente en los parametros, por ejemplo:

```rust
// Desestructuramos la tupla directamente en los parametros
fn print_coordinates(&(x, y): &(i32, i32)) {
  println!("Current location: ({}, {})", x, y);
}

fn main() {
  let point = (3, 5);
  print_coordinates(&point);
}
```

Y lo mismo en los closures (funciones anonimas):

```rust
let puntos = vec![(0, 1), (2, 3), (4, 5)];

puntos.iter().for_each(|(x, y)| println!("({}, {})", x, y));
```