Un test es una funcion comun y corriente en Rust pero que tiene el atributo **#[test]** encima. Cuando ejecutamos *cargo test* Rust compila un binario especial que corre todas las funciones marcadas con ese atributo y reporta cuales pasaron y cuales faltaron

Un test falla cuando hace **panic!**, un test pasa cuando termina SIN PANICS

Cuando creamos un proyecto de 0 con `cargo new mi_proyecto --lib` Cargo nos genera automaticamente un archivo llamado **lib.rs** dentro de la carpeta **src** ya que le estamos indicando a Cargo que queremos crear un crate de tipo libreria:

```rust
// Genera por defecto esto dentro de src/lib.rs

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

```

El **#[cfg(test)]** le dice al compilador que ese modulo solo existe cuando corremos **cargo test**, no se incluye en el binario de produccion.

Rust provee 3 macros para hacer tests:

- **assert!(condicion)**: Falla si la condicion es falsa, por ejemplo:

  ```rust
  assert!(larger.can_hold(&smaller));
  assert!(!smaller.can_hold(&larger)); // Lo negamos con '!'
  ```

- **assert_eq!(izq, der) y assert_ne!(izq, der)**: Falla si los valores no son iguales o si son iguales respectivamente. Por ejemplo:

  ```rust
  assert_eq!(result, 4); // Si falla nos dice left: 3, right: 4
  assert_ne!(result, 5); // Si falla nos dice que los valores son iguales cuando no deberian serlo
  ```

  Ojo, para usar **assert_eq!** y **assert_ne!** con nuestros propios tipos de datos el tipo necesita **implementar los traits PartialEq y Debug**. La forma mas simple de hacer que nuestro tipo implemente esos traits es usando el atributo **#[derive(PartialEq, Debug)]** en la definicion de la struct o enum. Por ejemplo:
  
  ```rust
  #[derive(PartialEq, Debug)]
  struct MiStruct { ... }
  ```

Listo ya vimos los 3 macros, otra cuestion es que tambien podemos crear **mensajes de error personalizados** en caso de que falle el test para ello podemos pasar un segundo argumento a los macros, por ejemplo:

```rust
// Si el assert falla nos va a mostrar el mensaje "El resultado no es el esperado"
assert_eq!(result, 4, "El resultado no es el esperado");
```

Esto es muy util cuando tenemos muchos tests y necesitamos entender rapido que fallo.

Hay algo muy util que es el **#[should_panic]** que basicamente nos permite **testear algo que deberia fallar**. A veces queremos verificar que nuestro codigo falle correctamente ante entradas invalidas. Para eso usamos ese atributo, por ejemplo:

```rust
#[test]
#[should_panic]
fn greater_than_100() {
    Guess::new(200); // Esto debe paniquear. Si hace panic! Entonces el test pasa, si no hace panic! el test falla
}
```

El problema de usar el **#[should_panic]** es que es **impreciso** ya que pasa con cualquier panic!, incluso uno inesperado. Para ser mas especificos podemos exigir que el mensaje panic! contenga cierto texto usando el atributo **expected**:

```rust
#[test]
#[should_panic(expected = "El valor debe estar entre 1 y 100")]
fn greater_than_100() {
    Guess::new(200); 
}
```

Ahora el test solo pasa si el panic ocurre y el mensaje contiene ese substring. Si el codigo paniquea por otra razon entonces el test falla y nos lo dice.

Tambien podemos hacer tests con **Result\<T, E>**, en lugar de panics podemos escribir tests que devuelvan Result. Veamos un ejemplo:

```rust
#[test]
fn it_works() -> Result<(), String> {
    let result = add(2, 2);
    if result == 4 {
        Ok(())
    } else {
        Err(format!("El resultado fue {}, pero se esperaba 4", result))
    }
}
```

Como vemos es un test que devuelve un Result, si devuelve Ok entonces el test pasa, si devuelve Err entonces el test falla y nos muestra el mensaje de error.

La ventaja es que podemos usar el operador **?** dentro del test para propagar errores, lo cual es muy util cuando tenemos funciones que devuelven Result y queremos testearlas. La limitacion es que no podemos usar los macros **assert!**, **assert_eq!** y **assert_ne!** dentro de un test que devuelve Result, ya que esos macros hacen panic! y no devuelven Result. En su lugar debemos usar condicionales y devolver Err con un mensaje de error personalizado como hicimos antes.

La gran duda es una vez escritos los tests **¿Como los ejecutamos?** Para eso usamos **cargo test** simplemente. Correran todos los tests y la salida tipica sera algo asi:

```
running 2 tests
test tests::larger_can_hold_smaller ... ok
test tests::smaller_cannot_hold_larger ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Y si algo llega a fallar:

```
test tests::it_adds_two ... FAILED

failures:
---- tests::it_adds_two stdout ----
assertion `left == right` failed
  left: 5
  right: 4
```

