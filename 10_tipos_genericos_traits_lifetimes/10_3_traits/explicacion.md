Un trait es lo mas parecido a una **interfaz** en otros lenguajes de programacion. Un trait **define comportamiento que un tipo puede tener**. Es el mecanismo de Rust que tiene para decir *"este tipo sabe hacer X"*. La analogia mas cercana con otros lenguajes es:

- En C: No hay nada pero se puede simular con punteros a funciones.

- En Java/Go: Serian las **interfaces**.

- En Python: Serian las **clases abstractas**.

Un trait se lo define de la siguiente manera:s

```rust
pub trait Resumen {
  fn resumir(&self) -> String; // Definimos un metodo resumir() que devuelve un String, pero no le damos implementacion, solo colocamos la firma del metodo, osea que los tipos que implementen este trait van a tener que implementar este metodo resumir() y devolver un String
}
```

Esto define que cualquier tipo que implemente *Resumen* debe tener un metodo *resumir()* que devuelva un String. El cuerpo como vimos esta vacio, es solo la firma, un contrato.

Entonces ahora vamos a ver un ejemplo de como implementar un trait en un tipo:

```rust
pub trait Resumen {
  fn resumir(&self) -> String;
}

pub struct ArticuloNoticioso {
  pub titulo: String,
  pub autor: String,
  pub contenido: String,
}

// Implementamos el metodo resumir() para el struct ArticuloNoticioso, que es el tipo que implementa el trait Resumen
impl Resumen for ArticuloNoticioso {
  // Al ser una implementacion de un trait tenemos que implementar si o si el metodo resumir(). Es aca donde justamente definimos el comportamiento
  fn resumir(&self) -> String {
    format!("{} por {}", self.titulo, self.autor)
  }
}

// Tambien podemos tener otro struct e implementarle el mismo trait Resumen, eso quiere decir que tendra el mismo metodo resumir() pero con un comportamiento distinto podria ser:
pub struct Tweet {
  pub usuario: String,
  pub contenido: String,
  pub retweets: u32,
  pub likes: u32,
}

// Implementamos el trait Resumen al struct Tweet ahora, tranquilamente el metodo resumir() podria hacer otra cosa
impl Resumen for Tweet {
  fn resumir(&self) -> String {
    format!("{}: {}", self.usuario, self.contenido)
  }
}
```

Entonces vemos que ambos structs implementan el mismo trait Resumen, pero cada uno tiene su propia implementacion del metodo resumir().

Recordar que al implementarle un trait a un struct la sintaxis es **impl \<Trait> for \<Tipo>**

Otra cuestion importante es que podemos definir una implementacion por defecto de un metodo en un trait, osea que si un struct implementa ese trait y no implementa ese metodo, va a usar la implementacion **por defecto**, por ejemplo:

```rust
pub trait Resumen {
  // Ahora no es solo la firma de la funcion en el trait sino que ademas le definimos cierta logica POR DEFECTO
  fn resumir(&self) -> String {
    String::from("(Leer mas...)") // Esta es la implementacion por defecto del metodo
  }
}

struct ArticuloNoticioso {
  pub titulo: String,
  pub autor: String,
  pub contenido: String,
}

impl Resumen for ArticuloNoticioso {
  // No implementamos el metodo resumir() en este struct, entonces el struct ArticuloNoticioso, al implementar el trait Resumen, va a usar la implementacion por defecto en el metodo resumir()
}
```

Tambien podemos hacer que una implementacion por defecto pueda llamar a otros metodos del mismo trait, incluso si esos otros metodos no tienen default, osea:

```rust

// Vemos que el metodo autor() lo definira el struct que implemente el trait Resumen, pero el metodo resumir() vemos que tiene comportamiento por defecto, pero ademas llama al metodo autor() que no tiene default y lo definira el struct que implemente el trait Resumen, entonces el metodo resumir() va a usar la implementacion por defecto pero va a llamar al metodo autor() que sera definido en el struct que implemente el trait Resumen
pub trait Resumen {
    fn autor(&self) -> String;  // sin default, obligatorio

    fn resumir(&self) -> String {  // con default, usa autor()
        format!("(Leer más de {}...)", self.autor())
    }
}
```

De esta ultima manera forzamos a que cada tipo implemente solo *autor()* y el *resumir()* se obtiene el por defecto.

Algo muy ventajoso de los traits es que **los traits pueden ser usados como parametros de funciones**. Esto seria funciones que reciben traits como parametros!! Por ejemplo:

```rust
pub fn notificar(item: &impl Resumen) { // La funcion notificar() recibe un parametro item que es una referencia a un tipo que implementa el trait Resumen, osea que item puede ser de cualquier tipo que implemente el trait Resumen, como por ejemplo ArticuloNoticioso o Tweet
    println!("Noticia: {}", item.resumir()); // Llamamos al metodo resumir() del trait Resumen, que sera el metodo implementado por el tipo concreto de item
}
```

Entonces asi logramos como esta funcion solamente acepte tipos de datos ArticuloNoticioso como Tweer, o cualquier otro tipo que implemente el trait Resumen. Esto es muy util para poder escribir funciones genericas que puedan trabajar con distintos tipos de datos que compartan un mismo comportamiento definido por un trait. A esto se le llama **trait bounds** (restricciones de trait)

Tambien otra alternativa para definir parametros de funciones que sean de un tipo que implemente un trait es la siguiente, y es usando los tipos de datos genericos, por ejemplo:

```rust
pub fn notificar<T: Resumen>(item: &T) { // La funcion notificar() recibe un parametro item que es una referencia a un tipo generico T, y T tiene que implementar el trait Resumen, osea que item puede ser de cualquier tipo generico T que implemente el trait Resumen, como por ejemplo ArticuloNoticioso o Tweet
    println!("Noticia: {}", item.resumir()); // Llamamos al metodo resumir() del trait Resumen, que sera el metodo implementado por el tipo concreto de item
}
```

Ahora bien, estas 2 ultimas alternativas representan lo mismo, basicamente un parametro de funcion que es de un tipo que implementa un trait particular. Sin embargo cuando la funcion empieza a recibir varios parametros ahi se nota la diferencia entre los 2 metodos, veamoslo:

```rust
// En este caso la funcion recibe 2 parametros item1 y item2 que son referencias a tipos que implementan el trait Resumen, pero no necesariamente tienen que ser del mismo tipo, osea item1 podria ser ArticuloNoticioso (ya que implementa el trait Resumen) y item2 podria ser Tweet (ya que tambien implementa el trait Resumen)
pub fn notificar(item1: &impl Resumen, item2: &impl Resumen) {}

// En este otro caso la funcion tambien recibe 2 parametros item1 y item2 que son referencias a tipos genericos T y T, y que ademas ambos tipos genericos tienen que implementar el trait Resumen, pero como se trata del mismo tipo generico T entonces si o si tiene que ser el mismo tipo que implementa el trait Resumen, osea por ejemplo item1 y item2 podrian ser ambos Tweet o ambos ArticuloNoticioso, pero no uno de cada tipo.
pub fn notificar<T: Resumen>(item1: &T, item2: &T) {}
```

No solamente podemos hacer que una funcion reciba como parametros tipos de datos que implementen un tipo de trait, sino que implementen varios tipos de traits, por ejemplo:

```rust
// En este caso al funcion recibe como parametro una referencia a un tipo de dato que impleemnta el trait Resumen + Display:
pub fn notificar(item: &(impl Resumen + Display)) {}

// En este otro caso es lo mismo pero ahora usando tipos genericos. La funcion recibe como parametro una referencia al tipo generico T que implementa el trait Resumen + Display.
pub fn notificar<T: Resumen + Display>(item: &T) {}
```

Puede suceder que la funcion reciba parametros donde se tengan en cuenta muchos traits, por ejemplo en este siguiente caso la funcion recibe como parametro 2 referencias a tipos genericos T y U donde T implementa el trait Display + Clone y U implementa el trait Clone + Debug, entonces la firma de la funcion seria asi:

```rust
pub fn notificar<T: Display + Clone, U: Clone + Debug>(item1: &T, item2: &U) {}
```

Como vemos la firma de la funcion es muy larga, en ese caso cuando se da esta situacion tenemos la clausula **WHERE** donde aporta una mayor legibilidad para expresar lo mismo, por ejemplo podemos hacer:

```rust
// Por ejemplo en este caso la funcion recibe 2 referencias a tipos genericos donde T implementa el trait Display + Clone y U implementa el trait Clone + Debug, pero ahora usando la clausula WHERE para mejorar la legibilidad de la firma de la funcion. La clausula WHERE tiene que siempre antes de la llave de apertura de la funcion
pub fn notificar<T, U>(item1: &T, item2: &U) -> String
where
    T: Display + Clone,
    U: Clone + Debug,
{}
```

El unico proposito de where es unicamente mejorar la legibilidad, no aporta nada al comportamiento, la logica es la misma.

Hasta ahora vimos funciones que reciben parametros donde tienen que cumplir con ciertos traits, pero ademas en el retorno de la funcion podemos establecer que el tipo de retorno de la funcion debe implementar un trait particular, por ejemplo:

```rust
// En este caso la funcion crear_resumen() recibe como parametro nada y devuelve un tipo de dato que implementa el trait Resumen, osea que el tipo de retorno de la funcion puede ser cualquier tipo que implemente el trait Resumen, como por ejemplo ArticuloNoticioso o Tweet
pub fn crear_resumen() -> impl Resumen {
    // Podemos devolver la instancia de Tweet ya que implementa el trait Resumen
    Tweet {
        usuario: String::from("usuario123"),
        contenido: String::from("Este es un tweet de ejemplo"),
    }
}
```

Pero ojo, esto tiene una unica restriccion y es que **el tipo de retorno de la funcion debe ser siempre el mismo tipo concreto que implementa el trait**, osea que no podemos hacer algo como:

```rust
// Si bien la funcion devuelve en ambos casos del if else un tipo que implementa el trait Resumen, en este caso el tipo de retorno de la funcion no es siempre el mismo tipo concreto que implementa el trait, ya que en el if devolvemos un Tweet y en el else devolvemos un ArticuloNoticioso, entonces esto NO COMPILARIA.
pub fn crear_resumen() -> impl Resumen {
    if condicion {
        Tweet {
            usuario: String::from("usuario123"),
            contenido: String::from("Este es un tweet de ejemplo"),
        }
    } else {
        ArticuloNoticioso {
            titulo: String::from("Titulo del articulo"),
            autor: String::from("Autor del articulo"),
            contenido: String::from("Contenido del articulo"),
        }
    }
}
```

Por ultimo podemos implementar metodos en un tipo generico solo cuando el tipo concreto implemente ciertos traits:

```rust
// Creamos el tipo Par que recibe un tipo generico T
struct Par<T> {
    x: T,
    y: T,
}

// Al tipo le implementamos un metodo solo cuando el tipo generico T implemente el trait Display y PartialOrd, osea que el metodo sera valido solo para tipos concretos que implementen esos traits, por ejemplo si T = i32 entonces si es valido ya que i32 en la libreria estandar implementa Display y PartialOrd, pero si T = Vec<i32> entonces no es valido ya que Vec<i32> no implementa Display
impl<T: Display + PartialOrd> Par<T> {
    fn comparar(&self) {
        if self.x >= self.y {
            println!("El mayor es x: {}", self.x);
        } else {
            println!("El mayor es y: {}", self.y);
        }
    }
}
```

> **NOTA**: Hay una regla que se llama regla de **orphan** que quiere decir que podemos implementar un trait para un tipo de dato del la libreria estandar (`impl MiTrait for Vec\<T>)`), tambien podemos implementar un trait de la libreria estandar para un tipo NUESTRO (`impl Display for MiStruct`) sin embargo NO PODEMOS implementar un trait de la libreria estandar para un tipo de dato de la libreria estandar (`impl Display for Vec\<T>`)
