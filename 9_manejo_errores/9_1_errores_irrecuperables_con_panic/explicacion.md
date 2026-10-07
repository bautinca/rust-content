# Manejo de Errores
Rust tiene una serie de caracteristicas para manejar algo que sale mal. En muchos casos **Rust te obliga a reconocer la posibilidad de un error y tomar alguna accion antes de que el codigo se compile**. Este ultimo criterio hace que el programa sea mas robusto al garantizar que descubrira errores y los manejara adecuadamente antes de implementar el codigo en produccion.

Rust agrupa los errores en 2 categorias:

- **Errores Recuperables**: Por ejemplo un error de archivo no encontrado, lo mas probable es que solo queramos informar el problema al usuario y volver a intentar la operacion. En este caso, el programa puede continuar ejecutandose.

- **Errores Irrecuperables**: Siempre son sintomas de errores, como intentar acceder a una ubicacion mas alla del final de un arreglo, por lo que queremos detener inmediatamente el programa.

La mayoria de los lenguajes no distinguen entre estos 2 tipos de errores y los manejan de la misma manera utilizano las **excepciones**. **Rust no tiene excepciones**, en cambio tiene el tipo `Result` para errores recuperables y el macro `panic!` para errores irrecuperables.

## Errores Irrecuperables con `panic!`
El **panic!** es una macro que detiene la ejecucion del programa cuando ocurre un error **irrecuperable**. Cuando ocurre un panic, el programa imprime un mensaje de error y termina la ejecucion. Por ejemplo, si intentamos acceder a un indice fuera del rango de un arreglo, Rust generara un panic.

Entonces cuando ocurre un panic:

1. Imprime el mensaje de error
2. Deshace el stack
3. Termina repentinamente el programa

Por ejemplo podemos forzar un error irrecuperable lanzando un panic con el sig. codigo:

```rust
fn main() {
    panic!("Este es un error irrecuperable");
}
```

Obviamente el caso mas comun no es llamar a proposito a *panic!* sino que lo dispare la biblioteca estandar de Rust por ejemplo accediendo a un indice invalido de un vector:

```rust
fn main() {
    let v = vec![1, 2, 3];
    v[99];
}
```

En C esto seria valido, es decir, podriamos leer memoria de otro proceso completamente distinto cosa que seria una grave vulnerabilidad de seguridad. En Rust, en cambio, esto genera un panic y termina el programa.

Hay algo bueno en Rust que es el **backtrace** donde al ocurrir el panic la salida nos muestra toda una cadena de llamadas hasta donde ocurrio el error que origino el panic. La clave es **buscar la primera linea que mencione un archivo tuyo**, luego todo lo que esta por encima de esa linea es codigo interno de Rust o de bibliotecas de terceros. Para hacer un backtrace al momento de compilar el programa debemos setear la variable de entorno `RUST_BACKTRACE=1` y luego ejecutar el programa. Por ejemplo:

```bash
$ RUST_BACKTRACE=1 cargo run
```

Por otro lado, por defecto Rust hace **Unwiding**, es decir, al ocurrir el panic Rust recorre el stack hacia atras liberando memoria antes de terminar el programa. Esto es util para liberar recursos, pero es costoso en tiempo de ejecucion. Es por ello que si tenemos un binario mas pequeño y queremos que el programa termine inmediatamente sin liberar recursos podemos configurar el *Cargo.toml* asi:

```toml
[profile.release]
panic = 'abort'
```

Con **abort** lo que estamos diciendo es que cuando ocurra un panic Rust no hara *unwiding*, osea no liberara memoria y terminara inmediatamente el programa, sino que terminara el programa sin liberar recursos. Esto es util para binarios mas pequeños y con menos overhead de tiempo de ejecucion.

**¿Cuando usar panic!?** Las reglas son:

- Usar panic cuando el estado es **imposible de recuperar** y continuar seria un bug, violaciones de contratos, indices invalidos, invariantes rotos, etc.

- Usar result (luego lo veremos a detalle) cuando el error es **esperable** y el codigo que llama puede decidir que hacer. Fallo de red, archivo no encontrado, input invalido del usuario, etc.