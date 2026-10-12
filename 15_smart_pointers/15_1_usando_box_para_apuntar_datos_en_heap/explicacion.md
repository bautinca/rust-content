# Smart Pointer

¿Que es un Smart Pointer? Las referencias comunes y corrientes &T son referencias que apuntan a un valor en memoria, no tienen logica propia ni posee los datos. Un smart pointer en cambio **es una estructura que actua como puntero pero ademas tiene metadatos y comportamiento extra**. La diferencia clave con las referencias es que los smart pointers generalmente **son dueños** de los datos a los que apuntan.

Los smart pointers ademas eliminan esos bugs generalizados que se producen al usar referencias, como dangling pointers, memory leaks y double frees. En C es muy comun cometer los errores de liberar memoria que ya fue liberada, o liberar memoria que nunca fue asignada, etc. Bueno con los smart pointers esto no pasa.

Los smart pointers se implementan como **structs** que estos a su vez tienen 2 traits que son **Deref** y **Drop** que mas adelante los veremos.

## Box\<T>: El smart pointer mas simple
Lo que hace **Box** es mover un valor al heap y devolver un puntero a ese valor en el stack, nada mas que eso. No tiene overhead, sin capacidades especiales, ni nada. Solo indirecciona a un valor en el heap y no mas que eso. Veamos un ejemplo de un smart pointer con Box:

```rust
fn main() {
  let b = Box::new(5); // b es un smart pointer en el stack que apunta al valor 5 en el heap
  println!("b = {}", b); // Podemos usarlo igual que si fuera un i32 normal
} // Salimos del scope y el valor en el heap es liberado automaticamente
```

### Tipos recursivos
El siguiente enum por ejemplo no compila:

```rust
enum List {
  Cons(i32, List),
  Nil,
}
```

Este enum no compila ya que el compilador necesita saber en tiempo de compilacion cuantos bytes ocupa el enum **List** pero Cons tiene un campo de tipo List, entonces el compilador no hay manera de que sepa cuanto ocupa el enum List.

La solucion a esto ultimo es romper la recursividad usando **Box** entonces podemos hacer algo como:

```rust
enum List {
  Cons(i32, Box<List>), // BIEN
  Nil,
}
```

Lo que conseguimos es que el compilador pueda saber cuantos bytes ocupa el enum **List** porque ahora el campo de tipo List es un Box<List>, que es un puntero a un valor de tipo List, y los punteros como bien sabemos tienen un tamaño fijo.

No podria haber hecho lo siguiente?

```rust
enum List {
  Cons(i32, &List), // MAL
  Nil,
}
```

No, no se puede usar una referencia &List porque las referencias no son dueñas de los datos a los que apuntan, y en este caso el enum List es dueño de sus datos. Si usas una referencia, el compilador no sabria cuanto tiempo de vida tiene esa referencia y podria ser un dangling pointer. Por eso se usa Box, que es un smart pointer que es dueño de los datos a los que apunta

Entonces ahora habiendo creado el enum List, podemos crear una lista enlazada de la siguiente manera:

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
```

Ahora el compilador sabe exactamente cuanto ocupa Cons que seria un i32 + un puntero a List que es de tamaño fijo independientemente a donde apunte.

### Trait Deref
Es un trait que como dijimos antes, es implementado por los smart pointers y nos permite **sobrecargar el operador de desreferencia**. Esto significa que podemos usar el operador `*` para acceder al valor al que apunta el smart pointer en resumidas cuentas, asi como teniamos para las referencias donde usabamos `*` para acceder al valor al que apuntaba la referencia, bueno lo mismo para los smart pointers. Por ejemplo:

```rust
// Para referencias normales
let x = 5;
let y = &x; // y es una referencia a x. La referencia vive en el stack y apunta a un valor en el stack (ya que x vive en el stack como sabemos)

assert_eq!(5, *y); // Usamos * para acceder al valor al que apunta la referencia, nada nuevo
```

Bueno el smart pointer Box implementa el trait Deref para hacer justamente esto mismo

```rust
let x = 5;
let y = Box::new(x); // y es un smart pointer viviendo en el stack que apunta a un valor en el heap ahora, x ya no existe en el stack, ya que Box::new mueve el valor al heap y devuelve un puntero a ese valor en el stack

assert_eq!(5, *y); // Usamos * para acceder al valor al que apunta el smart pointer
```

Tambien podemos implementar nuestro propio trait Deref para nuestros propios tipos, por ejemplo:

```rust
use std::ops::Deref;

struct MyBox<T>(T); // Creamos un struct que es un smart pointer generico, que puede apuntar a cualquier tipo T

// Implementamos el constructor para MyBox, recibe un valor de tipo T y devuelve un MyBox<T> que apunta a ese valor
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implementamos el trait Deref para MyBox, para que podamos usar el operador * para acceder al valor al que apunta el smart pointer
impl<T> Deref for MyBox<T> {
    type Target = T; // El tipo al que apunta el smart pointer es T

    fn deref(&self) -> &T { // Implementamos el metodo deref que devuelve una referencia al valor al que apunta el smart pointer
        &self.0 // Devolvemos una referencia al valor al que apunta el smart pointer, que es el primer campo del struct MyBox
    }
}
```

Entonces el metodo `deref` es el que nos permite usar el operador `*`. Como vemos el metodo `deref` devuelve una referencia (no el valor directo) para no mover el valor fuera del smart pointer, ya que si movemos el valor fuera del smart pointer, el smart pointer ya no tendria sentido y perderiamos la propiedad del valor.

Entonces cuando escibimos `*y` sobre algo que implementa el trait Deref, lo que hace Rust es expandirlo a `*(y.deref())` y luego el operador `*` desreferencia la referencia que devuelve el metodo `deref`, entonces al final obtenemos el valor al que apunta el smart pointer.

### Deref coercion
El trait Deref nos permite usar el operador `*` para acceder al valor al que apunta el smart pointer, pero tambien nos permite hacer algo llamado **Deref coercion** que es una caracteristica del compilador que nos permite **convertir automaticamente un tipo a otro tipo** cuando el compilador espera un tipo y nosotros le pasamos otro tipo que implementa el trait Deref. Veamos un ejemplo:

```rust
fn hello(name: &str) { // La funcion hello espera un tipo &str
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust")); // Creamos un MyBox<String> que apunta a un String en el heap
    hello(&m); // Pasamos una referencia a m, que es un MyBox<String>, pero la funcion hello espera un &str
}
```

Esto ultimo es valido porque el compilador de manera automatica hace **deref coercion** y convierte el tipo `&MyBox<String>` a `&String` usando el metodo `deref` que implementamos antes, y luego convierte el tipo `&String` a `&str` usando el metodo `deref` que implementa la struct String. Entonces al final obtenemos un `&str` que es lo que la funcion hello espera. Si no fuera por el deref coercion tendriamos que escribir algo como `hello(&(*m)[..])` para obtener un `&str` a partir de un `MyBox<String>`, lo cual es mucho mas engorroso y feo. Es decir, primero tendriamos que desreferenciar el smart pointer con `*m`, luego obtener una referencia al String con `&(*m)`, y luego obtener un slice del String con `&(*m)[..]` para finalmente obtener un `&str`. Pero gracias al deref coercion, el compilador hace todo eso por nosotros de manera automatica.

Estas coerciones se resuelven en tiempo de compilacion por lo que tiene un **costo runtime de 0**.

### Trait Drop
Ya vimos el trait Deref que es lo que tienen los smart pointers, ahora veamos el otro trait que tienen los smart pointers que es el **trait Drop**. Este trait nos permite **definir un comportamiento que se ejecuta cuando un valor sale de scope**. Esto es muy util para liberar recursos, como memoria, archivos, sockets, etc. Por ejemplo, el smart pointer Box implementa el trait Drop para liberar la memoria que apunta cuando sale de scope pero nosotros podemos implementar nuestro propio trait Drop para nuestros propios tipos. Por ejemplo:

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer { // Implementamos el trait Drop para CustomSmartPointer
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer con `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers creados");
}   // d se dropea primero, luego c (orden inverso a la creación). Al salir del scope automaticamente se llama al metodo drop de cada uno de los CustomSmartPointer y se imprime el mensaje correspondiente ya que lo tenemos definido en dicho metodo.
```

Entonces al finalizar la funcion main la salida seria algo como:

```
CustomSmartPointers creados
Dropping CustomSmartPointer con `other stuff`!
Dropping CustomSmartPointer con `my stuff`!
```

Pero ojo, **no podemos llamar al metodo drop manualmente**, Rust lo prohibe porque despues lo llamaria automaticamente al salir del scope y eso generaria un double free, el bug clasico de C. El metodo drop() se llama automaticamente por Rust al salir del scope.

Y si necesitamos liberarlo antes de salir del scope? Bueno en ese caso se usa la funcion **drop()** pero de la libreria estandar, que es diferente al metodo drop() del trait Drop. La funcion drop() de la libreria estandar toma un valor y lo libera inmediatamente, llamando al metodo drop() del trait Drop. Por ejemplo:

```rust
use std::ops::Drop;

drop(c);
println!("c ya fue dropeado");
```

