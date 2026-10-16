use std::thread;
use std::sync::{mpsc, Arc, Mutex};

// Aca crearemos el struct ThreadPool que nos permitira crear un pool de hilos para manejar las conexiones entrantes al servidor web multithread. Esto nos permite limitar la cantidad de hilos que se crean y evitar que el servidor se quede sin recursos, esto es particularmente util si por ejemplo nos hacen un ataque de denegacion de servicio (DoS) y nos mandan muchas solicitudes al mismo tiempo. Con un pool de hilos, podemos limitar la cantidad de hilos que se crean y evitar que el servidor se quede sin recursos.
pub struct ThreadPool {
  workers: Vec<Worker>, // Un vector de workers que representan los hilos del pool
  sender: Option<mpsc::Sender<Job>>, // Un canal para enviar trabajos a los hilos del pool
}

// Job es un alias de tipo para un closure 'boxeado'. FnOnce porque cada closure se ejecuta 1 sola vez, Send para transferirlo entre hilos
// y 'static porque el closure debe vivir por todo el tiempo de vida del programa. Esto es necesario porque los hilos del pool pueden vivir por todo el tiempo de vida del programa y no sabemos cuando se van a ejecutar los closures que se les envian.
type Job = Box<dyn FnOnce() + Send + 'static>;

// El struct de cada worker sera asi:
struct Worker {
  id: usize, // Un identificador unico para cada worker
  thread: Option<thread::JoinHandle<()>>, // El hilo del worker. Option porque el hilo puede ser None si el worker no esta activo
}

// Entonces cada worker tiene un ID y un hilo.
// Lo que haremos ahora sera implementar los metodos para el pool de hilos
impl ThreadPool {
  // El constructor new recibe unicamente la cantidad de hilos 
  pub fn new(size: usize) -> ThreadPool {
    // Validar que la cantidad sea mayor a 0
    assert!(size > 0);

    // Creamos un canal para enviar trabajos a los hilos del pool
    let (sender, receiver) = mpsc::channel();
    // El receiver debe ser compartido entre los hilos del pool, por lo que lo envolvemos en un Arc y un Mutex. Arc para poder compartirlo entre los hilos y Mutex para poder acceder a el de manera segura entre los hilos.
    let receiver = Arc::new(Mutex::new(receiver));

    // Creamos un vector de workers con la cantidad de hilos especificada
    let mut workers = Vec::with_capacity(size);

    // En cada iteracion del bucle creamos un worker y lo agregamos al vector de workers
    for id in 0..size {
      // Al crear el worker este recibe un ID y el receiver del canal para poder recibir trabajos. El worker se encargara de ejecutar los trabajos que reciba del canal.
      workers.push(Worker::new(id, Arc::clone(&receiver))); // Arc::clone para clonar el Arc y poder compartirlo entre los hilos del pool
    }
    ThreadPool { workers, sender: Some(sender) }
  }

  // Ahora implementaremos el metodo execute que recibe un closure 'F' y lo envia al canal para que un worker lo ejecute. El closure debe ser 'boxeado' para poder enviarlo a traves del canal, ya que los canales no pueden enviar tipos genericos.
  pub fn execute<F>(&self, f: F)
  where F: FnOnce() + Send + 'static, // El closure debe ser FnOnce, Send y 'static para poder enviarlo a traves del canal y ejecutarlo en otro hilo
  {
    let job = Box::new(f); // Boxeamos el closure para poder enviarlo a traves del canal
    self.sender.as_ref().unwrap().send(job).unwrap(); // Enviamos el trabajo al canal. Si ocurre un error al enviar el trabajo, el programa se detiene con un panic
  }
}

// Como dijimos el receiver se envuelve en un Arc y un Mutex porque multiples workers necesitan compartir el mismo receptor. Arc para
// multiple ownership y Mutex para garantizar que solo un worker tome el job a la vez.

// Ahora implementamos metodos del worker que seria basicamente el constructor unicamente:
impl Worker {
  // El worker recibe su ID, y el receiver del canal para poder recibir trabajos
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    // Creamos un hilo para el worker. El hilo se ejecuta en un bucle infinito y espera a recibir trabajos del canal. Cuando recibe un trabajo, lo ejecuta y vuelve a esperar a recibir otro trabajo.
    let thread = thread::spawn(move || loop {
      let message = receiver.lock().unwrap().recv(); // Bloqueamos el mutex para poder acceder al receiver y recibimos un trabajo del canal. Si ocurre un error al recibir el trabajo, el programa se detiene con un panic

      match message {
        Ok(job) => {
          println!("Worker {id} got a job; executing.");
          job(); // Ejecutamos el trabajo recibido
        }
        Err(_) => {
          println!("Worker {id} disconnected; shutting down.");
          break; // Si ocurre un error al recibir el trabajo, salimos del bucle y terminamos el hilo del worker
        }
      }
    });

    Worker {
      id,
      thread: Some(thread), // Guardamos el hilo del worker en el struct
    }
  }
}

// Entonces cada worker corre un loop infinito donde adquiere el lock del mutex, recibe un job del canal, libera el lock y ejecuta el job.

// Lo ultimo que nos queda por hacer es implementar el metodo Drop para el pool de hilos. Esto es necesario porque cuando el pool de hilos se destruye, debemos asegurarnos de que todos los hilos del pool se detengan correctamente y no queden hilos colgando. Para esto, enviaremos un mensaje de cierre a cada worker y luego esperaremos a que cada worker termine de ejecutar su trabajo antes de destruir el pool de hilos.
impl Drop for ThreadPool {
  fn drop(&mut self) {
    drop(self.sender.take()); //Cerramos el canal, los workers reciben Err

    // Iteramos por los workers del thread pool
    for worker in &mut self.workers {
      // Para cada worker hacemos que espere a que su hilo termine de ejecutar su trabajo antes de destruir el pool de hilos. Esto garantiza que no queden hilos colgando cuando el pool de hilos se destruye.
      println!("Shutting down worker {}", worker.id);
      if let Some(thread) = worker.thread.take() { // Si el worker tiene un hilo
        thread.join().unwrap(); // Esperamos a que el hilo termine de ejecutar su trabajo antes de destruir el pool de hilos. Si ocurre un error al esperar a que el hilo termine, el programa se detiene con un panic
      }
    }
  }
}