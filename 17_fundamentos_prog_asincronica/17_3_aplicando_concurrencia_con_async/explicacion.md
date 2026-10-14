Vamos a resolver problemas que resolvimos con hilos pero ahora con tareas asincronicas. El objetivo es mostrar las diferencias entre ambos enfoques.

## Crear nueva tarea con spawn_task
Cuando hablamos de programacion asincronica, hablamos de **tareas asincronicas**. Es decir, asi como dentro de un proceso tenemos varios hilos, dentro de un hilo podemos tener varias tareas asincronicas. Para crear una nueva tarea asincronica usamos la funcion **spawn_task** que seria el equivalente a crear un nuevo hilo con **thread::spawn**.

Asi como **thread::spawn** crea un hilo en el sistema operativo, **trpl::spawn_task** registra una nueva tarea asincronica en el **runtime** de Rust.

```rust
use std::time::Duration;

fn main() {
  // Inicializamos un runtime y ejecutamos el Future que le pasamos
  trpl::run(async {

    // Creamos una nueva tarea asincronica
    trpl::spawn_task(async {
      for i in 1..10 {
        println!("hi number {i} from the first task!");
        // Aca hacemos un sleep asincronico, es decir, el runtime puede ir a hacer otra cosa mientras tanto, si el Future no esta listo, el runtime vuelve a ejecutar la funcion desde donde se quedo
        trpl::sleep(Duration::from_millis(500)).await;
      }
    });

    // Aca estamos en la tarea principal y podemos hacer otra cosa mientras tanto
    for i in 1..5 {
      println!("hi number {i} from the main task!");
      // Aca hacemos un sleep asincronico, es decir, el runtime puede ir a hacer otra cosa mientras tanto, si el Future no esta listo, el runtime vuelve a ejecutar la funcion desde donde se quedo
      trpl::sleep(Duration::from_millis(500)).await;
    }
  });
}
```

Entonces basicamente lo que sucede en este ultimo codigo es que tenemos 2 tareas asincronicas corriendo al mismo tiempo, la tarea principal y la tarea creada con **spawn_task**. Ambas tareas pueden ir a hacer otra cosa mientras tanto, si alguna de las tareas no esta lista, el runtime vuelve a ejecutar la funcion desde donde se quedo.

Sin embargo si nos fijamos tenemos **el mismo problema que con los hilos**, es decir, si la tarea principal termina antes que la tarea creada con **spawn_task**, el runtime termina y la tarea creada con **spawn_task** no tiene tiempo de terminar. La solucion es usar un **join handle**, pero en lugar de un `.join()` tenemos que hacer un `.await` al **join handle**. Veamos un ejemplo:

```rust
let handle = trpl::spawn_task(async {
    for i in 1..10 {
        println!("hi number {i} from the first task!");
        trpl::sleep(Duration::from_millis(500)).await;
    }
});

for i in 1..5 {
    println!("hi number {i} from the second task!");
    trpl::sleep(Duration::from_millis(500)).await;
}

// La tarea principal espera a que la tarea creada con spawn_task termine antes de terminar
handle.await.unwrap();  // espera que la primera tarea termine (es un Future)
```

La diferencia logica con los hilos es que no necesitamos crearle un hilo al sistema operativo. Los bloques async se compilan en futures anonimos y el runtime los ejecuta de forma concurrente por su cuenta. Es decir, el runtime se encarga de ejecutar los futures que le pasamos, y si alguno de los futures no esta listo, el runtime puede ir a hacer otra cosa mientras tanto. Esto es lo que hace que la programacion asincronica sea mas eficiente que la programacion con hilos, ya que no necesitamos crear un hilo por cada tarea asincronica, sino que podemos tener varias tareas asincronicas corriendo en un solo hilo.

## trpl::join: Esperar multiples Futures
En lugar de joinear de a uno, podemos joinear varios Futures a la vez con **trpl::join**. Esto es util cuando tenemos varias tareas asincronicas que queremos esperar a que terminen antes de continuar con la ejecucion del programa. Veamos un ejemplo:

```rust
let fut1 = async {
    for i in 1..10 {
        println!("hi number {i} from the first task!");
        trpl::sleep(Duration::from_millis(500)).await;
    }
};

let fut2 = async {
    for i in 1..5 {
        println!("hi number {i} from the second task!");
        trpl::sleep(Duration::from_millis(500)).await;
    }
};

trpl::join(fut1, fut2).await;
```

Si nos fijamos tenemos 2 tareas asincronicas corriendo, por otro lado la tarea principal espera a que ambas tareas asincronicas terminen antes de continuar con la ejecucion del programa con **trp::join**.

Si recordamos nosotros antes usamos **trpl::race** pero ¿Cual es la diferencia? La diferencia es que **trpl::race** espera a que termine la primera tarea asincronica que termine, mientras que **trpl::join** espera a que terminen todas las tareas asincronicas.

## Paso de mensajes con channels async
Asi como teniamos channels para pasar mensajes entre hilos con **mpsc::channel**, tambien tenemos channels para pasar mensajes entre tareas asincronicas con **trpl::channel**. La diferencia la explicaremos luego:

```rust
// Creamos el canal asincronico con trpl::channel
let (tx, mut rx) = trpl::channel();

let val = String::from("hi");

// Mandamos el mensaje
tx.send(val).unwrap(); // envia el mensaje al canal

// Recibimos el mensaje
let received = rx.recv().await.unwrap(); // espera a recibir el mensaje del canal
println!("Got: {received}");
```

Diferencias clave con los canales de los hilos:

1. Si o si *rx* tiene que ser mutable, ya que el metodo **recv** cambia el estado del canal, es decir, si recibimos un mensaje, el canal cambia de estado y por eso necesitamos que sea mutable.

2. `rx.recv()` devuelve un Future, es decir, si no hay mensajes en el canal, el Future no esta listo y el runtime puede ir a hacer otra cosa mientras tanto. Cuando llegue un mensaje al canal, el Future se vuelve a ejecutar desde donde se quedo.

3. Es importante destacar que **no bloquea al hilo**, es decir, si no hay mensajes en el canal, el runtime puede ir a hacer otra cosa mientras tanto pero el hilo no se bloquea, sigue ejecutando otras tareas asincronicas.

Una cuestion importante es que supongamos que tenemos una sola tarea asincronica y dentro de esa misma tarea hacemos un send y un receive es logico que el codigo es **secuencial**:

```rust
// Todo esto dentro de una misma tarea asincronica se ejecutaria de manera secuencial

tx.send(val).unwrap(); // envia el mensaje al canal
trpl::sleep(Duration::from_millis(500)).await; // espera 500ms
let received = rx.recv().await.unwrap(); // espera a recibir el mensaje del canal
println!("Got: {received}");
```

Dentro de un solo bloque async todo se ejecuta a modo lineal, primero todos los sends y luego todos los receives. Todavia no hay concurrencia real.

La solucion para tener concurrencia real seria logicamente tener **varios bloques async separados con join**. Osea para que *tx* y *rx* corran concurrentemente hay que moverlos a bloques async independientes:

```rust
// Creamos el canal asincronico con trpl::channel
let (tx, mut rx) = trpl::channel();

// Creamos tx_fut que es un Future que envia varios mensajes al canal
let tx_fut = async {
  let vals = vec![
    String::from("hi"),
    String::from("from"),
    String::from("the"),
    String::from("future"),
  ];

  for val in vals {
    tx.send(val).unwrap(); // envia el mensaje al canal
    trpl::sleep(Duration::from_millis(500)).await; // espera 500ms
  }
};

// Creamos rx_fut que es un Future que recibe los mensajes del canal
let rx_fut = async {
  while let Some(value) = rx.recv().await { // espera a recibir el mensaje del canal
    println!("Got: {value}");
  }
};

trpl::join(tx_fut, rx_fut).await; // Espera a que ambos Futures terminen
```

Lo que hicimos fue crear 2 bloques async independientes, uno para enviar mensajes y otro para recibir mensajes. De esta manera, ambos bloques async corren concurrentemente y el runtime puede ir a hacer otra cosa mientras tanto.

El runtime ahora alterna entre *tx_fut* y *rx_fut* en cada punto *await*. Los mensajes llegan con 500ms de separacion en lugar de todos juntos al final.

El `while let Some(value) = rx.recv().await` es un bucle que espera a recibir mensajes del canal. Cuando el canal se cierra, el bucle termina y el Future termina:

- El `rx.recv()` devuelve `Some(value)` mientras haya mensajes en el canal, en general el *rx.recv()* devuelve un enum `Option<T>`.

- Devuelve `None` cuando el canal se cierra, es decir, cuando todos los *tx* se han caido y no hay mas mensajes en el canal.

## Async move
Al igual que con los hilos que haciamos `thread::spawn(move || { ... })` para que los hilos capturen el ownership de las variables que usan, en las tareas asincronicas hacemos `async move { ... }` para que las tareas asincronicas capturen el ownership de las variables que usan. Esto es importante porque si no hacemos `move`, las variables que usamos dentro del bloque async tienen que vivir mientras dure la tarea asincronica, y si la tarea asincronica se ejecuta despues de que la variable se haya ido, el programa va a fallar.

Veamos un ejemplo:

```rust
let (tx, mut rx) = trpl::channel();

let tx_fut = async move { // Captura el ownership de tx
  let vals = vec![
    String::from("hi"),
    String::from("from"),
    String::from("the"),
    String::from("future"),
  ];

  for val in vals {
    tx.send(val).unwrap(); // envia el mensaje al canal
    trpl::sleep(Duration::from_millis(500)).await; // espera 500ms
  }
}; // tx sale del scope y se cae, el canal se cierra, rx recibe None y termina el bucle while

let rx_fut = async move { // Captura el ownership de rx
  while let Some(value) = rx.recv().await { // espera a recibir el mensaje del canal
    println!("Got: {value}");
  }
};
trpl::join(tx_fut, rx_fut).await; // Espera a que ambos Futures terminen
```

Por otro lado, asi como podiamos tener multiples productores en los hilos, tambien podemos tener multiples productores en las tareas asincronicas. Para eso tenemos que clonar el *tx* y pasarle el ownership a cada tarea asincronica que queremos que produzca mensajes. Veamos un ejemplo:

```rust
let (tx, mut rx) = trpl::channel();
let tx1 = tx.clone(); // Clonamos el tx para que cada tarea asincronica tenga su propio tx

// Tarea asincronica 1
let tx1_fut = async move { // Captura el ownership de tx1
  let vals = vec![
    String::from("hi"),
    String::from("from"),
    String::from("the"),
    String::from("first"),
  ];

  for val in vals {
    tx1.send(val).unwrap(); // envia el mensaje al canal
    trpl::sleep(Duration::from_millis(500)).await; // espera 500ms
  }
};

// Tarea asincronica 2
let tx2_fut = async move { // Captura el ownership de tx
  let vals = vec![
    String::from("hi"),
    String::from("from"),
    String::from("the"),
    String::from("second"),
  ];

  for val in vals {
    tx.send(val).unwrap(); // envia el mensaje al canal
    trpl::sleep(Duration::from_millis(500)).await; // espera 500ms
  }
};

// Tarea asincronica que recibe los mensajes del canal
let rx_fut = async move { // Captura el ownership de rx
  while let Some(value) = rx.recv().await { // espera a recibir el mensaje del canal
    println!("Got: {value}");
  }
};

// Tarea principal que espera a que todas las tareas asincronicas terminen
trpl::join(tx1_fut, tx2_fut, rx_fut).await; // Espera a que todos los Futures terminen
```

## Diferencias claves entre hilos y tareas asincronicas
A continuacion veremos una tablita comparativa entre ambos metodos para llevar a cabo la concurrencia:
||Threads|Async|
|--|--|--|
|Modelo|1 hilo SO por tarea|N tareas en 1 hilo|
|Inicio|Inmediato al hacer spawn|Lazy, hasta que se hace await|
|Esperar|`join()` bloquea el hilo|`.await` cede el control|
|Comunicar|`mpsc:channel` sincrono|`trpl::channel` con `.await`|
|Joinear|`handle.join()`|`trpl::join(fut1, fut2).await`|
|Costo|Caro (stack del SO por hilo)|Barato (una estructura de datos por tarea)|

La mayor diferencia entre ambos metodos es el **COSTO**: Crear miles de hilos tiene un costo significativo para el Sistema Operativo. Crear miles de tareas asincronicas dentro de un mismo hilo es mucho mas barato porque son simplemente Futures (estructuras de datos) manejados por el runtime de Rust. Por eso, si tenemos muchas tareas que hacer, es mejor usar tareas asincronicas en lugar de hilos.