Los traits **Send** y **Sync** son 2 traits fundamentales en Rust para la concurrencia y el paralelismo. Estos traits nos permiten determinar si un tipo de dato puede ser enviado a otro hilo o si puede ser compartido entre hilos de manera segura.

Si el tipo de dato tiene:

- **Send**: Significa que el tipo de dato puede ser enviado entre hilos. Esto es importante porque en Rust, por defecto, los tipos de datos no son compartidos entre hilos y para poder enviar un tipo de dato a otro hilo necesitamos que este implemente el trait Send. Por ejemplo , los tipos primitivos como i32, f64, bool, etc. implementan Send, mientras que los tipos que contienen referencias a datos no compartidos (como &T) no implementan Send.

- **Sync**: Significa que el tipo de dato puede ser referenciado desde varios hilos de manera segura. Esto es importante porque en Rust, por defecto, los tipos de datos no son compartidos entre hilos y para poder compartir un tipo de dato entre hilos necesitamos que este implemente el trait Sync. Por ejemplo, los tipos primitivos como i32, f64, bool, etc. implementan Sync, mientras que los tipos que contienen referencias a datos no compartidos (como &T) no implementan Sync.

La gran mayoria de los tipos de datos (como los primitivos) tienen estos traits implementados por defecto. Sin embargo las excepciones notables son:
|Tipo|Send|Sync|Porque|
|--|--|--|--|
|Rc|No|No|Porque el contador de referencias no es atomico, por ende no es thread-safe|
|RefCell|Si|No|La verificacion de acceso seguro no es thread-safe|
|Mutex|Si|Si|Usa sincronizacion interna|
|Arc|Si|Si|Contador atomico|

En resumidas cuentas si queremos compartir un recurso entre hilos de manera segura, debemos usar tipos de datos que implementen Send y Sync. Por ejemplo, si queremos compartir un contador entre hilos, podemos usar un Mutex envuelto en un Arc, ya que ambos implementan Send y Sync.

Este cuadro compartivo tambien es muy bueno donde basicamente relaciona todos los smart pointers que hemos visto y si son optimos para single-thread o multi-thread:

|Single-thread|Multi-thread|Para que|
|--|--|--|
|`Rc<T>`|`Arc<T>`|Multiple ownership|
|`RefCell<T>`|`Mutex<T>`|Interior mutability|
|`Rc<RefCell<T>>`|`Arc<Mutex<T>>`|Multiple ownership + Interior mutability|

Y es asi como terminamos de ver todos los smart pointers que son: `Box`, `Rc`, `RefCell`, `Arc` y `Mutex`. Cada uno de ellos tiene su propio uso y es importante entender cuando usar cada uno de ellos para poder escribir codigo seguro y eficiente en Rust.