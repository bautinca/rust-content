Ya vimos que la funcion **main** es el punto de entrada a la aplicacion y es la funcion mas importante de todas. 

Por convencion el nombre de todas las funciones debe ser **minuscula y guiones bajos** por ejemplo:

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

Podemos llamar a cualquier funcion que hayamos definido en cualquier parte del codigo. En este ultimo caso definimos la funcion *another_function()* despues de la funcion *main()*, sin embargo tambien podriamos haberla definido antes. A Rust no le importa donde definimos las funciones, solo que esten definidas en algun lugar en un ambito que pueda ser visto por el invocador.

Las funciones reciben parametros donde podemos definir el tipo de dato de esos parametros que recibe:

```rust
fn main(){
  another_function(5);
}

fn another_function(x: i32){ // Decimos que la funcion reciba si o si tipos de datos i32 como argumento/parametro
  println!("The value of x is: {x}");
}
```

Es importante que en las firmas de cada funcion si o si **tenemos que declarar el tipo de dato de el/los parametros**. Esto es una muy buena costumbre y que tambien ayuda al compilador a detectar errores rapidamente ya que sabe ahora que tipo de dato espera la funcion

Una funcion puede recibir multiples argumentos/parametros como:

```rust
fn main(){
  print_labeled_measurement(5, 'h');
}

// Especificamos el tipo de dato de cada argumento/parametro que recibe la funcion
fn print_labeled_measurement(value: i32, unit_label: char){
  println!("The measurement is: {value}{unit_label}");
}
```

## Sentencias y Expresiones
Las funciones estan compuestas por una serie de sentencias y al finalizar termina en una expresion:

- funcion 1:
  - sentencia 1.1
  - sentencia 1.2
  - sentencia 1.3
  - etc...
  - expresion

Hasta ahora todas las funciones que vimos tienen sentencias pero no una expresion final. 

- **Sentencias**: Son instrucciones que realizan alguna accion y no devuelven un valor

- **Expresiones**: Evaluan a un valor resultante

```rust
fn main(){
  let y = 6; // Sentencia
  // Esta funcion en verdad no devuelve nada, no tiene expresion
}
```

Por ejemplo si vemos la sentencia *let y = 6;* esto no devuelve ningun valor cosa que es muy distinto a lenguajes como C o Ruby donde la asignacion si devuelve valor que es el de la asignacion, en esos lenguajes podriamos escribir algo como x = y = 6 ya que el y = 6 devuelve 6 por lo tanto x = 6. Pero ASI NO ES COMO FUNCIONA RUST

Las **expresiones** como 5+4 evaluan a un valor y componen la mayor parte del resto del codigo que escribiremos en RUst. Una simple operacion matematica como 5+4 es una expresion que evalua el valor 9, las expresiones siempre evaluan a un valor. Las expresiones pueden ser parte de las sentencias, como vemos *let y = 6* es una expresion que evalua al valor 6. Llamar a una funcion es una expresion ya que evalua a determinado valor, llamar a una macro tambien es una expresion. Un nuevo bloque de ambito creado (scope) con llaves es tambien una expresion, por ejemplo:

```rust
fn main() {
    let y = { // Este scope es una expresion ya que devuelve el valor 4, por lo tanto quedaria en y=4
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

La expresion:

```rust
{
    let x = 3;
    x + 1
}
```

Esto ultimo es una expresion ya que devuelve el valor de 4. Ese valor se enlaza a *y* como parte de la sentencia let. Algo importante a destacar es que el *x+1* no tiene ; final, esto es porque si le agregamos un ; entonces convertimos la expresion *x+1* en una sentencia, por lo que no devolveria ningun valor

## Funciones con valores de retorno
Relacionandolo con esto ultimo las funciones pueden devolver valores al codigo que las llama. El valor de retorno que devuelve una funcion debemos declar el tipo de dato que devuelve asi:

```rust
fn five() -> i32 { // En este caso la funcion retorna un tipo de dato i32
  5 // Esto es una expresion que es lo que retorna la funcion
}

fn main() {
  let x = five(); // Todo esto es una sentencia compuesta por la expresion five(), por lo que no devuelve valor por si solo, por eso termina en ;

  println!("The value of x is: {x}"); // Esto tambien seria una sentencia por eso termina en ;
}
```

En Rust el valor de retorno de la funcion es sinonimo del **valor de la ultima expresion en el bloque del scope de la funcion**. Podemos hacer que la funcion retorne un valor antes de que finalice en la ultima expresion utilizando la palabra clave **return** y especificando el valor, pero la mayoria de las funcioones devuelven la ultima expresion implicitamente

Veamos otro ejemplo:

```rust
fn main(){
  let x = plus_one(5); // Esto es una sentencia compuesta por la expresion plus_one(5)
  println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
  x+1 // Esto es una expresion que es lo que retorna la funcion plus_one
}
```

Si colocasemos al final del *x+1* un ; entonces daria error en tiempo de compilacion. La definicion de la funcion *plus_one* dice que devolvera un i32 pero las sentencias no evaluan un valor. Por lo tanto no se devuelve nada en la funcion *plus_one*, lo que contradice con la definicion de la funcion y da como resultado error.