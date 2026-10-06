# Introduccion

## Terminos Clave

Terminos clave del mundo Rust:

- **Rust**: Lenguaje de programacion

- **Cargo**: Es el gestor de paquetes integrado de Rust para gestionar dependencias

- **crates.io**: Es un repositorio de paquetes Rust que pueden añadirse como dependencias

- **Compilador**: Rust tiene su propio compilador, eso es que traduce el codigo Rust a codigo maquina y durante la compilacion proporciona errores en tiempo de compilacion

Primero instalamos **Rust + Cargo** logicamente

¿Que tiene que ver CARGO en todo esto? Imaginemos que somos un desarrollador backend usando Rust y estamos trabajando
en un proyecto complejo, es obivo que necesitamos usar diferentes librerias o 'paquetes' para manejar por un lado la base de datos,
por otro lado realizar calculos complejos, por otro para interactuar con APIs, y asi... Aca es donde **CARGO** se vuelve fundamental

Pensemos en Cargo como un asistente personal para nuestros proyectos de Rust. No solo nos ayuda a crear nuevos proyectos con una estructura organizada, sino que tambien se encarga de descargar y gestionar todas las depedencias que nuestro codigo Rust necesita. Esto es crucial en el desarrollo backend en proyectos de ciencia de la computacion donde a menudo se utilizan muchas librerias externas.

Conectandolo con **crates.io** cuando nuestro proyecto necesita una funcionalidad especifica como una libreria para procesar datos o para manejar peticiones HTTP lo que hace Cargo es buscar estos paquetes necesarios en **crates.io**. Esto es basicamente un repositorio central donde los desarrolladores de Rust comparten sus librerias!. Es como una gran biblioteca de herramientas que podemos usar en nuestros propios proyectos

Por ejemplo supongamos que estamos desarrollando el backend de una aplicacion con Rust y la app necesita conectarse a una base de datos PostgreSQL. En lugar de escribir todo el codigo de 0 para la conexion a la base de datos, podemos usar Cargo para añadir un **"crate"** (se le dice a un paquete de Rust) como *postgres* de **crates.io** a nuestro proyecto. Cargo es la que se encarga de descargarla y hacerla disponible para nuestro codigo.

> **NOTA**: El comando `rustup` sirve para todo lo que es el manejo de las versiones de Rust, por ello por ejemplo si queremos actualizar Rust a la ultima version tenemos que hacer `rustup update` o si lo queremos desinstalar `rustup self uninstall`