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

  if stream.read(&mut buffer).is_err() {
    return;
  }

  let request = String::from_utf8_lossy(&buffer);
  let request_line = request
    .lines()
    .next()
    .unwrap_or_default();

  let (status_line, contents) = if request_line == "GET / HTTP/1.1" {
    ("HTTP/1.1 200 OK", include_str!("../index.html"))
  } else {
    ("HTTP/1.1 404 NOT FOUND", "<h1>404 - Page not found</h1>")
  };

  let response = format!(
    "{status_line}\r\nContent-Length: {}\r\nContent-Type: text/html; charset=UTF-8\r\nConnection: close\r\n\r\n{contents}",
    contents.len()
  );

  let _ = stream.write_all(response.as_bytes());
}