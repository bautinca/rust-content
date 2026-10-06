Un **struct** es un tipo de dato personalizado que te permite empaquetar y nombrar multiples valores relacionados que forman un grupo significativo. En un lenguaje POO seria como los **atributos de un objeto** 

# Definiendo e Instanciando Structs
Los structs son similares a las tuplas, en ambos casos se mantienen multiples valores relativos. Al igual que en las tuplas las partes de un struct pueden ser de distintos tipos. Pero a diferencia de las tuplas nosotros mismos en un struct nombramos a cada pieza de datos para que quede claro y que significan esos valores. Por lo tanto los structs son mas 'flexibles' que las tuplas.

Para definir un struct debemos usar la palabra **struct** y el nombre del struct completo. El nombre del struct debe describir el significado de los datos que se agrupan. Entre llaves definimos los nombres y tipos de datos que los llamamos **campos**. Por ejemplo veamos el sig. struct que almacena info sobre la cuenta de un usuario:

```rust
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}
```

Ya habiendo definido el struct le podemos crear una **instancia** que seria como un objeto de ese struct. Logicamente al definir la instancia debemos definir el valor de los distintos campos. Creamos una instancia al declarar el nombre del struct y luego agregar llaves que contienen *clave:valor* pares, donde las claves son los nombres de los campos y los valores son los datos que queremos almacenar en esos campos. No tenemos que especificar los campos en el mismo orden en que fueron definidos. En otras palabras la definicion del struct es como una plantilla general para el tipo y las instancias llenan esa plantilla con datos particulares para crear valores del tipo. Por ejemplo instanciaremos un usuario:

```rust
fn main(){
  let user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someemail@example.com"),
    sign_in_count: 1,
  };
}
```

Para acceder a un valor especifico de un struct usamos la notacion **punto**, por ejemplo para acceder a la direccion de correo del usuario podemos hacer `user1.email`.

Logicamente si la instancia es mutable entonces podemos cambiar un valor asignado a un campo particular, veamos un ejemplo:

```rust
fn main(){
  let mut user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
  };

  user1.email = String::from("anotheremail@example.com");
}
```

Rust no nos permite marcar solo ciertos campos como mutables, si queremos cambiar un campo de un struct debemos marcar **toda la instancia** como mutable.

Como cualquier expresion, podemos construir una nueva instancia del struct como la ultima expresion en el cuerpo de la funcion para devolver implicitamente esa nueva instancia, veamos un ejemplo:

```rust
// En este caso la funcion retorna una instancia del struct User
fn build_user(email: String, username: String) -> User {
  User {
    active: true,
    username: username,
    email: email,
    sign_in_count: 1,
  }
}
```

## Usando la abreviatura Field Init
Supongamos que el struct tiene miles de campos y que queremos crear una instancia de ese struct dentro de una funcion, logicamente la funcion deberia recibir como parametros todos esos campos y seria muy engorroso. Para solucionar esto podemos usar la **Abreviatura Field Init** para reescribir el ejemplo anterior de la siguiente manera:

```rust
fn build_user(email: String, username: String) -> User {
  User {
    active: true,
    username, // Abreviatura Field Init
    email,    // Abreviatura Field Init
    sign_in_count: 1,
  }
}
```

Es decir, Aquí, estamos creando una nueva instancia del struct User, que tiene un campo llamado email. Queremos establecer el valor del campo email en el valor del parámetro email de la función build_user. Debido a que el campo email y el parámetro email tienen el mismo nombre, solo necesitamos escribir email en lugar de email: email.

## Creando Instancias de otras Instancias con Sintaxis de Struct Update
Suele ser util crear una nueva instancia de un struct que incluya la mayoria de los valores de otra instancia pero cambie algunos. Podemos hacer esto usando la **sintaxis de struct update**. Veamos un ejemplo:

```rust
fn main() {
  let user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("another@example.como"),
    sign_in_count: 1,
  };

  let user2 = User {
      active: user1.active, // Usamos el valor de user1 para el campo active
      username: user1.username, // Usamos el valor de user1 para el campo username
      email: String::from("another@example.com"),
      sign_in_count: user1.sign_in_count, // Usamos el valor de user1 para el campo sign_in_count
  };
}
```

Ahora bien, aca no usamos la sintaxis de struct update, usando la sintaxis de struct update podemos lograr el mismo efecto con menos codigo exactamente asi:

```rust
fn main() {
  let user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("another@example.com"),
    sign_in_count: 1,
  };

  let user2 = User {
    email: String::from("yetanother@example.com"),
    ..user1 // Usamos la sintaxis de struct update para copiar los valores de user1 a user2 en el resto de los campos
  };
}
```

Entonces la instancia *user2* tiene el mismo valor que *user1* en los campos *active*, *username* y *sign_in_count*, pero tiene un valor diferente en el campo *email*.

Algo importante es que si o si el *..user1* debe ir al final para especificar que cualquier campo restante debe obtener sus valores del campo correspondiente en *user1*, pero podemos elegir especificar valores para tantos campos como queramos en cualquier orden, independientemente de donde se encuentre el *..user1*.

Notar ademas que la sintaxis de update struct usa **=** como asignacion, esto es porque mueve los datos. En este ejemplo ya no podemos usar *user1* despues de crear *user2* porque los campos *username* y *email* de *user1* son de tipo String y se movieron a *user2*. Si hubieramos dado a *user2* nuevos valores de *String* para *email* y *username*, y por lo tanto solo usamos los valores de *active* y *sign_in_count* de *user1*, entonces podriamos seguir usando *user1* despues de crear *user2* porque los campos *active* y *sign_in_count* implementan el trait **Copy** como vimos antes y por lo tanto no se mueven. Veamos un ejemplo en codigo:

```rust
fn main() {
  let user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("another@example.com"),
    sign_in_count: 1,
  };

  let user2 = User {
    email: String::from("yetanother@example.com"),
    ..user1
  };

  println!("{}", user1.username); // Esto no compila porque el campo username de user1 se movio a user2 al ser de tipo String y no implementa el trait Copy

  println!("{}", user2.email); // Esto si compila porque el campo email de user2 es un nuevo valor de tipo String
  
  println!("{}", user1.active); // Esto si compila porque el campo active de user1 implementa el trait Copy y por lo tanto no se movio a user2

  println!("{}", user2.active); // Esto si compila porque el campo active de user2 es un valor copiado del campo active de user1
}
```

## Usando Structs de Tuplas sin campos nombrados para crear diferentes tipos
Rust tambien admite structs de tuplas, osea literalmente se llaman **structs de tuplas**. Los structs de tuplas son similares a las tuplas normales, pero tienen la ventaja de que podemos darles un nombre y crear un tipo distinto. Por ejemplo, podemos definir un struct de tupla para representar un color RGB y otro para representar un punto en 3D:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
}
```

En este ultimo ejemplo usamos 2 structs de tupla llamados *Color* y *Point*.

Notar ademas que las variables *black* y *origin* son diferentes tipos ya que son instancias de diferentes structs de tupla. Por ejemplo una funcion que toma un parametro de tipo *Color* no puede tomar un *Point* como argumento, incluso si ambos tipos estan compuestos por 3 valores i32. A diferencia de las tuplas, los structs de tuplas requieren que nombre del tipo de estructura cuando las desestructuras, por ejemplo si queremos acceder a los valores de *black* y *origin* podemos hacer lo siguiente:

```rust
fn main() {
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);

  let Color(r, g, b) = black; // Desestructuramos el struct de tupla Color en variables r, g, b
  let Point(x, y, z) = origin; // Desestructuramos el struct de tupla Point en variables x, y, z

  println!("Color: r={}, g={}, b={}", r, g, b);
  println!("Point: x={}, y={}, z={}", x, y, z);
}
```

## Structs de Unidad sin Campos
Tambien podemos definir structs que no tienen ningun campo, estos se les llama **structs de unidad** porque se comportan de manera simiar a la unidad (). Los structs de unidad pueden ser utiles en situaciones donde necesitamos implementar un trait en un tipo pero no necesitamos almacenar datos en el struct. Por ejemplo, podemos definir un struct de unidad llamado *AlwaysEqual* que implementa el trait *PartialEq* para siempre devolver true cuando se comparan dos instancias de este struct:

```rust
struct AlwaysEqual; // Declaramos un struct de unidada sin campos

// Aca le implementamos un Trait al struct AlwaysEqual para que siempre devuelva true cuando se comparan dos instancias de este struct
impl PartialEq for AlwaysEqual {
  fn eq(&self, _other: &Self) -> bool {
    true
  }
}

fn main() {
  let a = AlwaysEqual;
  let b = AlwaysEqual;

  println!("Are a and b equal? {}", a == b); // Esto imprimira true
}
```

