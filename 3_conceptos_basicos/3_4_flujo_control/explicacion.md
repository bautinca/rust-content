## Flujo de Control
Es la capacidad de ejecutar algun codigo dependiendo de si una condicion es true y ejecutar la condicion repetidamente mientras la condicion sea true.

Las construcciones mas comunes que le permiten controlaar el flujo de ejecucion del codigo Rust son las expresiones **if** y los **bucles**

### Expresiones IF
El IF nos permite dividir el codigo segun condiciones dadas, veamos un ejemplo:

```rust
fn main() {
  let number = 3;

  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }
}
```

Opcionalmente se incluye *else* si la condicion del if no se cumple entonces se ejecutaria el bloque de codigo del else.

Es importante mencionar que la condicion debe ser un tipo de dato **bool**. Si la condicion no es un booleano entonces obtendremos error, por ejemplo:

```rust
fn main() {
    let number = 3;

    if number { // Aca obtenemos error ya que 'number' no es un booleano
        println!("number was three");
    }
}
```

Esto ultimo es diferente a comparacion de lenguajes como Ruby o JavaScript. Rust no intentara convertir automaticamente los tipos no booleanos en un booleano. Debemos ser explicitos y **siempre proporcionar a if un booleano como su condicion**

Por otro lado podemos manejar multiples condiciones if-else asi:

```rust
fn main(){
  let number = 6;

  if number % 4 == 0{
    println!("number is divisible by 4");
  } else if number % 3 == 0{
    println!("number is divisible by 3");
  } else if number % 2 == 0{
    println!("number is divisible by 2");
  } else {
    println!("number is not divisible by 4, 3, or 2");
  }
}
```

Este programa cuando se ejecuta verifica cada condicion if en orden y ejecuta el primer cuerpo para el cual la condicion se evalua como true. Rust solo ejecuta el bloque para la primera condicion true y una vez encuentra una **ni siquiera verifica el resto**

Es posible que si tenemos muchos if-else debamos refactorizar el codigo por lo que en ese caso es mas conveniente usar la instruccion **match** que vimos anteriormente.

Ahora bien, el bloque if-else si nos fijamos es una expresion ya que devuelve un valor o sino el otro, por lo tanto podemos usar dicha expresion para definir variables, por ejemplo:

```rust
fn main() {
  let condition = true;
  let number = if condition {5} else {6}; // Si se cumple la condicion entonces tiene el valor de 5, sino tiene el valor de 6
}
```
Es importante mencionar que el resultado que retornan ambas ramas debe **ser el mismo**, es por ello que si se ejecuta el bloque if devuelve un i32 y si se ejecuta el bloque else devuelve tambien un i32, esto tiene que ser asi. Estaria mal si por ejemplo devolvemos tipos de datos distintos:

```rust
fn main() {
    let condition = true;

    // Esta mal porque el bloque if se evalua como un entero y la expresion del bloque else se evalua como una string. Esto no funciona porque las variables deben tener un solo tipo de dato asignado. Y Rust necesita saber en tiempo de compilacion que tipo de dato tiene la variable number
    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
```

### Bucles
Es util ejecutar un bloque de codigo mas de una vez, para ello tenemos **BUCLES**.

Rust tiene 3 tipos de bucles que son los siguientes:

#### Loop
El loop le dice a Rust que ejecute el codigo una y otra vez para siempre o hasta que indiquemos que explicitamente se detenga, por ejemplo:

```rust
fn main() {
  loop {
    println!("again!");
  }
}
```

El loop ejecuta lo que tiene dentro del scope indefinidamente hasta que lo detengamos con ctrl+c

Si queremos salir del bucle infinito usamos la palabra **break** para salir explicitamente del bucle en esa misma linea que colocamos el break.

Tambien esta la instruccion **continue** que seria decirle al bucle que omita todo lo que viene luego y ejecute una nueva iteracion

Uno de los usos que tiene loop es volver a intentar una operacion que sabe que puede fallar, como por ejemplo verificar si un hilo completo su trabajo. Tambien podemos asignar valores a variables con el uso del bucle:

```rust
fn main() {
  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10{
      break counter * 2;
    } // Esto termina sin ; ya que es una expresion para asignar valor a la variable result
  }; // Recordar que termina en ; ya que todo esto es una sentencia
}
```

Basicamente en el programa iteramos y en cada iteracion sumamos counter+1 hasta que llegue a 10 y luego el valor de result sera 10*2=20

Si tenemos bucles dentro de bucles (loops dentro de loops) entonces el break y continue se aplican al bucle mas interior en ese punto. Opcionalmente podemos especificar una **etiqueta de bucle** que se puede usar con break y continue para especificar que esas palabras clave se aplican al bucle etiquetado en lugar del bucle mas interior, por ejemplo veamos un ejemplo de 2 bucles anidados:

```rust
fn main() {
    let mut count = 0;
    // Primer bucle llamado 'counting_up'
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        // Segundo bucle anidado
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // Este break solo sale del bucle interno
                break;
            }
            if count == 2 {
                // Si se ejecuta este break entonces salimos del bucle grande, osea salimos de ambos bycles.
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

#### While
A menudo necesitamos evaluar una condicion dentro de un bucle mientras la condicion sea true, el bucle se ejecuta. Si la condicion deja de ser true el programa entonces llama a break por si solo y se detiene todo el bucle. Por ejemplo:

```rust
fn main() {
  let mut number = 3;

  while number != 0 {
    println!("{number}");

    number -= 1;
  }

  println!("LISTOFF!!!");
}
```

#### For
En realidad seria un plus de **while**, tambien podemos usar el mismo bucle while para recorrer los elementos de una coleccion, por ejemplo un arreglo, string, tupla, etc. todo lo que sean justamente enums. Por ejemplo:

```rust
fn main() {
  let a = [10, 20, 30, 40, 50];
  let mut index = 0;

  while index < 5 {
    println!("the value is: {}", a[index]);
    index += 1;
  }
}
```

Sin embargo si bien se puede simular el for con un while lo cierto es que este ultimo codigo es propenso a errores. Por ejemplo ya habria error si cambia la definicion del arreglo y por ejemplo pasa de tener 5 elementos a 4. Tambien es lento porque el compilador agrega codigo de tiempo de ejecucion para realizar la verificacion condicional de si el indice esta dentro de los limites del arreglo en cada iteracion del bucle.

Como una alternativa mas directa podemos usar el propio **FOR** donde ejecutamos un codigo para cada elemento de una coleccion al recorrer sus elementos, por ejemplo:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

Lo mas importante es que ahora hemos aumentado la seguridad del codigo y eliminado la posiblidad de errores que podrian deberse a ir mas alla del final del arreglo o no ir suficientemente lejos y perder algunos elementos. El codigo maquina generado a partir de los bucles for tambien puede ser mas eficiente, porque no es necesario comparar el indice con la longitud del arreglo en cada iteracion.

Usando el bucle for no necesitariamos recordar cambiar cualquier otro codigo si cambia el nnumero de valores en el arreglo.

La seguridad que proporciona el for() lo convierte en uno de los metodos de bucle mas usados en Rust. 

Tambien en lugar de hacer una cuenta regresiva con el bucle while tambien podriamos hacer una cuenta regresiva usando for que es mas seguro:

```rust
fn main() {
  // En este caso hacemos una cuenta regresiva de 1 a 4 pero en reversa (rev()) usando for
  for number in (1..4).rev() {
    println("{number}!");
  }
  print("LISTOFF!!!");
}
```