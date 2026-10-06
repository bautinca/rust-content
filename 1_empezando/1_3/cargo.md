# Cargo

Ahora vamos a hablar de **Cargo**. Cargo es el sistema de compilacion y administrador de paquetes de Rust. Muchos que usan Rust para proyectos grandes usan Cargo ya que Cargo maneja muchas dependencias por nosotros, como descargar las bibliotecas de las que depende nuestro codigo, y compilar esas bibliotecas

> **NOTA**: Llamamos como dependencias a las bibliotecas de las que depende nuestro codigo del proyecto

Es logico que programas mas simples como el 'hola mundo' no tienen dependencias por lo que no hace falta Cargo, pero a medida que armemos proyectos mas complejos agregaremos dependencias y para ello Cargo es fundamental.

Vamos a crear un proyecto usando Cargo! Para levantar un proyecto con Cargo se ejecuta lo sig. en la terminal:

```shell
cargo new hello_cargo
cd hello_cargo
```

Lo que hace es crear un proyecto usando Cargo donde se creara un directorio llamado *hello_cargo*. Cargo dentro del directorio ha generado estos archivos:

## Cargo.toml
El archivo TOML se ve asi:
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

Basicamente el *[package]* es un encabezado de seccion que indica que las siguientes declaraciones estan configurando este paquete. A medida que agreguemos mas info. a este archivo, agregaremos otras secciones

Luego las sig. 3 lineas indican info. de config. que Cargo necesita para compilar el programa como el nombre, la version y la edicion de Rust que se usara.

La ultima linea *[dependencies]* es el comienzo de una seccion para que enumere cualquier dependencia de tu proyecto. En Rust los paquetes de codigo se denominan **crates**. Es logico que para este proyecto simple de 'hola mundo' no necesitamos otros crates.

## Directorio src/ con main.rs 
Dentro de *./hello_cargo/src/* esta el archivo *main.rs* que dice lo siguiente:
```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo espera que nuestros archivos de origen vivan dentro del directorio *src/*. El directorio del proyecto de nivel superior es solo para archivos README, info. de licencia, archivos de config. y cualquier otra cosa que no este relacionada con nuestro codigo fuente dentro de *src/*.

Si arrancamos un proyecto simple sin usar Cargo y luego queremos usar Cargo es facil, tan solo tenemos que crear la estructura tal cual como hace Cargo, deberiamos crear un archivo *Cargo.toml* adecuado y mover el codigo fuente a dentro del directorio *src/*.

Una forma sencilla de crear un Cargo.toml rapidamente si no tenemos uno para nuestro proyecto es ejecutar lo siguiente en la terminal:
```shell
cargo init
```

Esto creara un cargo.toml automaticamente para nuestro proyecto.

## Construir y ejecutar un proyecto de Cargo
Ya sabemos como compilar y ejecutar un proyecto simple, pero ¿Como lo hacemos con Cargo? Nos tenemos que parar sobre el directorio del proyecto *hello_cargo/* y ejecutar lo sig. en la terminal:
```shell
cargo build
```

La linea lo que hace es compilar todo el proyecto donde al compilarlo vamos a ver que nos largara algunos archivos donde el que mas interesa es:

- **Cargo.lock**: Este archivo rastrea las versiones exactas de las dependencias del proyecto. Como el proyecto *hello_cargo* no tiene dependencias es obvio que en el archivo tendra poco contenido. Este archivo por lo general **nunca se toca manualmente** ya que todo se genera automaticamente

Ahora bien, ya compilamos el proyecto con Cargo y como lo ejecutamos? Con la sig. linea en la terminal:
```shell
cargo run
```

Lo bueno de este utlimo comando es que en realidad es un **compilacion + ejecucion**.

Cargo tambien proporciona el sig. comando:
```shell
cargo check
```

Lo que hace este ultimo comando es comprobar rapidamente nuestro codigo para asegurar que compila antes de compilarlo (por lo que no produce el ejecutable)

Muchas veces el *cargo check* se usa mas que el *cargo build* ya que omite el paso de compilar el codigo para producir un ejecutable. Si estamos verificando continuamente el proyecto mientras escribimos codigo usar el *cargo check* es mas adecuado ya que acelera el proceso de informarnos si nuestro proyecto todavia aun esta compilando. Por lo que la gran mayoria usa *cargo check* a menudo mientras codean para estar seguros de que compila y luego al final de todo usan *cargo build* para compilar y estan listos para usar el ejecutable generado.

