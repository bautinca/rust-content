Antes de entrar en tema hay un detalle importante, y es que existen **2 tipos de argumentos para cargo test**: los argumentos de **control** y los argumentos de **test**.

Los **argumentos de control** son aquellos que se utilizan para configurar el entorno de ejecución del test, como por ejemplo la cantidad de hilos, el tiempo de espera, entre otros. Por otro lado, los **argumentos de test** son aquellos que se utilizan para definir las condiciones específicas del test que se va a ejecutar, como los parámetros de entrada, los valores esperados, etc.

Por ejemplo en la bash para definir los argumentos de control y los argumentos de test se hace de la siguiente manera:

```bash
cargo test [flags de control] -- [flags de test]
```

Todo lo que va antes del **--** son los argumentos de control y todo lo que va despues del **--** son los argumentos de test.

Esto es importante saberlo para explicar lo siguiente.

Por defecto todos los tests se ejecutan **en paralelo**, osea en hilos separados de forma paralela. Esto es rapidisimo pero introduce un problema: Si 2 tests comparten estado (un archivo en disco, una variable de entorno, una base de datos) entonces puede producirse el famoso **race condition** y los tests fallan. La solucion logicamente seria correr todos los tests de forma secuencial en un unico hilo y esto se hace con el argumento de control `--test-threads=1`. Esto hace que todos los tests se ejecuten en un unico hilo y por lo tanto no hay posibilidad de que se produzca un **race condition**.

```bash
cargo test --test-threads=1
```

Logicamente al ser un unico hilo en que se ejecuten los tests va a tardar mas, pero lo bueno es que los tests no se pisan entre sí. Esto es por ejemplo lo que usariamos si nuestros tests leen y escriben el mismo archivo.

Otra cuestion importante sobre los tests es que **cualquier println! dentro de un test no se va a ver en la consola**. En la consola unicamente aparecera el output de los tests que fallen. Esto es porque cargo test por defecto **captura el output de los tests** y solo lo muestra si el test falla. Esto es util para no llenar la consola de output innecesario Por ejemplo:

```rust
fn prints_and_returns_10(a: i32) -> i32 {
  println!("I got the value {a}");
  10
}

#[test]
fn this_test_will_pass() {
  let value = prints_and_returns_10(4);
  assert_eq!(10, value);
  // El "I got the value 4" no se va a ver en la consola porque el test pasa
}

#[test]
fn this_test_will_fail() {
  let value = prints_and_returns_10(8);
  assert_eq!(5, value);
  // El "I got the value 8" si se va a ver en la consola porque el test falla
}
```

Aun asi podemos tambien ver el output de los tests que pasan si usamos el argumento de control `--show-output`.

Una pregunta es ¿Como corremos un test especifico? Bueno le tenemos que pasar el nombre exacto del test como argumento de test. Por ejemplo si tenemos un test llamado `this_test_will_fail` podemos correrlo asi:

```bash
cargo test this_test_will_fail
```

Solo corre el test que se llame exactamente *this_test_will_fail*

Otra cuestion es **¿Como ignorar tests?** Osea supongamos que tenemos cientos de tests pero hay uno particular que es ultra pesado ¿Como lo ignoramos y hacemos que corran todos los tests salvo ese? Bueno para eso tenemos el atributo `#[ignore]` que se pone arriba del test que queremos ignorar. Por ejemplo:

```rust
#[test]
fn it_works() {
  assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
  // Test que tarda 1 hora en correr
}
```

Entonces si en la bash hacemos:

```bash
cargo test # Corre todos los tests menos el expensive_test
cargo test -- --ignored # Corre solo el expensive_test
cargo test -- --include-ignored # Corre todos los tests incluyendo el expensive_test
```

## Tests unitarios vs Tests de Integracion
Rust formalmente distingue 2 tipos de tests:

- **Tests Unitarios**: Son aquellos que se escriben dentro del mismo archivo que el código que estan testeando. Por ejemplo si tenemos un archivo `src/lib.rs` podemos escribir tests unitarios dentro de ese mismo archivo. Los tests unitarios tienen acceso a los elementos privados del modulo donde se encuentran. El proposito de estos es testear funciones individuales y asegurarse de que funcionan correctamente de manera aislada

- **Tests de Integracion**: Son aquellos que se escriben en archivos separados dentro de la carpeta `tests/`. Por ejemplo si tenemos un archivo `tests/integration_test.rs` podemos escribir tests de integracion dentro de ese archivo. Los tests de integracion no tienen acceso a los elementos privados del modulo donde se encuentran, solo pueden acceder a los elementos publicos. El proposito de estos es probar la integracion de varios modulos y asegurarse de que funcionan correctamente juntos.

En los **tests unitarios** hay una convencion de que se escriben dentro de un modulo llamado `tests` al final de cada archivo con el codigo por ejemplo:

```rust
fn internal_adder(left: u64, right: u64) -> u64 {
  left + right // Funcion privada (sin pub)
}

// ----------- Tests unitarios ----------

#[cfg(test)] // Se lo tenemos que colocar arriba de este modulo para indicar que el modulo este se tenga en cuenta cuando corremos 'cargo test'
mod tests {
  // Dentro de este modulo escribimos los tests unitarios que testean unicamente cuestiones del archivo mismo
  use super::*; // Importamos todo lo del modulo padre (el archivo mismo) para poder testearlo

  // Primer test unitario
  #[test]
  fn test_interno() {
    // Podemos testear las funciones privadas del modulo
    assert_eq!(4, internal_adder(2, 2));
  }
}
```

El `use super::*;`es necesario. Como el modulo *tests* es un modulo hijo del archivo donde esta definido, para poder acceder a las funciones privadas del modulo padre necesitamos hacer `use super::*;` para importar todo lo del modulo padre.

Por el lado de los **Tests de integracion** la convencion es que se escriben en archivos separados dentro de la carpeta `tests/` y no tienen acceso a las funciones privadas del modulo. La estructura del proyecto seria:

```bash
adder/
├── Cargo.toml
├── src/
│   └── lib.rs # crate de libreria
└── tests/
    └── integration_test.rs # Los tests de integracion
```

Cada archivo dentro de la carpeta `tests/` es un crate de libreria independiente y por lo tanto no tiene acceso a las funciones privadas del modulo.

```rust
// archivo tests/integration_test.rs
use adder::add_two; // Importamos la funcion publica del crate de libreria

// No necesitamos usar #[cfg(test)] porque este archivo solo se compila cuando corremos test.
#[test]
fn it_adds_two() {
  let result = add_two(2);
  assert_eq!(4, result);
}
```

Si queremos correr los tests de integracion podemos hacerlo asi:

```bash
cargo test --test integration_test # Corre unicamente los tests del archivo integration_test.rs
```

## Codigo compartido entre tests de integracion
Si tenemos varios tests de integracion que comparten codigo, podemos crear un modulo llamado `common` dentro de la carpeta `tests/` y poner ahi el codigo compartido, pero ojo, no se crea un archivo `common.rs` sino que se crea una carpeta `common/` con un archivo `mod.rs` adentro. La estructura seria:

```bash
tests/
├── common/
│   └── mod.rs      # acá va el código compartido
└── integration_test.rs
```

Entonces dentro del archivo helper `tests/common/mod.rs` podemos poner funciones, structs, etc que queramos compartir entre los tests de integracion. Por ejemplo:

```rust
// archivo tests/common/mod.rs
pub fn setup() {
  // Configuracion comun para los tests de integracion
}
```

Y dentro de un test de integracion podemos usarlo asi:

```rust
// archivo tests/integration_test.rs
mod common; // Importamos el modulo common
use adder::add_two;

#[test]
fn it_adds_two() {
  common::setup(); // Llamamos a la funcion setup del modulo common
  let result = add_two(2);
  assert_eq!(4, result);
}
```

