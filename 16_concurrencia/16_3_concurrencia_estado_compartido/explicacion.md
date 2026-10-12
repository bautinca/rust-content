Hasta ahora vimos como crear hilos y como sincronizarlos a traves de canales, pero hasta ahora en ningun momento hemos hablado de **compartir memoria** entre hilos. En memoria compartida entre hilos basicamente hay muchos ownerships para un unico recurso compartido entre hilos y por lo tanto es necesario tener cuidado con el acceso a este recurso compartido para evitar condiciones de carrera y otros problemas de concurrencia.

Obviamente que tener recursos compartidos entre hilos agrega complejidad a la concurrencia pero por suerte Rust nos ofrece varias herramientas para lidiar con el estado compartido.

# Mutex
Un **Mutex** (exclusion mutua) garantiza que **un solo hilo a la vez** pueda acceder al recurso compartido que protege. El mecanismo es un lock:

1. Antes de usar el dato -> Tomamos el lock del mutex

2. Cuando terminamos de usaar el dato -> Liberamos el lock del mutex

Lo bueno que tiene Rust es que en C tambien tenemos el mecanismo de Mutex pero en C es muy facil olvidarse de liberar el lock y eso puede generar problemas de concurrencia. En Rust, el lock se libera automaticamente cuando la variable que contiene el lock sale del scope. Es decir, no hace falta hacer explicitamente un *unlock()*.

Veamos como usar un Mutex con un solo hilo:

```rust
use std::sync::Mutex; // De la libreria estandar importamos el Mutex

fn main() {
  let m = Mutex::new(5); // El dato que queremos proteger con el Mutex es el 5. Osea la variable 'm' sera un recurso compartido entre hilos y el Mutex nos ayudara a protegerlo de accesos concurrentes.

  {
    let mut num = m.lock().unwrap(); // El hilo principal toma el lock del mutex y obtiene acceso al recurso compartido. El lock() devuelve un Result, por lo que usamos unwrap() para obtener el valor o hacer panic si hubo un error.
    *num = 6; // Hacemos que el recurso compartido sea 6.
  } // El lock se libera automaticamente cuando 'num' sale del scope

  println!("m = {:?}", m); // Imprimimos el valor del recurso compartido protegido por el Mutex. Deberia imprimir "m = Mutex { data: 6 }" ya que el recurso compartido fue modificado a 6.
}
```

Cuestiones importantes de este ultimo codigo:

- El `m.lock()` bloquea el hilo actual hasta que pueda obtener el lock del mutex. En este caso unicamente hay un hilo, por lo que el lock se obtiene inmediatamente. Pero en un escenario con multiples hilos, si otro hilo ya tiene el lock, el hilo actual se bloquea hasta que el lock sea liberado.

- Al adquirir el lock devuelve un **MutexGuard**, que es un tipo especial de smart pointer que implementa el trait `Deref` (como todo smart pointer) y permite acceder al recurso compartido a traves de la referencia que devuelve. Cuando el MutexGuard sale del scope, el lock se libera automaticamente llamando al metodo `drop()` del MutexGuard.

- El tipo de la variable *m* es `Mutex<i32>` pero no `i32`. Esto es porque el Mutex es un contenedor que protege el recurso compartido. Entonces el sistema nos obliga a llamar a **lock()** antes de poder acceder al *i32* interno. No podemos acceder a el de otra forma

Veamos el siguiente ejemplo de uso de un Mutex:

```rust
let counter = Mutex::new(0); // Creamos un contador protegido por un Mutex

for _ in 0..10 { // Creamos 10 hilos
  thread::spawn(move || {
    let mut num = counter.lock().unwrap(); // Cada hilo toma el lock del Mutex y obtiene acceso al recurso compartido
    *num += 1; // Incrementamos el contador
  });
}
```

Este ultimo codigo **NO COMPILA** y es porque la variable *counter* no es compartida entre hilos. En Rust, por defecto, las variables no son compartidas entre hilos y para poder compartirlas necesitamos usar un tipo especial de smart pointer llamado **Arc** (Atomic Reference Counted). El Arc permite que multiples hilos tengan ownership del mismo recurso compartido y ademas es seguro para concurrencia.

Basicamente el smart pointer Arc es como **Rc** (que ya lo vimos) pero con **operaciones atomicas** para poder ser usado en un contexto de concurrencia. Esto significa que el Arc puede ser compartido entre hilos y cada hilo puede tener su propia referencia al recurso compartido.

```rust
use std::sync::{Arc, Mutex}; // Importamos Arc y Mutex de la libreria estandar
use std::thread;

fn main() {
  let counter = Arc::new(Mutex::new(0)); // Creamos un contador protegido por un Mutex y lo envolvemos en un Arc para poder compartirlo entre hilos
  
  let mut handles = vec![]; // Creamos un vector para guardar los resultados de los hilos hijos que crearemos

  for _ in 0..10 {
    let counter = Arc::clone(&counter); // Clonamos el Arc para que cada hilo tenga su propia referencia al recurso compartido. El recurso compartido esta almacenado en el heap y cada smart pointer Arc se encuentra en el stack haciendo referencia al mismo recurso del heap obviamente.

    // Creamos hilo hijo en cada iteracion
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap(); // Cada hilo toma el lock del Mutex y obtiene acceso al recurso compartido
      *num += 1; // Incrementamos el contador
    }); // El hilo hijo sale del scope y el lock se libera automaticamente

    handles.push(handle); // Guardamos el handle del hilo hijo en el vector de handles
  }

  // Hacemos que el hilo principal espere a que todos los hilos hijos terminen su ejecucion
  for handle in handles {
    handle.join().unwrap(); // Esperamos a que el hilo hijo termine su ejecucion y obtenemos el resultado. Si hubo un error, hacemos panic.
  }

  // Finalmente al finalizar la ejecucion de todos los hilos hijos hacemos que el hilo principal imprima el contador, logicamente deberia estar en 10 ya que cada hilo hijo incremento el contador en 1 y en ningun momenti hubo race condition ya que el Mutex nos protege de eso.
  println!("Result: {}", *counter.lock().unwrap());
}
```

Basicamente cada hilo hijo generado toma su clon de Arc, luego cadad hilo hijo llama al lock() (si el lock esta libre lo toma, sino se queda en estado bloqueante). Luego cuando un hilo hijo toma el lock incrementa el contador +1 y luego el lock se libera automaticamente cuando el *num* sale del scope y es asi como otro hilo en estado bloqueante puede tomar el lock y hacer lo mismo.

`counter` en el ejemplo es inmutable externamente (`let counter`, no `let mut counter`), pero `Mutex<T>` provee mutabilidad interior — igual que `RefCell<T>`. La diferencia es que `Mutex<T>` es thread-safe y `RefCell<T>` no.

## El riesgo que no puede evitar Rust: Deadlocks
Si recordamos Rc venia con el riesgo de referencias circulares y era algo que Rust no podia evitar. Del lado de los Mutex tenemos un riesgo que Rust no puede evitar y son los **DEADLOCKS**

Un deadlock ocurre cuando:

- Un hilo A tiene el lock de *mutex1* y espera al lock de *mutex2*

- El hilo B tiene el lock de *mutex2* y espera el lock de *mutex1*

- Ambos hilos esperan que el otro libere el lock y ninguno de los dos puede continuar, por lo que se produce un deadlock. El sistema completo se paraliza.

Rust no puede detectar esto en tiempo de compilacion. Es algo de lo que nos tendremos que encargar nosotros como programadores tratar de evitarlo con buenas costumbres de diseño. Hay estrategias para evitar deadlocks como siempre adquirir los locks en el mismo orden en todos los hilos, o usar timeouts, etc.