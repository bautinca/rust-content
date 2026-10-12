Manejar la concurrencia de forma eficiente y segura es otro de los objetivos de Rust. La **concurrencia** es donde diferentes partes de un programa se ejecutan de forma **independiente** y a modo **alternado**.

Por otro lado esta la programacion **paralela** es donde diferentes partes del programa se ejecutan **al mismo tiempo**.

Tanto programacion paralela como la concurrencia son muy importantes en un mundo donde cada vez tenemos mas computadoras con multiples procesadores y nucleos.

Historicamente la concurrencia ha sido muy propensa a errores pero Rust busca evitar esto.

Inicialmente Rust penso que garantizar la seguridad de la memoria y prevenir problemas de concurrencia eran 2 objetivos separados que se resolvian con distintos metodos. Sin embargo, se dieron cuenta que la seguridad de la memoria y la seguridad de la concurrencia estaban muy relacionadas y que se podian resolver con un mismo metodo. Muchos errores propios de la concurrencia son errores en tiempo de compilacion y no en tiempo de ejecucion, lo que hace que Rust sea un lenguaje muy seguro.

Por lo tanto en lugar de pasar mucho tiempo tratando de replicar el error en tiempo de ejecucion, Rust nos permite encontrar el error en tiempo de compilacion y corregirlo antes de que el programa se ejecute.

Como resultado a esto ultimo podemos corregir el codigo mientras lo estamos trabajando en desarrollo en lugar de tener que esperar a que el programa se ejecute en ambiente de produccion.

# Threads/hilos
En la mayoria de los SO actuales el codigo de un programa ejecutado se ejecuta en **un proceso** y el SO administra multiples procesos a la vez. Dentro de un programa tambien pueden haber diferentes partes independientes que se ejecutan de forma alternada, a estas partes se les llama **threads** o **hilos**.

Por ejemplo un servidor web podria tener multiples hilos para que pudiera responder a mas de una solicitud al mismo tiempo

Dividir la computacion de nuestro programa en multiples hilos para ejecutar multiples tareas al mismo tiempo puede mejorar el rendimiento del programa pero tambien agregar complejidad debido a que los hilos pueden ejecutarse simultaneamente no hay ninguna garantia inherente sobre el orden en que las partes del codigo en diferentes hilos se ejecutan. Este comportamiento puede originar problemas como:

1. **Race conditions**: Los hilos pueden acceder a la misma variable al mismo tiempo y el resultado final depende del orden en que se ejecuten los hilos. Esto puede causar resultados inesperados y errores difíciles de reproducir.

2. **Deadlocks**: Ocurre cuando 2 hilos estan esperando el uno al otro, evitando que ambos hilos continuen por lo tanto se bloquea todo el programa.

3. Ciertos bugs que ocurren solo en ciertas situaciones y son dificiles de reproducir y arreglar de manera confiable.

Rust intenta mitigar los efectos negativos de usar hilos.

Los lenguajes de programacion implementan hilos de maneras distintas y muchos SOs proporcionan una API que el lenguaje puee llamar para crear nuevos hilos. La biblioteca estandar de Rust utiliza un metodo **1:1** en la implementaion de hilos, lo que significa que **cada hilo de Rust se corresponde con un hilo del SO**. Hay otros crates que implementan hilos de manera diferente, por ejemplo el crate **rayon** implementa un modelo de hilos **M:N** donde multiples hilos de Rust corresponden a multiples hilos del SO. Esto permite que el crate **rayon** pueda crear y destruir hilos de manera mas eficiente, pero tambien puede ser mas dificil de depurar.

## Creando nuevo hilo con Spawn
Para crear un nuevo hilo en Rust se utiliza la funcion **thread::spawn** y le pasamos un closure (funcion anonima) que contiene el codigo que queremos ejecutar en el nuevo hilo. Por ejemplo:

```rust
use std::thread; // Importamos el modulo thread de la biblioteca estandar
use std::time::Duration; // Importamos el modulo time de la biblioteca estandar

fn main() {
  // Hilo principal

  // Hilo hijo
  thread::spawn(
    || {
      for i in 1..10 {
        println!("hi number {i} from the spawned thread!");
        thread::sleep(Duration::from_millis(1)); // Hacemos que el hilo duerma por 1 milisegundo
      }
    }
  );

  for i in 1..5 {
    println!("hi number {i} from the main thread!");
    thread::sleep(Duration::from_millis(1)); // Hacemos que el hilo duerma por 1 milisegundo
  }
}
```

> **NOTA**: Cuando el hilo principal termina de ejecutarse, el hilo hijo tambien termina aunque no haya terminado de ejecutar su codigo. Esto es porque el hilo principal es el hilo que inicia la ejecucion del programa y cuando este termina, el programa termina.

Entonces si ejecutamos este ultimo programa la salida seria algo asi:

```
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

Las llamadas a los *sleeps* en ambos hilos fuerzan a que el hilo principal y el hilo hijo se alternen en la ejecucion.

Los hilos probablemente se turnarian, pero esto no esta garantizado, esto ya depende de nuestro SO como los turna pero en eso ya no nos metemos. En esta ultima ejecucion el hilo principal termino antes que el hilo hijo, por lo tanto el hilo hijo no pudo terminar de ejecutar su codigo.

¿Como hacer que el hilo principal espere a que el hilo hijo termine de ejecutar su codigo antes de terminar el programa? Para esto podemos usar la funcion **join** que nos permite esperar a que un hilo termine de ejecutarse antes de continuar con la ejecucion del programa. Resulta que **thread::spawn** devuelve un **JoinHandle\<T>**. Un JoinHandle es un valor de propiedad que, cuando se llama a su metodo **join**, espera a que su hilo asociado termine de ejecutarse y luego devuelve el resultado del hilo. Por ejemplo:

```rust
use std::thread;
use std::time::Duration;

fn main() {
  // Hilo principal

  // Hilo hijo lo guardamos en una variable para poder llamar a su metodo join
  let handle = thread::spawn(
    || {
      for i in 1..10 {
        println!("hi number {i} from the spawned thread!");
        thread::sleep(Duration::from_millis(1));
      }
    }
  );

  for i in 1..5 {
    println!("hi number {i} from the main thread!");
    thread::sleep(Duration::from_millis(1));
  }

  // Antes de que finalice el hilo principal, llamamos al metodo join del hilo hijo para esperar a que termine de ejecutarse
  handle.join().unwrap(); // Si el hilo hijo termina con un error, unwrap() hara que el hilo principal tambien termine con un error. Esto es util para propagar errores de hilo hijo al hilo principal.
}
```

Llamar a **join** en el handle bloquea el hilo que esta actualmente en ejecucion hasta que el hilo representado por el handle termine. Bloquear un hilo significa que ese hilo se impide realizar un trabajo o salir. Debido a que hemos puesto la llamada a **join** despues del bucle *for* del hilo principal entonces la salida sera algo asi donde se alterna la ejecucion de ambos hilos sin embargo esta garantizado que ambos terminen ya que en caso de terminar antes el hilo principal este esperara al hilo hijo a que termine de ejecutarse:

```
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

Ahora veamos que uscede cuando movemos la llamada a **handle.join()** antes del bucle for en el hilo principal:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
```

Lo que sucedera es que el hilo principal primero esperara a que el hilo hijo termine de ejecutarse antes de continuar con su bucle for, por lo tanto la salida sera algo asi, primero se ejecutara el hilo hijo y luego el hilo principal:

```
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

## Usando move Closures con Threads
A menudo usamos la keyword **move** con closures. Recordemos que el move en closures indica que el closure toma ownership de las variables que utiliza. Esto es especialmente útil cuando trabajamos con hilos, ya que nos permite mover datos al hilo en lugar de tener que compartirlos entre hilos, lo que puede llevar a problemas de concurrencia.

Observemos que en el siguiente ejemplo el closure que pasamos a **thread::spawn** no tiene argumentos, no estamos usando ningu ndato del hilo principal en el codigo del hilo creado. Para usar datos del hilo principal en el hilo creado, el closure del hilo creado debe capturar los valores que necesita. Lo siguiente estaria mal:

```rust
use std::thread;

fn main() {
  let v = vec![1, 2, 3];

  let handle = thread::spawn(|| {
    println!("Here's a vector: {:?}", v); // Dentro del hilo hijo estamos tratando de usar la variable v que esta en el hilo principal, esto no es permitido ya que el hilo hijo puede ejecutarse despues de que el hilo principal termine y por lo tanto la variable v ya no existira. Esto esta MAL
  });

  handle.join().unwrap();
}
```

Para solucionar esto tenemos que agregar la keyword **move** antes del closure, forzamos al closure a tomar ownership de los valores que esta usando en lugar de permitir que Rust infiera que deberia pedir prestado los valores. En resumidas cuentas el hilo hijo debe tomar ownership de la variable y no usar una referencia de la misma en el hilo principal. Entonces el codigo correcto seria:

```rust
use std::thread;

fn main() {
  let v = vec![1, 2, 3];

  let handle = thread::spawn(move || { // Agregamos la keyword move para que el closure tome ownership de la variable v
    println!("Here's a vector: {:?}", v); // Ahora el hilo hijo tiene ownership de la variable v y puede usarla sin problemas
  });

  handle.join().unwrap();
}

```
Entonces eso quiere decir que cuando el ownership de *v* pase al hilo hijo entonces el hilo principal no podra usar la variable *v* ya que el ownership de la misma ya no le pertenece.


