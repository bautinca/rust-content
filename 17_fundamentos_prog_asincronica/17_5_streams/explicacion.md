Imaginemos que tenemos una cinta transportadora que trae cajas una por una. Nosotros estamos parados al lado esperando a que llegue la proxima caja, la procesamos y esperamos a la siguiente. Esto es justamente un ejemplo de **procesamiento secuencial** y es lo que es un **STREAM**: Una secuencia de valores que llegan de a uno, a lo largo del tiempo. Comparando con otros tipos que ya vimos:

- **Vec**: Una caja con todo dentro donde todos los datos YA están disponibles.

- **Iterador**: Una cinta transportadora que nos trae las cajas INSTANTANEAMENTE, para ello hacemos un `next()` sincrono.

- **Future**: Esperamos a que nos llegue una caja, solo un valor asincrono

- **Stream**: Una cinta transportadora que nos trae las cajas de a una pero con pausas entre ellas, para ello hacemos `.next().await` asincrono

Es decir, basicamente un Stream es la combinacion de un **Iterador + Future**: Hay multiples valores que llegan en el tiempo, es decir, llegan de a uno pero no sabemos cuando van a llegar.

Es tan facil armar un Stream que basicamente a partir de un Iterador podemos armar un Stream:

```rust
use trpl::StreamExt; // Para poder usar el metodo .next() de Stream

fn main() {
  trpl::run(async {
    // Creamos un array normal
    let numeros = [1, 2, 3, 4, 5];

    // Lo convertimos a un Stream
    let mut stream = trpl::stream_from_iter(numeros);

    // Lo recorremos igual que un for pero con un .await ya que justamente mientras recorremos el stream estamos esperando a que lleguen los valores, y en ese tiempo muerto podemos hacer otras cosas
    while let Some(numero) = stream.next().await {
      println!("Recibi el numero: {}", numero);
    }
  });
}
```

Entonces si ejecutamos este ultimo codigo la salida seria algo como:

```
Llegó el número: 1
Llegó el número: 2
Llegó el número: 3
Llegó el número: 4
Llegó el número: 5
```

Por ahora es identico a un iterador comun y corriente, pero la diferencia real se hace notar cuando **los valores tardan en llegar**. En este ultimo ejemplo es porque justamente ya tenemos todos los valores disponibles, pero si por ejemplo los valores vienen de:

- Mensajes de un chat (llegan cuando alguien escribe)
- Datos de un sensor (llegan cada X segundos)
- Resultados de busqueda (llegan a medida que el servidor los encuentra)
- Lineas de un archivo grande (llegan a medida que se lee del disco)

Para todos estos casos un `Vec` no sirve ya que no podemos tener todos los valores de antemano, tampoco un `Future` ya que no es un solo valor. Un **stream** es exactamente lo que necesitamos.

Hagamos un ejemplo mas real donde los mensajes llegan con retraso:

```rust
use std::time::Duration; // Para simular el retraso
use trpl::StreamExt;

fn main() {
  trpl::run(async {
    // Creamos un stream de mensajes que llegan con retraso
    let mut mensajes = stream_de_mensajes();

    // Imprimimos los mensajes a medida que van llegando
    while let Some(msg) = mensajes.next().await {
      println!("Recibi el mensaje: {}", msg);
    }
  });
}

// Esta funcion devuelve un Stream de mensajes que llegan con retrasoa
fn stream_de_mensajes() -> impl trpl::Stream<Item = String> {

  // Creamos un canal de mensajes asincrono
  let (tx, rx) = trpl::channel();

  // Creamos una tarea separada que envia mensajes con pausas
  trpl::spawn(async move {
    let palabras = ["hola", "como", "estas", "?"];
    // Iteramos por cada una de las palabras y las vamos enviando por el canal con retrasos
    for palabra in palabras {
      // Simulamos un retraso de 500 milisegundos
      trpl::sleep(Duration::from_millis(500)).await;
      // Enviamos la palabra al canal
      tx.send(String::from(palabra)).unwrap();
    }
  });

  // Devolvemos el receptor del canal como un Stream
  trpl::ReceiverStream::new(rx)
}

```

Entonces basicamente lo que conseguimos es que el Stream `mensajes` le ira llegando un mensaje cada 500 milisegundos, y mientras tanto nuestro programa puede seguir haciendo otras cosas, y cuando llegan los mensajes se procesaran (se printean) a medida que van llegando. Entonces la salida seria algo como:

```
Recibi el mensaje: hola -> Llega a los 500ms
Recibi el mensaje: como -> Llega a los 1000ms
Recibi el mensaje: estas -> Llega a los 1500ms
Recibi el mensaje: ? -> Llega a los 2000ms
```

Lo importante aca es que mientras esperamos al proximo mensaje el runtime de Rust puede hacer **otras cosas**, no esta bloqueado a nada.

Por otro lado los metodos de un Stream son **igual que el de los iteradores**, por ejemplo podemos usar `filter`, `map`, `fold`, etc. Pero la diferencia es que todos estos metodos son **asincronos**, es decir, devuelven un Stream y no un valor final. Por ejemplo:

```rust

// Creamos el stream (si nos fijamos ya recibe todos los datos de una vez, pero es solo para el ejemplo)
let mut stream = trpl::stream_from_iter([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

// Filtro map, misma logica que un iterador comun
let mut doubles = stream.map(|n| n * 2); // Esto produce un nuevo Stream que va a ir llegando de a uno los valores del stream original multiplicados por 2

// Filtro filter, misma logica que un iterador comun
let mut pares = stream.filter(|n| n % 2 == 0);

// Take donde limitamos la cantidad de valores que queremos recibir
let mut primeros_tres = stream.take(3);
```

La diferencia con los iteradores comunes y corrientes es que siempre usamos `.next().await` para ir consumiendolos y en el tiempo muerto el Runtime hace otra cosa.

## Timeout
Que pasa si un mensaje tarda muchisimo en llegar? Por ejemplo si estamos esperando un mensaje de un chat y el otro usuario se desconecta. Veamos un caso:

```rust
use std::time::Duration;
use trpl::StreamExt;

fn main() {
  trpl::run(async {
    let mut mensajes = stream_con_retrasos().timeout(Duration::from_millis(200)); // Hacemos que el Stream espere maximo 200ms por mensaje, si no llega nada en ese tiempo se devuelve un error
  
    // Puede suceder que el mensaje llegue a tiempo o no, entonces hacemos un match para atender el caso feliz y triste
    while let Some(resultado) = mensajes.next().await {
      match resultado {
        Ok(msg) => println!("Llego: {msg}"),
        Err(_) => eprintln!("No llego ningun mensaje en el tiempo limite"),
      }
    }
  });
}
```

Si prestamos atencion con `.timeout()` el stream deja de devolver `String` al hacerle `mensajes.next().await` y empieza a devolver `Result<String, Elapsed>`:

- `Ok(msg)` -> El mensaje llego a tiempo
- `Err(_)` -> Pasaron los 200ms sin que llegara nada

## Merge: Combinar 2 streams en uno
Basicamente pasamos de tener una fuente de datos a 2 fuentes de datos que queremos procesarlos juntos en orden en que llegan los datos:

```rust
// Stream A: Mensajes de un usuario
// Stream B: Notificaciones del sistema

let mensajes = stream_de_mensajes(); // Simulamos que llegan mensajes de un usuario
let notificaciones = stream_de_notificaciones(); // Simulamos que llegan notificaciones del sistema

// Para combinar ambos streams en un unico steam tendriamos que usar .merge()
let mut combinado = mensajes.merge(notificaciones);

// Listo ahoa tenemos un unico stream, podemos ir solicitandole los valores a medida que van llegando, sin importar de donde vienen
while let Some(msg) = comnbinado.next().await {
  println!("Recibi: {msg}");
}
```

El stream combinado produce elementos de cualquiera de los 2, en el orden en que llegan. No esperamos a que uno termine para empezar el otro.