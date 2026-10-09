// Nodo de la lista, esta compuesto por un dato generico
// Y un puntero al siguiente nodo
struct Node<T> {
    pub dato: T,
    pub proximo: Option<Box<Node<T>>>, // El Box es necesario para que el tamaño del nodo sea conocido en tiempo de compilación
                                    // El box es un puntero inteligente que permite almacenar datos en el heap
}

pub struct List<T> {
    primero: Option<Box<Node<T>>>, // El head es un puntero al primer nodo de la lista
    pub largo: usize,
}

impl <T> List<T> {

  // Constructor creamos lista vacia
  pub fn nueva() -> Self {
    Self {
      primero: None,
      largo: 0,
    }
  }

  // Indica si la lista esta vacia
  pub fn esta_vacia(&self) -> bool {
    self.largo == 0
  }

  // Devuelve la cantidad de elementos en la lista
  pub fn largo(&self) -> usize {
    self.largo
  }

  // Inserta un elemento al inicio de la lista
  pub fn insertar_inicio(&mut self, dato: T) {
    let nuevo_nodo = Box::new(Node {
      dato: dato,
      proximo: self.primero.take(), // El siguiente del nuevo nodo es el nodo que actualmente es el head de la lista
    });

    self.primero = Some(nuevo_nodo); // El head de la lista ahora es el nuevo nodo
    self.largo += 1; // Incrementamos el largo de la lista
  }

  // Inserta un elemento al final de la lista
  pub fn insertar_final(&mut self, dato: T) {
    let nuevo_nodo = Box::new(Node {
      dato: dato,
      proximo: None, // El siguiente del nuevo nodo es None porque es el ultimo nodo de la lista
    });

    // El as_mut nos permite obtener una referencia mutable al nodo actual para poder modificarlo
    match self.primero.as_mut() {
      None => {
        self.primero = Some(nuevo_nodo); // Si la lista esta vacia, el head de la lista es el nuevo nodo
      }

      // Si la lista no esta vacia tenemos que recorrerla hasta el final para insertar el nuevo nodo
      Some(mut nodo_actual) => {
        while let Some(ref mut siguiente) = nodo_actual.proximo {
          nodo_actual = siguiente; // Avanzamos al siguiente nodo
        }
        nodo_actual.proximo = Some(nuevo_nodo); // El siguiente del ultimo nodo es el nuevo nodo
      }
    }

    self.largo += 1;
  }

  // Eliminamos primer nodo devuelve el dato del nodo eliminado
  pub fn borrar_primero(&mut self) -> Option<T> {
    let mut nodo = self.primero.take()?; // Tomamos el nodo actual y lo eliminamos de la lista

    // Actualizamos el head de la lista al siguiente nodo
    self.primero = nodo.proximo.take();

    // Decrementamos el largo de la lista
    self.largo -= 1;

    // Ahora si devolvemos el dato del nodo eliminado
    Some(nodo.dato) // Recordar que tenemos que retornar un Some() ya que el nodo eliminado es un Option<T>
  }

  // Vemos el primer nodo de la lista sin eliminarlo
  pub fn ver_primero(&self) -> Option<&T> {
    self.primero.as_ref().map(|nodo| &nodo.dato) // Devolvemos una referencia al dato del primer nodo
  }

  // Vemos el ultimo nodo de la lista sin eliminarlo
  pub fn ver_ultimo(&self) -> Option<&T> {
    // Si la lista esta vacia, no hay ultimo nodo
    if self.primero.is_none() {
      return None;
    }

    let mut actual = self.primero.as_deref()?; // El as_deref nos permite obtener una referencia al nodo actual para poder acceder a sus datos

    while let Some(siguiente) = actual.proximo.as_deref() {
      actual = siguiente;
    }

    // Si salimos del while es porque actual es el ultimo nodo de la lista
    // Es decir, no tendria sentido hacer un actual.proximo.as_deref() ya estando en el ultimo nodo ya que no se puede referenciar a un nodo que no existe
    Some(&actual.dato) // Devolvemos una referencia al dato del ultimo nodo
  }

  // Devolvemos una referencia al elemento ubicado en el 'indice'
  pub fn obtener(&self, indice: usize) -> Option<&T> {
    // Si el indice es mas que el largo de la lista entonces es invalido
    if indice >= self.largo {
      return None;
    }

    // Obtenemos referencia al primer nodo de la lista
    let mut actual = self.primero.as_deref();

    // Recorremos la lista hasta llegar al nodo en el indice deseado
    for _ in 0..indice {
      actual = actual?.proximo.as_deref();
    }

    // Ya llegados al nodo en el indice deseado devolvemos una referencia al dato del nodo
    actual.map(|nodo| &nodo.dato)
  }

  // Para borrar un nodo en un indice especifico
  pub fn borrar_en(&mut self, indice: usize) -> Option<T> {
    // Si el indice es mas que el largo de la lista entonces retornamos None
    if indice >= self.largo {
      return None
    }

    // Si el indice es 0 entonces seria equivalente a borrar el primer nodo
    if indice == 0 {
      return self.borrar_primero();
    }

    // Obtenemos referencia al primer nodo de la lista y lo llamamos anterior
    let mut anterior = self.primero.as_mut()?;

    // Recorremos la lista hasta llegar al nodo en el indice-1 deseado
    for _ in 0..(indice - 1) {
      anterior = anterior.proximo.as_mut()?;
    }

    // Ahora anterior es el nodo en el indice-1, por lo que el nodo a borrar es el siguiente
    let mut nodo_a_borrar = anterior.proximo.take()?; // Tomamos el nodo a borrar y lo eliminamos de la lista
    // Ahora el siguiente del nodo anterior es el siguiente del nodo a borrar
    anterior.proximo = nodo_a_borrar.proximo.take();

    self.largo -= 1;

    // Devolvemos el dato del nodo eliminado
    Some(nodo_a_borrar.dato)
  }

  // Insertamos un nodo en un indice especifico
  pub fn insertar_en(&mut self, indice: usize, dato: T) -> bool {
    // Si el indice es mas largo que el largo de la lista entonces es invalido
    if indice > self.largo {
      return false;
    }

    // Si el indice es 0 entonces seria equivalente a insertar al inicio de la lista
    if indice == 0 {
      self.insertar_inicio(dato);
      return true;
    }

    // Tomamos referencia al primer nodo de la lista
    let mut anterior = match self.primero.as_mut() {
      Some(nodo) => nodo,
      None => return false, // Si la lista esta vacia y el indice es mayor a 0 entonces es invalido
    };

    // Recorremos la lista hasta llegar al nodo en el indice-1 deseado
    for _ in 0..(indice - 1) {
      anterior = match anterior.proximo.as_mut() {
        Some(nodo) => nodo,
        None => return false, // Si llegamos a un nodo que no tiene siguiente entonces el indice es invalido
      };
    }

    // Creamos el nuevo nodo
    let nuevo_nodo = Box::new(Node {
      dato: dato,
      proximo: anterior.proximo.take(), // El siguiente del nuevo nodo es el siguiente del nodo anterior
    });

    // Ahora el proximo del anterior es el nuevo nodo
    anterior.proximo = Some(nuevo_nodo);

    self.largo += 1;
    true
  }
}

// ---------------- TESTS UNITARIOS ----------------

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn crear_lista_vacia() {
    let lista: List<i32> = List::nueva();
    assert!(lista.esta_vacia());
    assert_eq!(lista.largo(), 0);
  }

  #[test]
  fn insertar_inicio_lista() {
    let mut lista: List<i32> = List::nueva();
    lista.insertar_inicio(1);
    assert!(!lista.esta_vacia());
    assert_eq!(lista.largo(), 1);
    assert_eq!(lista.ver_primero(), Some(&1));
    assert_eq!(lista.ver_ultimo(), Some(&1));
  }

  #[test]
  fn insertar_final_lista() {
    let mut lista: List<i32> = List::nueva();
    lista.insertar_final(1);
    assert!(!lista.esta_vacia());
    assert_eq!(lista.largo(), 1);
    assert_eq!(lista.ver_primero(), Some(&1));
    assert_eq!(lista.ver_ultimo(), Some(&1));
  }

  #[test]
  fn insertar_multiples_lista() {
    let mut lista: List<i32> = List::nueva();
    for i in 0..100 {
      lista.insertar_final(i);
    }
    assert_eq!(lista.largo(), 100);
    assert_eq!(lista.ver_primero(), Some(&0));
    assert_eq!(lista.ver_ultimo(), Some(&99));
  }

  #[test]
  fn borrar_primero_lista() {
    let mut lista: List<i32> = List::nueva();
    lista.insertar_final(1);
    lista.insertar_final(2);
    assert_eq!(lista.borrar_primero(), Some(1));
    assert_eq!(lista.largo(), 1);
    assert_eq!(lista.ver_primero(), Some(&2));
    lista.borrar_primero();
    assert!(lista.esta_vacia());
  }

  #[test]
  fn borrar_en_lista() {
    let mut lista: List<i32> = List::nueva();
    lista.insertar_final(1);
    lista.insertar_final(2);
    lista.insertar_final(3);
    assert_eq!(lista.borrar_en(1), Some(2));
    assert_eq!(lista.largo(), 2);
    assert_eq!(lista.ver_primero(), Some(&1));
    assert_eq!(lista.ver_ultimo(), Some(&3));
  }

  #[test]
  fn insertar_en_lista() {
    let mut lista: List<i32> = List::nueva();
    lista.insertar_final(1);
    lista.insertar_final(3);
    assert!(lista.insertar_en(1, 2));
    assert_eq!(lista.largo(), 3);
    assert_eq!(lista.ver_primero(), Some(&1));
    assert_eq!(lista.ver_ultimo(), Some(&3));
    assert_eq!(lista.obtener(1), Some(&2));
  }
}