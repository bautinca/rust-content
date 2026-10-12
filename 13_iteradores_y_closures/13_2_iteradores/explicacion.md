# Iteradores
Un iterador es un objeto que produce una secuencia de valores uno a la vez. La caracteristica clave en Rust es que son **LAZY**, es decir, no producen todos los valores de una vez, sino que los generan bajo demanda. Esto permite trabajar con secuencias potencialmente infinitas y optimizar el uso de memoria. Crear un iterador no itera, solo los prepara para ser iterados.

Todo iterador implementa el trait **Iterator** que es un trait de la libreria estandar. Veamos como esta definido el trait en la libreria estandar:

```rust
pub trait Iterator {
  type Item; // Tipo de cada elemento
  fn next(&mut self) -> Option<Self::Item>; // Unico metodo obligatorio

  //...
  // Muchos otros metodos
}
```

El metodo importante es **next()** ya que si nos fijamos devuelve un enum **Option** que puede ser **Some(valor)** si hay un valor disponible o **None** si no hay mas valores. Esto es lo que permite que los iteradores sean infinitos, ya que no sabemos cuantos valores van a producir. Es todo lo que necesitamos implementar para tener un iterador completo, el resto de los metodos (map, filter, sum, etc.) vienen gratis.

## Llamar a next() manualmente
Veamos el siguiente ejemplo:

```rust
let v1 = vec![1, 2, 3];
let mut v1_iter = v1.iter(); // Creamos un iterador a v1. Necesita ser si o si mut ya que el metodo next() le modifica el estado interno. Si nos fijamos el metodo next() recibe una referencia mutable a self.

// Y ahora si podemos iterar manualmente sobre el iterador obteniendo cada elemento del vector con el metodo next()
assert_eq!(v1_iter.next(), Some(&1)); // Recordar que el metodo next() devuelve un Option por eso usamos Some()
assert_eq!(v1_iter.next(), Some(&2));
assert_eq!(v1_iter.next(), Some(&3));
assert_eq!(v1_iter.next(), None); // Ya no hay mas elementos, devuelve None
```

Entonces el **for** que vemos en un iterador es solo un atajo para llamar a **next()** hasta que devuelva **None**.

Existen 3 formas de crear iteradores desde colecciones en Rust:

```rust
v.iter() // Iterador que devuelve referencias a los elementos. Es el que acabamos de ver en el ejemplo anterior.

v.iter_mut() // Iterador que devuelve referencias mutables a los elementos. Permite modificar los elementos del vector.

v.into_iter() // Iterador que devuelve los elementos con ownership. Esto significa que el vector original ya no puede ser usado despues de llamar a este metodo, ya que los elementos han sido movidos.
```

## 2 Categorias de metodos

Exsiten 2 categorias de metodos iteradores que son los siguientes:

1. **Adaptadores Consumidores**: Son metodos que consumen el iterador y devuelven un valor final. Por ejemplo **sum()**, **collect()**, **fold()**, etc. Una vez que se llama a un metodo consumidor, el iterador ya no puede ser usado nuevamente. Osea son metodos que internamente llaman a **next()** hasta que devuelve **None** y devuelven un valor final.

    ```rust
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum(); // Consumidor que devuelve la suma de todos los elementos del iterador. Internamente el sum() llama a next() hasta que devuelve None y devuelve la suma de todos los elementos.

    // A partir de este punto ya no podemos usar v1_iter ya que fue consumido por sum()
    
    assert_eq!(total, 6);
    ```
  
2. **Adaptadores de Iterador**: No consumen el iterador, sino que devuelven un nuevo iterador. Son tambien lazy los nuevos iteradores, no hacen nada hasta qeu algo los consume. Por ejemplo:

    ```rust
    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let v2_iter = v1_iter.map(|x| x + 1); // Devuelve un nuevo iterador que devuelve los elementos del iterador original incrementados en 1.
    ```

    Ahora bien, el nuevo iterador **v2_iter** no hace nada hasta que se consuma, por ejemplo con un collect() o un sum(). Por ejemplo:

    ```rust
    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let v2_iter = v1_iter.map(|x| x + 1);

    let v2: Vec<i32> = v2_iter.collect(); // Ahora si se ejecuta el map() y se devuelve un vector con los elementos incrementados en 1.
    assert_eq!(v2, vec![2, 3, 4]);

    // Cabe recalcar que v2_iter fue consumido por collect() y ya no puede ser usado nuevamente. En cambio v1_iter no fue consumido y puede ser usado nuevamente.
    ```

    Los adaptadores de iterador mas usados son generalmente los siguientes:

    ```rust
    // map()
    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let v2_iter = v1_iter.map(|x| x + 1);
    let v2: Vec<i32> = v2_iter.collect();
    assert_eq!(v2, vec![2, 3, 4]);

    // filter() -> Se queda con los elementos que cumplan la condicion del closure
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let v1_iter = v1.iter();
    let v2_iter = v1_iter.filter(|x| *x % 2 == 0); // Se queda con los elementos pares
    let v2: Vec<i32> = v2_iter.collect();
    assert_eq!(v2, vec![2, 4]);
    ```

    Veamos otro ejemplo de filter() un poco mas rico donde ademas usamos conceptos de closures

    ```rust
    #[derive(PartialEq, Debug)] // Para poder usar assert_eq!() necesitamos implementar PartialEq y Debug
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect() // El closure de filter captura shoe_size del entorno. into_iter() consume el Vec<Shoe> original y devuelve un iterador que devuelve los elementos con ownership. filter() devuelve un nuevo iterador que se queda con los elementos que cumplen la condicion del closure. collect() consume el iterador y devuelve un Vec<Shoe> con los elementos filtrados.
    }
    ```

    Tambien podemos encadenar varios adaptadores de iterador para finalmente devolver un adaptador de consumidor. Por ejemplo:

    ```rust
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    
    let resultado = v1
    .iter() // Primero obtenemos un iterador que devuelve referencias a los elementos del vector (si fuera un into_iter devuelve los elementos con ownership)
    .filter(|&&x| x % 2 == 0) // Serian referencias de referencias entonces, por eso usamos &&x para desreferenciar dos veces y obtener el valor original. Nos quedamos con los elementos pares
    .map(|&x| x * x) // Elevamos al cuadrado cada elemento
    .collect(); // Finalmente consumimos el iterador y devolvemos un Vec<i32> con los elementos filtrados y elevados al cuadrado.
    
    assert_eq!(resultado, vec![4, 16, 36]);
    ```

## Alternativa ejemplo minigrep
Hay una funcion en nuestra aplicacion minigrep que es la funcion publica **search()** que busca la query en el texto y devuelve un vector con las lineas que contienen la query. La implementacion original de esta funcion es la siguiente:

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }
  results
}
```

Si nos fijamos usamos un bucle comun y corriente **for**. Sin embargo tambien se puede resolver con iteradores y lo mejor de todo es que la **performance es la misma**. Veamos como se veria la funcion search() usando iteradores:

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

Vemos que la implementacion con iteradores es mucho mas concisa y legible. Ademas, si nos fijamos, el iterador es lazy, es decir, no se ejecuta hasta que se llama a **collect()**. Esto significa que si el vector de resultados es muy grande, no se van a generar todos los resultados de una vez, sino que se van a ir generando bajo demanda.