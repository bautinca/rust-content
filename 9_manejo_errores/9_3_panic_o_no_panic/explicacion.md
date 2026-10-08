# Panic o no Panic?
Si el error podria ser manejado por quien llamo a la funcion normalmente devuelve un **Result**. En cambio si se rompio alguna condicion que hace que el programa este en estado invalido y no tiene sentido continuar entonces **panic**.

Recordemos que el panic hace que el programa deje de ejecutarse normalmente, en cambio el Result permite que el programa continue ejecutandose y que el error sea manejado por quien llamo a la funcion.

Pero ojo, la diferencia no esta solo en **error grave vs error leve**. La cuestion es ¿Quien deberia decidir que hacer ante el error? Es por ello que **es recomendable devolver Result suele ser buena opcion predeterminada cuando una funcion puede fallar**. Esto permite que quien llame a la funcion decida que hacer ante el error.

Entonces nunca deberiamos usar panic!? No hay algunas situaciones que es preferible por ejemplo:

- En ejemplos
- En prototipos
- En tests

El unwrap() y expect() suelen ser muy usados en ejemplos y prototipos. Ya vimos que por ejemplo el unwrap:

```rust
let archivo = File::open("archivo.txt").unwrap();

// Es lo mismo que...
let archivo = match File::open("archivo.txt") {
    Ok(archivo) => archivo,
    Err(error) => panic!(),
};
```

Entonces usar el unwrap suele ser util para **ejemplos educativos** ya que seria innecesario siempre escribir el match para el manejo de errores. Por ejemplo si en un ejemplo tenemos:

```rust
let archivo = File::open("archivo.txt").unwrap();
```

Es mas comun hacerle un unwrap() que escribir el match completo. Esto es porque en ejemplos educativos no nos interesa el manejo de errores, sino mostrar como abrir un archivo.

Es por ello que es aceptable usar unwrap() y expect() en ejemplos.

Tambien el unwrap() y expect() son muy usados en **prototipos**, por ejemplo supongamos que estamos desarrollando algo rapidamente y no nos interesa por el momento manejar los errores del programa, entonces podriamos usar unwrap() y expect() para que el programa se detenga si ocurre un error. Esto nos permite enfocarnos en la logica principal del programa sin preocuparnos por el manejo de errores.

Luego mas adelante cuando tengamos claro como deberia comportarse el programa ahi si reemplazamos los unwrap() y expect() por un manejo de errores mas adecuado.

Y en los **TESTS**? Hasta ahora no hablamos nada de tests y mas adelante lo hablaremos, pero los unwrap() y expect() tambien son muy usados en tests. Esto es porque si un test falla, queremos que el programa se detenga y nos muestre el error. Por ejemplo si tenemos un test que verifica que una funcion devuelve un valor correcto, y esa funcion devuelve un error, entonces queremos que el test falle y nos muestre el error.

Por ejemplo supongamos que tenemos el sig. test:

```rust
#[test]
fn test_sumar() {
    let resultado = sumar(2, 3).unwrap();
    assert_eq!(resultado, 5);
}
```

Si la funcion sumar() devuelve un Err entonces queremos que todo el test falle, por lo tanto si a proposito queremos que falle ahi si tiene sentido usar unwrap() o expect() ya que lanzan el panic en caso de error, osea logicamente no queremos que los tests digan *"bueno ocurrio el error pero queremos continuar"*

Por otro lado, cambiando de tema, puede suceder que una operacion tenga tecnicamente la posibilidad de fallar pero nosotros tenemos la exactitud de que no va a fallar, por ejemplo:

```rust
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");
```

aqui estamos usando expect() porque sabemos que la direccion IP es valida y no va a fallar NUNCA, entonces si por alguna razon falla, queremos que el programa se detenga y nos muestre el error. El parse() devuelve siempre un Result ya que puede fallar o no, pero bueno, aca tenemos la exactitud de que nunca va a fallar.

Ahora bien, pudimos usar expect() sin problemas ya que tenemos la exactitud de que siempre va a funcionar ya que esta 'hardcoded', pero si el parametro lo ingresa el usuario? 

```rust
use std::net::IpAddr;

let ip_usuario: IpAddr = input_usuario.parse()?; 
```

En este caso no podemos usar expect() ya que el usuario puede ingresar cualquier cosa y por lo tanto puede fallar, entonces en este caso es mejor usar el operador ? para propagar el error y que quien llame a la funcion decida que hacer ante el error. El usuario tranquilamente podria ingresar una string que es una IP valida y otra que no lo es, por ejemplo "asd", "132sk", etc. Por lo tanto si o si tenemos que manejar el Result

Entonces en resumen **se recomienda usar panic! cuando continuar ejecutando el programa podria dejarlo en un estado incorrecto**. Un estado incorrecto puede significar que se rompio una:

- Suposicion
- Garantia
- Condicion
- Contrato
- Invariante

Por ejemplo imaginemos que tenemos una funcion donde por logica de negocio siempre recibe >= 1 entonces si alguien intenta llamarla con 0, entonces es mejor que el programa se detenga y nos muestre el error ya que no tiene sentido continuar ejecutando el programa en ese estado, por lo tanto es util el panic!

O con un contrato roto lo podemos imaginar como una **promesa** que hace una funcion, por ejemplo si una funcion promete que siempre devuelve un valor positivo, entonces si alguien la llama y devuelve un valor negativo, entonces es mejor que el programa se detenga y nos muestre el error ya que no tiene sentido continuar ejecutando el programa en ese estado.

Cuando **intentamos acceder por fuera de los limites** por defecto la biblioteca estandar de Rust hace un **panic!**, por ejemplo:

```rust
let numeros = vec![1, 2, 3];
let x = numeros[99]; // Esto hace panic! ya que estamos accediendo a un indice que no existe
```

Hace un panic! por defecto ya que acceder a memoria que no pertenece a la estructura seria un grave problema de seguridad por lo tanto no tiene sentido seguir ejecutando el programa en ese estado, esto ultimo seria una violacion de contrato.

Sin embargo ojo, **no todo dato invalido deberia provocar un panic!** ya que por ejemplo supongamos que le pedimos email al usuario y sin querer ingresa numeros, esta confusion no deberia porque producir un panic! ya que el usuario tranquilamente se podria haber equivocado, por lo tanto en esos casos es mejor devolver un **Result** y que quien llame a la funcion decida que hacer ante el error.

Entonces para **problemas esperados** normalmente se usa **Result** en cambio para **problemas inesperados** normalmente se usa **panic!**

Una restriccion muy buena es elegir el **sistema de tipos**, es decir, si una funcion no deberia recibir un valor invalido, entonces podemos usar el sistema de tipos para que la funcion solo reciba valores validos y asi evitar el panic! por ejemplo:

```rust
fn procesar(numero: u32) {
    // ...
}
```

Lo que logramos es que la funcion procesar() solo puede recibir valores positivos por su tipo de dato *u32*, entonces el compilador ya nos ayuda a garantizar esa condicion.

Supongamos que estamos programando un juego de adivinanza que siempre esta entre *1 <= guess <= 100* entonces lo que podriamos hacer es simplemente usar un if asi:

```rust
if guess < 1 || guess > 100 {
    panic!("El numero debe estar entre 1 y 100");
}
```

Ahora imaginemos que tneemos muchas funciones que trabajan con numeros entre 1 y 100 entonces en todas esas funciones deberiamos repetir ese bloque if en muchos lugares, esto es molesto y ademas aumenta la posibilidad de errores

Lo optimo seria crear un nuevo tipo de dato que solo pueda contener valores entre 1 y 100, entonces el compilador nos ayudaria a garantizar esa condicion y no tendriamos que repetir el bloque en muchos lugares. Por ejemplo:

```rust
// Definimos el tipo de dato nuevo por lo tanto logicamente necesitamos un struct para definirlo
pub struct Guess {
    value: u32,
}

// Ahora le implementamos un constructor que se encargue de validar el valor y crear la instancia del struct
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("El numero debe estar entre 1 y 100");
        }
        Guess { value } // Retornamos la instancia del struct con el valor ya validado
    }
}
```

Entonces listo, ya no tenemos que preocuparnos por validar el valor en cada funcion que usemos, ya que el compilador nos garantiza que el valor siempre va a estar entre 1 y 100. Por lo tanto ahora podemos usar el tipo de dato Guess en nuestras funciones y no tendremos que preocuparnos por validar el valor en cada funcion.

Porque usamos el *panic!* dentro del constructor? Porque justamente el tipo de dato Guess tiene un **contrato** que dice que el valor siempre debe estar entre 1 y 100, entonces si alguien intenta crear una instancia de Guess con un valor fuera de ese rango, estamos rompiendo el contrato y por lo tanto es mejor que el programa se detenga y nos muestre el error.

```rust
Guess::new(50); // Esto funciona bien
Guess::new(150); // Esto hace panic! ya que estamos rompiendo el contrato del tipo de dato Guess
```

Otra cosa que nos fijamos es que:

```rust
pub struct Guess { // El struct es publico por lo tanto cualquiera puede crear una instancia de Guess
    value: u32, // Sin embargo el campo value es PRIVADO por lo tanto nadie puede modificarlo directamente, solo puede ser modificado a traves del constructor new() que valida el valor
}
```

Lo colocamos a proposito en privado ya que esto evita que desde fuera alguien haga algo como:

```rust
let guess = Guess { value: 150 }; // Esto rompe el contrato del tipo de dato Guess ya que estamos creando una instancia con un valor fuera de rango
```

La forma correcta de crear una instancia de Guess es usando el constructor new() que valida el valor y garantiza que siempre va a estar entre 1 y 100.

```rust
let guess = Guess::new(50); // Esto funciona bien
let guess = Guess::new(150); // Esto hace panic! ya que estamos rompiendo el contrato del tipo de dato Guess
```

Es decir, si queremos instanciar Guess si o si tenemos que usar el constructor new() que valida el valor y garantiza que siempre va a estar entre 1 y 100.

Esto ultimo que hicimos **elimina comprobaciones repetidas**, entonces por ejemplo en una funcion podemos hacer que reciba el tipo de dato Guess y estar seguros de que el valor siempre va a estar entre 1 y 100 sin necesidad de validar el valor en cada funcion:

```rust
fn procesar(guess: Guess) {
    // Aqui podemos estar seguros de que el valor siempre va a estar entre 1 y 100 sin necesidad de validar el valor en cada funcion
}   
```

Es decir, si recibe un tipo de dato Guess sabemos que ese tipo de dato fue creado con el constructor new() y a su vez sabemos que dicho constructor valida el valor entre 1 y 100

