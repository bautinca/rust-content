# Sintaxis de Metodos
Los metodos ya los conocemos, son similares a las funciones. A diferencia de las funciones los metodos **se definen dentro del contexto de una struct** y su primer parametro siempre es **self** que representa la misma instancia de la struct en la que se llama al metodo.

## Definiendo Metodos
Veamos el sig. ejemplo:

```rust
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle { // La palabra reservada impl nos permite definir metodos para la struct Rectangle
  fn area(&self) -> u32 { // Definimos un metodo llamado area que toma una referencia a self, osea la instancia del rectangulo a la que le aplicamos este metodo, y devuelve un valor de tipo u32
    self.width * self.height // Calculamos el area multiplicando el ancho por el alto de la instancia de Rectangle
  }
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!("El area de rect1 es {}", rect1.area()); // Llamamos al metodo area en la instancia rect1 y mostramos el resultado
}


```

Para definir la funcion dentro del contexto de *Rectangle* iniciamos un bloque **impl** (implementacion). **Todo lo que este dentro del bloque impl estara asociado al tipo Rectangle**

En la firma para area, usamos &self en vez de rectangle: &Rectangle. El &self es en realidad una abreviatura para self: &Self. Dentro de un bloque impl, el tipo Self es un alias para el tipo al que pertenece el bloque impl. Los métodos deben tener un parámetro llamado self de tipo Self para su primer parámetro, por lo que Rust nos permite abreviar esto con solo el nombre self en el primer parámetro. Ten en cuenta que aún necesitamos usar el & antes de la abreviatura self para indicar que este método toma prestada la instancia Self, al igual que hicimos en rectangle: &Rectangle. Los métodos pueden tomar la propiedad de self, tomarlo prestado de forma inmutable, como hemos hecho aquí, o tomarlo prestado de forma mutable, al igual que pueden hacerlo con cualquier otro parámetro.

Elegimos &self aquí por la misma razón que usamos &Rectangle en la versión de la función: no queremos tomar la propiedad, y solo queremos leer los datos en la estructura, no escribir en ella. Si quisiéramos cambiar la instancia en la que hemos llamado al método como parte de lo que el método hace, usaríamos &mut self como primer parámetro. Tener un método que tome la propiedad de la instancia usando solo self como primer parámetro es raro

La razón principal para usar métodos en vez de funciones, además de proveer la sintaxis de método y no tener que repetir el tipo de self en cada firma de método, es para la organización

Nota que podemos elegir darle al método el mismo nombre que uno de los campos del struct. Por ejemplo, podemos definir un método en Rectangle que se llame width asi:

```rust
impl Rectangle {
  fn width(&self) -> bool {
    self.width > 0
  }
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!("El ancho de rect1 es {}", rect1.width()); // Llamamos al metodo width en la instancia rect1 y mostramos el resultado. En este caso el metodo widht() seria un metodo que devuelve un booleano indicando si el ancho del rectangulo es mayor a 0
}
```

A veces, pero no siempre, cuando damos un método el mismo nombre que un campo queremos que solo retorne el valor en el campo y no haga nada más. Los métodos como este se llaman **getters** y Rust no los implementa automáticamente para los campos de un struct como lo hacen otros lenguajes

Los getters son útiles porque puedes hacer que el campo sea privado, pero el método sea público, y así permitir acceso de solo lectura a ese campo como parte de la API pública del tipo

## Metodos con mas parametros
Practiquemos usando métodos implementando un segundo método en la estructura Rectangle. Esta vez queremos que una instancia de Rectangle tome otra instancia de Rectangle y retorne true si el segundo Rectangle puede completamente caber dentro de self (el primer Rectangle); de lo contrario, debería retornar false

```rust
impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool { // Definimos un metodo llamado can_hold que toma una referencia a self y otra referencia a otra instancia de Rectangle llamada other, y devuelve un valor de tipo bool
    self.width > other.width && self.height > other.height // Retornamos true si el ancho y el alto de self son mayores que los de other, de lo contrario retornamos false
  }
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  let rect2 = Rectangle {
    width: 10,
    height: 40,
  };

  let rect3 = Rectangle {
    width: 60,
    height: 45,
  };

  println!("rect1 puede contener a rect2? {}", rect1.can_hold(&rect2)); // Llamamos al metodo can_hold en la instancia rect1 pasando una referencia a rect2 y mostramos el resultado
  println!("rect1 puede contener a rect3? {}", rect1.can_hold(&rect3)); // Llamamos al metodo can_hold en la instancia rect1 pasando una referencia a rect3 y mostramos el resultado
}
```

Sabemos que queremos definir un método, por lo que estará dentro del bloque impl Rectangle. El nombre del método será can_hold, y tomará un préstamo inmutable de otro Rectangle como parámetro. Podemos decir cuál será el tipo del parámetro mirando el código que llama al método: rect1.can_hold(&rect2) pasa &rect2, que es un préstamo inmutable a rect2, una instancia de Rectangle. Esto tiene sentido porque solo necesitamos leer rect2 (en lugar de escribir, lo que significaría que necesitaríamos un préstamo mutable), y queremos que main conserve la propiedad de rect2 para que podamos usarlo nuevamente después de llamar al método can_hold. El valor de retorno de can_hold será un Booleano, y la implementación verificará si el ancho y alto de self son mayores que el ancho y alto del otro Rectangle, respectivamente

## Funciones asociadas
Todas las funciones definidas dentro de un bloque impl se llaman **funciones asociadas**, osea metodos = funciones asociadas. Sin no es tan asi porque si el metodo no tiene un parametro self, entonces no es un metodo sino una **funcion asociada**. Las funciones asociadas se llaman usando la sintaxis ::, como en Rectangle::square. Por ejemplo, podemos definir una función asociada llamada square que cree un cuadrado de un tamaño dado:

```rust
impl Rectangle {
  fn square(size: u32) -> Self { // Definimos una funcion asociada llamada square que toma un parametro size de tipo u32 y devuelve una instancia de Rectangle
    Self { // Retornamos una instancia de Rectangle con el ancho y alto iguales al parametro size
      width: size,
      height: size,
    }
  }
}

fn main() {
  let square = Rectangle::square(30); // Llamamos a la funcion asociada square pasando el valor 30 y guardamos el resultado en la variable square

  println!("El area del cuadrado es {}", square.area()); // Llamamos al metodo area en la instancia square y mostramos el resultado
}
```

Que otra funcion asociada ya hemos usado? Por ejemplo para el struct **String** tenemos la funcion asociada **String::from** que nos permite crear un String a partir de un &str. Por ejemplo:

```rust
fn main() {
  let s = String::from("Hola mundo"); // Llamamos a la funcion asociada String::from pasando el valor "Hola mundo" y guardamos el resultado en la variable s

  println!("El string es: {}", s);
}
```

## Bloques impl multiples
Cada struct tiene permitido tener multiples bloques impl y no esta restringido a tener solo uno si o si, por ejemplo:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

```

De poder se puede, sin embargo no hay razon para separar estos metodos en multiples bloques impl, sin embargo es una sintaxis valida. Entonces basicamente el struct Rectangle tiene los 2 metodos disponibles area y can_hold, y podemos llamarlos desde cualquier instancia de Rectangle