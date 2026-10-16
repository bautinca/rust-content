use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

mod threadpool;

use threadpool::ThreadPool;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
  let pool = ThreadPool::new(4); // Creamos un pool de hilos con 4 hilos. Esto significa que el servidor puede atender hasta 4 clientes al mismo tiempo. Si hay más de 4 clientes conectados al mismo tiempo, los clientes adicionales tendrán que esperar a que uno de los hilos del pool termine de procesar una solicitud antes de poder ser atendidos.
                                 // Porque no destinamos un hilo por cliente? Porque si hay muchos clientes conectados al mismo tiempo, el servidor podría quedarse sin recursos y colapsar. Con un pool de hilos, podemos limitar la cantidad de hilos que se crean y evitar que el servidor se quede sin recursos, esto es particularmente util si por ejemplo
                                 // nos hacen un ataque de denegacion de servicio (DoS) y nos mandan muchas solicitudes al mismo tiempo. Con un pool de hilos, podemos limitar la cantidad de hilos que se crean y evitar que el servidor se quede sin recursos.

  println!("Servidor escuchando en http://127.0.0.1:8080");

  for stream in listener.incoming() {
    match stream {
      // Por cada conexion entrante, el servidor usa un hilo del pool para procesar la solicitud HTTP. Esto permite que el servidor pueda atender múltiples clientes al mismo tiempo, ya que cada cliente es atendido por un hilo diferente.
      Ok(stream) => {
        pool.execute(move || handle_connection(stream)); // El pool.execute recibe un closure igual que con thread::spawn.
      }
      Err(error) => eprintln!("No se pudo aceptar la conexión: {error}"),
    }
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1024];

  if stream.read(&mut buffer).is_err() {
    return;
  }

  let request = String::from_utf8_lossy(&buffer);
  let request_line = request.lines().next().unwrap_or_default();

  let (status_line, contents) = match request_line {
    "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", include_str!("../index.html")),
    // Agregamos nuevo endpoint /sleep que simula una solicitud que tarda 5 segundos en procesarse. Esto nos permite probar el comportamiento del servidor web multithread, ya que si un cliente hace una solicitud a /sleep, el servidor puede seguir atendiendo otras solicitudes mientras espera a que termine de procesar la solicitud a /sleep.
    "GET /sleep HTTP/1.1" => {
      std::thread::sleep(std::time::Duration::from_secs(10));
      ("HTTP/1.1 200 OK", include_str!("../index.html"))
    }
    _ => ("HTTP/1.1 404 NOT FOUND", "<h1>404 - Page not found</h1>")
  };

  let response = format!(
    "{status_line}\r\nContent-Length: {}\r\nContent-Type: text/html; charset=UTF-8\r\nConnection: close\r\n\r\n{contents}",
    contents.len()
  );

  let _ = stream.write_all(response.as_bytes());
}