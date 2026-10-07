# String
Ya vimos la coleccion vectores, ahora vamos a ver otra coleccion que es la de **strings**, que nos permite almacenar texto codificado en UTF-8.

Pero antes que nada hay una confusion muy comun entre **String vs &str**. Aunque ambos representan texto UTF-8 tienen diferencias importantes:

- **String**: 
  - Es un tipo de dato que representa una cadena de texto **mutable**
  - Se almacena en memoria dinamica (**heap**)
  - Al ser mutable puede crecer o decrecer en tamaño
  - Codificado en UTF-8
  - Ownership
  
  ```rust
  let mut s = String::from("Hola");

  s.push_str(", mundo!"); // Agrega texto al final de la cadena
  
  println!("{}", s); // Imprime "Hola, mundo!"
  ```

- **&str**: 
  - Es un tipo de dato que representa una cadena de texto **inmutable**, osea es una **REFERENCIA** a un string. Osea una referencia a un texto que se almacena en otro lugar
  - Se almacena en memoria estatica (**stack**)
  - No puede crecer ni decrecer en tamaño ya que se almacena en stack
  - Codificado en UTF-8
  
  ```rust
  let s: &str = "Hola, mundo!";
  
  println!("{}", s); // Imprime "Hola, mundo!"
  ``` 

Una forma sencilla de pensarlo es que el *&str* dice *"mira este texto que ya exsite"* en cambio el *String* dice *"mira este texto que yo cree y puedo cambiarlo ya que soy su propietario/ownership"*.

Entonces ya que sabemos la diferencia ahora vamos a ver como crear el String:

```rust
// Asi podemos crear un String vacio de entrada
let mut s = String::new(); // Inicialmente tenemos s = "", en este caso 's' es owner (propietario) del String

// Tambien podemos crear un String a partir de un &str (literal de texto)
let s = String::from("Hola, mundo!"); // Inicialmente tenemos s = "Hola, mundo!", al ser una String 's' es owner (propietario) del String

// Tambien podemos crear un String a partir de un &str (literal de texto) usando to_string()
let s = "Hola, mundo!".to_string(); // Inicialmente tenemos s = "Hola, mundo!", al ser una String 's' es owner (propietario) del String
```

Elegir entre String y to_string() es meramente una cuestion de preferencia personal.

Ahora bien, Rust puede almacenar una enorme variedad de caracteres ya que utiliza la codificacion UTF-8 por ejemplo:

```rust
let s = String::from("Hola");
let s = String::from("こんにちは");
let s = String::from("你好");
let s = String::from("Здравствуйте");
```

Todos son validos, esto es valido porque UTF-8 permite representar caracteres de practicamente todos los idiomas.

Hay una parte clave que es **EN UTF-8 NO TODOS LOS CARACTERES OCUPAN LA MISMA CANTIDAD DE BYTES**. El tipo de dato String por defecto esta implementado como un **Vec\<u8>**. Es decir, es una coleccion de bytes. La mayoria de los caracteres ASCII ocupan 1 byte por ejemplo:

```rust
let s = String::from("Hola"); // 4 bytes (cada caracter ocupa 1 byte)

s.len(); // Devuelve 4 ya que cada caracter ocupa 1 byte
```

Pero **ojo con String::len()** ya que devuelve la cantidad de bytes que ocupa el String y no la cantidad de caracteres como podemos creer, por ejemplo:

```rust
let s = String::from("hola");

println!("{}", s.len()); // Devuelve 4 ya que cada caracter ocupa 1 byte

let s = String::from("こんにちは");

println!("{}", s.len()); // Devuelve 15 ya que cada caracter ocupa 3 bytes!!! sin embargo no tiene 15 caracteres, tiene 5 caracteres.
```

Otra cosa fundamental es lo siguiente:

```rust
let s = String::from("Hola");

let h = s[0]; // Esto ESTA MAL porque Rust no sabe que queremos decir con posicion 0, ¿Queremos el primer byte? El primer caracter?
```

Este ultimo problema se vuelve evidente con por ejemplo si tenemos un string del '3', veamos:

```rust
let s = String::from("3");

// El caracter 3 ocupa 2 bytes en UTF-8 entonces conceptualmente 3 -> [208, 151] (2 bytes) entonces si hacemos s[0] que deberia devolver 3 o 208? Osea el primer caracter o el primer byte que ocupa el caracter?
```

Es justamente por este ultimo cuestionamiento que Rust **no permite indexar un string con un entero**.

Por otro lado hay **3 formas de mirar un String**, osea Rust lo interpreta de 3 maneras que son:

- **Bytes**: Es la forma mas basica de ver un String, es decir como una coleccion de bytes. Por ejemplo para el caracter *Зд* ocupa 4 bytes en UTF-8 osea *Зд* -> [208, 151, 208, 180] (4 bytes) y por lo tanto s.len() devuelve 4. Si queremos ver todos los bytes del caracter podemos hacer:

  ```rust
  let s = String::from("Зд");

  for b in s.bytes() {
      println!("{}", b); // Imprime 208, 151, 208, 180
  }
  ```

- **Valores escalares Unicode (char)**: Es la forma mas comun de ver un String, es decir, como una coleccion de caracteres Unicode. Por ejemplo para el caracter *Зд* ocupa 2 caracteres Unicode osea *Зд* -> ['З', 'д'] (2 caracteres) y por lo tanto s.chars().count() devuelve 2. Si queremos ver todos los caracteres Unicode del caracter podemos hacer:

  ```rust
  let s = String::from("Зд");

  for c in s.chars() {
      println!("{}", c); // Imprime З, д, cada uno es un char, y aca hay una diferencia y confusion muy importante, u8 representa 1 BYTE, en cambio char representa 1 CARACTER UNICODE, y un caracter Unicode puede ocupar mas de 1 byte en UTF-8
  }
  ```

- **Grafemas**: Es la forma mas avanzada de ver un String, es decir como una coleccion de grafemas, que son los caracteres que el humano ve en pantalla. Por ejemplo el caracter 'é' es un solo caracter para el humano pero internamente puede estar representado por 2 caracteres Unicode (e + ´) y por lo tanto ocupar 2 bytes en UTF-8. Por ejemplo tenemos la letra नमस्ते pero internamente en representacion de bytes son 18 bytes, osea de tipo u8, en cambio si lo vemos en representacion Unicode (char) tenemos 6 caracteres unicode ['न', 'म', 'स', '्', 'त', 'े'] osea son 6 valores de tipo char. Y luego el grafema seria solo 1 caracter que es lo que el humano ve en la pantalla.

En resumen:

- **Byte**: Representacion mas basica de un String
- **Char**: Representacion de un String como una coleccion de caracteres Unicode
- **Grafema**: Representacion de un String como una coleccion de caracteres que el humano ve

Entonces volviendo al cuestionamiento anterior si hacemos por ejemplo:

```rust
let s = String::from("Зд");

s[0] // -> Esta mal ya que Rust no sabe si queremos el primer byte, o el primer Unicode o el primer grafema, entonces Rust no permite indexar un String con un entero.
```

Por otro lado:

```rust
let s = String::from("Зд");

// Asi obtengo los bytes del String
for b in s.bytes() {
    println!("{}", b); // Imprime 208, 151, 208, 180
}

// Asi obtengo los caracteres Unicode del String
for c in s.chars() {
    println!("{}", c); // Imprime З, д
}

// Y logicamente el grafema seria lo que el humano ve, osea el caracter "Зд" 
```

Y si quiero obtener un **slice** de la String? Bueno lo podemos hacer pero tenemos que especificar un **rango de bytes** por ejemplo:

```rust
let s = String::from("hola");

let s = &s[0..2]; // Esto es valido ya que estamos especificando un rango de bytes, osea los bytes 0 y 1, que corresponden a los caracteres 'h' y 'o'
```

Ahora cuidado con los rangos, ya que nos pueden originar un panic, por ejemplo supongamos que la String es el caracter 3 y dijimos que esta compuesta por 2 bytes entonces si hacemos:

```rust
let s = String::from("3");

let s = &s[0..1]; // Esto es invalido ya que estamos especificando un rango de bytes que no corresponde a un caracter completo, osea el byte 0 corresponde al primer byte del caracter '3' pero el byte 1 corresponde al segundo byte del caracter '3', entonces estamos cortando el caracter '3' en dos partes y eso no es valido, por lo tanto Rust nos va a dar un panic. Recordar que LOS SLICES SE HACEN PENSANDO EN BYTES.
```

Hasta ahora vimos como leer Strings, pero ahora vamos a ver como **modificarlos**:

```rust
let mut s = String::from("foo");

s.push_str("bar"); // Agrega "bar" al final de la cadena, resultado s = "foobar"
```

Por otro lado la funcion *push_str()* no toma el ownership del String que le pasamos, osea que podemos seguir usando el String que le pasamos despues de usar *push_str()*, por ejemplo:

```rust
let mut s = String::from("foo");
let s2 = "bar";

s.push_str(s2); // Agrega "bar" al final de la cadena, resultado s = "foobar"

// Como la funcion push_str no toma el ownership del String s2 podemos seguir usando s2 despues de usar push_str()
println!("{}", s2); // Imprime "bar"
```

Luego ademas del *push_str* tambien hay otro:

```rust
let mut s = String::from("lo");
s.push('l'); // Agrega el caracter 'l' al final de la cadena, resultado s = "lol"
```

La diferencia esta en que la funcion *push_str* recibe un tipo de dato **&str** mientras que la funcion *push* recibe un tipo de dato **char**.

Tambien hay otro modo de concatenacion de strings que es usando el operador **+** por ejemplo:

```rust
let s1 = String::from("Hola, ");
let s2 = String::from("mundo!");

let s3 = s1 + &s2; // s1 ya no es valido ya que el operador + toma el ownership de s1, y s2 es pasado como referencia, resultado s3 = "Hola, mundo!"

// Como el operador + toma el ownership de s1, ya no podemos usar s1 despues de la concatenacion, en cambio si podemos seguir usando s2 ya que fue pasado como referencia.

// Para entenderlo a profundidad el operador + la firma es la siguiente fn add(self, s: &str) -> String, osea que toma ownership de self (s1) y s2 es pasado como referencia.
```

Por ultimo hay otra manera de concatenar strings que es usando la macro **format!** por ejemplo:

```rust
let s1 = String::from("Hola, ");
let s2 = String::from("mundo!");

let s3 = format!("{}{}", s1, s2); // Esto es valido y no toma ownership de s1 ni de s2, resultado s3 = "Hola, mundo!"

// Al no tomar el ownership entonces por ejemplo podriamos hacer:
println!("{}", s1); // Imprime "Hola, "
println!("{}", s2); // Imprime "mundo!"
```




