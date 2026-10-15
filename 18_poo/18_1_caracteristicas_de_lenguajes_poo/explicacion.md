La **POO** es una forma de modelar programas. Basicamente tenemos objetos que se envian mensajes entre si, pero la clave son justamente los **objetos**.

Ahora veremos que papel juega Rust en la POO y como se diferencia de otros lenguajes.

La gran pregunta incomoda es **¿Es Rust un lenguaje orientado a objetos?**. La respuesta es que dependiendo de que entendemos por POO.

## Objetos = Datos + Comportamiento: Lo cumple Rust
Si nos aferramos a la sig. definicion que dice: *"Los programas orientados a objetos estan compuestos por objetos. Un objeto empaqueta tanto datos como los procedimientos que operan en esos datos"*

En Rust los `structs` y los `enums` tienen datos encapsulados, los bloques `impl` les agregan **metodos**. Eso es exactamente lo que describe la definicion de POO. Por lo tanto podemos decir que segun ese criterio **Rust cumple con ser un lenguaje POO**, por ejemplo:

```rust
// Datos encapsulados en un struct
pub struct AveragedCollection {
  list: Vec<i32>,
  average: f64,
}

// Comportamientos
impl AveragedCollection {
  pub fn add(&mut self, value: i32) { ... }
  pub fn remove(&mut self) -> Option<i32> { ... }
  pub fn average(&self) -> f64 { ... }
}
```

## Encapsulacion: Lo cumple Rust
La encapsulacion significa que los detalles internos de un objeto no son accesibles desde fuera, solo se puede interactuar a traves de su API publica. En Rust esto se logra con el conocido `pub` donde si recordamos de las primeras unidades dijimos que **todo por defecto es pivado** y solo con `pub` exponemos lo que queremos que sea publico.

En el ultimo codigo de ejemplo si nos fijamos, el struct `AveragedCollection` tiene dos campos `list` y `average` que son privados, y solo podemos interactuar con ellos a traves de los metodos `add`, `remove` y `average`. Esto es encapsulacion.

```rust
// Con campos privados, esto no es posible desde fuera
coleccion.list.push(1); // Error, list es privado

// Solo podemos interactuar con la coleccion a traves de sus metodos publicos
coleccion.add(1);
```

La ventaja de esto es que podemos cambiar completamente la implementacion interna (por ejemplo reemplazar `Vec<i32>` por `HashSet<i32>`) sin que el codigo externo lo note, siempre que la API publica permanezca igual. Rust cumple con esto.

## Herencia: No lo cumple Rust
Aca Rust se aparta del POO clasico. **NO EXISTE LA HERENCIA EN LOS STRUCTS**, no podemos hacer que un struct herede los campos y metodos de otro struct sin usar una macro.

Porque usariamos herencia? Por 2 razones:

- **Reutilizar codigo**: En Rust podemos usar **traits** para definir comportamientos que pueden ser implementados por multiples structs. Esto nos permite compartir codigo sin necesidad de herencia. Por ejemplo:

  ```rust
  pub trait Summary {
      fn summarize(&self) -> String {
          String::from("(Leer más...)")  // implementación por defecto
      }
  }

  // Cualquier tipo que implemente Summary tiene summarize() gratis
  struct Tweet { ... }
  impl Summary for Tweet {}  // usa la implementación por defecto
  ```

  Esto ultimo es equivalente a una clase padre con un metodo implementado que las subclases heredan. Pero en Rust no hay herencia, sino que cada struct implementa el trait y obtiene el comportamiento.

- **Polimorfismo**: Podemos usar un tipo hijo donde se espera el tipo padre. En Rust esto se logra con **trait objects** que lo veremos un poco mas adelante.

Pero porque no tenemos una herencia tradicional en Rust? Bueno Rust decidio no incluir la herencia clasica por lo siguiente:

La herencia fuerza a que una subclase herede **TODO** lo de la clase padre, aunque no todo tenga sentido para ella. Esto puede llevar a:

1. Metodos heredados que no tienen sentido para la subclase

2. Diseños rigidos que son dificiles de modificar

3. Cambiar la clase padre puede romper todas las subclases, lo que hace que el codigo sea dificil de mantener

Es por ello que muchos lenguajes modernos (incluidos Rust) se estan alejando de la herencia clasica y prefieren usar **traits** como mecanismo principal

### Polimorfismo: Parametrico vs Dinamico
Hay una distincion importante de polimorfismo en Rust que es la siguiente:

- **Polimofismo parametrico**: Es cuando el compilador sabe en tiempo de compilacion que tipo de objeto estamos usando. Esto es lo que ocurre cuando usamos genericos y traits. Por ejemplo:

  ```rust
  // La funcion notificar puede recibir cualquier tipo que implemente Summary. Por lo tanto si implementa Summary sabemos que tiene el metodo summarize() y podemos llamarlo. Esto es polimorfismo parametrico.
  fn notificar<T: Summary>(item: &T) {
    println!("{}", item.summarize());
  }
  ```

  En este caso es parametrico ya que el compilador sabe en tiempo de compilacion que tipo de objeto estamos usando y puede optimizar el codigo. El compilador genera una version de la funcion `notificar` para cada tipo que usemos que implemente el trait `Summary`, por lo tanto no hay overhead en tiempo de ejecucion.

- **Polimorfismo dinamico**: Es cuando el compilador no sabe en tiempo de compilacion que tipo de objeto estamos usando. Esto ocurre cuando usamos **trait object**. Por ejemplo:

  ```rust
  // La funcion notificar puede recibir cualquier tipo que implemente Summary. Pero en este caso no sabemos en tiempo de compilacion que tipo de objeto estamos usando, por lo tanto el compilador no puede optimizar el codigo y hay un overhead en tiempo de ejecucion.
  fn notificar(item: &dyn Summary) {
    println!("{}", item.summarize());
  }
  ```

  En este caso es dinamico ya que el compilador no sabe en tiempo de compilacion que tipo de objeto estamos usando y tiene que hacer una busqueda en tiempo de ejecucion para encontrar el metodo `summarize()`. Esto tiene un overhead en tiempo de ejecucion, pero nos permite tener mas flexibilidad. Esto de los traits objects lo veremos luego.