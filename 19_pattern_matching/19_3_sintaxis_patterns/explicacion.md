## Coincidiendo con literales
El caso mas simple de patrones es el de los literales, que son valores fijos como `5`, `"hola"` o `true`. Un patron literal coincide con un valor si y solo si el valor es exactamente igual al literal. Por ejemplo:

```rust
let x = 5;
match x {
    5 => println!("x es cinco"),
    _ => println!("x no es cinco"),
}
```

## Variables nombradas: cuidado con shadowing
Cuando declaramos una variable dentro de un `match`, crea un **nuevo scope** por lo que puede ocultar una variable externa con el mismo nombre (osea shadowing). Por ejemplo:

```rust
let x = Some(5);
let y = 10;

match x {
  Some(50) => println!("Got 50"),
  Some(y) => println!("Matched, y = {y}", y), // Aqui y es una nueva variable, no la 'y'.
  _ => println!("Default case, x = {:?}", x),
}
```
En el segundo brazo del `match` lo que sucede es que `y` es una **nueva variable** que coincide con cualquier valor dentro de `Some`. No confundirse, no es la `y = 10` de afuera.

## Multiples patrones con `|`
El operador `|` funciona como **OR**, coincide con cualquiera de los valores:

```rust
let x = 1;

match x {
  1 | 2 => println!("uno o dos"),
  3 => println!("tres"),
  _ => println!("otro numero"),
}
```

## Rangos con `..=`
El operador `..=` permite coincidir con un rango de valores. Por ejemplo:

```rust
let x = 5;

match x {
  1..=5 => println!("x esta entre 1 y 5"),
  _ => println!("x es mayor que 5"),
}
```

## Desestructurando Structs
Podemos extraer los campos de un struct directamente con pattern:

```rust
struct Point { x: i32, y: i32 }

let p = Point { x: 0, y: 7 };

let Point { x: a, y: b } = p; // Hicimos que a = 0 y b = 7

// Otra forma de hacer lo mismo seria:
let Point { x, y } = p; // Aqui x = 0 y y = 7
```

Y en un `match` podemos combinar con literales para casos especificos:

```rust
match p {
  Point { x: 0, y } => println!("Sobre el eje y en {y}"),
  Point { x, y: 0 } => println!("Sobre el eje x en {x}"),
  Point { x, y } => println!("En algun otro lugar ({x}, {y})"),
}
```

## Desestructurando Enums
Tambien podemos desestructurar enums, por ejemplo:

```rust
// Definimos un enum
enum Message {
  Quit,
  Move {x: i32, y: i32},
  Write(String),
  ChangeColor(i32, i32, i32),
}

match msg {
  Message::Quit => println!("Quit"),
  Message::Move {x, y} => println!("Move to ({x}, {y})"),
  Message::Write(text) => println!("Text message: {text}"),
  Message::ChangeColor(r, g, b) => println!("Change color to ({r}, {g}, {b})"),
}
```

Tambien a su vez podemos desestructurar enums dentro de enums:

```rust
enum Color {
  Rgb(i32, i32, i32),
  Hsv(i32, i32, i32),
}

enum Message {
  ChangeColor(Color),
}

match msg {
  Message::ChangeColor(Color::Rgb(r, g, b)) => println!("Change color to RGB ({r}, {g}, {b})"),
  Message::ChangeColor(Color::Hsv(h, s, v)) => println!("Change color to HSV ({h}, {s}, {v})"),
  _ => println!("Other message"),
}
```

## Desestructurando Structs y Tuplas mezcladas
Tambien podemos desestructurar structs y tuplas mezcladas, por ejemplo:

```rust
let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 }); // Aqui feet = 3, inches = 10, x = 3, y = -10
```

## Ignorando valores con `_`
Si no nos interesa un valor podemos ignorarlo con `_`:

```rust
fn foo(_: i32, y: i32) {
  println!("y = {y}");
}

foo(5, 10); // Aqui el primer argumento es ignorado
```

Tambien podemos ignora parte de un valor, por ejemplo algunos elementos de una tupla:

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
  (first, _, third, _, fifth) => println!("first = {first}, third = {third}, fifth = {fifth}"),
}
```

Tambien si declaramos una variable y no la usamos, Rust nos dara un warning. Para evitarlo podemos usar `_` al inicio del nombre de la variable:

```rust
let _x = 5; // Rust no dara warning por no usar _x
let y = 10; // Rust dara warning por no usar y
```

Por otro lado el `..` nos permite ignorar el resto de los valores de una tupla o struct:

```rust
struct Point { x: i32, y: i32, z: i32 }
let origin = Point { x: 0, y: 0, z: 0 };

match origin {
  Point { x, .. } => println!("x = {x}"),
}
```

Con las tuplas el `..` puede ir al principio, al medio o al final. Pero ojo, solo puede haber un `..` en el patron, no se puede repetir:

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
  (first, .., last) => println!("first = {first}, last = {last}"),
  (.., second, ..) => println!("second = {second}"), // Esto no es valido, solo puede haber un `..` en el patron
}
```

## Match guard: Condicionales adicionales
Un match guard es una condicion `if` extra despues del patron que tambien debe ser verdadera. Por ejemplo:

```rust
let num = Some(4);

match num {
  Some(x) if x % 2 == 0 => println!("x es par: {x}"), // Si nos fijamos este brazo recibe una condicion extra, que es que x sea par. A eso se le llama match guard.
  Some(x) => println!("x es impar: {x}"),
  None => println!("no hay valor"),
}
```

Hay un uso importante con los Match guards que es que basicamente resuelve el problema de **shadowing** que explicamos anteriormente cuando queremos usar una variable externa dentro de un `match`. Por ejemplo:

```rust
let x = Some(5);
let y = 10;

match x {
  Some(50) => println!("Got 50"),
  Some(n) if n == y => println!("Matched, n = {n}"), // Aqui n es una nueva variable, pero si nos fijamos la comparamos con la variable externa y = 10. Esto es posible gracias al match guard.
  _ => println!("Default case, x = {:?}", x),
}
```

Por otro lado con el `|` el match guard se aplica a todos los patrones, por ejemplo:

```rust
let x = 4;

match x {
  4 | 5 | 6 if x % 2 == 0 => println!("x es par y esta entre 4 y 6"),
  _ => println!("x no cumple la condicion"),
}
```

## Bindings: Capturar y verificar al mismo tiempo
El operador `@` crea una variable que guarda el valor al **mismo tiempo** que lo comprueba contra un patron. Por ejemplo:

```rust
enum Message {
  Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
  Message::Hello { id: id_variable @ 3..=7 } => println!("id variable esta entre 3 y 7: {id_variable}"), // En este caso id_variable es una nueva variable que guarda el valor de id, pero al mismo tiempo estamos verificando que id este entre 3 y 7.
  Message::Hello { id: 10..=12 } => println!("id esta entre 10 y 12"), // Aca estamos verificando que id este entre 10 y 12, pero no estamos guardando el valor en una variable.
  Message::Hello { id } => println!("id es otro valor: {id}"), // Aca estamos guardando el valor de id en una variable, pero no estamos verificando nada.
}
```

Basicamente:

- **Primer brazo**: En el primer brazo estamos verificando que `id` este entre 3 y 7, y al mismo tiempo estamos guardando el valor de `id` en la variable `id_variable`.

- **Segundo brazo**: En el segundo brazo estamos verificando que `id` este entre 10 y 12, pero no estamos guardando el valor de `id` en ninguna variable.

- **Tercer brazo**: En el tercer brazo estamos guardando el valor de `id` en la variable `id`, pero no estamos verificando nada.

Entonces el `@` nos da lo mejor de los 2 mundos, verificar y capturar en una sola operacion.