Repasando conceptos:

- **Paralelismo**: 2 tareas corren **al mismo tiempo** en 2 CPUs distintas

- **Concurrencia**: Solo una CPU **alterna** entre 2 tareas, pero no corren al mismo tiempo. Avanza en una mientras la otra espera

Entonces, el **async** de Rust es principalmente **CONCURRENTE**, no paralelo. 

## ¿Que es un Future?
Un Future es basicamente **un valor que aun no esta listo, pero lo estara en algun momento**. En Rust cualquier tipo de dato seria un Future siempre y cuando implemente el trait `Future`:

```rust
pub trait Future {
  type Output; // El tipo de dato que se espera obtener cuando el Future este listo
  fn poll(...) -> Poll<Self::Output>; // La funcion poll() se utiliza para verificar si el Future esta listo
}
```

El proceso de ir verificando constantemente si un Future esta listo se le llama **polling**. La funcion `poll()` devuelve un enum `Poll` que puede ser:

- `Poll::Pending`: El Future aun no esta listo, y se debe seguir esperando
- `Poll::Ready(value)`: El Future ya esta listo y devuelve el valor esperado

Por otro lado estan los conceptos de **async** y **await**:

- **async**: Marca un bloque o funcion como asincrono. El compilador lo convierte en un tipo que implementa **Future**. Esto significa que cuando se llama a una funcion async, no se ejecuta inmediatamente, sino que devuelve un Future que representa la ejecucion de esa funcion!!

- **await**: Dentro de un bloque/funcion async, pausa la ejecucion hasta que el **Future** este listo y devuelve el valor que representa. Esto permite escribir codigo asincrono de manera secuencial, como si fuera sincrono.

Hagamos una primera funcion async para entender mejor:

```rust
use trpl::Html;

// Marcamos la funcion como asincronica (async)
async fn page_title(url: &str) -> Option<String> {
  
  let response = trpl::get(url).await; // Espera la respuesta HTTP asincronica
  
  let response_text = response.text().await; // Espera el texto de la respuesta

  Html::parse(&response_text)
    .select_first("title")
    .map(|title| title.inner_text())
}
```

Lo importante es que cada **.await** es un **punto de pausa potencial**. El runtime del programa puede ir a hacer otra cosa mientras esperamos a la red, osea esperamos a que la respuesta HTTP llegue, y mientras tanto podemos atender otras tareas. Esto es lo que hace que el programa sea **concurrente**.

Basicamente el `.await` le dice al runtime: "Hey, no estoy listo para continuar, puedes hacer otra cosa mientras tanto". Y cuando el Future esta listo, el runtime vuelve a ejecutar la funcion desde donde se quedo.

La sintaxis **await** es **postfija**, es decir, va despues de la expresion, no antes. Esto ultimo permite encadenar multiples await en una sola linea:

```rust
// Basicamente es lo anterior pero en una unica linea, encadenando los await
let response_text = trpl::get(url).await.text().await;
```

## Los Futures son LAZY
En Rust crear un Future **no hace nada**. Solo cuando hacemos `.await` arranca a ejecutarse, por ejemplo:

```rust
use trpl::Html;

// Marcamos la funcion como asincronica (async)
async fn page_title(url: &str) -> Option<String> {
  
  let response = trpl::get(url).await.text().await;

  Html::parse(&response)
    .select_first("title")
    .map(|title| title.inner_text())
}

let title_fut_1 = page_title(&args[1]); // Esto en realidad NO HACE NADA, solo crea un Future que representa la ejecucion de la funcion page_title

let title_fut_2 = page_title(&args[2]); // Esto en realidad NO HACE NADA, solo crea otro Future, solo representa la ejecucion de la funcion page_title

// Solo al hacerles await arrancan a ejecutarse
trpl::select(title_fut_1, title_fut_2).await;
```

Que pasa cuando declaramos una funcion como **async**? Basicamente nosotros en codigo escribimos esto:

```rust
async fn page_title(url: &str) -> Option<String> {
  
  let response = trpl::get(url).await.text().await;

  Html::parse(&response)
    .select_first("title")
    .map(|title| title.inner_text())
}
```

Pero el compilador lo transforma en algo asi:

```rust
fn page_title(url: &str) -> impl Future<Output = Option<String>> {
  async move {
    let response = trpl::get(url).await.text().await;

    Html::parse(&response)
      .select_first("title")
      .map(|title| title.inner_text())
  }
}
```

Si nos fijamos al ejecutar la funcion sin await lo que nos devuelve es un **Future**. En realidad el **async fn** es azucar sintactico.

## La maquina de estados invisible
En cada punto donde hay un `.await`, el compilador genera una **maquina de estados** que recuerda en que parte de la ejecucion se quedo la funcion. Esto es lo que permite que cuando el Future este listo, el runtime pueda continuar la ejecucion desde donde se quedo

Cada vez que el runtime vuelve a ejecutar el Future, sabe exactamente en que estado estaba y desde donde continuar. El compilador genera todo esto **automaticamente**, nosotros solo nos ocupamos de escribir el *.await* y el compilador se encarga del resto

## Main no puede ser async
La unica funcion que no puede ser async es la funcion **main**, osea:

```rust
async fn main() { // Esto no compila

}
```
Esto esta mal ya que el codigo async necesita un **RUNTIME** que se encargue de ejecutar los Futures, y la funcion main no tiene un runtime por defecto. Main es el punto de entrada a la app, no hay nada por "encima" que lo maneje. El runtime tiene que inicializarse en main, no se main en sí.

La solucion es `trpl::run` ¿Que es? Es una funcion que inicializa un runtime y ejecuta el Future que le pasemos. Por ejemplo:

```rust
async fn page_title(url: &str) -> Option<String> {
  
  let response = trpl::get(url).await.text().await;

  Html::parse(&response)
    .select_first("title")
    .map(|title| title.inner_text())
}

fn main() {
  let args: Vec<String> = std::env::args().collect();

  // Inicializamos un runtime y ejecutamos el Future que le pasamos
  trpl::run(async {
    let ur1 = &args[1];
    // Aca hacemos await al Future que devuelve la funcion page_title. Si el Future no esta listo, el runtime puede ir a hacer otra cosa mientras tanto, si el Future esta listo, el runtime vuelve a ejecutar la funcion desde donde se quedo
    match page_title(ur1).await {
      Some(title) => println!("Title: {}", title),
      None => println!("No title found"),
    }
  })
}
```

Como vemos el `trpl::run` toma un Future (si vemos recibe un *async*), por debajo inicializa el runtime y bloquea la funcion *main* hasta que el future complete. Es el puente entre el mundo sincronico de main y el mundo async.

Veamos un ejemplo final pero ahora en lugar de 1 URL tenemos 2 URLs:

```rust
use trpl::{Either, Html};

async fn page_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)  // devuelve la URL para saber quién ganó
}

fn main() {
  let args: Vec<String> = std::env::args().collect();

  // Inicializamos un runtime y ejecutamos el Future que le pasamos
  trpl::run(async {
    let title_fut_1 = page_title(&args[1]); // Esto en realidad no hace nada, solo crea un Future que representa la ejecucion de la funcion page_title
    
    let title_fut_2 = page_title(&args[2]); // Esto en realidad no hace nada, solo crea otro Future que representa la ejecucion de la funcion page_title

    // Aca hacemos await a ambos Futures, y el runtime puede ir a hacer otra cosa mientras tanto, si alguno de los Futures esta listo, el runtime vuelve a ejecutar la funcion desde donde se quedo
    let (url, maybe_title) =
      match trpl::race(title_fut_1, title_fut_2).await {
        Either::Left(left) => left,
        Either::Right(right) => right,
      };

    // Aca ya sabemos que uno de los Futures esta listo, y podemos imprimir el resultado
    println!("{url} respondio primero");
    match maybe_title {
      Some(title) => println!("Title: {}", title),
      None => println!("No title found"),
    }
  })
}
```

Entonces el `trpl::race` toma 2 Futures y devuelve un Future que se resuelve cuando **cualquiera de los 2 Futures** se resuelve, cuando un Future se resuelve lo devuelve como `Either::Left` o `Either::Right` y el otro Future se cancela. Either es como el enum Result pero sin nocion de exito/fracaso. Esto permite que el runtime pueda ir a hacer otra cosa mientras tanto, y cuando uno de los Futures este listo, el runtime vuelve a ejecutar la funcion desde donde se quedo.

Para los que tenemos curiosidad el enum Either esta definido asi:

```rust
pub enum Either<L, R> {
    Left(L), // Gano el primer argumento
    Right(R), // Gano el segundo argumento
}
```