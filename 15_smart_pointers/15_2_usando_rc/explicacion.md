# Rc\<T>: Multiple ownership
Ya vimos el smart pointer Box que nos permite tener un solo ownership de un valor en el heap, pero que pasa si queremos tener multiples owners de un mismo valor en el heap? Para eso tenemos el smart pointer `Rc<T>` que nos permite tener multiples owners de un mismo valor en el heap.

A veces necesitamos tener multiples partes de un programa que compartan los mismos datos, es por ello que este smart pointer es muy util.

Rc\<T> mantiene un **contador de referencias**. Cada vez que alguien quiere compartir los datos, el contrador sube. Cuando alguien deja de usarlos baja. Y cuando llega a 0 los datos se liberan automaticamente. Esto es muy util para evitar memory leaks y dangling pointers.

El problema con el smart pointer `Box<T>` es que transfiere el ownership, no podemos tener 2 owners del mismo dato. Veamos:

```rust
use std::rc::Rc;

enum List {
  Const(i32, Rc<List>),
  Nil,
}

fn main() {
  let a = Rc::new(List::Const(5, Rc::new(List::Nil))); // a es el owner de List::Const(5, Rc::new(List::Nil)) que esta guardado en el heap
  
  let b = List::Cons(3, Rc::clone(&a)); // b es el owner de List::Const(3, Rc::clone(&a)) que esta guardado en el heap. Rc::clone(&a) sube el contador de referencias de a

  let c = List::Cons(4, Rc::clone(&a)); // c es el owner de List::Const(4, Rc::clone(&a)) que esta guardado en el heap. Rc::clone(&a) sube el contador de referencias de a

} // Salimos del scope, se libera c, baja el contador de referencias de 'a' a 2, luego se libera b, baja el contador de referencias de 'a' a 1, y finalmente se libera a, baja el contador de referencias de 'a' a 0 y se libera la memoria del heap.
```

`Rc::clone(&a)` **no hace una copia profunda de los datos** solo incrementa el contador de referencias, es por eso que usamos `Rc::clone` en lugar de `a.clone()`. Si usas `a.clone()` estarías haciendo una copia profunda de los datos, lo cual no es lo que queremos.

> **NOTA**: Ojo, los datos en el heap son los mismos, es decir, basicamente tenemos varios punteros en el stack apuntando a los mismos datos en el heap y todos los punteros en el stack son owners de los datos en el heap. Cuando el contador de referencias llega a 0, los datos en el heap se liberan automaticamente.

Este contador de referencias lo vamos a ver en accion:

```rust
use std::rc::Rc;

fn main(){
  let a = Rc::new(5);
  println!("Contador de referencias de a: {}", Rc::strong_count(&a)); // Contador de referencias de a: 1

  let b = Rc::clone(&a);
  println!("Contador de referencias de a: {}", Rc::strong_count(&a)); // Contador de referencias de a: 2

  {
    let c = Rc::clone(&a);
    println!("Contador de referencias de a: {}", Rc::strong_count(&a)); // Contador de referencias de a: 3
  } // Salimos del scope, se libera c, baja el contador de referencias de 'a' a 2

  println!("Contador de referencias de a: {}", Rc::strong_count(&a)); // Contador de referencias de a: 2
}
```

El decremento al salir del scope es automatico, lo maneja el trait **Drop** que dijimos que lo tienen todos los smart pointers.

Ojo, `Rc<T>` solo permite **referencias inmutables**. Esto es porque si tenemos multiples ownerships con mutabilidad entonces corremos riesgos de data races. Si queremos tener multiples ownerships con mutabilidad entonces tenemos que combinarlo con `RefCell<T>` para tener mutabilidad en tiempo de ejecucion pero en un unico hilo.

Si queremos smart pointers con mutabilidad y multiples ownerships en un entorno multihilo entonces tenemos que usar otro smart pointer llamado `Arc<T>` que es un smart pointer thread-safe que tambien lo veremos mas adelante.

# RefCell\<T>: Mutabilidad interior
Recordemos las **reglas de borrowing** en Rust que son:

1. Podemos tener multiples referencias inmutables a un valor. (Varias &T)
2. Solo podemos tener una referencia mutable a un valor. (Un &mut T)
3. No podemos tener las 2 al mismo tiempo

El compilador de Rust verifica todo esto en tiempo de compilacion, el problema es que a veces el compilador es **demasiado conservador** y rechaza codigo que en realidad es correcto porque no puede probarlo estaticamente.

Para esto la solucion es el smart pointer **RefCell** donde le decimos al compilador *"confia en mi, yo me encargo de verificar las reglas de borrowing pero en runtime"*

Refcell lo podemos pensar como una caja con las siguientes caracteristicas:

```
RefCell<T>
┌─────────────────────────────┐
│  valor: T                   │  ← el valor que queremos guardar
│  conteo_lecturas: 0         │  ← cuántos &T activos hay (referencias inmutables)
│  hay_escritura: false       │  ← si hay un &mut T activo (referencia mutable)
└─────────────────────────────┘
```

Entonces cada vez que solicitamos acceso hacemos:

- `.borrow()` → para obtener una referencia inmutable al valor. Esto incrementa el `conteo_lecturas` en 1. Si `hay_escritura` es true entonces panic ya que como dijimos no podemos tener referencias inmutables y mutables al mismo tiempo (se rompe la regla 3 de borrowing)

- `.borrow_mut()` → para obtener una referencia mutable al valor. Esto pone `hay_escritura` en true. Si `conteo_lecturas` es mayor a 0 entonces panic ya que como dijimos no podemos tener referencias inmutables y mutables al mismo tiempo (se rompe la regla 3 de borrowing), o por ejemplo si ya hay una referencia mutable activa y alguien quiere otra referencia mutable entonces panic ya que como dijimos solo podemos tener una referencia mutable a un valor (se rompe la regla 2 de borrowing)

- Cuando el acceso sale del scope, el `conteo_lecturas` baja en 1 o `hay_escritura` vuelve a false dependiendo de si era una referencia inmutable o mutable.

Si violamos alguna de las reglas de borrowing, el programa hace panic en tiempo de ejecucion en lugar de error en compilacion.

Entonces la diferencias con las referencias normales y RefCell es que:

```rust
// Con referencias normales el compilador verifica las reglas de borrowing en tiempo de compilacion
let mut x = 5;
let y = &x; // referencia inmutable
let z = &mut x; // referencia mutable, esto es un error de compilacion porque
// no podemos tener referencias inmutables y mutables al mismo tiempo (rompemos la regla 3 de borrowing)

// Con RefCell le decimos al compilador que confie en nosotros y no verifique las reglas de borrowing en tiempo de compilacion, sino en tiempo de EJECUCION, y si alguna regla falla entonces hace panic
let x = RefCell::new(5);
let y = x.borrow(); // referencia inmutable, el Ref<i32> seria como un &i32
let z = x.borrow_mut(); // referencia mutable, esto es un panic en tiempo de ejecucion porque no podemos tener referencias inmutables y mutables al mismo tiempo (rompemos la regla 3 de borrowing)
```

La diferencia de una referencia normal con RefCell es el **CUANDO** se detecta el error, no si se detecta o no.

Veamos un ejemplo usando RefCell:

```rust
use std::cell::RefCell;

fn main() {
  let dato = RefCell::new(vec![1, 2, 3]); // Creamos un RefCell que contiene un vector en el heap

  {
    let lectura = dato.borrow(); // referencia inmutable, seria como un &Vec<i32>
    println!("Valor: {:?}", *lectura); // Valor: [1, 2, 3]
  } // lectura se libera al salir del scope  


  {
    let mut escritura = dato.borrow_mut(); // referencia mutable, seria como un &mut Vec<i32>
    escritura.push(4); // al ser una referencia mutable podemos modificar el valor

  }  // escritura se libera al salir del scope, ahora podemos hacer otra referencia inmutable o mutable

  // Leemos el valor nuevamente
  println!("{:?}", dato.borrow()); // Valor: [1, 2, 3, 4]
}
```

Los `{}` son importantes para liberar las referencias inmutables o mutables y poder hacer otras referencias. Si no los ponemos, el programa hace panic en **tiempo de ejecucion**.

Veamos otro ejemplo, supongamos que tenemos el sig. trait:

```rust
pub trait Messenger {
  fn send(&self, msg: &str); // La referencia a self es inmutable, por lo tanto no podemos modificar el estado interno del struct que implementa este trait
}
```

Y queremos crear un mock para testear que el `send` se llama correctamente:

```rust
struct MockMessenger {
  mensajes: Vec<String>, // Queremos guardar los mensajes enviados para testear que se llamo correctamente el send
}

// Le implementamos el trait Messenger a MockMessenger
impl Messenger for MockMessenger {
  fn send(&self, msg: &str) {
    self.mensajes.push(String::from(msg)); // Esto es un error de compilacion porque la referencia a self es inmutable, por lo tanto no podemos modificar el estado interno del struct
  }
}
```

¿La solucion? Usar RefCell para poder modificar el estado interno del struct aunque la referencia a self sea inmutable:

```rust
use std::cell::RefCell;

struct MockMessenger {
  mensajes: RefCell<Vec<String>>, // Ahora podemos modificar el estado interno del struct aunque la referencia a self sea inmutable. Vemos que el Vec vive dentro de un RefCell
}

impl MockMessenger {
  // Metodo constructor
  fn new() -> MockMessenger {
    MockMessenger {
      mensajes: RefCell::new(vec![]), // Inicializamos el RefCell con un vector vacio
    }
  }
}

// Implementamos el trait Messenger a MockMessenger
impl Messenger for MockMessenger {
  fn send(&self, msg: &str) {
    self.mensajes.borrow_mut().push(String::from(msg)); // Ahora podemos modificar el estado interno del struct aunque la referencia a self sea inmutable. Vemos que el Vec vive dentro de un RefCell, por lo tanto podemos hacer borrow_mut() para obtener una referencia mutable al Vec
  }
}

#[test]
fn test_aviso_75_porciento() {
  let mock = MockMessenger::new(); // Creamos un MockMessenger

  assert_eq!(mock.mensajes.borrow().len(), 0); // Verificamos que el vector de mensajes esta vacio. Al hacerle mock.mensajes.borrow() estamos obteniendo una referencia inmutable al vector de mensajes, por lo tanto podemos leer su longitud.

  mock.send("Hola, mundo!"); // Enviamos un mensaje

  assert_eq!(mock.mensajes.borrow().len(), 1); // Verificamos que el vector de mensajes tiene un mensaje. Al hacerle mock.mensajes.borrow() estamos obteniendo una referencia inmutable al vector de mensajes, por lo tanto podemos leer su longitud.
}
```

Es decir, el `struct MockMessenger` es **exactamente inmutable** pero **internamente mutable** gracias a RefCell. Esto es lo que se conoce como **mutabilidad interior**.

Hagamos un repaso de las reglas del borrowing con RefCell:

```rust
let c = RefCell::new(5);

// Multiples lecturas simultaneas (referencias inmutables) son permitidas
let r1 = c.borrow();
let r2 = c.borrow(); // Esto es permitido, ahora tenemos 2 referencias inmutables al mismo tiempo
println!("r1: {}, r2: {}", *r1, *r2); // r1: 5, r2: 5

// Leer y escribir al mismo tiempo lanza PANIC
let r3 = c.borrow(); // Esto es permitido, ahora tenemos 3 referencias inmutables al mismo tiempo
let r4 = c.borrow_mut(); // Esto es un panic en tiempo de ejecucion porque no podemos tener referencias inmutables y mutables al mismo tiempo (rompemos la regla 3 de borrowing)
```

Tambien 2 escrituas al mismo tiempo lanza panic:

```rust
let c = RefCell::new(5);
let r1 = c.borrow_mut(); // Esto es permitido, ahora tenemos 1 referencia mutable
let r2 = c.borrow_mut(); // Esto es un panic en tiempo de ejecucion porque no podemos tener 2 referencias mutables al mismo tiempo (rompemos la regla 2 de borrowing)
```

Si podemos tener una escritura despues de una lectura que haya terminado:

```rust
let c = RefCell::new(5);

{
  let r1 = c.borrow(); // Esto es permitido, ahora tenemos 1 referencia inmutable
}

let r2 = c.borrow_mut(); // Esto es permitido, ahora tenemos 1 referencia mutable porque la referencia inmutable anterior ya salio del scope
```

Sabemos que RefCell nos da mas libertades pero tambien nos da mas responsabilidades, por ejemplo un error muy comun es **tener 2 borrows activos al mismo tiempo** sin darse cuenta:

```rust
let c = RefCell::new(vec![1, 2, 3]);

let lectura = c.borrow();
println!("{:?}", lectura);

c.borrow_mut().push(4); // Esto es un panic en tiempo de ejecucion porque no podemos tener referencias inmutables y mutables al mismo tiempo (rompemos la regla 3 de borrowing)
```

La solucion es asegurarse de que el primer borrow termine antes y es justamente usando scopes con `{}`

```rust
let c = RefCell::new(vec![1, 2, 3]);

{
    let lectura = c.borrow();
    println!("{:?}", lectura);
}   // lectura termina acá

c.borrow_mut().push(4);  // OK
```

## Cuando usar RefCell?

Cuando usarlo:

- Cuando tenemos una interfaz con &self inmutable pero necesitamos mutar el estado interno (como el ejemplo del mock Messenger)

- El compilador rechaza codigo correcto porque no puede probar que las reglas de borrowing se cumplen

- Estas seguro de la logica de que nunca vamos a tener 2 borrows conflictivos al mismo tiempo.

No usarlo cuando:

- Podriamos hacer simplemente un &mut self

- Podriamos reestructurar el codigo para que el compilador lo acepte normalmente sin usar RefCell

## Combinacion Rc + RefCell: Multiples owners con mutabilidad interior (unico hilo)
Este es un patron muy poderoso. Sabemos que Rc nos da multiples owners de un mismo valor en el heap (solo lectura) y RefCell nos da mutabilidad interior (solo un owner). Si combinamos ambos podemos tener multiples owners de un mismo valor en el heap con mutabilidad interior. Veamos un ejemplo:

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
  Cons(Rc<RefCell<i32>>, Rc<List>), // El valor es un Rc<RefCell<i32>> Basicamente podemos tener multiples owners gracias a Rc y ademas mutabilidad interior gracias a RefCell
  Nil,
}

fn main() {
  let value = Rc::new(RefCell::new(5)); // Creamos un Rc<RefCell<i32>> que contiene el valor 5


  let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil))); // 'a' es el owner de la lista que contiene el valor 5.

  let b = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)); // 'b' es el owner de la lista que contiene el valor 3 y apunta a 'a'. 'b' tiene su propio valor 3 pero comparte el ownership del valor 5 con 'a' gracias a Rc.

  let c = List::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a)); // 'c' es el owner de la lista que contiene el valor 4 y apunta a 'a'. 'c' tiene su propio valor 4 pero comparte el ownership del valor 5 con 'a' gracias a Rc.

  *value.borrow_mut() += 10; // Modificamos el valor dentro del RefCell, ahora es 15. Esto es visible para todos los owners de value, es decir, a, b y c.

  println!("a: {:?}", a); // a: Cons(RefCell { value: 15 }, Nil)
  println!("b: {:?}", b); // b: Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
  println!("c: {:?}", c); // c: Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil)) 
}
```

Lo que logramos es que *a, b* y *c* compartan el ownership del mismo dato (via Rc) y ese dato es mutable (via RefCell). Cuando modificamos *value* el cambio es visible desde las 3 listas.

Veamos otro ejemplo para entender mejor la combinacion de Rc + RefCell:

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
  let shared_value = Rc::new(RefCell::new(10)); // Creamos un valor compartido en el heap con mutabilidad interior

  let owner1 = Rc::clone(&shared_value); // owner1 comparte el ownership del valor
  let owner2 = Rc::clone(&shared_value); // owner2 comparte el ownership del valor

  println!("Valor inicial: {}", shared_value.borrow()); // Valor inicial: 10

  *owner1.borrow_mut() += 5; // Modificamos el valor desde owner1
  println!("Valor después de owner1: {}", shared_value.borrow()); // Valor después de owner1: 15

  *owner2.borrow_mut() *= 2; // Modificamos el valor desde owner2
  println!("Valor después de owner2: {}", shared_value.borrow()); // Valor después de owner2: 30
}
```

# Resumen de los 3 smart pointers

| |Box|Rc|RefCell|
|---|---|---|-------|
|**Owners**|1|Multiples|1|
|**Borrows**|mut o inmut|inmut|mut o inmut|
|**Verificacion**|Tiempo compilacion|Tiempo compilacion|Tiempo ejecucion|
|**Threads**|Single/Multi|Single|Single|
|**Multithrading**|N/A|Arc|Mutex|