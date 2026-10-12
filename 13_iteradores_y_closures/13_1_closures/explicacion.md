El diseño de Rust se ha inspirado en muchos lenguajes ya preexistentes y una influencia significativa en la **programacion funcional**. La programacion en un estilo funcional se centra en el uso de **funciones** como bloques de construccion para la programacion. En este estilo, las funciones son tratadas como **ciudadanos de primera clase**, lo que significa que pueden ser asignadas a variables, pasadas como argumentos a otras funciones y devueltas como valores desde otras funciones

Veremos los **Closures** que son una construccion similar a una funcion que puede almacenarse en una variable o pasarse como argumento a otra funcion.

Si venimos de Python son basicamente las **lambda** que son closures simples. En C no existen en cambio (se simulan con punteros a funciones). Las closures de Rust son mas potentes que las lambda de Python y tienen costo 0 en runtime.

La sintaxis es la siguiente:

```rust
// Funcion normal (como lo venimos haciendo)
fn add_one_v1(x: i32) -> i32 {
    x + 1
}

// Closure equivalente (lo asignamos a variable)
let add_one_v2 = |x: i32| -> i32 { x + 1 };

// Closure sin asignar tipos (los deduce el compilador de Rust, habiamos dicho que cuando no le asignamos el tipo por ejemplo a un entero entonces por defecto es i32)
let add_one_v3 = |x| x + 1;

// Tambien podemos definir closures sin las llaves (cuando es solo 1 expresion)
let add_one_v4 = |x| x + 1;
```

De todas estas 4 maneras podemos definir el mismo closure que es *x+1*. Basicamente las `|`reemplazan a los `()`de los parametros. Las anotaciones de tipo son opcionales ya que el compilador tambien las infiere.

Luego cuando el compilador infiere el tipo de dato ya le **queda fijo** por ejemplo:

```rust
let ejemplo = |x| x; // A esta closure no le definimos los tipos

let s = ejemplo(String::from("Hola")); // Aqui el compilador infiere que x es un String por lo tanto queda fijo que x siempre es de tipo String

let n = ejemplo(5); // Como x es de tipo String aca no compilaria y tiraria error
```

Lo bueno de un closure es que puede **capturar** variables del entorno donde fue definido, en cambio las funciones normales NO. Por ejemplo:

```rust
impl Inventory {
  fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
    user_preference.unwrap_or_else(|| self.most_stocker()) // La closure dentro de unwrap_or_else captura el self del entorno donde fue definido, el || captura el self y lo pasa a la funcion most_stocker
  }
}
```

Recordar que `unwrap_or_else` es un metodo de Option que recibe un closure como argumento y lo ejecuta solo si el Option es None. En este caso, la closure captura `self` del entorno donde fue definida, permitiendo acceder a los métodos y propiedades de la instancia de `Inventory`. Esto demuestra cómo los closures pueden interactuar con su contexto, algo que no es posible con funciones normales.

Al igual que las referencias, un closure puede capturar el entorno de **3 formas** y el compilador elige siempre la menos restrictiva:

1. **Referencia inmutable**: El closure puede leer variables del entorno pero **no modificarlas**. Esto es útil cuando solo necesitamos acceder a los valores sin alterarlos. Por ejemplo:

    ```rust
    fn main() {
      let list = vec![1, 2, 3];

      let only_borrows = || println!("Desde closure: {list:?}"); // El closure solo toma prestada la referencia inmutable de list. Es como si el closure recibiera un &list inmutable como argumento.

      println!("Antes de llamar: {list:?}"); // OK
      only_borrows(); // OK
      println!("Despues de llamar: {list:?}"); // OK
    }
    ```

    El closure solo necesita leer *list* asi que captura una **referencia inmutable**. Podemos seguir usando *list* normalmente en el scope.

2. **Referencia mutable**: En este caso el closure puede modificar las variables del entorno recibiendo una referencia mutable. Esto es útil cuando necesitamos cambiar el estado de las variables capturadas. Por ejemplo:

    ```rust
    fn main() {
      let mut list = vec![1, 2, 3];

      let mut borrows_mutably = || list.push(4); // El closure toma prestada la referencia mutable de list. Es como si el closure recibiera un &mut list como argumento, pero esto lo deduce el compilador sin que le indiquemos nada que explicitamente recibe una referencia mutable. En este caso al vector le agregamos el 4

      // ACA NO PODEMOS USAR LIST DIRECTAMENTE PORQUE EL CLOSURE TIENE UNA REFERENCIA MUTABLE SOBRE ELLA
      // println!("Antes de llamar: {list:?}"); // ERROR: no se puede usar list directamente

      borrows_mutably(); // OK
      println!("Despues de llamar: {list:?}"); // OK. Se imprime [1, 2, 3, 4] porque el closure modifico list agregando un nuevo elemento

      // Aca si podemos usar list directamente ya que el closure termino de modificarlo
    }
    ```

    En este caso el closure necesita modificar la variable *list* asi que el compilador deduce que debe capturar una **referencia mutable**. Entre la definicion y la llamada del closure no podemos usar *list* directamente porque el closure tiene una referencia mutable sobre ella.

3. **Tomando el ownership**: El closure puede tomar posesion de las variables del entorno, lo que significa que el closure se convierte en el propietario de esas variables. Esto es útil cuando queremos transferir la propiedad de los datos al closure, especialmente en contextos de concurrencia o cuando necesitamos mover datos a otro hilo. Por ejemplo:

    ```rust
    use std::thread; // De la libreria estandar de Rust importamos el modulo thread para poder crear hilos

    fn main() {
      let list = vec![1, 2, 3];

      thread::spawn(move || {println!("Desde el thread: {list:?}")}).join().unwrap(); // El closure toma posesion de list usando move. Esto es necesario porque el closure se ejecuta en otro hilo y necesita tener ownership de list para poder usarla dentro del thread. El metodo join espera a que el thread termine y devuelve un Result, por eso usamos unwrap para obtener el valor o panique si hubo error
    }
    ```

    El **move** es importante porque **fuerza al closure a tomar el ownership de las variables que captura** (en este caso *list* aunque solo necesite leerla). Es necesario aca porque el nuevo thread/hilo podria vivir mas que el thread principal. Si *list* fuera liberada desde el hilo principal antes de que el hilo secundario termine, entonces el hilo secundario estaria accediendo a memoria liberada, lo que causaria un error de memoria. Al usar *move*, el closure se asegura de que tiene su propia copia de *list* y no depende del hilo principal para acceder a ella.

## Los 3 traits Fn
Dependiendo de como el closure captura el entorno, implementa uno o mas de estos traits:

- **FnOnce**: Este trait se implementa siempre. En este caso se mueven los valores capturados por el closure hacia afuera. Por lo tanto como mueve los valores capturados hacia afuera solo puede ser llamado **una vez**. Por ejemplo:

    ```rust
    impl<T> Option<T> {
      pub fn unwrap_or_else<F>(self, f: F) -> T
      where
          F: FnOnce() -> T, // F es un closure que implementa el trait FnOnce, lo que significa que puede ser llamado al menos una vez.
      {
          match self {
              Some(x) => x,
              None => f(), // El closure f() se llama solo si self es None.
          }
      }
    }
    ```

- **FnMut**: Este trait se implementa si el closure muta los valores capturados pero no los mueve fuera, por lo tanto este closure puede ser llamado varias veces. Ejemplo:

    ```rust
    let mut list = [
      Rectangle { width: 10, height: 1 },
      Rectangle { width: 3, height: 5 },
      Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width); // Se llama varias veces el closure que implementa el trait FnMut, ya que sort_by_key necesita llamar al closure varias veces para ordenar la lista. Puede llamar a dicho closure varias veces porque no mueve los valores capturados hacia afuera, solo los muta.
    ```

    El closure anterior `|r| r.width` funcionaba porque no movia los valores capturados hacia afuera, solo los mutaba. Pero si por ejemplo a *sort_by_key* le pasamos un closure que mueve los valores capturados hacia afuera, entonces no podria ser llamado varias veces y tiraria error de compilacion, por ejemplo:

    ```rust
    let value = String::from("closure called");

    list.sort_by_key(|r| {
      sort_operations.push(value); // Movemos value hacia afuera del closure por lo tanto este mismo closure no puede ser llamado varias veces por sort_by_key, pero como sort_by_key necesita llamar al closure varias veces para ordenar la lista entonces tiraria error de compilacion.
      r.width
    })
    ```

    Osea *value* es un String y se mueve fuera del closure en la primer llamada y ya no estara disponible para las siguientes llamadas, por lo tanto el closure no puede ser llamado varias veces y sort_by_key no podria usarlo. En este caso el closure implementaria el trait FnOnce y no FnMut. Por lo tanto toda closure que implementa FnOnce es incompatible con sort_by_key

    La solucion correcta seria mutar un contador en lugar de mover un valor, osea:

    ```rust
    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
      num_sort_operations += 1; // Mutamos un contador en lugar de mover un valor hacia afuera del closure, por lo tanto este closure puede ser llamado varias veces por sort_by_key.
      r.width
    })

    // Por lo tanto este closure es FnMut y no FnOnce por lo que es compatible con la funcion sort_by_key que necesita llamar al closure varias veces para ordenar la lista.
    ```

- **Fn**: Este trait se implementa si el closure no muta ni mueve los valores capturados hacia afuera, por lo tanto puede ser llamado muchas veces e incluso en paralelo.

Entonces sabiendo estos diferentes traits y sus implicaciones, podemos entender mejor cómo funcionan los closures en Rust. Por ejemplo que:

- **Closures que solo leen**: Reciben referencia inmutable y son Fn + FnMut + FnOnce. Pueden ser llamados muchas veces y en paralelo.

- **Closures que mutan**: Reciben referencia mutable y son FnMut + FnOnce. Pueden ser llamados varias veces pero no en paralelo.

- **Closures que mueven**: Reciben ownership y son FnOnce. Solo pueden ser llamados una vez y no en paralelo.
