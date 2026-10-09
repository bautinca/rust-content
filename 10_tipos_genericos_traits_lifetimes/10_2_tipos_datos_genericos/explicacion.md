La sintaxis formal es poner el parametro de tipo entre *<>* despues del nombre de la funcion, osea

```rust
// Indicamos que la funcion es generica (T) y que recibe una referencia a un slice de T y devuelve una referencia a T
fn mayor<T>(lista: &[T]) -> &T {
    let mut mayor = &lista[0];
    for item in lista {
        if item > mayor {
            mayor = item;
        }
    }
    mayor
}
```

Podemos tener multiples parametros de tipo:

```rust
// La funcion es de tipo generico (T, U) por lo que recibe una referencia a un slice de T y otro valor de tipo U y devuelve una referencia a T
fn mayor<T, U>(lista: &[T], otro: U) -> &T {
    // ...
}
```

Por convencion se usan las letras mayusculas **T, U, V**

No solo podemos parametrizar funciones, tambien podemos parametrizar por ejemplo **structs**!:

```rust
struct Punto<T> {
    x: T,
    y: T,
}

let entero = Punto { x: 5, y: 10 }; // Siendo T = i32
let flotante = Punto { x: 1.0, y: 4.0 }; // Siendo T = f64
```

Pero ojo, como los atributos x e y son del mismo tipo generico T eso quiere decir que si o si deben ser del mismo tipo de dato, no podemos hacer por ejemplo:

```rust
let mixto = Punto { x: 5, y: 4.0 }; // Esto no compila porque T = i32 y T = f64, no puede ser ambos a la vez
```

Tambien podemos usar datos genericos en **enums**, de hecho ya lo veniamos usando en el **Option** y **Result** al hacer *Option\<T>* y *Result\<T, E>*, por ejemplo:

```rust
enum Option<T> { // El enum Option recibia el tipo de dato generico T
    Some(T),
    None,
}

enum Result<T, E> { // Recordemos que el enum Result devuelve el valor exitoso de tipo T o el error de tipo E
    Ok(T),
    Err(E),
}
```

Tambien podemos definir nuestro propio enum Result, por ejemplo:

```rust
enum MiResult<T, E> {
    Exito(T),
    Fallo(E),
}
```

Por otro lado, cuando implementamos metodos para un struct generico si o si tenemos que indicar el tipo generico en la implementacion (impl) por ejemplo:

```rust
struct Punto<T> {
    x: T,
    y: T,
}

impl<T> Punto<T> { // Indicamos que la implementacion es para el struct generico Punto<T>
    // El metodo x() toma una referencia a la propia instancia de Punto<T> y devuelve una referencia al atributo x de tipo T
    fn x(&self) -> &T {
        &self.x
    }
}
```

El *\<T>* despues del impl le dice a Rust que T es un parametro de tipo, no un tipo concreto. Esto nos permite usarlo en toda la implementacion (impl)

Tambien podemos implementar metodos para un struct generico pero solo para un tipo de dato concreto, por ejemplo:

```rust
struct Punto<T> {
    x: T,
    y: T,
}

impl Punto<f32> { // Indicamos que la implementacion es para el struct generico Punto<T> pero solo cuando T = f32, osea este metodo es valido solo para Punto<f32> pero no para Punto<i32> por ejemplo
    fn distancia_desde_origen(&self) -> f32 {
        // Sacamos la raiz cuadrada de la suma de los cuadrados de x e y, que son de tipo f32
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

Otro caso mas avanzado es que un metodo puede tener sus propios parametros de tipo distintos a los del struct generico, por ejemplo:

```rust
struct Punto<X1, Y2> {
  x: X1,
  y: Y2,
}

impl<X1, Y1> Punto<X1, Y1> { // Indicamos que la implementacion es para el struct generico Punto<X1, X2>

    // El propio metodo tiene sus propios parametros de tipo generico <X2, Y2> distintos a los del struct generico <X1, Y1>, y devuelve un nuevo Punto<X1, Y2> que tiene el x del primer punto y el y del segundo punto. Si nos fijamos retorna un punto con el mismo tipo de dato del primer punto (X1) y el mismo tipo de dato del segundo punto (Y2)
    fn mezclar<X2, Y2>(self, otro: Punto<X2, Y2>) -> Punto<X1, Y2>  {
        Punto {
            x: self.x,
            y: otro.y,
        }
    }
}
```

Aca el metodo *mezclar()* toma un *Punto\<X2, Y2>* y devuelve un *Punto\<X1, Y2>*, osea combina el *x* del primer punto con el *y* del segundo punto, y los tipos de datos pueden ser distintos.

Por ultimo respecto a los tipos de datos genericos esta el concepto de **monomorfizacion** y es algo particular y positivo de Rust por sobre otros lenguajes como C++ o Java.

En Java existen tambien los tipos de datos genericos pero a diferencia de Rust, en Java los tipos genericos son eliminados en tiempo de compilacion y reemplazados por Object, lo que significa que no se sabe el tipo de dato real en tiempo de ejecucion y se pierde informacion de tipo. Esto se llama **type erasure**.

En Rust el compilador hace Monomorfizacion, esto quiere decir que cuando se compila el codigo, el compilador genera una version concreta de la funcion o struct generico para cada tipo de dato que se use, por ejemplo si tenemos una funcion generica *mayor\<T>* y la usamos con *i32* y *f64*, el compilador genera dos versiones concretas de la funcion: *mayor_i32* y *mayor_f64*. Esto permite que el codigo sea mas eficiente y seguro, ya que se sabe el tipo de dato real en tiempo de ejecucion y no se pierde informacion de tipo.

Por ejemplo si usamos un struct de tipo generico *Punto\<T>* con *i32* y *f64*, el compilador genera dos versiones concretas del struct: *Punto_i32* y *Punto_f64*. Por ejemplo:

```rust
struct Punto<T> {
    x: T,
    y: T,
}

let entero = Punto { x: 5, y: 10 }; // El compilador genera una version concreta del struct Punto_i32. Osea generaria un "struct Punto_i32 { x: i32, y: i32 }"

let flotante = Punto { x: 1.0, y: 4.0 }; // El compilador genera una version concreta del struct Punto_f64, osea generaria un "struct Punto_f64 { x: f64, y: f64 }"
```

Luego el binario final tiene codigo especifico para cada tipo, **exactamente igual a si lo hubiesemos escrito a mano**. El costo en runtime es **CERO**, el unico costo es el tiempo de compilacion ya que justamente habria mas codigo que compilar algo que es logico, y tambien logicamente el tamaño del binario final generado.


