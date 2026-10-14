Uniendo todo sobre los hilos, Futures y async/await **¿Cuando usar cada una?**

La respuesta es que **son complementarios, no competidores**. La respuesta no es que elegimos uno u otro, sino que debemos elegir la herramienta adecuada para el problema que estamos resolviendo.

Por ejemplo el siguiente codigo combina ambas estrategias:

```rust
use std::{thread, time::Duration};

fn main() {
  // Creamos un canal para comunicarnos entre hilos
  let (tx, mut rx) = trpl::channel();

  // Un hilo del SO que envia numeros. Envia cada 100ms un numero del 0 al 9 por el canal
  thread::spawn(move || {
    for i in 0..10 {
      tx.send(i).unwrap();
      thread::sleep(Duration::from_millis(100));
    }
  });

  // Un bloque async que recibe los numeros del canal y los imprime
  trpl::run(async {
    while let Some(i) = rx.recv().await {
      println!("Recibido: {}", i);
    }
  });
}
```

Lo que logramos en este ultimo codigo es que tenemos un hilo del SO que envia numeros cada 100ms y un bloque async que recibe esos numeros y los imprime. Esto nos permite tener un hilo del SO que se encarga de enviar datos y un bloque async que se encarga de recibirlos y procesarlos. Si nos fijamos el hilo tiene una latencia de 100ms entre cada envio, en ese tiempo el bloque async puede estar haciendo otras cosas, como recibir y procesar los numeros que ya han sido enviados. Esto nos permite tener un programa mas eficiente y con mejor rendimiento.

## Diferencias entre hilos y async/await

El hilo es un *"dispara y olvidate"* es decir, un hilo del SO es como contratar a alguien y mandarlo a trabajar. Una vez que lo lanzamos trabaja completamente de forma independiente.

```rust
thread::spawn(|| {
  // Esto corre solo, sin interrupciones. El SO decide cuando le toca ejecutar este hilo
});
```

Es decir:

- El SO interrumpe o reanuda el hilo cuando quiere
- El SO no tiene forma nativa de cancelarlo ni comunicarse con el salvo por los canales

En cambio el async es mas un *"control fino"* donde una tarea async es como hacer nosotros mismos ese trabajo pero pausando estrategicamente para que otras tareas puedan ejecutarse. Es decir, nosotros decidimos cuando pausar y cuando reanudar la tarea.

```rust
trpl::spawn_task(async {
  // Cada .await es un punto donde decidimos pausar, el runtime puede ejecutar otras tareas en esos puntos
});
```

En este caso:

- El runtime (no el SO) decide que ejecutar en cada punto `await`
- Podemos tener miles de tareas async en un solo hilo del SO
- Podemos cancelar una tarea, componerla con otras o agregarle timeouts

La siguiente tabla comparativa en cada situacion muestra cual estrategia es la mas util:

|Situacion|Herramienta|
|--|--|
|Trabajo muy paralelizable|Hilos|
|Muchas operaciones de I/O|Async/await|
|Mezcla de ambas|Hilos + Async/await|
|Sistema embedido sin SO|Async/await|
|Necesitamos cancelacion o timeouts|Async/await|

En resumidas cuentas si el trabajo es **muy paralelizable** entonces lo mejor son los **hilos**, en cambio si el trabajo es muy concurrente y hay muchos mensajes, intervalos y fuentes distintas de I/O entonces lo mejor es usar **async/await**. Y si tenemos una mezcla de ambas situaciones entonces podemos combinar ambas estrategias.

Una pregunta es **¿Porque las tareas async son mas baratas que los hilos?**. Podemos lanzar millones de tareas async en una computadora moderna sin quedarnos sin memoria. Con hilos en cambio cada hilo necesita **su propio stack de memoria**, entonces si lanzamos millones de hilos literalmente nos quedamos **sin ram**

La razon es que una tarea async como dijimos es simplemente un **struct** (la maquina de estados que genera el compilador). Una tarea async dormida ocupa solo los bytes de sus variables y no mas que eso. Un hilo del SO dormido ocupa tipicamente entre 8KB y 1MB solo por el stack!!.

> **NOTA**: Un detalle importante. Los runtimes como tokio (que trpl usa por debajo) internamente sí usan hilos. Lo que hacen es usar un pool de hilos del OS y distribuir las tareas async entre ellos eficientemente. Así que en la práctica, cuando escribís código async con tokio: Tenés N hilos del OS (generalmente uno por core).Sobre esos hilos corren potencialmente millones de tareas async. El runtime se encarga de moverlas entre hilos para aprovechar todos los cores. Es lo mejor de los dos mundos: la eficiencia de async con el paralelismo real de múltiples cores.
