// Definimos constante que seria la capacidad inicial del stack
const CAPACIDAD_INICIAL: usize = 50;
const FACTOR_REDIMENSION: usize = 2;
const FACTOR_REDUCCION: usize = 4;

// Definimos la estructura del stack/pila
pub struct Stack<T> {
  datos: Vec<T>, // En el fondo la pila es un vector, pero lo encapsulamos para que el usuario no tenga acceso directo a el y asi poder controlar la capacidad de la pila y su redimensionamiento
  capacidad: usize,
}

// Primitivas del stack/pila
impl<T> Stack<T> {

  // Constructor del stack
  pub fn crear() -> Self {
    Self {
      datos: Vec::with_capacity(CAPACIDAD_INICIAL), // La pila sera inicializada con una capacidad inicial de 50 elementos, osea reservamos memoria para 50 elementos. Recordar que al ser un vector esta memoria se reserva en el heap.
      capacidad: CAPACIDAD_INICIAL,
    }
  }

  // Si la pila esta vacia
  pub fn esta_vacia(&self) -> bool {
    self.datos.is_empty()
  }

  // Devolver referencia a elemento del tope de la pila
  pub fn tope(&self) -> Option<&T> {
    self.datos.last()
  }

  // Agregar elemento al tope de la pila, devuelve true si fue exitoso
  pub fn apilar(&mut self, valor: T) -> bool {
    // Si la pila esta llena, la redimensionamos
    if self.datos.len() == self.capacidad {
      let nueva_capacidad = self.capacidad * FACTOR_REDIMENSION;

      // Al obtener la nueva capacidad de la pila si sobrepasa la capacidad total
      // de memoria disponible que efectivamente tiene la pila ya reservada, si es asi entonces deberiamos reservar la cantidad de memoria exacta para la nueva capacidad de la pila
      if nueva_capacidad > self.datos.capacity() { // capacity devuelve la cantidad de elementos que puede contener el vector sin necesidad de redimensionar
        let memoria_necesaria = nueva_capacidad - self.datos.capacity();

        // Ahora reservamos la memoria necesaria para la nueva capacidad
        if self.datos.try_reserve_exact(memoria_necesaria).is_err() {
          return false; // Si no se pudo reservar la memoria, devolvemos false
        }
      }

      // La nueva capacidad de la pila se lo asignamos al atributo capacidad de la pila
      self.capacidad = nueva_capacidad;
    }

    // Ahora si con la nueva capacidad de la pila y la memoria reservada, agregamos el elemento al tope de la pila
    self.datos.push(valor);
    true
  }

  // Desapila y devuelve el elemento del tope desapilado
  pub fn desapilar(&mut self) -> Option<T> {
    // Si la pila esta vacia retornamos None
    if self.esta_vacia() {
      return None;
    }

    // La capacidad de la pila se reduce cuando la cantidad de elementos es menor o igual a un cuarto de la
    // capacidad inicial
    if self.datos.len() * FACTOR_REDUCCION <= self.capacidad && self.capacidad / FACTOR_REDIMENSION >= CAPACIDAD_INICIAL {
      // Si es asi entonces reducimos la capacidad de la pila por el factor de redimensionamiento
      let nueva_capacidad = self.capacidad / FACTOR_REDIMENSION;

      // Ahora reducimos la capacidad de la pila, el shrink_to() reduce la capacidad del vector a la cantidad de elementos que contiene, en este caso a la nueva capacidad de la pila
      self.datos.shrink_to(nueva_capacidad);
      self.capacidad = nueva_capacidad;
    }

    self.datos.pop()
  }

  // Devuelve la cantidad de elementos en la pila
  pub fn cantidad(&self) -> usize {
    self.datos.len()
  }

  // Devuelve la capacidad de la pila (ojo no confundir con cantidad, la cantidad son los elementos que efectivamente tiene la pila, mientras que la capacidad es la cantidad de elementos que puede contener la pila sin necesidad de redimensionar)
  pub fn capacidad(&self) -> usize {
    self.capacidad
  }
}

