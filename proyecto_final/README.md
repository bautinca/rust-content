# Proyecto Final: Construyendo un servidor web multithread

En este proyecto haremos un servidor web con Rust donde simplemente en root `/` nos devolvera un mensaje de *hola* y se vea algo asi:

[](./images_readme/image.png)

Nuestra secuencia de trabajo sera la siguiente:

1. Aprender un poco sobre los protocolos TCP y HTTP
2. Escuchar conexiones TCP en un socket
3. Analizar un pequeño numero de peticiones HTTP
4. Crear una respuesta HTTP adecuada
5. Mejorar el rendimiento de nuestro servidor con un **thread pool**

Por lo tanto escribiremos un servidor web HTTP muy basico y el thread pool manualmente para que podamos aprender ideas y tecnicas generales detras de los *crates* que podriamos usar en el futuro donde nos abstraemos varios de estos detalles que veremos en este proyecto.