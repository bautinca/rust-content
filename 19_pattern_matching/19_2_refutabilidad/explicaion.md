# Refutabilidad
Los patrones se dividen en 2 categorias:

- **Irrefutable**: Siempre coincide con cualquier valor posible, nunca puede fallar. Por ejemplo:

  ```rust
  let x = 5; // x siempre va a coincidir con cualquier cosa, es algo irrefutable
  ```

- **Refutable**: Puede coincidir o no con un valor, puede fallar. Por ejemplo:

  ```rust
  let (x, y) = (1, 2); // Este patron es refutable, si el lado derecho no es una tupla de 2 elementos fallara
  ```

La cuestion es que cada tipo acepta ciertos patrones y otros no. Un resumen seria:

| Contexto | Acepta |
|----------|--------|
| `let` | Solo irrefutables |
|Parametros de funcion|Solo irrefutables|
|Bucles for|Solo irrefutables|
|`if let`|Refutables (y avisa si ponemos irrefutables)|
|`while let`|Refutables (y avisa si ponemos irrefutables)|
|`match`|Refutables (excepto el ultimo brazo, que debe ser irrefutable)|

La logica es clara, solo `let`, `for` y parametros de funcion son irrefutables. No tienen rama alternativa, si el patron no coincide el programa **falla**. En cambio el `if let` y `while let` si tienen un `else` o simplemente no ejecutan el bloque (pueden manejar el fallo)

El error seria por ejemplo usar un patron refutable en un `let` por ejemplo al hacer algo asi:

```rust
let Some(x) = some_option; // Esto es un error, porque Some(x) es refutable y let espera un patron irrefutable
```

La cuestion es que `let` no puede hacer nada si el valor es en realidad `None`. El compliador de Rust incluso sugiere la solucion de en su lugar hacer un `let...else` por ejemplo:

```rust
let Some(x) = some_option else {
  return; // o cualquier otra accion que quieras hacer si no coincide
}
// Aca afuera x esta disponible y esta garantizado que es el valor dentor de Some
```

Por otro lado si por ejemplo usamos un patron irrefutable en un `if let` el compilador nos avisara que el patron es irrefutable y que podemos usar un `let` normal en su lugar:

```rust
if let x = 5 { // Esto es un error, porque x es irrefutable
  println!("x es {}", x);
}
```

Esto ultimo tiene sentido, si el patron siempre coincide, no tiene sentido usar un `if let`, podemos simplemente hacer un `let` normal.