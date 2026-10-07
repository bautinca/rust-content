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

Entonces usar el unwrap suele ser util para **ejemplos educativos** ya que seria innecesario siempre escribir el match para el manejo de errores