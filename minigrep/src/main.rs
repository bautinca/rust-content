use std::env; // De la libreria estandar de Rust importamos
              // el modulo env que nos permite acceder a las variables de entorno y argumentos de la linea de comandos

use std::fs; // De la libreria estandar de Rust importamos
              // el modulo fs que nos permite acceder al sistema de archivos

use std::process; // De la libreria estandar de Rust importamos
              // el modulo process que nos permite acceder a las funciones del sistema operativo

use std::error::Error; // De la libreria estandar de Rust importamos
              // el modulo error que nos permite manejar errores

use minigrep::search; // Importamos la funcion search del modulo minigrep
use minigrep::search_case_insensitive; // Importamos la funcion search_case_insensitive del modulo minigrep

// Funcion que ejecuta la logica principal de la aplicacion
// Basicaente lo que hace es leer el contenido del archivo y buscar el texto en el contenido del archivo
fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents: String = fs::read_to_string(config.filename)?; // Leemos el contenido del archivo y lo guardamos en una variable. Usasmos el ? ya que propaga el error hacia arriba
  
  // Si la variable de entorno IGNORE_CASE esta seteada, entonces hacemos una busqueda case insensitive, si no hacemos una busqueda case sensitive
  let results = if config.ignore_case {
    search_case_insensitive(&config.query, &contents)
  } else {
    search(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }

  Ok(())
}

// Struct que representa la configuracion de la aplicacion, es decir, el texto a buscar y el nombre del archivo donde buscar
struct Config {
  query: String,
  filename: String,
  ignore_case: bool,
}

// Implementamos constructores para el struct Config
impl Config {
  /// Crea una nueva instancia de Config a partir de los argumentos de la línea de comandos.
  fn build(args: &Vec<String>) -> Result<Config, &'static str> {
    // Si no hay suficientes argumentos lanzamos un panic
    if args.len() < 3 {
      return Err("No se pasaron suficientes argumentos. Uso: minigrep <texto_a_buscar> <archivo>");
    }

    let query = args[1].clone(); // El texto a buscar
    let filename = args[2].clone(); // El nombre del archivo donde buscar
    let ignore_case = env::var("IGNORE_CASE").is_ok(); // Si la variable de entorno IGNORE_CASE esta seteada, entonces ignore_case sera true, sino sera false
    Ok(Config { query: query, filename: filename, ignore_case: ignore_case })
  }
}




// Punto de entrada de la aplicacion
fn main() {
  // Obtenemos los argumentos de CLI y los guardamos en un vector de Strings
  let args: Vec<String> = env::args().collect();

  // Importante es que siempre args[0] es el nombre del binario, en este caso seria minigrep, por lo que el primer argumento que le pasemos a la aplicacion sera args[1] y el segundo argumento sera args[2]
  // Los argumentos reales siempre arrancan a partir de args[1]

  // Entonces si por ejemplo por CLI hacemos: 'cargo run -- pepito archivo.txt' entonces args[1] sera 'pepitito' y args[2] sera 'archivo.txt'
  // entonces el vector de strings quedaria asi: args = ["minigrep", "pepitito", "archivo.txt"]

  // Creamos una instancia de Config con los argumentos de CLI para parsear los argumentos y guardarlos en un struct
  // El unwrap_or_else recibe un closure (funcion anonima) donde si el Result es Ok devuelve el valor interior
  // y si es Err ejecuta la closure con el error process::exit(1) donde termina el programa con codigo de salida 1 sin el ruido de panic!
  let config: Config = Config::build(&args).unwrap_or_else(|err| {
    eprintln!("Error al construir la configuración: {}", err);
    process::exit(1); // Si hay un error al construir la configuracion, mostramos un mensaje de error y terminamos la ejecucion del programa finalizando el proceso con un codigo de salida 1
  });

  // Colocamos algunos logs
  println!("Buscando el texto '{}' en el archivo '{}'", config.query, config.filename);

  // Sabemos que al hacer run(config) nos da un Result, si es Ok no hacemos nada, pero si es Err mostramos un mensaje de error y terminamos la ejecucion del programa finalizando el proceso con un codigo de salida 1
  if let Err(e) = run(config) {
    // El print va por stderr para que se vea en rojo y no se confunda con la salida normal del programa
    // por lo tanto usamos eprintln! en vez de println!
    eprintln!("Error al ejecutar la aplicación: {}", e);
    process::exit(1); // Si hay un error al ejecutar la aplicacion, mostramos un mensaje de error y terminamos la ejecucion del programa finalizando el proceso con un codigo de salida 1
  }
}