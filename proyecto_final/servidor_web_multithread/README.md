El problema que teniamos en el servidor web singlethread es que es justamente de un solo hilo, eso quiere decir que si por ejemplo una solicitud tarda mucho en procesarse, el servidor no puede atender otras solicitudes hasta que termine de procesar la primera. Esto puede generar una mala experiencia para los usuarios, ya que pueden tener que esperar mucho tiempo para recibir una respuesta del servidor.

Aca crearemos un nuevo servidor web, pero esta vez de manera multithread. Esto significa que cada vez que un cliente se conecte al servidor, se creará un nuevo hilo para manejar esa conexión, lo que permite que el servidor pueda atender múltiples clientes al mismo tiempo.

Para levantar el servidor web tenemos que hacer:

```bash
cargo run
```

Luego para hacer la prueba de que efectivamente sea multithread podemos hacer en una pestaña entrar al endpoint `/sleep` y en otra pestaña entrar al endpoint `/`. Si el servidor es multithread, la segunda pestaña debería poder cargar la página sin tener que esperar a que termine de procesarse la solicitud a `/sleep`. Si el servidor fuera singlethread, la segunda pestaña tendría que esperar a que termine de procesarse la solicitud a `/sleep` antes de poder cargar la página.