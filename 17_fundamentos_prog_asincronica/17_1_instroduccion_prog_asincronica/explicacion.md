## ¿Porque es necesaria la prog. asincronica?
Resulta que hay operaciones que pueden tardar mucho tiempo en completarse, por ejemplo exportar un video, descargar un archivo grande, etc. Es decir, son operaciones con mucha carga I/O (Input/Output). Si estas operaciones se ejecutan de manera sincronica entonces si o si el programa en su totalidad se va a quedar bloqueado hasta que la operacion termine, lo cual no es deseable. Por eso es necesario que estas operaciones se ejecuten de manera **asincronica**, es decir, que se ejecuten en segundo plano y que el programa pueda seguir ejecutandose mientras estas operaciones se completan.

Existen 2 tipos de operaciones:

1. **Limitadas por CPU**: Son operaciones que requieren mucho procesamiento, como por ejemplo calcular el factorial de un numero muy grande. Estas operaciones se ejecutan de manera **sincronica** y no se pueden ejecutar en segundo plano.

2. **Limitadas por I/O**: Son de las que hablamos anteriormente, son operaciones que requieren mucho tiempo para completarse, como por ejemplo descargar un archivo grande. Estas operaciones se ejecutan de manera **asincronica** y pueden ejecutarse en segundo plano. Se las llama Input/Output porque requieren de la entrada y salida de datos, por ejemplo leer un archivo, escribir un archivo, descargar un archivo, etc.

Ante las operaciones bloqueantes uno puede pensar ¿Y no es mejor crear distintos hilos? Por ejemplo si tenemos una operacion de I/O no es mejor que esta corra en otro hilo aparte? Lo cierto es que se puede hacer pero en la practica no se hace ya que los hilos tienen un costo de memoria y de CPU, y si tenemos muchas operaciones bloqueantes entonces vamos a tener muchos hilos corriendo y eso va a hacer que el programa se vuelva lento. Por eso es mejor usar la **programación asincronica**, ya que permite ejecutar muchas operaciones bloqueantes en segundo plano sin necesidad de crear muchos hilos.

Rust para llevar a cabo la programacion asincronica dentro de un mismo hilo ofrece la abstraccion **async** y **await**.

## Concurrencia vs Paralelismo
De esta diferencia ya hablamos antes basicamente:

- **Concurrencia**: Es la capacidad de un programa de ejecutar varias tareas a modo alternado, es decir, que el programa puede ejecutar varias tareas y avanzar en todas ellas, pero no necesariamente al mismo tiempo.

- **Paralelismo**: Es la capacidad de un programa de ejecutar varias tareas al mismo tiempo, es decir, que el programa puede ejecutar varias tareas y avanzar en todas ellas al mismo tiempo.

La idea es que el codigo **async** en Rust **siempre es concurrente**, es decir, que puede ejecutar varias **tareas** a modo alternado, pero no necesariamente al mismo tiempo. Esto es porque el codigo **async** se ejecuta dentro de un mismo hilo, y un hilo solo puede ejecutar una tarea a la vez. Por lo tanto, el codigo **async** no es paralelo, ya que no puede ejecutar varias tareas al mismo tiempo.