## Tipos de Datos

Cada valor en Rust es de un **tipo de dato** que le dice a Rust que tipo de dato se esta especificando para que sepa como trabajar con ese dato. Hay 2 subconjuntos de tipos principales que son los siguientes

### Tipos Escalares
Representan **un solo valor**. Rust tiene 4 tipos de escalares principales que son los siguientes

#### Tipos de Enteros
Es un entero al fin y al cabo 0, 3, 1, 39, -20, -120, etc.
Por ejemplo si una variable es de tipo i8 quiere decir que es un entero con signo de 8 bits, o si es u64 quiere decir que es un entero sin signo de 64 bits. Si queremos generalizar y decir que un entero es sin signo sin saber cuantos bits usar entonces podemos usar usize, y si es un entero con signo pero no sabemos cuantos bits asignarle entonces podemos usar isize.

Si no especificamos el tipo de dato entonces Rust por defecto le asigna i32

```rust
let x: u32 = 42;
```

#### Tipos de Punto Flotante
Son numeros con coma, o decimales, tenemos 2 tipos nada mas que son f32 (flotante de 32 bits) o f64 (flotante de 64 bits). Por defecto Rust le asigna el tipo de dato f64.

```rust
let x: f32 = 3.0;
```

#### Tipo booleano
Obviamente tiene 2 valores true/false corta. Se les asigna 1 byte de tamaño y se especifica con bool, ejemplo:

```rust
let f: bool = false;
```

#### El tipo caracter
Son caracteres simples como 'z', 'a' o incluso emojis o demas caracteres raros pero siempre es 1 caracter. Se los define con tipo de dato char:

```rust
let c: char = 'z'; // Es importante colocar comillas simples '' para este tipo de dato., por defecto ocupa 4 bytes
```

### Tipos Compuestos
Agrupan multiples valores en un solo tipo. Rust tiene 2 tipos de compuestos que son los siguientes

#### Tipo Tupla
Se agrupan varios valores de **distintos tipos** en un solo tipo de compuesto que seria la tupla. Las tuplas tienen longitud fija y una vez declaradas luego su tamaño no puede modificarse. Cada posicion de la tupla tiene su tipo, por ejemplo:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

Para obtener el valor de una tupla se puede utilizar pattern matching para 'desarmar' una tupla asi:

```rust
let tup = (500, 6.4, 1); // Por defecto si no especificamos el tipo de dato en cada posicion de la tupla todo lo hace Rust en f64 e i32

// Aplicamos pattern matchin para "desarmar" la tupla
let (x, y, z) = tup;

// Imprimimos el valor x de la tupla
println!("The value of x is: {x}");
```

Tambien podemos acceder a un elemento de la tupla directamente por su indice asi:

```rust
let x: (i32, f64, u8) = (500, 6.4, 1);
let primer_elemento = x.0; // El de indice 0 es 500 obvio
let segundo_elemento = x.1;
let tercer_elemento = x.2;
```

La tupla sin valores, osea vacia () se le llama **UNIT**

#### Tipo Arreglo
Otra forma de tener una coleccion de multiples valores es un arreglo (array). A diferencia de la tupla **CADA ELEMENTO DEL ARREGLO DEBE SER DEL MISMO TIPO**. Los arreglos en Rust tambien tienen **longitud fija**

```rust
let a = [1, 2, 3, 4, 5]; // Por defecto es logico que son todos i32
```

Sin embargo el arreglo es muy distinto a un **vector** (que aun no vimos). El tipo de dato **Vector** es un tipo de coleccion similar proporcionada por la std **cuya longitud se puede modificar** ya que un vector vive en el heap.

Por tanto si no estamos seguros de si tenemos que usar un arreglo o vector es casi seguro que debamos usar un vector entonces.

Los arreglos son utiles cuando se sabe que el numero de elementos que posee no cambiara. Por ejemplo si trabajamos con dias de la semana lo casi seguro es que debamos representarlo mediante un arreglo ya que siempre los dias de la semana son 7 por ende nunca cambiara la longitud el arreglo

```rust
let days = ["lunes", "martes", "miercoles", "jueves", "viernes", "sabado", "domingo"];
```

Si queremos especificar el tipo de dato que contendra el arreglo y la cantidad de elementos entonces se hace asi:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5]; // Arreglo de 5 elementos donde todos son i32
```

Para acceder a elementos de un arreglo tambien se puede hacer al igual que una tupla mediante su indice asi:

```rust
let a = [1, 2, 3, 4, 5];

let first = a[0];
let second = a[1];
```

Veamos ahora el siguiente codigo:

```rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    // Tomamos lo que ingrese el cliente en la terminal
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    // Lo parseamos para pasarlo de string a usize (entero)
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // Obtenemos el elemento del array segun el indice que ingreso el usuario
    let element = a[index];

    // Lo imprimimos
    println!("The value of the element at index {index} is: {element}");
}
```

Ahora bien ¿Que sucede cuando el cliente ingresa un indice invalido? Por ejemplo que es demasiado alto? Bueno lo que sucedera es que el array tiene indices del 0 al 4 pero si por ejemplo el usuario ingresa "7" entonces esto lanzara una excepcion en **tiempo de ejecucion** y la app entrara en panico en la linea 128, y esto esta bien que suceda ya que el compilador de Rust no puede saber que valor introducira el usuario cuando ejecute el codigo mas tarde.