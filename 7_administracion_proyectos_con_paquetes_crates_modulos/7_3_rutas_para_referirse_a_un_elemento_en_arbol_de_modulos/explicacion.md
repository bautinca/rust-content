# Incluyendo rutas al ambito con la palabra clave use
Ya vimos como usar la palabra clave *use*. Agregar *use* y una ruta en un ambito es similar a crear un enlace simbolico en el filesystem. Al agregar `use crate::front_of_house::hosting` en la raiz del crate, hace que *hosting* sea ahora un nombre valido en ese ambito como si el modulo *hosting* estuviera definido en ese ambito.

Veamos un ejemplo, por ejemplo la siguiente manera seria incorrecto:


```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist(); // Esto tiraria error al compilar porque hosting no es visible en este ambito
    }
}
```

Para hacer que lo anterior funcione , podemos agregar `use crate::front_of_house::hosting;` dentro del modulo *customer* para que *hosting* sea visible en ese ambito. 

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist(); // Ahora esto funciona porque hosting es visible en este ambito
    }
}
``` 

Traer el módulo padre de la función al ámbito con use significa que tenemos que especificar el módulo padre cuando llamamos a la función. Especificar el módulo padre cuando llamamos a la función hace que quede claro que la función no está definida localmente, al tiempo que minimiza la repetición de la ruta completa por lo que esta ultima practica es la preferida de hacer, por ejemplo:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

mod customer {
    pub fn eat_at_restaurant() {
        add_to_waitlist(); // Ahora esto funciona porque add_to_waitlist es visible en este ambito sin embargo hosting no es visible en este ambito, por lo que no podemos llamar a hosting::add_to_waitlist() directamente
    }
}
```

Por otro lado, cuando traemos structs, enums y otros items con use, es idiomático especificar la ruta completa, por ejemplo:

```rust
use std::collections::HashMap; // Nos traemos el struct HashMap de la biblioteca estandar al ambito de un crate binario

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

No hay una razón fuerte detrás de este idioma: es solo la convención que ha surgido, y la gente se ha acostumbrado a leer y escribir código Rust de esta manera.

La excepción a este idioma es si estamos trayendo dos elementos con el mismo nombre al ámbito con declaraciones use, porque Rust no lo permite. Ahora veremos cómo traer dos tipos Result al ámbito que tienen el mismo nombre pero módulos padres diferentes, y cómo referirse a ellos.

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

Vemos que como *fmt* y *io* son los módulos padres de Result, podemos referirnos a ellos como *fmt::Result* y *io::Result*. Esto es una forma de desambiguar los nombres de los elementos que tienen el mismo nombre pero diferentes módulos padres. Si o si teniamos que especificar el modulo padre para poder referirnos a ellos, de lo contrario Rust no sabria a cual de los dos Result nos estamos refiriendo.

## Proporcionando nuevos nombres con el keyword as
Hay otra solución a este problema de traer dos elementos con el mismo nombre al ámbito con use: después de la ruta, podemos especificar as y un nuevo nombre local, o alias, para el tipo. Veamos un ejemplo:

```rust
use std::fmt::Result;
use std::io::Result as IoResult; // Aca usamos as para darle un nuevo nombre local a Result de io, para poder referirnos a el como IoResult

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

## Re-exportando nombres con pub use
Cuando traemos un nombre al ámbito con la keyword use, el nombre es privado para el ámbito en el que lo importamos. Si queremos que el nombre este disponible para otros modulos que usan nuestro modulo como una libreria podemos usar pub use para re-exportar el nombre. Esto es útil cuando queremos que los usuarios de nuestro modulo puedan usar un nombre sin tener que conocer la ruta completa del modulo donde se define. Veamos un ejemplo:

```rust
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

pub use crate::front_of_house::hosting; // Re-exportamos hosting para que otros modulos puedan usarlo sin conocer la ruta completa

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // Esto antes no funcionaba pero ahora funciona porque hosting es visible en este ambito gracias a la re-exportacion que hicimos con pub use.
}
```

Re exportar es util cuando la estructura interna de nuestroo codigo es diferente de como los programadores que llaman a nuestro codigo pensarian sobre el dominio. Con pub use podemos escribir nuestro codigo con una estructura pero exponer una estructura diferente

## Usando paquetes externos
Habiamos hecho un proyecto de juego adivinanzas donde usamos el paquete externo **rand** para obtener numeros aleatorios. Para usar el paquete rand a nuestro proyecto agregamos una linea al *Cargo.toml* siguiente: 

```toml
[dependencies]
rand = "0.8.5"
```

Añadir esta linea al *Cargo.toml* de nuestro proyecto le dice a Cargo que descargue todo el paquete *rand* y cualquier dependencia Crates que tenga y haga que absolutamente todo el contenido del paquete *rand* este disponible para nuestro proyecto.

Luego para llevar las definiciones de *rand* al ambito de nuestro paquete agregamos una linea **use** que comienza con el nombre del paquete *rand* y enumera los items que queremos traer al ambito, si recodamos nosotros nos trajimos al ambito el trait *Rng* y ademas llamamos a la funcion del paquete *thread_rng* asi:

```rust
use rand::Rng; // Nos taemos del paquete rand el trait Rng para poder usarlo en este ambito ya que thread_rng devuelve un objeto que implementa el trait Rng

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100); // Llamamos a la funcion thread_rng del paquete rand y luego llamamos al metodo gen_range del trait Rng que nos trajimos al ambito con use
}
```

Los miembros de la comunidad de Rust han puesto muchos paquetes a disposicion en **crates.io** y traer cualquiera de elos a nuestro paquete involucra estos mismos pasos, listarlo primero en el archivo *Cargo.toml* y luego usar la palabra clave *use* para traer los items que queremos al ambito de nuestro paquete.

Tener en cuenta que la biblioteca estandar de rust **std** tambien es una crate externa a nuestro paquete. Debido a que la biblioteca estandar se envia con el lenguaje Rust no necesitamoss cambiar *Cargo.toml* para usarla, Pero si necesitamos referirnos a el con *use* para traernos items de alli al ambito de nuestro paquete. Por ejemplo con HashMap usuarimos esta linea:

```rust
use std::collections::HashMap; // Nos traemos el struct HashMap de la biblioteca estandar al ambito de un crate binario
```

Esta ultima es una ruta absoluta que comienza con *std* osea el nombre del crate de la biblioteca estandar.

## Usando rutas anidadas para limpiar listas use grandes
Si estamos usando varios elementos definidos en el mismo crate o el mismo módulo, enumerar cada elemento en su propia línea puede ocupar mucho espacio vertical en nuestros archivos. Por ejemeplo:

```rust
use std::cmp::Ordering;
use std::io;
```

En su lugar podemos usar rutas anidadas para agrupar elementos que comparten un prefijo común, lo que hace que la lista de *use* sea más concisa y legible. Por ejemplo, podemos reescribir las líneas anteriores así:

```rust
use std::{cmp::Ordering, io};
```

En programas mas grandes, traer muchos items al ambito desde el mismo crate o modulo usando rutas anidadas puede reducir la cantidad de declaraciones use necesarias en gran medida.

Podemos usar una ruta anidada en cualquier nivel de una ruta, lo que es útil cuando combinamos dos sentencias use que comparten una sub-ruta. Otro ejemplo:

```rust
use std::io;
use std::io::Write;
```

En su lugar con rutas anidadas podemos hacer:

```rust
use std::io::{self, Write};
```

Es decir nos traemos a *io* y *Write* al ambito, y *self* se refiere a *io* en este caso. Esto es util cuando queremos traer un modulo y algunos de sus submodulos al ambito.

## El operador asterisco (Glob)
Si queremos incluir al ambito todos los elementos publicos definidos en una ruta podemos especificar esa ruta seguido del operador glob *. Por ejemplo:

```rust
use std::collections::*;
```

Esta sentencia *use* trae todos los elementos publicos (pub) definidos en el modulo *collections* de la biblioteca estandar al ambito. Esto es util cuando queremos usar muchos elementos de un modulo y no queremos escribir una sentencia *use* por cada uno de ellos.

Ojo al usar este operador ya que el operador puede hacer mas dificil saber que elementos estan en el ambito y donde se definio un elemento que esta siendo utilizado en nuestro programa. Ademas si la dependencia cambia sus definiciones entonces lo que hemos importado tambien cambia lo que puede provocar errores de compilacion cuando actualicemos la dependencia. por ejemplo si la dependencia añade una definicion con el mismo nombre que una definicion nuestra en el mismo ambito.