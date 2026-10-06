// A continuacion definiremos en Rust un programa para calcular el area de un rectangulo
// usando structs

struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  // Instanciamos la clase Rectangle
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!("The area of the rectangle is {} square pixels.", area(&rect1));
}

// Funcion que calcula el area dado un objeto rectangulo
fn area(rectangle: &Rectangle) -> u32 { // Recibe referencia a instancia Rectangle y devuelve un u32
  rectangle.width * rectangle.height
}

























