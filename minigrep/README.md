Vamos a construir una version simplificada de la funcion **grep** de la linea de comandos. La funcion **grep** busca un patron en un archivo y devuelve las lineas que contienen ese patron.

Basicamente a nuestra aplicacion al ejecutarla le pasaremos un texto a buscar y luego le pasamos el nombre del archivo donde buscar. Por ejemplo:

```bash
cargo run -- texto_a_buscar archivo.txt
```

Para una busqueda normal sensitive case (default) seria:
```bash
cargo run -- to poem.txt # Buscamos el texto 'to' en el archivo poem.txt pero distinguiendo mayusculas y minusculas
```

Para una busqueda insensitive case (no importa mayusculas o minusculas) tenemos que setear la variable de entorno **IGNORE_CASE** a cualquier valor, por ejemplo 1:
```bash
IGNORE_CASE=1 cargo run -- to poem.txt # En este caso buscamos el texto 'to' en el archivo poem.txt sin importar mayusculas o minusculas
```