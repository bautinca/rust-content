## Variables y Mutabilidad
Por defecto las variables son **INMUTABLES**, sin embargo las podemos hacer **MUTABLES**

Pero ¿Porque Rust prefiere que las variables por defecto sean inmutables? Cuando una variable es inmutable una vez que un valor esta vinculado a un nombre, no puede cambiar ese valor. Por ejemplo:
```rust
fn main(){
  let x = 5;
  println!("The value of x is: {x}");
  let x = 6; // ESTO ESTA MAL Y TIRARA ERROR DE COMPILACION
  println!("The value of x is: {x}");
}
```

Es importante obtener errores en **tiempos de compilacion** cuando intentamos cambiar un valor que esta designado como INMUTABLE ya que esta situacion puede conducir a errores.

El compilador de Rust (Cargo) esta hecho de tal forma que cuando uno firma que un valor no cambiara, realmente no cambiara por lo que no tenemos que rastrearlo nosotros mismos, por lo tanto el codigo es mas facil de razonar.

Aun asi las variables las podemos hacer mutables agregando el **mut** al definir una variable.
```rust
fn main(){
  let mut x = 5; // Hacemos la variable x mutable
  println!("The value of x is: {x}");
  x = 6; // Ahora si esta bien ya que la variable x puede cambiar su valor.
  println!("The value of x is: {x}");
}
```

## Constantes
Al igual que las variables inmutables las constantes son valores que **estan vinculados a un nombre y no se les permite cambiar** pero ¿Que diferencia hay entre variables inmutables y constantes?

Primero no se puede usar mut con constantes. Las constantes **SIEMPRE SON INMUTABLES** y es imposible hacerlas mutables a diferencia de las variables. 

Para declarar constantes tenemos que usar la palabra clave **const** en lugar de *let* que es para variables y el tipo de valor debe ser anotado (tenemos que definir el tipo de valor si o si)

Las constantes se pueden declarar ambito, es decir, en cualquier scope por ejemplo el scope global.

La ultima diferencia con las variables es que las constantes solo se pueden establecer en una expresion, no en el resultado de un valor que solo se podria calcular en tiempo de ejecucion, es decir, al momento que declaramos la constante ahi mismo en esa linea la tenemos que definir si o si, por ejemplo:
```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

**SI SO SI SE USAN MAYUSCULAS CON GUIONES BAJOS** para las constantes. Y lo bueno de definir esta constante es que se resuelve la operacion en TIEMPO DE COMPILACION!. El compilador de Rust es capaz de evaluar un conjunto limitado de operaciones en tiempo de compilacion

Las constantes son validas durante todo el tiempo que se ejecuta un programa, dentro del ambito en que se declararon. Esta propiedad hace que las constantes sean utiles para los valores en el dominio de la aplicacion que varias partes del programa necesitan conocer como el numero max. de jugadores, o la vel. de la luz.

## Shadowing
Podemos declarar una nueva variable con el mismo nombre que una variable anterior. Basicamente la primer variable es 'ocultada' por la segunda lo que significa que la segunda variable es lo que el compilador vera cuando use el nombre de la variable. En efecto la segunda variable oculta la primera tomando cualquier uso del nombre de la variable para si misma hasta que se haga **shadowing** sobre lam isma variable.

Podemos ocultar una variable asi:
```rust
fn main(){
  let x = 5;

  let x = x + 1; // Esta variable oculta la primera

  {
    let x = x*2;
    // Dentro de este scope se imprime el valor de la variable x dentro del scope, osea 6*2=12, se imprime 12
    println!("The value of x in the inner scope is: {x}");
  }

  // Aca se imprime 6 ya que la segunda definicion de x en el scope generla tapa la primera
  println!("The value of x is: {x}");
}
```

Este seria el Shadowing para variables inmutables, es diferente de marcar una variable como mut porque obtendremos un error de tiempo de compilacion si accidentalmente intentamos volver a asignar esta variable sin usaar la palabra clave let. Al usar let, podemos realizar algunas transformaciones en un valor, pero la variable debe ser **INMUTABLE** despues de que se haya completado esas transformaciones.

La otra diferencia entre mut y el shadowing es que, debido a que efectivamente estamos creando una nueva variable cuando usamos la palabra clave let nuevamente, podemos cambiar el tipo de valor pero reutilizar el mismo nombre. Por ejemplo podemos tener la variable llamada 'spaces' pero que es una string y la variable llamada 'spaces' pero que es un entero:

```rust
// La primera variables es de tipo string
let spaces = "    "; 
// La segunda es de tipo numerico
let spaces = spaces.len();
```

El shadowing nos ahorra tener que pensar en nombres diferentes como spaces_str o spaces_num, en su lugar podemos **reutiliar nombres** como 'spaces'. Es decir, en memoria se almacenan como 2 variables distintas.

Pero ojo al usar mut! Ya que recordemos que en el mut es para hacer las variables mutables por lo tanto si hacemos algo como lo anterior pero con mut entonces estariamos modificando el valor de una misma variable en memoria!! Por ejemplo:

```rust
let mut spaces = "   ";
spaces = spaces.len(); // Esto tira error en tiempo de compilacion ya que cambiamos el tipo de dato de sting a num por mas que sea mutable la variable 'spaces'
```

Nos tira error porque estamos mutando de tipos de variables sin hacer un parseo, es decir, no es solo que cambiamos el valor sino que ademas modificamos el tipo de valor de string a num y eso no se puede para variables mutables.

En resumen el **shadowing** en Rust es una tecnica que permite declarar una nueva variable con el mismo nombre que una existente en el mismo scope, ocultando la variable original. A diferencia de la mutabilidad (mut) el shadowing crea un **nuevo enlace** en memoria y permite cambiar el tipo de dato de la variable, algo que no es posible con mut