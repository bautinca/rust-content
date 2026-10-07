# HashMaps
Ya vimos colecciones de Strings y Vectores, ahora nos queda ver otra colección muy importante: los **HashMaps**.

Es una estructura que permite almacenar **pares clave-valor**, es util cuando queremos asociar un dato con otro. En Rust se representa como **HashMap\<K,V>** donde:

- **K**: Tipo de dato de la clave (Key)
- **V**: Tipo de dato del valor (Value)

Por ejemplo podriamos tener **HashMap\<String, i32>*, donde la clave es un String y el valor es un entero.

Es ideal usar los HashMaps cuando queremos relacionar 2 cosas, por ejemplo las notas de los alumnos:

```
"Bautista" -> 10
"Juan" -> 8
"Ana" -> 9
```

Para este caso un HashMap encajaria perfectamente donde para almacenar dichos pares se haria asi:

```rust
// Importamos de la libreria estandar la estructura HashMap
use std::collections::HashMap;

let mut notas: HashMap<String, i32> = HashMap::new(); // Creamos un HashMap vacio, logicamente mutable porque lo iremos llenando

notas.insert(String::from("Bautista"), 10); // Insertamos el par clave-valor
notas.insert(String::from("Juan"), 8);
notas.insert(String::from("Ana"), 9);
```

Por otro lado ¿Como hacemos para obtener un valor dada la clave? Para eso usamos el metodo **get** que nos devuelve un **Option\<&V>**, esto es porque la clave que le pasamos en el metodo puede existir o no, y si no existe entonces nos devolveria **None**. Por eso es importante manejar el caso de que la clave no exista.

```rust
let nota_bautista = notas.get("Bautista"); // Devuelve Some(&10) porque la clave existe
let nota_juan = notas.get("Juan"); // Devuelve Some(&8) porque la clave existe
let nota_ana = notas.get("Ana"); // Devuelve Some(&9) porque la clave existe
let nota_pedro = notas.get("Pedro"); // Devuelve None porque la clave no existe
```

Entonces es asi como luego de hacer un **get** podemos atender ambos casos, en caso de que el valor este presente y en caso de que no lo este ya que la clave no existe en el HashMap, recordar que para cubrir todos los casos debemos usar un **match** asi:

```rust
match notas.get("Bautista") {
    Some(nota) => println!("La nota de Bautista es: {}", nota),
    None => println!("No se encontro la nota de Bautista"),
}
```

Algo que llama la atencion es porque el metodo **get** devuelve un **Option\<&V>** y no un **Option\<V>**? Es decir, porque en caso de devolver un valor devuelve una referencia al mismo? Esto es porque **el HashMap es ownership de los valores que contiene** por lo tanto no quiere ceder el ownership y es por eso que si le pedimos los valores al HashMap nos retornara las referencias de sus valores.

Por otro lado tambien se puede iterar sobre un HashMap asi:

```rust
for (alumno, nota) in &notas {
    println!("La nota de {} es: {}", alumno, nota);
}
```

Pero ojo, el HashMap **no esta pensado para mantener elementos ordenados**

Hablando un poco mas del tema Ownership supongamos que tenemos lo siguiente:

```rust
let campo = String::from("color");
let valor = String::from("azul");

let mut mapa = HashMap::new();

mapa.insert(campo, valor); // Aca lo que sucede es que el ownership de campo y valor se mueve al HashMap, por lo tanto no podemos usar mas esas variables porque ya no nos pertenecen, si intentamos hacer algo como:
println!("{}", campo); // Esto nos dara un error de compilacion porque ya no tenemos ownership de la variable campo ya que paso al HashMap
println!("{}", valor); // Esto nos dara un error de compilacion porque ya no tenemos ownership de la variable valor ya que paso al HashMap
```

Pero ojo, esto de la transferencia de ownership habiamos dicho que depende del tipo de dato, por ejemplo si en lugar de usar Strings usamos enteros, entonces no habria problema porque los enteros habiamos dicho que implementan el trait **Copy** y por lo tanto no se mueve el ownership sino que se copia el valor, entonces en ese caso si podriamos seguir usando las variables despues de insertarlas en el HashMap.

```rust
let x = 10;

mapa.insert(String::from("numero"), x);

println!("{x}"); // Esto funciona ya que lo que se pasa al HashMap es una copia del valor de x y no el ownership del mismo ya que los enteros tienen el trait Copy
```

Que pasa si insertamos un valor para una clave que ya existia? Bueno entonces **se actualizara el valor en dicha clave** Por ejemplo:

```rust
let mut notas: HashMap<String, i32> = HashMap::new();

notas.insert(String::from("Bautista"), 10); // Insertamos el par clave-valor
notas.insert(String::from("Bautista"), 9); // Insertamos el par clave-valor con la misma clave, entonces el valor de la clave "Bautista" se actualizara a 9
```

Entonces con esto damos a entender que **las claves son UNICAS** y no pueden repetirse a diferencia de los valores.

Un metodo muy util del HashMap es **entry()** que nos permite insertar un valor en una clave si es que esta no existe, y si existe entonces no hace nada. Por ejemplo:

```rust
let mut notas: HashMap<String, i32> = HashMap::new();

notas.insert(String::from("Bautista"), 10); // Insertamos el par clave-valor

notas.entry(String::from("Bautista")).or_insert(9); // Como la clave "Bautista" ya existe, entonces no hace nada y el valor de la clave "Bautista" sigue siendo 10
notas.entry(String::from("Juan")).or_insert(8); // Como la clave "Juan" no existe, entonces inserta el par clave-valor "Juan" -> 8
```

Basicamente con el *entry()* nos preguntamos si la clave existe o no, y si no existe entonces insertamos el valor con *or_insert()*, y si existe entonces no hacemos nada y el valor de la clave sigue siendo el mismo.

Para entender bien que tan util nos puede servir el *entry()* por ejemplo el clasico contador de palabras, con el *entry()* se lo puede hacer de manera muy facil y elegante, por ejemplo:

```rust
use std::collections::HashMap;

let mut contador_palabras: HashMap<String, i32> = HashMap::new();

let texto = "hola mundo hola rust hola hashmaps";

for palabra in texto.split_whitespace() {
    let contador = contador_palabras.entry(palabra.to_string()).or_insert(0); // Si la palabra no existe en el HashMap, entonces inserta la palabra con valor 0, y si existe entonces devuelve una referencia al valor de la palabra
    *contador += 1; // Como devuelve una referencia mutable al valor de la palabra, lo tenemos que desreferenciar para poder modificar el valor, entonces le sumamos 1 al valor de la palabra
}
```

Algo que tenemos que entender bien es que `let contador = mapa.entry(palabra).or_insert(0);` devuelve una **referencia mutable** al valor de la clave y es por eso que para incrementar +1 a la aparicion de la palabra tenemos que desreferenciarla con `*contador += 1;` para poder modificar el valor.

Entonces ya sabemos varios tipos de colecciones, una pregunta es **¿Cuando usar Vector y cuando un HashMap?**. El Vector nos interesa si queremos una **secuencia ordenada**, en cambio el HashMap lo usamos cuando no nos interesa el orden y queremos una asociacion clave -> valor.

Ademas otra diferencia es que en los Vectores accedemos a los valores por su **indice** y en los HashMaps accedemos a los valores por su **clave**.

¿Que tipos de datos pueden ser claves? Bueno ojo, **las claves tienen que cumplir ciertos requisitos**, principalmente implementar los traits necesarios para poder ser comparadas y para poder ser hasheadas, por lo tanto los tipos de datos que pueden ser claves son aquellos que implementan los traits **Eq** y **Hash**. Por ejemplo los tipos de datos primitivos como enteros, flotantes, booleanos y Strings pueden ser claves, pero los tipos de datos compuestos como Vectores y HashMaps no pueden ser claves porque no implementan los traits necesarios.



