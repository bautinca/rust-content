Los **tipos genericos**, **traits** y **lifetimes** son herramientas para **reducir duplicacion de codigo** sin sacrificar performance ni seguridad. Vamos a ver los 3 conceptos juntos ya que muchas veces se los usa juntos en la practica.

## Tipo de dato generico
Supongamos que tenemos el sig. codigo:

```rust
fn mayor_i32(lista: &[i32]) -> &i32 {
    let mut mayor = &lista[0];
    for item in lista {
        if item > mayor {
            mayor = item;
        }
    }
    mayor
}

fn mayor_char(lista: &[char]) -> &char {
    let mut mayor = &lista[0];
    for item in lista {
        if item > mayor {
            mayor = item;
        }
    }
    mayor
}
```

Mas alla de lo que hagan ambas funciones vemos que la estructura de ambas funciones es la **misma**. La unica diferencia es el tipo de dato que reciben y retornan. Es aca donde viene ideal el **tipo de dato generico** ya que estos tipos de datos nos permiten **abstraer sobre tipos**. Permiten escribir codigo que funciona para **cualquier tipo de dato** sin saber de antemano cual seria, por ejemplo:

```rust
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

El parametro **T** es un **placeholder** para un tipo de dato que sera especificado en tiempo de compilacion.

Cuando llamamos a la funcion *mayor()* con *&[i32]* entonces **T se convierte en i32** y cuando llamamos a la funcion *mayor()* con *&[char]* entonces **T se convierte en char**. Esto es lo que nos permite escribir codigo generico.

Una duda que surge es ¿Que onda la **performance** de los tipos de datos genericos? La respuesta es que **no hay perdida de performance** ya que el compilador de Rust genera una version de la funcion para cada tipo de dato que se use. Por ejemplo, si usamos la funcion *mayor()* con *&[i32]* y *&[char]* entonces el compilador generara 2 versiones de la funcion, una para cada tipo de dato. Esto es lo que se conoce como **monomorfizacion**.

## Traits
Los traits **definen las operaciones que puede hacer un tipo de dato**, seria muy equivalente a **INTERFAZ** en otros lenguajes. Por ejemplo implementamos el sig. trait *Resumen*:

```rust
pub trait Resumen {
    fn resumen(&self) -> String;
}
```

Entonces lo que logramos es que cualquier tipo de dato que tenga el trait/interfaz *Resumen* pueda usar la funcion *resumen()*. Por ejemplo:

```rust
pub struct Noticia {
    pub titulo: String,
    pub autor: String,
    pub contenido: String,
}

impl Resumen for Noticia { // Implementamos el trait Resumen para el struct Noticia por lo tanto el struct Noticia puede usar la funcion resumen()
    fn resumen(&self) -> String {
        format!("{}, por {}", self.titulo, self.autor)
    }
}
```

## Lifetimes
Es la novedad mas reciente de Rust y resuelven un problema muy especifico sobre las referencias. Cuando una funcion recibe referencias y devuelve una referencia ¿Cual es la referencia que devuelve? Veamos un ejemplo:

```rust
fn mas_larga<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

Como vemos la funcion recibe 2 referencias a strings y devuelve una referencia a string. El problema es que el compilador no sabe cual de las 2 referencias devolver, si x o y. Por lo tanto nos pide que le digamos cual es la referencia que vamos a devolver. Para eso usamos **lifetimes**.

La anotacion **'a** le dice al compilador: *"la referencia que devuelvo vive al menos tanto como la mas corta entre x e y"*. Sin esto el compilador no puede garantizar que la referencia devuelta sigue siendo valida despues de que la funcion termina. Es el clasico problema **use-after-free** que teniamos en el lenguaje C, osea el compilador justamente busca evitar esto.

Por ejemplo para entenderlo bien podemos ver el siguiente ejemplo:

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let resultado = mas_larga(string1.as_str(), string2);
    println!("La cadena mas larga es {}", resultado); // La referencia 'resultado' es valida ya que la referencia devuelta vive al menos tanto como la mas corta entre string1 y string2.
}

fn mas_larga<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

Como vemos en el ejemplo, la funcion *mas_larga()* recibe 2 referencias a strings y devuelve una referencia a string, pero no sabe si devolver la referencia x o y. La anotacion **'a** le dice al compilador que la referencia devuelta vive al menos tanto como la mas corta entre x e y. Por lo tanto el compilador puede garantizar que la referencia devuelta sigue siendo valida despues de que la funcion termina.

## En la practica
Como dijimos, en la practica se usan estos 3 conceptos juntos muchas veces. Por ejemplo, podemos tener una funcion generica que recibe un tipo de dato generico que implementa un trait y devuelve una referencia a ese tipo de dato generico con un lifetime especifico., por ejemplo una funcion real podria verse asi:

```rust
fn funcion<'a, T>(x: &'a str, y: &'a str, extra: T) -> &'a str
where
    T: Display,
{
    println!("{extra}");
    if x.len() > y.len() { x } else { y }
}
```