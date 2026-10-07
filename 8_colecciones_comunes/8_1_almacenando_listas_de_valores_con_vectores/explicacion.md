Hay una estructura de datos muy util que aun no vimos que son las **colecciones**. La mayoria de los tipos de datos que ya vimos representan un valor especifico, pero las colecciones pueden representar un conjunto de valores. A diferencia de un array y tupla los datos a los que apuntan estas colecciones se almacenan en el **heap** lo que significa entonces que la cantidad de datos que tendra la coleccion no esta limitada por el tamaño de la memoria del stack, por tanto la cantidad de datos de la coleccion no necesita conocerse en el momento de la compilacion y puede crecer o disminuir a medida que se ejecuta el programa. Cada tipo de coleccion tiene distintas capacidades y costos y elegir el apropiado para la situacion actual es una habilidad que desarrollaremos. Veremos 3 tipos de colecciones que se usan casi siempre que son:

- **Vectores**: Nos permite almacenar un numero variable de valores uno al lado del otro

- **String**: Es una coleccion de caracteres, de hecho ya lo vimos! Es el tipo **String** que estudiamos antes, bueno este tipo de dato esta implementado como una coleccion de caracteres, por lo que podemos decir que es un vector de caracteres.

- **Hash map**: Nos permite asociar un valor con una clave especifica.

## Vectores
Un **Vec\<T>** es una coleccion que almacena elementos **del mismo tipo** contiguos en memoria en el **heap**. Por lo tanto como se almacena en el heap puede crecer o disminuir su tamaño a medida que se ejecuta el programa. La **T** hace referencia al tipo de dato que se almacena en el vector es un generico, puede ser por ejemplo un vector Vec\<i32\> que almacena enteros de 32 bits, o Vec\<String\> que almacena cadenas de caracteres. Los vectores son muy utiles cuando necesitamos almacenar una cantidad variable de datos del mismo tipo.

Como se crea al vector? Hay 2 formas:

```rust
let v: Vec<i32> = Vec::new(); // Crea un vector vacio de enteros de 32 bits

let v = vec![1, 2, 3]; // Crea un vector con los valores 1, 2 y 3 ya de entrada. El tipo de dato lo deduce el compilador a partir de los valores que se le pasan al vector.
```

Logicamente creamos un vector no mutable, si queremos agregarle valores debemos declararlo como **mut**:

```rust
let mut v = Vec::new(); // Crea un vector vacio de enteros de 32 bits i32 por defecto

v.push(1); // Agrega el valor 1 al vector
v.push(2); // Agrega el valor 2 al vector
v.push(3); // Agrega el valor 3 al vector
```

Para poder leer los elementos de un vector se hace de 2 formas con comportamientos distintos, veamos:

```rust
let v = vec![1, 2, 3, 4, 5];

// Forma 1: Podemos acceder a los elementos del vector usando el operador de indexacion, pero si el indice que le pasamos no existe el programa se detendra con un panic
let third: &i32 = &v[2]; // Accedemos al tercer elemento del vector, que es el valor 3

// Logicamente v es una referencia inmutable en este caso cuyo puntero apunta al primer elemento del vector, por lo que podemos acceder a los elementos del vector usando el operador de indexacion.

// Forma 2: Con el metodo get() que devuelve un Option<&T> que nos permite manejar el caso en que el indice no exista sin que el programa se detenga con un panic (mas seguro)
let third: Option<&i32> = v.get(2); // Accedemos al tercer elemento del vector, que es el valor 3. Entonces como es un Option puede haber 2 casos, que el valor exista o que no exista y logicamente debemos manejar ambos casos, veamos:
match third {
    Some(val) => println!("El tercer elemento es {}", val), // Si el valor existe lo imprimimos
    None => println!("No existe el tercer elemento"), // Si no existe
}
```

La diferencia entre ambos metodos ya lo contamos y es que:

- **Forma 1**: Si el indice no existe el programa en plena ejecucion se detendra con un panic, lo que significa que el programa se cierra y no podemos manejar el error.

- **Forma 2**: Si el indice no existe el metodo get() devuelve un Option::None, lo que significa que podemos manejar el error y continuar con la ejecucion del programa. Por eso es mas seguro usar el metodo get() para acceder a los elementos de un vector.

Entonces el **get()** es la version **segura** que te obliga a manejar el caso invalido.

Hay una confusion muy comun con los vectores y es el tema de **referencias + mutaciones**, veamos un ejemplo:

```rust
// Esto codigo no compila:

let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0]; // Creamos una referencia inmutable al primer elemento del vector

v.push(6); // Intentamos agregar un elemento al vector, pero esto no compila porque estamos intentando mutar el vector que es mutable PEEERO mientras tenemos una referencia inmutable a uno de sus elementos!!
```

Porque no compilaria esto? La razon es porque cuando hacemos un *push* puede suceder que el vector no tenga suficiente espacio en memoria del heap (recordemos que un vector usa bloques de memoria contiguos), por lo tanto si ese es el caso Rust le pide al Sistema Operativo un bloque mas grande por lo que **copia TODOS LOS ELEMENTOS ahi y libera el bloque viejo**. Entonces como hace una copia de todos los elementos del vector y libera el bloque viejo, la referencia inmutable que teniamos al primer elemento del vector ahora apuntaria a un bloque de memoria que ya no existe, por lo que Rust no permite que tengamos una referencia inmutable a un elemento del vector mientras estamos mutando el vector (no importa que el vector sea mutable o no). Este tipo de fallas en memoria los detecta Rust en tiempo de compilacion cosa que C y C++ no hacen y por eso son tan propensos a errores de memoria.

¿Como iteramos un vector? Asi:

```rust
// Iteracion para lectura (inmutable)
let v = vec![100, 32, 57];
for i in &v {
  println!("{}", i); // Imprime 100, 32, 57
}

// Iteracion para modificar in-place (mutable)
let mut v = vec![100, 32, 57];
for i in &mut v {
  // Necesitamos desreferenciar (*) para poder modificar el valor al que apunta la referencia mutable
  *i += 50; // Modifica el valor de cada elemento del vector sumando 50
}
```

En el loop mutable la variable **i** es una referencia mutable a cada elemento del vector, por lo que para modificar el valor al que apunta debemos desreferenciarla con el operador `*`.

Hay un truco para almacenar multiples tipos y es hacer un **enum + vec** asi, *Vec\<T>* sabemos que solo acepta un tipo. Si necesitamos mezclar varios tipos en un vector podemos crear un enum que contenga todos los tipos que queremos almacenar y luego crear un vector de ese enum, osea:

```rust
enum MiEnum {
    Tipo1(i32),
    Tipo2(String),
    Tipo3(f64),
}

let row = vec![
    MiEnum::Tipo1(42),
    MiEnum::Tipo2(String::from("Hola")),
    MiEnum::Tipo3(3.14),
];
```

Podemos hacer esto porque si bien Rust necesita saber de antemano en tiempo de compilacion cuanta memoria ocupa cada elemento del vector, al usar un enum lo sabe porque todas las variantes son del mismo tipo de dato (el enum) y el compilador reserva espacio para la variante mas grande del enum.

Si los datos del vector se almacenan en el heap entonces ¿Cuando se libera la memoria del heap? La memoria del heap se libera automaticamente cuando el vector sale de su scope, es decir cuando la variable que contiene el vector deja de existir. Esto es parte del sistema de ownership de Rust, que garantiza que no haya fugas de memoria ni referencias colgantes. No hay un free manual como en C, osea:

```rust
{
    let v = vec![1, 2, 3, 4, 5]; // v es creado y ocupa memoria en el heap
} // v sale de scope y la memoria del heap se libera automaticamente
```



