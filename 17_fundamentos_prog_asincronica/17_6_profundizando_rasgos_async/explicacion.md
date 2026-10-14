## Async/Await por dentro
A continuacion vamos a ver como es internamente que trabaja el `async/await`.

Cuando usamos `async/await` en Rust lo que hace internamente es convertirlo en llamadas a un trait llamado **Future**. La definición de este trait es la siguiente:

```rust
pub trait Future {
  type Output;
  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

Lo interesante de este trait (recordemos que un trait es como una interfaz en otros lenguajes) es que tiene el unico metodo `poll` que devuelve un enum llamado `Poll<T>`.

Este `Poll<T>` es un enum que tiene 2 variantes:

```rust
enum Poll<T> {
  Ready(T), // El future termino, aca esta el resultado
  Pending, // Todavia no esta listo el future, vuelve a preguntar despues
}
```

Es muy parecido a `Option` pero con una diferencia muy importante: EL `Pending` no significa *"no hay nada"*, significa *"Actualmente no hay nada, pero algo habra"*. El runtime de Rust sabe que luego tendra que volver a llamar a `poll` para ver si ya esta listo el resultado.

Cuando el runtime de Rust ve que el `poll` devuelve un `Pending` entonces el runtime aprovecha para hacer otras cosas mientras espera que el future termine.

Esto es basicamente lo que hace el runtime de Rust, basicamente periodicamente hace `poll` al Future para ver si ya esta listo el resultado (devuelve `Ready`), si no lo esta entonces hace otras cosas mientras espera que el future termine.

Entonces cuando hacemos un `.await`:

```rust
let resultado = mi_future.await;
```

El compilador de Rust lo expande a:

```rust
loop {
  match mi_future.poll() {
    Poll::Ready(valor) => break valor, // Si el Future termino, rompe el loop y devuelve el valor
    Poll::Pending => { /*Cede el control al runtime, vuelve luego */ }
  }
}
```

Ojo, este loop **NO ES BLOQUEANTE**, cada vez que el Future le dice al runtime que esta en `Pending`, el runtime puede ir a ejecutar otros futures que esten listos para ejecutarse.

## Que es Pin
Imaginemos que tenemos una cadena donde cada eslabon tiene grabada la direccion exacta del eslabon que tiene al lado. Si moves la cadena de lugar, las direcciones grabadas quedan apuntando a donde estaba antes, ahora son incorrectas.

Esto es lo que pasa exactamente con los Futures. El compilador al ver Futures los convierte en una **maquina de estados** (un Struct al fin de cuentas) donde este struct puede tener campos que apuntan entre si, osea **auto-referencias**. Si ese struct se mueve en memoria entonces las referencias internas que tiene ese struct quedan invalidadas ya que apuntan a direcciones de memoria invalidas porque apuntan a la direccion vieja del struct.

Es decir, graficamente seria:

```
fut1 (en memoria)
┌─────────────────┐
│ campo_a: 0      │
│ campo_b: 1      │◄──┐
│ ref_a: ──────────────┘  ← apunta a campo_b dentro del mismo struct
└─────────────────┘
```

Si movemos el Future `fut1` a otra direccion de memoria el campo `ref_a` (que es una referencia) quedaria apuntando a la direccion vieja de `campo_b` que ya no es valida.

La solucion a esto? Al Future **PINNEARLO** en memoria. Es decir, `Pin<T>` es un wrapper que le garantiza al compilador *"lo que hay dentro no va a cambiar de direccion de memoria"*. Una vez pinneado el Future este puede tener auto-referencias de forma segura, por ejemplo:

```rust
let fut = mi_future;

let fut = Box::pin(fut); // Pinneamos el Future en memoria, ahora no se puede mover en el heap
let fut = pin!(fut); // Otra forma de pinnear un Future en memoria pero en el stack
```

Por otor lado esta el concepto de **Unpin**, la mayoria de los tipos de Rust no tienen auto-referencias, son perfectamente seguros de mover. Para estos tipos existe el trait `Unpin` que es basicamente una declaracion que dice *"este tipo puede moverse libremente, no necesita garantias de pin"*.

El compilador implementa `Unpin` automaticamente para casi todos los tipos. Los unicos a los que no les implementa son los **Futures** que el compilador genera de **bloques Async** (ya que estos bloques Async pueden tener auto-referencias).

Entonces ¿Cuando necesitamos pinnear un Future?

- Cuando trabajamos obviamente con Futures en colecciones (por ejemplo en un Vector y al hacer `join_all`).

- Cuando el compilador nos da el error *"cannot be unpinned"*

Cuando no necesitamos pensar en Pin?

- En el 99% de codigo async cotidiano
- Cuando usamos `.await` en un Future directamente
- Cuando usamos `join!`, `race`, `spawn_task`

Porque el `join_all` necesita el Pin? Porque si recordamos cuando ponemos Futures en un Vector, el Vector puede crecer y reubicar sus elementos en memoria, entonces si tenemos un Future que tiene auto-referencias y lo ponemos en un Vector, si el Vector se reubica entonces las referencias internas del Future quedan invalidas. Por eso necesitamos pinnear los Futures antes de ponerlos en un Vector.

## Trait Stream por dentro
Como dijimos el Stream es la combinacion de **Future + Iterador**. El trait Stream esta implementado asi:

```rust
trait Stream {
  type Item;
  fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>; // Es muy parecido al trait Future, pero en vez de devolver un valor devuelve un Option<T> porque puede devolver multiples valores a lo largo del tiempo
}
```

Si nos fijamos las diferencias con el trait Future son:

- El metodo es `poll_next` en vez de `poll`

- Si nos fijamos devuelve un `Poll::Ready(Some(valor))` cuando llego un elemento

- Devuelve `Poll::Ready(None)` cuando el stream termino y no va a llegar mas elementos

- Devuelve `Poll::Pending` cuando todavia no hay elementos disponibles, pero pueden llegar mas adelante

Si nos fijamos es la combinacion del trait `Iterator` donde el metodo es `next` y devuelve un `Option<T>` y el trait `Future` donde el metodo es `poll` y devuelve un `Poll<T>`. Por eso decimos que el Stream es la combinacion de Future + Iterador.

Repasando veamos el trait de `Iterator` y `Future`:

```rust
trait Iterator {
  type Item;
  fn next(&mut self) -> Option<Self::Item>;
}

trait Future {
  type Output;
  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

trait Stream {
  type Item;
  fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}
```