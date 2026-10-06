# Separando modulos en diferentes archivos
Hasta ahora la documentacion mostraba modulos definidos dentro **del mismo archivo**. Pero en proyectos reales esto escala mal. Rust nos permite mover cada modulo a su propio archivo y el arbol de modulos (la jerarquia logica) no cambia, solo cambiaria donde fisicamente vive el codigo.

¿Como funciona el rol de 'mod'? El *mod* en Rust no es un #include como en C/C++. No copia codigo. Lo que hace es decirle al compilador de Rust *"existe un modulo con este nombre, busca su codigo en un archivo separado"*

Entonces por ejemplo una vez que declaramos `mod front_of_house;` en algun lugar del arbol logico el compilador sabe que ese modulo existe y que debe buscar su codigo en un archivo llamado `front_of_house.rs`. Luego el resto del proyecto lo referencia por su ruta logica *crate::front_of_house::hosting*, no por el nombre del archivo

Veamos un ejemplo para entender todo esto. Partimos del archivo crate principal `src/lib.rs` que originalmente contenia el sig. codigo:

```rust
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Llamando a la funcion add_to_waitlist del modulo hosting
    crate::front_of_house::hosting::add_to_waitlist();
}
```

Lo que haremos sera mover el modulo `front_of_house` a su propio archivo y en este archivo que solo este la declaracion del modulo. El resultado sera:

```rust
// src/lib.rs
mod front_of_house; // Declaramos el modulo front_of_house, el compilador buscara su codigo en el archivo front_of_house.rs

pub use crate::front_of_house::hosting; // Reexportamos el modulo hosting para que sea accesible desde fuera del crate

pub fn eat_at_restaurant() {
    // Llamando a la funcion add_to_waitlist del modulo hosting
    crate::front_of_house::hosting::add_to_waitlist();
}
```

Hasta ahora lo que tenemos no compilaria ya que el archivo `front_of_house.rs` no existe. Creamos el archivo `src/front_of_house.rs` y movemos el contenido del modulo `front_of_house` a este archivo:

```rust
// src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

Entonces el compilador encontraria el `mod front_of_house;` en *lib.rs* y automaticamente buscaria el archivo *src/front_of_house.rs*. No le tenemos que decir el path, solo con el nombre del modulo es suficiente.

Si ademas queremos separar el submodulo `hosting` entonces solo cambiariamos el archivo *src/front_of_house.rs* para que solo tenga:

```rust
// src/front_of_house.rs
pub mod hosting; // Declaramos el submodulo hosting, el compilador buscara su codigo en el archivo hosting.rs
```

Y creamos el archivo *src/front_of_house/hosting.rs* con el contenido del submodulo:

```rust
// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```

Notemos como ahora *hosting.rs* vive dentro del directorio llamado *front_of_house/* reflejando lo que es la **jerarquia de modulos**. Entonces el compilador seguiria esta convencion:

|Situacion|Donde busca el archivo|
|----|--|
|`mod front_of_house;`|`src/front_of_house.rs`|
|`pub mod hosting;` dentro de `front_of_house.rs`|`src/front_of_house/hosting.rs`|

Luego todo lo demas, las rutas *crate::front_of_house::hosting*, los *use* y los *pub* queda **exactamente igual**. Solo cambiamos donde vive el codigo fisicamente con todo lo que vimos