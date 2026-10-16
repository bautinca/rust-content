Aca crearemos un servidor web HTTP singlethread para facilitar las cosas. El servidor web unicamente respondera a solicitudes GET a la ruta raiz ("/") con un archivo HTML simple. Cualquier otra solicitud sera respondida con un mensaje de error 404.

No hay una buen diseño en codigo ya que todo se encuentra en un solo archivo *main.rs* y no hay modulos ni funciones separadas. Esto es intencional para mantener el ejemplo simple y enfocado en la funcionalidad basica de un servidor web HTTP.

Para levantar el servidor tenemos que hacer:

```bash
cargo run
```

El servidor se quedara levantado escuchando por el puerto 8080. Lo que queremos probar aca es que el servidor es **single thread**, es decir, que solo puede atender una solicitud a la vez. Para probar esto podemos abrir dos pestañas en el navegador y hacer una solicitud a la ruta ("/sleep") en una pestaña y luego hacer otra solicitud a la ruta raiz ("/") en la otra pestaña. La segunda solicitud no sera atendida hasta que la primera solicitud termine de procesarse.