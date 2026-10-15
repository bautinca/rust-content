# Implementando un patron de diseño orientado a objetos
Que es el **State Pattern**? Basicamente es un patron de diseño que permite que un objeto **cambia su comportamiento segun su estado interno** pero desde fuera siempre se ve igual.

La analogia mas simple es un semaforo ya que siempre se ve igual, siempre tiene la misma interfaz pero su comportamiento cambia segun su estado (rojo, amarillo, verde). Si esta en verde los autos pueden pasar, si esta en rojo los autos deben detenerse y si esta en amarillo los autos deben reducir la velocidad. Y en ningun momento necesitamos saber que tipo de semaforo es ni en que estado esta para interactuar con el.

Vamos a construir un sistema con las sig. reglas:

1. Un post empieza como borrador (`Draft`)
2. Cuando este listo se pide una revision (`PendingReview`)
3. Una vez aprobado se publica (`Published`)
4. Solo los posts publicados muestran su contenido

La clave es que el codigo que usa el `Post` no necesita saber en que estado esta, siempre llama a metodos y el comportamiento correcto ocurre automaticamente:

```rust
let mut post = Post::new(); // El post arranca como borrador

post.add_text("I ate a salad today"); // Agregamos texto al borrador
assert_eq!("", post.content()); // El contenido del post es vacio porque esta en borrador

post.request_review(); // Se pide una revision al borrador
assert_eq!("", post.content()); // El contenido del post sigue siendo vacio porque esta en revision

post.approve(); // Se aprueba el post
assert_eq!("I ate a salad today", post.content()); // El contenido del post ahora es visible porque esta publicado
```

El tructo esta en que `Post` guarda su estado como un trait object:

```rust
pub struct Post {
  state: Option<Box<dyn State>>, // Es el estado actual del Post y si nos fijamos es un TRAIT OBJECT. Basicamente es un puntero a un objeto que implementa el trait State. Esto nos permite cambiar el comportamiento del Post sin cambiar su interfaz.
  content: String, // El contenido del post
}

// En el trait state declaramos los metodos request_review, approve y content
trait State {
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn approve(self: Box<Self>) -> Box<dyn State>;
  fn content<'a>(&self, post: &'a Post) -> &'a str { "" } // Por defecto el contenido de un post es vacio, pero cuando el post esta en estado publicado (Published) retornamos el contenido del post.
}

// Implementamos los distintos estados del post como structs que implementan el trait State
struct Draft {}
struct PendingReview {}
struct Published {}
```

Entonces basicamente un `Post` internamente tiene el `state` que es un **trait object** ya que es un puntero `Box` a un objeto que implementa el trait `State`. Esto nos permite cambiar el comportamiento del `Post` sin cambiar su interfaz.

Entonces ahora vamos a implementar el trait `State` para cada uno de los estados del post:

```rust
// El post esta en estado de borrador (Draft)
impl State for Draft {
  // Cuando se pide una revision, el post pasa a estado de revision (PendingReview), es por ello que retornamos un Box con el nuevo estado PendingReview. Logicamente seria un puntero Box al estado PendingReview.
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview {})
  }


  // Cuando se aprueba un post en estado de borrador, no pasa a ningun otro estado, por lo que retornamos el mismo estado (Draft) como un Box.
  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }
}

// El post esta en estado de revision (PendingReview)
impl State for PendingReview {
  // Cuando un post en estado de revision se le pide revision entonces logicamente no pasa a ningun otro estado, por lo que retornamos el mismo estado (PendingReview) como un Box.
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  // Cuando un post en estado de revision es aprobado, pasa a estado publicado (Published), es por ello que retornamos un Box con el nuevo estado Published. Logicamente seria un puntero Box al estado Published.
  fn approve(self: Box<Self>) -> Box<dyn State> {
    Box::new(Published {})
  }
}

// El post esta en estado publicado (Published)
impl State for Published {
  // Cuando un post en estado publicado se le pide revision, no pasa a ningun otro estado, por lo que retornamos el mismo estado (Published) como un Box.
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  // Cuando un post en estado publicado es aprobado, no pasa a ningun otro estado, por lo que retornamos el mismo estado (Published) como un Box.
  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }

  // Cuando se llama al metodo content de un post en estado publicado retornamos el contenido del post, ya que es el unico estado que permite mostrar el contenido.
  fn content<'a>(&self, post: &'a Post) -> &'a str {
    &post.content // Recordemos que post.content es un String, y el metodo content del trait State retorna un &str, por lo que retornamos una referencia al contenido del post.
  }
}
```

Y ahora implementamos los metodos de la struct `Post` que son los que van a interactuar con el estado interno del post:

```rust
impl Post {
  // Constructor del Post
  pub fn new() -> Post {
    Post {
      state: Some(Box::new(Draft {})), // Inicializamos el estado del post como Draft (borrador)

      content: String::new(), // Inicializamos el contenido del post como un String vacio
    }
  }

  // Metodo para agregar texto al post
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text); // Agregamos el texto al contenido del post
  }

  // Metodo para pedir revision del post (el post pasa de estado Draft a PendingReview)
  pub fn request_review(&mut self) {
    if let Some(s) = self.state.take() { // Tomamos el estado actual del Post
      self.state = Some(s.request_review()); // Llamamos al metodo request_review del estado actual y asignamos el nuevo estado al Post
    }
  }

  // Metodo para aprobar el post (el post pasa de estado PendingReview a Published)
  pub fn approve(&mut self) {
    if let Some(s) = self.state.take() { // Tomamos el estado actual del Post
      self.state = Some(s.approve()); // Llamamos al metodo approve del estado actual y asignamos el nuevo estado al Post
    }
  }

  // Metodo para obtener el contenido del post
  pub fn content(&self) -> &str {
    self.state.as_ref().unwrap().content(self) // Del estado del Post le hacemos un as_ref() para obtener una referencia al estado actual del post, esto porque sabemos que el estado del post tiene el metodo content que retorna el valor del contenido del post, es por ello que si recordamos el metodo content recibia como parametro una referencia al Post, por lo que le pasamos self (una referencia al Post) y retornamos el contenido del post.
  }
}
```

La ventaja de este diseño es que si el dia de mañana necesitamos un nuevo estado para el post, por ejemplo el estado `Rejected` (rechazado) entonces solo creamos un nuevo struct de dicho estado que implemente el trait `State`. No tocamos para nada el struct `Post`.

# Otro diseño
A continuacion vamos a presentar otro diseño mas *"Rust"*. Basicamente en esta version aprovechamos mejor el sistema de tipos de Rust y eliminamos la logica de los trait objects. En lugar de guardar el estado interno del Post como un trait object lo que hacemos es que **cada estado es un tipo distinto**.

Veamos:

```rust
// Creamos un struct por cada estado del post donde cada estado tiene el atributo content que es un string

// Post en estado publicado
pub struct Post {
  content: String,
}

// Post en estado borrador
pub struct DraftPost {
  content: String,
}

// Post en estado de revision
pub struct PendingReviewPost {
  content: String,
}
```

Como vemos es un enfoque mas "Rust" y no tan POO. Ahora vamos a implementar los metodos de cada estado del post.

```rust
impl Post {
  // Constructor, al crear un post este arranca en estado borrador (DraftPost)
  pub fn new() -> DraftPost {
    DraftPost {
      content: String::new(), // Inicializamos el contenido del post como un String vacio
    }
  }

  // Solo un post en estado publicado (Post) tiene content()
  pub fn content(&self) -> &str {
    &self.content
  }
}

// Implementamos los metodos de DraftPost (post en estado borrador)
impl DraftPost {
  // Metodo para agregar texto al post
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text); // Agregamos el texto al contenido del post
  }

  // Metodo para pedir revision del post (el post pasa de estado DraftPost a PendingReviewPost)
  pub fn request_review(self) -> PendingReviewPost {
    PendingReviewPost {
      content: self.content, // Retornamos un nuevo PendingReviewPost con el contenido del DraftPost
    }
  }
}

// Implementamos los metodos de PendingReviewPost (post en estado de revision)
impl PendingReviewPost {
  // Metodo para aprobar el post (el post pasa de estado PendingReviewPost a Post)
  pub fn approve(self) -> Post {
    Post {
      content: self.content, // Retornamos un nuevo Post con el contenido del PendingReviewPost
    }
  }
}
```

Entonces basicamente un post apenas lo creamos arranca en estado borrador `DraftPost`, cuando se le pide revision pasa a estado de revision `PendingReviewPost` y cuando se aprueba pasa a estado publicado `Post`. Cada estado tiene su propia struct y sus propios metodos, y el contenido del post se va pasando de un estado a otro.

Si notamos solamente se le puede pedir el contenido de un post cuando este esta en estado publicado `Post`, si intentamos pedir el contenido de un post en estado borrador `DraftPost` o en estado de revision `PendingReviewPost` no compila, ya que esos estados no tienen el metodo `content()`. Esto es una ventaja de este diseño ya que nos permite **aprovechar el sistema de tipos de Rust para evitar errores en tiempo de compilacion**.

Que tiene de bueno esta segunda version? Pues que convierte los errores de negocio en **ERRORES DE COMPILACION**. Con la primera version de trait objects si llamamos el metodo `content()` de un post en estado borrador o en estado de revision, el programa compila pero en tiempo de ejecucion el metodo retorna un **string vacio**. En cambio en la segunda version si intentamos llamar al metodo `content()` de un post en estado borrador o en estado de revision, el programa **no compila** y nos da un error de compilacion. Esto es una gran ventaja ya que nos permite detectar errores de negocio en tiempo de compilacion y no en tiempo de ejecucion como en la primera version.

Sin embargo cada estrategia como todo tiene sus ventajas y desventajas a continuacion vemos una tabla comparativa:

||State Pattern (trait objects)|Tipos como Estados|
|---|---|---|
|Interfaz exterior|Siempre el mismo tipo (`Post`)|El tipo cambia con el estado (`DraftPost`, `PendingReviewPost`, `Post`)|
|Errores de estado invalido|Se detectan en tiempo de ejecucion (retorna string vacio)|Se detectan en tiempo de compilacion (no compila)|
|Facil de extender|Si, solo agregamos un nuevo struct que implemente el trait `State`|Si, solo agregamos un nuevo struct para el nuevo estado|
|Logica centralizada|En cada estado|En cada estado|
|Mas "POO" clasico|Si, usa trait objects y polimorfismo|No, cada estado es un tipo distinto|
|Aprovecha mejor Rust|No, usa trait objects y polimorfismo|Si, cada estado es un tipo distinto y aprovecha el sistema de tipos de Rust|

Aca somos claros, el State Pattern funciona en Rust, PERRRROOO no aprovecha al maximo sus fortalezas. La version de tipos como estados es mas idiomatica porque **convierte errores de logica de negocio en errores de compilacion**
