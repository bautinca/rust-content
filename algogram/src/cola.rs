// De la libreria estandar obtenemos el modulo ptr que nos permite trabajar con punteros crudos al estilo de C
use std::ptr;

struct Nodo<T> {
  dato: T,
  proximo: Option<Box<Nodo<T>>>, // El Box es necesario para que el tamaño del nodo sea conocido en tiempo de compilación
                                // El box es un puntero inteligente que permite almacenar datos en el heap
}

pub struct Cola<T> {
  primero: Option<Box<Nodo<T>>>,
  ultimo: *mut Nodo<T>, // El puntero crudo es necesario para poder modificar el ultimo nodo de la cola
}

impl<T> Cola<T> {
  // Creamos cola vacia
  pub fn nueva() -> Self {
    Self {
      primero: None,
      ultimo: ptr::null_mut(), // Inicializamos el puntero crudo a null
    }
  }

  // Indica si la cola esta vacia
  pub fn esta_vacia(&self) -> bool {
    self.primero.is_none()
  }

  // Encolar un elemento al final de la cola
  pub fn encolar(&mut self, dato: T) {
    // Creamos el nuevo nodo con el dato generico
    let nuevo_nodo = Box::new(Nodo {
      dato: dato,
      proximo: None,
    });

    // Si el puntero al ultimo nodo es null significa
    // que la cola esta vacia, entonces el primer nodo de la cola sera
    // el nuevo nodo que acabamos de crear
    if self.ultimo.is_null() {
      // El primer y ultimo nodo de la cola sera el nuevo nodo que acabamos de crear
      self.primero = Some(nuevo_nodo);
      self.ultimo = self.primero.as_deref_mut().expect("El primer nodo deberia existir") as *mut Nodo<T>; // Obtenemos una referencia mutable al primer nodo de la cola y la convertimos a un puntero crudo
      
    } else {
      // La cola no esta vacia entonces el siguiente del ultimo nodo de la cola sera el nuevo nodo que acabamos de crear
      unsafe {
        (*self.ultimo).proximo = Some(nuevo_nodo); // Desreferenciamos el puntero crudo al ultimo nodo de la cola y le asignamos el nuevo nodo como su siguiente
        self.ultimo = (*self.ultimo).proximo.as_deref_mut().expect("El siguiente nodo deberia existir") as *mut Nodo<T>; // Obtenemos una referencia mutable al siguiente nodo del ultimo nodo de la cola y la convertimos a un puntero crudo
      }
    }
  }

  // Desencolar un elemento del frente de la cola
  pub fn desencolar(&mut self) -> Option<T> {
    // Tomamos el primer nodo de la cola
    let mut nodo_eliminado = self.primero.take()?;

    // Ahora el primero de la cola sera el siguiente del nodo que acabamos de eliminar
    self.primero = nodo_eliminado.proximo.take();

    // Si la lista queda vacia entonces el ultimo nodo de la cola sera null
    if self.primero.is_none() {
      self.ultimo = ptr::null_mut();
    }
    
    // Retornamos el dato del nodo que acabamos de eliminar
    Some(nodo_eliminado.dato)
  }

  // Vemos el primer elemento de la cola sin desencolarlo
  pub fn ver_primero(&self) -> Option<&T> {
    self.primero.as_ref().map(|nodo| &nodo.dato) // Devolvemos una referencia al dato del primer nodo
  }

  // Devolvemos la cantidad de elementos en la cola
  pub fn largo(&self) -> usize {
    let mut cantidad = 0;
    let mut actual = self.primero.as_deref();

    while let Some(nodo) = actual {
      cantidad += 1;
      actual = nodo.proximo.as_deref();
    }
    cantidad
  }
}

// ---------------- TESTS UNITARIOS ----------------

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn crear_cola_vacia() {
    let cola: Cola<i32> = Cola::nueva();
    assert!(cola.esta_vacia());
    assert_eq!(cola.largo(), 0);
  }

  #[test]
  fn encolar_cola() {
    let mut cola: Cola<i32> = Cola::nueva();
    cola.encolar(1);
    assert!(!cola.esta_vacia());
    assert_eq!(cola.largo(), 1);
    assert_eq!(cola.ver_primero(), Some(&1));
  }

  #[test]
  fn encolar_multiples_cola() {
    let mut cola: Cola<i32> = Cola::nueva();
    cola.encolar(1);
    cola.encolar(2);
    cola.encolar(3);
    assert_eq!(cola.largo(), 3);
    assert_eq!(cola.ver_primero(), Some(&1));
  }

  #[test]
  fn desencolar_cola() {
    let mut cola: Cola<i32> = Cola::nueva();
    cola.encolar(1);
    cola.encolar(2);
    assert_eq!(cola.desencolar(), Some(1));
    assert_eq!(cola.largo(), 1);
    assert_eq!(cola.ver_primero(), Some(&2));
    cola.desencolar();
    assert!(cola.esta_vacia());
  }

  #[test]
  fn desencolar_cola_vacia() {
    let mut cola: Cola<i32> = Cola::nueva();
    assert_eq!(cola.desencolar(), None);
  }
}



