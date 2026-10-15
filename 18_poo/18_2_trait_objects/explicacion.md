# Trait Objects
En Rust si quisieramos tener una lista con elementos de distintos tipos **no podemos**, por ejemplo si planeamos usar un `Vec<T>` si o si `T` debe ser **del mismo tipo**. Los **trait objects** son la solucion a este problema.

Imaginemos que tenemos distintos aparatos electronicos en casa, si bien todos son distintos tienen algo en comun: **Se pueden apagar**. Entonces podriamos tener un cajon con todos controles remotos mezclados y no importa cual agarremos siempre sabemos que tiene un boton de apagado, no necesitamos saber si el control remoto es de TV, aire, etc. solo sabemos que funciona. Esto es exactamente un trait object

Veamos un ejemplo:

```rust

// El boton 'apagar' seria el trait donde justamente tiene un metodo que nos permite apagar el aparato electronico
trait Apagable {
  fn apagar(&self);
}

// Los distintos aparatos electronicos
struct TV;
struct AireAcondicionado;
struct Parlante;

// Ahora implementamos el metodo apagar para c/u de los aparatos electronicos
impl Apagable for TV {
  fn apagar(&self) {
    println!("TV apagada");
  }
}

impl Apagable for AireAcondicionado {
  fn apagar(&self) {
    println!("Aire apagado");
  }
}

impl Apagable for Parlante {
  fn apagar(&self) {
    println!("Parlante apagado");
  }
}

// Ahora podemos tener un vector de trait objects, basicamente le decimos al vector que va a contener elementos que implementen el trait Apagable, y no importa el tipo de cada elemento, solo que implemente el trait

let cosas: Vec<Box<dyn Apagable>> = vec![
  Box::new(TV), // Recordar que al ser el smart pointer box eso quiere decir que el objeto al que apuntan vive en el heap y el puntero vive en el stack, esto es importante porque los trait objects no tienen un tamaño conocido en tiempo de compilacion, entonces no podemos tenerlos en el stack, por eso usamos box
  Box::new(AireAcondicionado),
  Box::new(Parlante)
];

// Ahora iteramos por el vector y llamamos al metodo apagar de cada elemento, todos sabemos que implementan el trait Apagable, entonces podemos llamar al metodo apagar sin importar el tipo de cada elemento
for cosa in cosas {
  cosa.apagar();
}
```

Entonces la salida seria algo asi:

```
TV apagada
Aire apagado
Parlante apagado
```

Una pregunta es porque necesitamos un `Box`?. Rust necesita saber en tiempo de compilacion cuando espacio ocupa cada elemento en memoria. Una TV puede ocupar 8 bytes, un aire acondicionado 24 bytes, y un parlante 16 bytes. Si mezclamos todos estos tipos en un vector entonces ¿Como sabria Rust cuanto espacio reservar para cada elemento? `Box` justamente resuelve esto, sabemos que `Box` es un puntero de tamaño fijo (8 bytes en sistemas de 64 bits) que apunta a un objeto en el heap. Por lo tanto lo que guardamos en el vector son punteros de tamaño fijo que c/u apunta a un objeto distinto en el heap.

Que diferencia hay entre los trait objects y usar datos genericos? Pues que en los datos genericos estamos **obligados a usar un solo tipo** para todos los elementos, por ejemplo:

```rust
let cosas: Vec<TV> = vec![TV, TV, TV]; // Todos los elementos son del mismo tipo

// En cambio con trait objects podemos tener distintos tipos de elementos que implementen el mismo trait, por ejemplo:
let cosas: Vec<Box<dyn Apagable>> = vec![
  Box::new(TV),
  Box::new(AireAcondicionado),
  Box::new(Parlante)
];
```

Que pasaria si intentamos meter en el vector un elemento que no implementa el trait Apagable? Por ejemplo:

```rust
let cosas: Vec<Box<dyn Apagable>> = vec![
  Box::new(TV),
  Box::new(AireAcondicionado),
  Box::new(Parlante),
  Box::new(String::from("Hola")) // Esto no compila porque String no implementa el trait Apagable
];
```

Nos sucedera un error en tiempo de compilacion, el compilador de Rust lo detecta antes de ejecutar

En resumidas cuentas el trait object vs datos genericos seria:

||Generico: `<T: Trait>`|Trait Object: `Box<dyn Trait>`|
|---|---|---|
|Tipos en la lista|Solo uno|Mezcla de tipos|
|Cuando se resuelve|En compilacion|Runtime|
|Performance|Maxima|Levemente menor|
|Flexibilidad|Limitada|Alta|