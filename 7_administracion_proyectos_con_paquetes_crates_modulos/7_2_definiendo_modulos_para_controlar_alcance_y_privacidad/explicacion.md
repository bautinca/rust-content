# Modulos
La idea central es aprender a **organizar el codigo dentro de un crate y decidir que partes pueden ser utilizadas desde otros modulos y cuales quedan privadas**. 

Imaginemos que tenemos un programa enorme asi:

```rust
fn iniciar_servidor() {
    // ...
}

fn conectar_base_de_datos() {
    // ...
}

fn autenticar_usuario() {
    // ...
}

fn crear_usuario() {
    // ...
}

fn eliminar_usuario() {
    // ...
}
```

Todo esta dentro del mismo archivo y estamos mezclando muchas responsabilidades en un solo lugar. Esto hace que a la larga el codigo sea dificil de mantener y entender.

Justamente es por ello que existen los **MODULOS** representados con *mod* para poder agrupar el codigo relacionado por ejemplo:

```rust
mod usuarios {
    fn autenticar() {
        // ...
    }

    fn crear() {
        // ...
    }
}

mod base_de_datos {
    fn conectar() {
        // ...
    }
}
```

Es decir ahora nuestro Crate esta compuesto por 2 modulos, por lo que tendriamos:

```
crate
├── usuarios // Modulo usuarios
│   ├── autenticar
│   └── crear
│
└── base_de_datos // Modulo base de datos
    └── conectar
```

Los modulos sirven para **organizar definiciones relacionadas** haciendo que el codigo sea mas facil de navegar y reutilizar

Rust representa los modulos como un **arbol** por ejemplo podriamos tener algo asi, modulo de modulos:

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

Entonces el arbol completo del crate seria:

```
crate
└── front_of_house
    ├── hosting
    │   ├── add_to_waitlist
    │   └── seat_at_table
    │
    └── serving
        ├── take_order
        ├── serve_order
        └── take_payment
```

Es decir, podemos tener **modulos de submodulos** tranquilamente, todo sea para organizar mejor la logica del codigo.

Entonces basicamente desde lo mas general a lo mas particular tendriamos: *Package -> Crate -> Module -> Submodule -> Funciones/structs/enums/etc.*

Ahora vamos de lleno a la sintaxis ¿Como declaramos un modulo? Con la palabra reservada `mod` y el nombre del modulo, por ejemplo:

```rust
mod hosting {
  // Contenido...
}
```

Ahora bien, lo bueno del uso de modulos es que podemos **organizarlo en archivos**! Por ejemplo podriamos tener asi:

```
src
├── main.rs
└── garden.rs
```

Entonces en *main.rs* simplemente podemos hacer en la cabecera:

```rust
mod garden; // Esto le dice a Rust que busque un archivo llamado garden.rs y lo cargue como modulo
```

Entonces basicamente dentro del *main.rs* podemos hacer una llamada a una funcion que este dentro del modulo garden asi:

```rust
mod garden;

fn main() {
    garden::plant();
}
```

Por otro lado los submodulos tambien pueden estar separados en archivos, por ejemplo podemos tener:

```
src/
├── main.rs // Modulo principal
├── garden.rs // Modulo garden
└── garden/
    └── vegetables.rs // Submodulo de garden
```

Entonces logicamente tendriamos:

```rust
// En main.rs
mod garden; // Carga el modulo garden
```

```rust
// En garden.rs
mod vegetables; // Carga el submodulo vegetables
```

```rust
// En vegetables.rs
pub fn plant() {
    println!("Plantando vegetales");
}
```

Entonces el arbol seria:

```
crate // Modulo principal
└── garden // Modulo garden
    └── vegetables // Submodulo vegetables
        └── plant // Funcion plant
```

Y es aca donde si vemos la funcion plant() es **publica** con *pub* y es aca donde entra el concepto de **privaciad**. 

**Por defecto en Rust los elementos de un modulo son privados** por ejemplo podemos tener:

```rust
mod usuarios {
    fn crear_usuario() { // Por defecto es privado
        println!("Creando usuario");
    }
}
```

Entonces si la funcion crear_usuario() es privada, no podemos llamarla desde otro modulo, por ejemplo:

```rust
mod usuarios {
    fn crear_usuario() { // Por defecto es privado
        println!("Creando usuario");
    }

    // La funcion crear_usuario() es privada, por lo que no puede ser llamada desde fuera del modulo usuarios, solo puede ser usada dentro del modulo usuarios, osea dentro de aca
}

fn main() {
    usuarios::crear_usuario(); // Esto no compila porque crear_usuario es privado
}
```

Entonces para hacer que algo sea publico usamos **pub** por ejemplo:

```rust
mod usuarios {
    pub fn crear_usuario() {
        println!("Creando usuario");
    }
}

fn main() {
    usuarios::crear_usuario(); // Esto compila porque crear_usuario es publico
}
``` 

Ahora bien, nosotros podemos hacer pub un modulo o pub en una funcion particular y son cosas distintas. Por ejemplo si tenemos:

```rust
mod usuarios {
    pub fn crear() {}
}
```

Esto quiere decir que la funcion crear() es publica pero el modulo usuarios es privado, por lo que no podemos acceder a la funcion crear() desde fuera del modulo usuarios.

Entonces si queremos que el modulo tambien sea accesible desde fuera tenemos que hacer publico el modulo tambien, osea:

```rust
pub mod usuarios {
    pub fn crear() {}
}
```

Y ahora si la funcion crear() es accesible desde fuera del modulo usuarios.

## Paths/Rutas
Necesitamos una forma de decirle a Rust donde esta algo dentro del arbol de modulos, por ejemplo podemos tener:

```
crate
└── garden
    └── vegetables
        └── Asparagus
```

Entonces podemos escribir lo siguiente en Rust:

```rust
crate::garden::vegetables::Asparagus // Esto es una ruta absoluta, que parte desde la raiz del crate hasta la funcion Asparagus. Primero decimos el crate, luego el modulo garden, luego el submodulo vegetables y finalmente la funcion Asparagus
```

Lo podemos pensar como una ruta de archvos. Entonces el *crate::* significa 'desde la raiz'

Si no queremos dar la ruta completa podemos usar **rutas relativas**, y es justamente para no escribir muchas veces la ruta absoluta que es muy larga, en su lugar podemos hacer:

```rust
use crate::garden::vegetables::Asparagus; // Esto es una ruta absoluta.

// Ahora podemos usar Asparagus directamente sin tener que escribir la ruta completa
let plant = Asparagus {};
```

Es decir, gracias a que usamos el *use* podemos usar la funcion Asparagus directamente sin tener que escribir la ruta completa

Si no usabamos *use* entonces al momento de llamar a la funcion Asparagus tendriamos que escribir la ruta completa:

```rust
let plant = crate::garden::vegetables::Asparagus {};
```

Justamente el *use* nos ahorra tener que escribir siempre la ruta completa.

Sin embargo ojo, una confusion muy comun es pensar que use hace publico algo, no, **USE NO HACE PUBLICO ALGO** por ejemplo:

```rust
use crate::usuarios::Usuario; // No estamos haciendo que Usuarios sea publico, simplemente estamos diciendo que dentro de este scope quiero poder escribir Usuario en lugar de toda la ruta siempre.
```

Recordar que la visibilidad de algo se controla con *pub* mientras que los atajos se controlan con *use*

Otra cosa muy importante es que **MODULO NO ES IGUAL A ARCHIVO**, podemos tener por ejemplo el siguiente proyecto:

```
src/
├── main.rs
└── garden.rs
```

Entonces podemos pensar que main.rs es un modulo y garden.rs es otro modulo, pero en realidad puede no ser asi, quiza garden.rscontiene la definicion del modulo garden, pero no necesariamente es un modulo en si mismo. El modulo conceptualmente es *garden* y el archivo es *garden.rs*