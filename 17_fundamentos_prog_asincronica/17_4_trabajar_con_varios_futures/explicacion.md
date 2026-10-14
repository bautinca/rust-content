Lo que sucede es que `join` acepta exactamente 2 futures y `join3` acepta exactamente 3. ¿Que pasa cuando queremos manejar una cantidad arbitraria o dinamica de Futures?

La solucion obvia podria ser meter todos los Futures en un vector y luego usar `trpl::join_all`, osea algo asi:

```rust
let futures = vec![tx1_fut, rx_fut, tx_fut];
trpl::join_all(futures).await;
```

Pero en realidad esto ultimo **no compila**. La razon es que aunque todos los bloques `async` devuelvan un `Future<Output = ()>` el compilador genera un tipo **unico y anonimo** para cada bloque `async`, y por lo tanto el vector `futures` no puede contener elementos de tipos diferentes. Por ejemplo 2 bloques `async` aunque devuelvan el mismo output son de tipos diferentes, y por lo tanto no pueden ser almacenados en un vector.

La solucion? usar `Box<dyn Future>`. Es decir, en vez de almacenar los futures directamente en el vector, lo que almacenamos es un puntero inteligente a un Future, y de esta manera todos los elementos del vector son del mismo tipo. Por ejemplo:

```rust
// Creamos un vector de punteros Box a Future
let futures: Vec<Box<dyn Future<Output = ()>>> = vec![
    Box::new(tx1_fut),
    Box::new(rx_fut),
    Box::new(tx_fut),
];

// Y ahora si podemos usar join_all
trpl::join_all(futures).await;
```

Pero ojo, en realidad esto nos salta un error de compilacion. El compilador necesita que los Futures dentro del vector esten **pinneados**, osea que no se muevan de lugar en memoria.

`Pin<T>` es un wrapper que le dice al compilador *"este valor no se va a mover"*. Una vez pinneado Rust garantiza que el Future permanece en la misma direccion de memoria hasta que se complete.

Veamos un ejemplo:

```rust
let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![
    Box::pin(tx1_fut),
    Box::pin(rx_fut),
    Box::pin(tx_fut),
];

trpl::join_all(futures).await;
```

Al hacer `Box::pin` en lugar de `Box::new` estamos asegurando que cada Future dentro del vector no se movera de lugar en memoria. Es decir, el `Box::pin` asigna el Future en el heap y lo pinnea ahi.

## Futures con distinto tipo de outputs
Como bien sabemos , los Futures pueden devolver distintos tipos de outputs. Por ejemplo, un Future puede devolver un `i32`, otro un `String`, y otro un `()`.

`join_all` requiere que **todos los Futures devuelvan el mismo output**. Si necesitamos Futures con distintos tipos usamos la macro `trpl::join!` que nos permite mezclar distintos tipos de outputs. Por ejemplo:

```rust
let fut1 = async { 42 }; // Este future devuelve un i32
let fut2 = async { "Hola".to_string() }; // Este future devuelve un String
let fut3 = async { () }; // Este future devuelve un ()

let (a, b, c) = trpl::join!(fut1, fut2, fut3);
println!("a: {}, b: {}, c: {:?}", a, b, c); // a: 42, b: Hola, c: ()
```

`join!` es un macro (no una funcion) que lo que permite es manejar un numero fijo de Futures con tipos heterogeneos y devolver una tupla con cada resultado. La contra es que el numero de Futures debe ser conocido en tiempo de compilacion.

Entonces hasta ahora con todos los *join's* que vimos seria:

|Join|Cantidad de futures|Tipo de output|
|--|--|--|
|`join`|2 en tiempo de compilacion|Tupla con 2 elementos, pueden diferir|
|`join3`|3 en tiempo de compilacion|Tupla con 3 elementos, pueden diferir|
|`join_all`|Es dinamico, se define en tiempo de ejecucion|Tupla con N elementos, deben ser iguales|
|`join!`|Es fijo en tiempo de compilacion|Tupla con N elementos, pueden diferir|

## Carreras de Futures: trpl::race
Cuando solo necesitamos que alguno de los Futures termine (no todos como lo hacen los `joins`) usamos `trpl::race`. `race` es una funcion que recibe un vector de Futures y devuelve el output del primer Future que termine. Por ejemplo:

```rust
// Primer Future
let slow = async {
    println!("'slow' started.");
    trpl::sleep(Duration::from_millis(100)).await; // Simulamos un Future lento con el strpl::sleep
    println!("'slow' finished.");
};

// Segundo Future
let fast = async {
    println!("'fast' started.");
    trpl::sleep(Duration::from_millis(50)).await;
    println!("'fast' finished.");
};

trpl::race(slow, fast).await;
```

Lo que hara `trpl::race` es que al hacer `.await` se ejecutaran ambos Futures concurrentemente, y el que termine primero sera el que devuelva su output. En este caso, el Future `fast` terminara antes que `slow`, y por lo tanto su output sera el que se devuelva. Cuando el primer Future termina automaticamente se cancelara el otro.

> **NOTA**: `trpl::race` evalua los Futures **en el orden en que se pasan como argumentos** por lo que el primero si bien es mas lento sera el que se ejecute primero. Si invertimos el orden de los argumentos, el orden de los mensajes *"started"* cambian. El Future pasado primero siempre tiene la oportunidad de avanzar hasta su primero `.await` antes de que el otro empiece.

Algo que no aclaramos es ¿Porque cuando queremos simular una tarea asincronica lenta usamos `trpl::sleep` y no `std::thread::sleep`? La razon es que `std::thread::sleep` bloquea **TODO EL HILO** en ejecucion, mientras que `trlp::sleep` solo bloquea el Future en cuestion, permitiendo que otros Futures puedan ejecutarse mientras tanto. Esto es fundamental para mantener la naturaleza asincronica de nuestro programa.

Tambien en lugar de usar `trpl::sleep` podriamos usar otra forma mas idiomatica que seria el `trpl::yield_now()`, que le dice al runtime que ceda el control y permita que otros Futures se ejecuten. Por ejemplo:

```rust
let slow = async {
    println!("'slow' started.");
    trpl::yield_now().await; // Cede el control a otros Futures
    println!("'slow' finished.");
};

let fast = async {
    println!("'fast' started.");
    trpl::yield_now().await; // Cede el control a otros Futures
    println!("'fast' finished.");
};

trpl::race(slow, fast).await;
```

El `yield_now()` es mas explicito: *"termine este pedazo de trabajo, el runtine puede ejecutar otro Future si quiere"*. Es mucho mas rapido que `trlp::sleep` porque no tiene granularidad de tiempo (1ms tipicamente), hoy en dia en 1ms las compuadoras pueden hacer mucho trabajo. El `yield_now()` es practicamente instantaneo.

Este modelo se llama **multitarea cooperativa**. Cada Future decide cuando ceder el control. Es responsabildiad del programador no monopoliar el runtime con operaciones largas sin puntos *.await*

Veamos el siguiente codigo:

```rust
// Esta funcion recibe un Future y un tiempo maximo de espera. Si el Future no termina en ese tiempo, se cancela y devuelve un error.
async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    // Si el Future termina antes del tiempo maximo, devuelve su output. Si no, devuelve un error con el tiempo maximo.
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),   // el future ganó → éxito
        Either::Right(_) => Err(max_time),    // sleep ganó → timeout
    }
}

fn main() {
    // Inicializamos el runtime tipo trpl
    trpl::run(async {

        // Primer tarea asincronica (Future)
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        // Lo que hace el timeout() es que si el Future `slow` no termina en 2 segundos, se cancela y devuelve un error. Si termina antes de los 2 segundos devuelve su output.
        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => println!("Failed after {} seconds", duration.as_secs()),
        }
    });
}
// Salida: Failed after 2 seconds
```
