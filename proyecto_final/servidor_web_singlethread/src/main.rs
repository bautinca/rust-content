use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
  let listener = TcpListener::bind("127.0.0.1:8080").unwrap(); // Escuchamos conexiones TCP en el puerto 8080

  println!("Servidor escuchando en http://127.0.0.1:8080");

  for stream in listener.incoming() { // Iteramos sobre las conexiones entrantes
    match stream { // Las conexiones entrantes son de tipo Result<TcpStream, Error>
      Ok(stream) => handle_connection(stream), // Si la conexion TCP es exitosa entonces llamamos a la funcion handle_connection para procesar la solicitud HTTP
      Err(error) => eprintln!("No se pudo aceptar la conexión: {error}"),
    }
  }
}

// Esta funcion maneja la conexion TCP del cliente y procesa la solicitud HTTP
fn handle_connection(mut stream: TcpStream) { // Recibe un TcpStream mutable. Este stream representa la conexion TCP con el cliente y nos permite leer la solicitud HTTP y escribir la respuesta HTTP
  let mut buffer = [0; 1024]; // Definimos un buffer de 1024 bytes para almacenar la solicitud HTTP entrante
                              // El buffer reserva espacio para leer hasta 1024 bytes de la peticion HTTP entrante. No son 1024 caracteres sino bytes.
                              // Esto es porque una peticion tipica del navegador puede verse asi al llegar una peticion GET del cliente que manda la solicitud HTTP:
                              
                              // GET / HTTP/1.1
                              // Host: localhost:8080
                              // Connection: keep-alive
                              // ...

                              // Para este servidor 1024 bytes de buffer es mas que suficiente ya que solo necesitamos leer la primer linea
                              // de la solicitud HTTP para determinar si es un GET a la ruta raíz ("/") o no. Si la solicitud HTTP es más grande que 1024 bytes, simplemente ignoramos el resto de la solicitud y respondemos con un 404 NOT FOUND.

  if stream.read(&mut buffer).is_err() { // Leemos la solicitud HTTP del cliente y la almacenamos en el buffer. Si ocurre un error al leer la solicitud, imprimimos un mensaje de error y retornamos
    return;
  }

  let request = String::from_utf8_lossy(&buffer); // Convertimos el buffer de bytes a un String para poder procesar la solicitud HTTP
  // Obtenemos la primera línea de la solicitud HTTP, que contiene el método HTTP, la ruta y la versión del protocolo. Si no hay ninguna línea, usamos un valor por defecto
  let request_line = request 
    .lines()
    .next()
    .unwrap_or_default();

  // Si la solicitud HTTP es un GET a la ruta raíz ("/"), entonces respondemos con un código de estado 200 OK y el contenido del archivo index.html. Si la solicitud HTTP es cualquier otra cosa, respondemos con un código de estado 404 NOT FOUND y un mensaje de error en HTML
  // Tambien agregamos nuevo endpoint /sleep que simula una solicitud que tarda 10 segundos en procesarse. Esto nos permite probar el comportamiento del servidor web singlethread, ya que si un cliente hace una solicitud a /sleep, el servidor no puede atender otras solicitudes hasta que termine de procesar la solicitud a /sleep.
  let (status_line, contents) = match request_line {
    "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", include_str!("../index.html")),
    "GET /sleep HTTP/1.1" => {
      std::thread::sleep(std::time::Duration::from_secs(10));
      ("HTTP/1.1 200 OK", include_str!("../index.html"))
    }
    _ => ("HTTP/1.1 404 NOT FOUND", "<h1>404 - Page not found</h1>")
  }; 

  // Construimos la respuesta HTTP con el código de estado y el contenido
  let response = format!(
    "{status_line}\r\nContent-Length: {}\r\nContent-Type: text/html; charset=UTF-8\r\nConnection: close\r\n\r\n{contents}",
    contents.len()
  );

  // Escribimos la respuesta HTTP en el stream de salida del cliente. Si ocurre un error al escribir la respuesta, lo ignoramos
  let _ = stream.write_all(response.as_bytes());
}