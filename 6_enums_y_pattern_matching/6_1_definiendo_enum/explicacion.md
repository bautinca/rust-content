Vamos a ver los **enums (enumeraciones)**. Un enum es un tipo de dato que define un conjunto de valores posibles. Por ejemplo, podemos definir un enum para representar los colores del arco iris, etc. Veremos un tipo particular de enum muy util que es el **Option**. Este enum nos permite representar un valor que puede estar presente o no. Es muy útil para manejar valores que pueden ser nulos o ausentes de manera segura.

Los structs te permiten agrupar campos relacionados y datos como un Rectangulo, con su ancho y largo. Por otro lado los enums te permiten decir que un valor es uno de un conjunto de valores posibles. Por ejemplo, podriamos querer decir que Rectangulo es uno de un conjunto de posibles formas que tambien incuye Circulo y Triangulo. Para hacer esto Rust tiene los enums.

Por ejemplo veamos las direcciones IP que son de 4 y 6 bytes, esas son todas las variantes de IP que existen, entonces podemos enumerar todas las variantes posibles.

Cualquier direccion IP puede ser una direccion de la version de 4 o 6 bytes, pero no ambas al mismo tiempo. Esa propiedad de las direcciones IP hace que las estructuras de datos enum sea apropiada porque un valor enum puede ser solo una de sus variantes. Tanto las direcciones de la version 4 como de la version 6 siguen siendo fundamentalmente direcciones IP, por lo que deben ser tratadas como el mismo tipo cuando el codigo esta maneajando situaciones que se aplican a cualquier tipo de direccion IP.

Veamos en codigo como definir el enum:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

Ahora *IpAddrKind* es un tipo de dato enum que podemos usar en otras partes de nuestro codigo

## Valores Enum
Podemos crear instancias de cada una de las variantes asi:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

Notar que las vaariantes del enum estan en el mismo espacio de nombres bajo su identificador y usamos los :: para separar los 2. Esto es util porque ahora ambos valores IpAddrKind::V4 y IpAddrKind::V6 son del mismo tipo IpAddrKind. Entonces por ejemplo podemos definir una funcion que tome cualquier instancia de IpAddrKind como argumento:

```rust
fn route(ip_kind: IpAddrKind) {
    // code to route the IP address
}
```

Y podemos llamar esta funcion con cualquiera de sus variantes

```rust
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

Y como podemos asignarle valor a una instancia de un enum? Bueno, podemos definir un struct que contenga una direccion IP y su tipo, y luego crear instancias de ese struct con diferentes valores de enum. Por ejemplo:

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

Sin embargo representar el mismo concepto de direccion IP con un enum y un struct es un poco redundante. Podemos simplificarlo usando **enum con datos asociados**. Esto nos permite almacenar datos directamente en cada variante del enum, eliminando la necesidad de un struct separado. Por ejemplo:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1")); // La instancia de la variante V4 contiene un String con la direccion IP
let loopback = IpAddr::V6(String::from("::1")); // La instancia de la variante V6 contiene un String con la direccion IP
```

Nos ahorramos de tener que definir un struct separado y podemos almacenar directamente la direccion IP en cada variante del enum. Esto hace que el codigo sea mas conciso y facil de leer.

Ahora tambien podemos ver otra cosa y es que el nombre de cada variante de enum que definimos tambien se convierte en una **funcion** que construye una instancia del tipo enum. Es decir nosotros dijimos que para instanciar un enum de tipo IpAddr debemos usar IpAddr::V4 o IpAddr::V6, pero en realidad V4() y V6() estas son funciones que toman un String y devuelven una instancia del IpAddr enum.

Hay otra ventaja de usar enum en lugar de struct y es que cada variante puede tener diferentes tipos y cantidades de datos asociadas a ella. La version 4 de direcciones IP siempre tendra 4 componentes numericos que tendran valores entre 0 y 255. Si quisieramos almacenar las direcciones IP de version 4 como cuatro valores u8 (bytes) pero aun asi expresar las direcciones V6 como un valor String no podriamos hacerlo con un struct, pero con los enums si:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8), // La variante V4 tiene 4 valores u8 asociados
    V6(String), // La variante V6 tiene un String asociado
}

let home = IpAddr::V4(127, 0, 0, 1); // La instancia de la variante V4 contiene 4 valores u8

let loopback = IpAddr::V6(String::from("::1")); // La instancia de la variante V6 contiene un String
```

Tambien podriamos hacer algo asi, por ejemplo tipos de un enum que reciben structs como datos asociados:

```rust
struct Ipv4Addr {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

struct Ipv6Addr {
    address: String,
}

enum IpAddr {
    V4(Ipv4Addr), // La variante V4 tiene un struct Ipv4Addr asociado
    V6(Ipv6Addr), // La variante V6 tiene un struct Ipv6Addr asociado
}
```

Veamos otro tipo de enum, por ejemplo este tiene incrustados una gran cantidad de variantes

```rust
enum Message {
    Quit, // No tiene datos asociados
    Move { x: i32, y: i32 }, // Tiene un struct anonimo
    Write(String), // Tiene un String asociado
    ChangeColor(i32, i32, i32), // Tiene 3 valores i32
}
```

Definiendo un enum con variantes es similar a definir diferentes tipos de definiciones de struct, excepto que el enum no use la palabra clave struct y todas las variantes estan agrupadas juntas bajo un mismo nombre de enum. Los siguientes structs podrian contener los mismos datos que las variantes de enum anteriores:

```rust
struct QuitMessage; // Unit Struct, no tiene datos asociados
struct MoveMessage { x: i32, y: i32 }; // Struct con 2 campos x e y de tipo i32
struct WriteMessage(String); // Tuple struct con un String
struct ChangeColorMessage(i32, i32, i32); // Tuple struct con 3 valores i32
```

Al igual que podemos definir metodos en structs usando impl, **podemos definir metodos en enums**. Por ejemplo definimos el siguiente enum mensaje con el metodo call():

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // code to process the message
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

El cuerpo del metodo usaria self para obtener el valor en el que llamamos el metodo. En este ejemplo hemos creado una variable m que contiene una instancia de la variante Write del enum Message, y luego llamamos al metodo call() en esa instancia. Dentro del metodo call(), podemos usar self para acceder a los datos asociados con la variante Write, como el String "hello".

## Enum Option
Hay un tipo de enum muy especial dentro de la libreria estandar de Rust que se llama **Option**. Este enum es muy util para representar un valor que puede estar presente o no. Por ejemplo si solicita el primer elemento de una lista no vacia obtendria un valor. Si solicita el primer elemento de una lista vacia entonces no obtendriamos nada. El enum Option puede prevenir errores que son extremadamente comunes en otros lenguajes de programacion.

Rust no tiene la caracteristica de null que muchos otros lenguajes tienen. Null es un valor que significa que no hay ningun valor alli. En los lenguajes con null, las variables siempre pueden estar en uno de 2 estados null o no null.

El problema con los valores null es que si intentamos usar un valor null como un valor no null entonces obtendremos errores de algun tipo, debido a que esta propiedad nula o no nula es omnipresente, es extremadamente facil cometer este tipo de errores en otros lenguajes con null.

Sin embargo, el concepto que null esta tratando de expresar sigue siendo util, un null es un valor que es actualmente invalido o ausente por alguna razon.

Rust entonces no tiene null, pero tiene un enum que puede codificar el concepto de un valor presente o ausente. Este enum es el famoso **Option\<T>** y esta definido automaticamente por la biblioteca estandar de Rust. El enum Option tiene 2 variantes:

```rust
enum Option<T> {
    Some(T), // Representa un valor presente de tipo T
    None, // Representa un valor ausente
}
```

El enum Option es tan util que incluso esta incluido en la biblioteca estandar de Rust por lo que no es necesario traerlo al contexto de ejecucion explicitamente. Sus variantes tambien estan incluidas en la biblioteca estandar, podemos usar Some y None directamente sin tener que escribir Option::Some o Option::None. Por ejemplo:

```rust
let some_number = Some(5); // some_number es de tipo Option<i32>
let some_string = Some("a string"); // some_string es de tipo Option<&str>
let some_char = Some('c'); // some_char es de tipo Option<char>
let absent_number: Option<i32> = None; // absent_number es de tipo Option<i32>, tenemos que especificarle que es absent_number porque Rust no puede inferir el tipo de dato de None
```

La sintaxis **\<T>** es una caracteristica de Rust que aun no se hablo. Es un parametro de **tipo generico de dato** que hablaremos mas adelante. Por ahora solo es importante saber que el enum Option puede contener cualquier tipo de dato, y que podemos usarlo para representar un valor que puede estar presente o ausente de manera segura.

Cuanto tenemos un valor Some, sabemos que un valor esta presente y el valor se mantiene dentro del Some. Cuanto tenemos un valor None en cierto sentido significa lo mismo que null: no tenemos un valor valido. Entonces ¿Porque tener Option\<T> es mejor que tener un null? Bueno, la diferencia es que Option\<T> es un tipo de dato que nos obliga a manejar el caso de ausencia de valor de manera segura. En otros lenguajes con null, podemos tener una variable que puede ser null y luego intentar usarla sin verificar si es null o no, lo que puede llevar a errores en tiempo de ejecucion. Con Option\<T>, el compilador nos obliga a manejar ambos casos: cuando hay un valor presente (Some) y cuando no hay un valor (None). Esto hace que nuestro codigo sea mas seguro y menos propenso a errores. Por ejemplo:

```rust
// La funcion retorna un Option<f64> que puede ser Some(valor) o None
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None // Si el denominador es 0, devolvemos None
    } else {
        Some(numerator / denominator) // Si el denominador no es 0, devolvemos Some con el resultado de la division
    }
}
```

Ahora bien el siguiente codigo se podria compilar?

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

Este ultimo codigo no se podria compilar porque estamos intentando sumar un valor de tipo i8 con un valor de tipo Option<i8>. El compilador nos dira que no puede sumar estos dos tipos de datos diferentes. Para poder sumar estos valores, primero debemos manejar el caso de Option y extraer el valor presente (Some) o manejar el caso de ausencia (None) de manera segura. Por ejemplo, podríamos usar un match para manejar ambos casos:

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = match y {
    // Extraemos el valor dentro de Some
    Some(value) => x + value, // Osea si y es Some(value), entonces sumamos x con el valor presente
    None => x, // En cambio si y es None, entonces sumamos x con 0 o simplemente devolvemos x
};
```

Es decir, si un tipo de dato es Option entonces debemos manejar ambos casos de manera segura (tanto para el valor presente como ausente) antes de poder usar el valor. Esto nos obliga a pensar en los casos de ausencia de valor y nos ayuda a evitar errores comunes relacionados con null en otros lenguajes de programación. Por ejemplo con la variable 'y' al declararla como Option el compilador ya sabe que para dicha variable existe la posibilidad de que no tenga un valor presente.

Si sabemos que para un tipo de dato nunca cabe la posibilidad de que sea None entonces no es necesario hacer que dicho tipo de dato sea Option ya que no existe un mundo donde esa variable sea None. Por ejemplo, si tenemos una variable que representa la edad de una persona, sabemos que siempre debe tener un valor presente (aunque sea 0), entonces no es necesario usar Option para esa variable. En cambio, si tenemos una variable que representa el número de teléfono de una persona, es posible que esa persona no tenga un número de teléfono, entonces sería apropiado usar Option para esa variable.


