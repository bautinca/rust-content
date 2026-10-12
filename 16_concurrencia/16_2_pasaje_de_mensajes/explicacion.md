# Pasaje de Mensajes
Hay un enfoque cada vez mas popular para garantizar una concurrencia segura que es el **Message passing** donde 2 hilos o **actores** se comunican entre si enviandose mensajes que contienen datos. Lo importante es que *"no se comunican compartiendo memoria, en su lugar, comparta memoria comunicandose"*

Muchos errores de concurrencia surgen justamente porque 2 o mas hilos **comparten memoria** y no se tiene un control adecuado de acceso a esa memoria compartida. En cambio, con el pasaje de mensajes, cada hilo tiene su propia memoria y no hay necesidad de sincronización para acceder a la memoria de otro hilo.

Para lograr concurrenciaa mediante el envio de mensajes la biblioteca estandar de Rust proporciona una implementacion de **canales** (channels). Una canal es un concepto de programacion general por el cual se envian datos de un hilo a otro.

Un canal tiene 2 partes, un **transmisor** (sender) y un **receptor** (receiver).
Desarrollaremos un programa que tiene un hilo para generar valores y enviarlos por un canal, y otro hilo que recibira los valores e imprimira por pantalla. Enviaremos valores simples entre hilos usando un canal para ilustrar la caracteristica. Una vez que nos hayamos familiarizado con la tecnica podriamos usar canales para cualquier hilo que necesite comunicarse entre si, como un sistema de chat, o sistema donde muchos hilos realizan partes de un calculo y envian las partes a un hilo que agerega los resultados.

Basicamente asi creamos un canal en Rust:

```rust
use std::sync::mpsc; // mpsc = multiple producer, single consumer. Basicamente tenemos multiples hilos que pueden enviar datos a un hilo que los recibe.

fn main() {
    let (tx, rx) = mpsc::channel(); // 't' de transmitter y 'r' de receiver. El metodo channel() devuelve una tupla con el transmisor y el receptor.
}
```

Si bien el canal es multiple producer, single consumer lo que haremos sera crear un solo productor, pero agregaremos multiples productores cuando hagamos que este ejemplo funcione.

Lo que haremos sera que un hilo hijo envie un string a traves del canal y el hilo principal lo recibe. Asi:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
  // Canal
  let (tx, rx) = mpsc::channel();

  // Hilo hijo
  thread::spawn(move || { // Usamos move ya que queremos mover la propiedad del transmisor (tx) al hilo hijo.
    let val = String::from("hi");
    tx.send(val).unwrap(); // Enviamos el string a traves del canal. El metodo send() devuelve un enum Result, por lo que usamos unwrap() para manejar el error.
  });
}
```

Ahora haremos que el hilo principal reciba el string enviado por el hilo hijo. Para eso usamos el metodo `recv()` del receptor (rx). Este metodo tambien devuelve un enum Result, por lo que usamos unwrap() para manejar el error. El metodo recv() bloquea el hilo principal hasta que reciba un mensaje.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
  });

  let received = rx.recv().unwrap(); // Recibimos el string enviado por el hilo hijo.
  println!("Recibido: {}", received);
}
```

El receptor tiene 2 metodos utiles, el **recv** y el **try_recv**. Estamos usando **recv** que basicamente bloqueara la ejecucion del hilo principal y esperara hasta que se envie un valor por el canal. Una vez que se envia el valor, **recv** lo devolvera en un enum **Result**. Cuando el transmisor se cierra, **recv** devolvera un error para indicar que no se enviaran mas valores.

El metodo **try_recv** es **no bloqueante** por lo que devolvera inmediatamente un enum **Result**. Un valor **Ok** que contiene un mensaje si hay uno disponible y un valor **Err** si no hay mensajes esta vez. Usar **try_recv** es util si este hilo tiene otro trabajo que hacer mientras espera mensajes: Podriamos escribir un bucle que llame a **try_recv** cada cierto tiempo, maneje un mensaje si hay uno disponible y de lo contrario haga otro trabajo por un tiempo hasta que vuelva a verificar.

Hemos usado **recv** en este ejemplo por temas de simplicidad, no tenemos otro trabajo para que haga el hilo principal mientras espera un mensaje por lo que no hay problema que el hilo principal espere en estado bloqueado.

## Canales y transferencia de ownership
Las reglas de ownership juegan un papel importante en el envio de mensajes porque ayudan a escribir codigo concurrente seguro. Intentaremos usar un valor *val* en el hilo generado despues de haberlo enviado por el canal. Veamos:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
    println!("val is {}", val); // Esto no compila porque val fue movido a tx.send() y ya no es valido en este hilo. Hubo TRANSFERENCIA DE OWNERSHIP por el canal, por lo tanto no podemos usarlo mas en este hilo. Este print daria error en la compilacion
  });

  let recived = rx.recv().unwrap();
  println!("Recibido: {}", recived); // Esto si compila porque recived es una copia del valor enviado por el hilo hijo, por lo que podemos usarlo en el hilo principal. Ahora el ownership de val fue transferido al hilo principal y podemos usarlo en este hilo.
}
```

## Enviando multiples valores y viendo al receptor esperando
El hilo generado ahora enviara varios mensajes y se pausara durante un segundo entre cada mensaje:

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1)); // Pausamos el hilo hijo durante 1 segundo entre cada mensaje enviado por el canal. Esto nos permite ver al hilo principal esperando por los mensajes.
    }
  });

  for received in rx { // El receptor es un iterador que devuelve un valor cada vez que se envia un mensaje por el canal. Cuando el transmisor se cierra, el iterador termina.
    println!("Recibido: {}", received);
  }
}
```

Vemos que en el hilo principal ya no estamos llamando explicitamente a la funcion **recv**, en su lugar estamos tratando **rx** como un **ITERADOR**. Para cada valor recibido, lo imprimimos. Cuando el canal esta cerrado, la iteracion termina.

Al ejecutar este ultimo codigo deberiamos ver el siguiente output, obviamente todos los valores son imprimidos con un delay de 1 segundo entre cada uno y los imprime el hilo principal:

```
Recibido: hi
Recibido: from
Recibido: the
Recibido: thread
```

## Creando multiples productores
Hasta ahora hemos visto un solo hilo productor que envia mensajes a un hilo consumidor. Pero el canal que nos traemos de la libreria estandar es **multiple producer, single consumer**. Esto significa que podemos tener multiples hilos productores enviando mensajes a un solo hilo consumidor. Para lograr esto, necesitamos **CLONAR** el transmisor (tx) para cada hilo productor. Veamos un ejemplo:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
  let (tx, rx) = mpsc::channel();

  // Clonamos un transmisor para el primer hilo productor
  let tx1 = tx.clone();

  // Creamos un hilo productor
  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    // El hilo productor envia mensajes por el canal cada 1 segundo
    for val in vals {
      tx1.send(val).unwrap(); // Enviamos el mensaje por el canal tras el clon del transmisor (tx1)
      thread::sleep(std::time::Duration::from_secs(1));
    }
  });

  // Creamos otro hilo productor que usara otro transmisor (tx)
  thread::spawn(move || {
    let vals = vec![
      String::from("more"),
      String::from("messages"),
      String::from("for"),
      String::from("you"),
    ];

    for val in vals {
      tx.send(val).unwrap(); // Enviamos el mensaje por el canal tras el transmisor original (tx)
      thread::sleep(std::time::Duration::from_secs(1));
    }
  });

  // Ahora el hilo principal sera el que recibira los mensajes de ambos hilos productores
  for received in rx {
    println!("Recibido: {}", received);
  }
}
```

Entonces basicamente tenemos 2 hilos hijos que son productores y envian mensajes a traves de un mismo canal compartido con el receptor que es el hilo padre. El hilo padre ira recibiendo los mensajes de ambos hilos hijos a modo concurrente. El orden de los mensajes recibidos puede variar en cada ejecucion del programa ya que los hilos hijos se ejecutan de manera concurrente y no hay garantia de que uno termine antes que el otro. Por ejemplo una salida posible podria ser:

```
Recibido: hi
Recibido: more
Recibido: from
Recibido: messages
Recibido: the
Recibido: for
Recibido: thread
Recibido: you
```


