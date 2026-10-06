# Flujo de Control Conciso con if let y let else
Vamos a ver 2 herramientas muy utiles para no tener que escribir un match completo cuando solo nos interesa un caso

## if let
El if let es simplemente un azucar sintactico para hacer un match cuando solo nos interesa un caso, por ejemplo si no tuvieramos if let entonces tendriamos que hacer asi si solo nos interesa un caso del match

```rust
let x = Some(5);
match x {
    Some(5) => println!("Got a five!"),
    _ => (),
}
```

Esto ultimo es valido pero no es muy conciso, podemos hacer lo mismo con if let de la siguiente manera

```rust
let x = Some(5);
if let Some(5) = x {
    println!("Got a five!");
}
```

La sintaxis seria `if let PATRON = EXPRESION { BLOQUE }` donde el bloque se ejecutara si la expresion coincide con el patron, de lo contrario no hara nada.

La contra del if let es que perdemos exhaustividad, es decir, no estamos manejando todos los casos posibles, solo el que nos interesa. Esto puede ser peligroso si no tenemos cuidado. Si nos interesa bien manejar todos los casos entonces es mejor usar match y punto.

## if let con else
Vamos un paso mas alla, ahora al if let le podemos agregar un else, de esta manera podemos manejar el caso que nos interesa y el resto de los casos en un solo bloque de codigo, por ejemplo

```rust
let x = Some(5);
if let Some(5) = x {
    println!("Got a five!");
} else {
    println!("Not a five!");
}
```

El else seria equivalente al _ del match, es decir, se ejecutara si la expresion no coincide con el patron. Esto ultimo es exactamente lo mismo que hacerlo con el match de la siguiente manera:

```rust
let x = Some(5);
match x {
    Some(5) => println!("Got a five!"),
    _ => println!("Not a five!"),
}
```

## let...else: Para quedarnos con el camino mas feliz
El problema que resuelve es que a veces queremos hacer un calculo si hay un valor y devolver algo por defecto si no lo hay. Con if let esto se pone feo cuando el cuerpo es complejo porque una rama devuelve un vlaor y la otra retorna de la funcion y eso te obliga a anidar el resto de la logica dentro del if, veamos:

```rust
fn get_username(user: Option<User>) -> String {
    let username = if let Some(user) = user {
        user.name
    } else {
        "Guest".to_string()
    };
    username
}
```

Este ultimo codigo funciona pero no es muy legible ya que tuvimos que meter el codigo dentro del if , podemos hacer lo mismo con let...else de la siguiente manera:

```rust
fn get_username(user: Option<User>) -> String {
    let Some(user) = user else {
        return "Guest".to_string();
    };
    user.name
}
```

Esto ultimo es lo mismo que el codigo anterior pero es mucho mas legible. La sintaxis es patron a la izquierda y expresion a la derecha, si la expresion no coincide con el patron entonces se ejecutara el bloque de codigo del else, si coincide entonces se ejecutara el resto del codigo despues del let. Esto es muy util para quedarnos con el camino mas feliz y evitar anidar codigo dentro de un if.

La ventaja de esta ultima manera es que el "camino feliz" osea el flujo normal de la funcion, queda todo al mismo nivel de identacion sin anidar nada.
