# Errores recuperables con Result
Cuando una operacion puede fallar de una manera que el programa puede manejar, en lugar de hacer panic! la funcion devuelve un **Result**.

En resumidas cuentas con *panic!* sucedia en un error irrecuperable, es decir, es como decir que el error que ocurrio fue tan grave que no hay chance de que el programa siga ejecutandose.

En cambio con el **Result** significa que tal operacion puede salir bien o fallar, es decir, es algo previsible, entonces para esas cosas previsibles nos devuelve cual de las 2 cosas ocurrio para que decidamos que hacer en caso del caso feliz y caso triste. Por ejemplo:

```rust
use std::fs::File;
fn main() {
    let f = File::open("hello.txt");
}
```

Lo cierto es que abrir un archivo puede fallar o no, es algo previsible, por tanto la funcion `File::open` devuelve un **Result** que puede ser `Ok` o `Err`. Por tanto el compilador nos obliga a manejar el caso de error, es decir, nos obliga a reconocer que la operacion puede fallar y tomar alguna accion antes de que el codigo se compile.

Exactamente Result seria un enum definido de la siguiente manera:

```rust
enum Result<T, E> {
    Ok(T), // Caso feliz, la operacion salio bien y devuelve un valor de tipo T
    Err(E), // Caso triste, la operacion fallo y devuelve un valor de tipo E que describe el error
}
```

Por ejemplo podriamos implementar una funcion de division donde sabemos que una division puede salir bien o fallar (cuando el denominador es 0), por lo tanto la funcion logicamente retornaria un Result:

```rust
fn dividir(dividendo: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        // Retornamos el caso triste con un mensaje de error
        Err(String::from("Error: No se puede dividir por cero"))
    } else {
        // Retornamos el caso feliz con el resultado de la division
        Ok(dividendo / divisor)
    }
}
```

Otra cosa importante es que cuando definimos los tipos de datos del Result el primer tipo de dato es el caso feliz y el segundo tipo de dato es el caso triste, osea:

```
Result<i32, String>
      │      │
      │      └── tipo del error
      │
      └── resultado exitoso
```

Veamos otro ejemplo por ejemplo con el **File::open** donde tambien nos devuelve un Result pero en este caso es un **Result\<File, std::io::Error>** ya que el caso feliz es un File y el caso triste es un std::io::Error que es un tipo de error que nos da mas informacion sobre el error ocurrido.

```rust
use std::fs::File;

let f = File::open("hello.txt"); // Esto devuelve un Result<File, std::io::Error>, osea un Ok(File) o un Err(std::io::Error)
```

La ventaja es que al 'atajar' el error **no destruye el programa** como lo hacia el panic!, vos decidimos las acciones a tomar si sucede tal error esperable.

Entonces logicamente el error lo manejamos con un **match**, por ejemplo al intentar abrir un archivo:

```rust
use std::fs::File;

let f = match File::open("hello.txt") {
    Ok(file) => file, // Caso feliz, el archivo se abrio correctamente
    Err(error) => {
        // Caso triste, nos permite reaccionar que hacer en caso de error, por ejemplo podemos hacer un panic! con el mensaje de error pero lo cierto es que podemos hacer cualquier cosa, no estamos obligados a hacer un panic.
        panic!("Error al abrir el archivo: {:?}", error);
    }
};
```

Entonces en caso de que haya pasado un error puede ser por varios motivos, porque el archivo no existe, porque no tenemos los permisos de lectura, etc. Para examinar el tipo de error podemos usar el metodo `kind()` que nos devuelve un enum de tipo `std::io::ErrorKind` que nos permite saber que tipo de error ocurrio y tomar decisiones en base a eso. Por lo tanto seria otro **match** anidado, un match para el caso triste/feliz al momento de abrir el archivo y otro match para el caso triste del error para saber que tipo de error ocurrio y tomar decisiones en base a eso.

```rust
use std::fs::File;
use std::io::ErrorKind;

let greeting_file = match File::open("hello.txt") {
    // Caso feliz abrimos abrimos el archivo
    Ok(file) => file,

    // Caso triste nos tira error al abrir el archivo, consultamos el tipo de error
    Err(error) => match error.kind() {
        // Si el error es que no se encontro el archivo, podemos crear el archivo
        ErrorKind::NotFound => {
            // A su vez al crear el archivo puede fallar, por lo tanto debemos manejar ese error tambien con otro match
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            }
        }
        // Para cualquier otro tipo de error al intentar abrir el archivo tiramos un panic! con el mensaje de error
        _ => {
            panic!("Problem opening the file: {error:?}");
        }
    },
};
```

Es decir, en resumidas cuentas tenemos 3 matchs anidados!!, El esqueleto seria el siguiente:

```
                File::open()
                     │
             ┌───────┴────────┐
             │                │
            Ok               Err
             │                │
             ↓                ↓
         usar archivo      ¿qué error?
                              │
                    ┌─────────┴─────────┐
                    │                   │
                 NotFound             otro
                    │                   │
                    ↓                   ↓
              File::create()         panic!
```

Hay un metodo muy usado que es el **unwrap()** donde es una especie de "atajo" donde se aplicaria asi:

```rust
use std::fs::File;

let f = File::open("hello.txt").unwrap(); // Si el archivo se abre correctamente devuelve el archivo, si no hace panic! con el mensaje de error
```

Es decir, por defecto unwrap() hace panic! en caso de error. Osea seria lo mismo que tener:

```rust
use std::fs::File;

let f = match File::open("hello.txt") {
    Ok(file) => file,
    Err(error) => panic!("Error al abrir el archivo: {:?}", error),
};

// Esto ultimo es lo mismo que tener unwrap()
```

Tambien esta el **expect()** que es similar a unwrap() pero nos permite personalizar el mensaje de error en caso de que falle la operacion:

```rust
use std::fs::File;
let f = File::open("hello.txt").expect("No se pudo abrir el archivo hello.txt"); // Si el archivo se abre correctamente devuelve el archivo, si no hace panic! con el mensaje de error personalizado
```

Generalmente el expect() suele ser preferible antes que el unwrap() ya que el expect() proporciona mas contexto sobre el error ocurrido, mientras que unwrap() solo nos dice que hubo un error sin dar detalles.

## Propagacion de Errores
Hasta ahora vimos como manejar los errores esperables pero hay un concepto muy importante que es el hecho de **Propagar Errores**. A veces la funcion no deberia decidir que hacer con el error sino simplemente propagarlo hacia la funcion que lo llamo. Por ejemplo:

```rust
fn leer_usuario() -> Result<String, std::io::Error>
```

Quiza en caso de error la funcion no deberia decidir que hacer sino simplemente propagar el error hacia la funcion que la llamo y esa funcion que la llamo es la que deberia decidir que hacer con el error. Por ejemplo si la funcion `leer_usuario` falla, la funcion que la llamo puede decidir si hacer un panic!, o mostrar un mensaje de error al usuario, o intentar leer otro archivo, etc.

Esta decision de llama **Propagacion de Errores**, por ejemplo si queremos hacer una funcion simplemente propague el error entonces:

```rust
use std::fs::File;

let mut username_file = match File::open("hello.txt") {
    Ok(file) => file,
    Err(e) => return Err(e), // Propagamos el error hacia la funcion que llamo a esta funcion
};
```

Hay otro operador muy famoso que es el **?** que es un atajo para propagar errores, es decir, hace lo mismo que el match anterior pero de una manera mas concisa:

```rust
use std::fs::File;
let mut username_file = File::open("hello.txt")?; // Si la operacion falla, el error se propaga hacia la funcion que llamo a esta funcion, si no falla devuelve el archivo abierto
```

Osea es tal cual lo mismo que hace el ultimo match. Por ejemplo la siguiente funcion intenta abrir un archivo usando el operador **?**:

```rust
use std::fs::File;
fn abrir_archivo() -> Result<File, std::io::Error> {
    let f = File::open("hello.txt")?; // Si la operacion falla, el error se propaga hacia la funcion que llamo a esta funcion, si no falla devuelve el archivo abierto
    Ok(f) // Si la operacion fue exitosa, devolvemos el archivo abierto envuelto en un Ok
}
```

Ojo no confundir **unwrap** con **?**. La diferencia es que ? si hay un error propaga el error hacia la funcion que llamo a esta funcion, mientras que unwrap hace panic! y termina el programa. Por lo tanto ? es mas seguro que unwrap ya que no termina el programa sino que permite manejar el error de manera mas controlada.

Lo bueno del **?** hace que el codigo sea mucho mas **legible** en caso de que tengamos muchos errores esperables sin necesidad de usar muchos matchs anidados, por ejemplo:

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // Si falla la operacion, el error se propaga hacia la funcion que llamo a esta funcion

    let mut username = String::new();

    username_file.read_to_string(&mut username)?; // Si falla la operacion, el error se propaga hacia la funcion que llamo a esta funcion

    Ok(username)
}
```

Visualmente es mucho mas entendible asi que con muchos matchs anidados. Si no tuvieramos el ? entonces deberiamos hacer:

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => return Err(e), // Propagamos el error hacia la funcion que llamo a esta funcion
    };

    let mut username = String::new();

    // Una vez abierto el archivo, leemos el contenido del archivo y lo guardamos en la variable username por eso le pasamos una referencia mutable a username
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => return Err(e), // Propagamos el error hacia la funcion que llamo a esta funcion
    }
}
```

## Ultima simplificacion

Por ultimo vamos a ver el metodo **fs::read_to_string** que es una funcion que hace todo lo que hicimos en el ejemplo anterior, es decir, abre un archivo y lee su contenido y lo devuelve como un String. Por lo tanto podemos simplificar nuestro codigo de la siguiente manera:

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    let username: String = fs::read_to_string("hello.txt");
    username
}
```

Es decir, fs::read_to_string hace absolutamente todo lo que vimos, automaticamente abre el archivo, crea el string, leer el contenido del archivo y devuelve el string y si algo falla en todo ese proceso devuelve un Result con el error correspondiente. Por lo tanto es una funcion que nos ahorra mucho codigo y es muy util para leer archivos de texto. Osea si algo falla devuele *Err(io::Error)*