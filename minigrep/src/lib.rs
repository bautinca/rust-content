// Usamos lifetime 'a paa indicar que el vector de resultados tiene la misma vida que el contenido del archivo
// Esto es porque justamente estamos devolviendo referencias a las lineas del contenido del archivo, por lo que no podemos devolver un vector de referencias a algo que ya no existe
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }
  results
}

// Es como search pero insensitive, es decir que no distingue entre mayusculas y minusculas
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase(); // Al pasarlo a lowercase ahora query es un String y no un &str
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) { // query ahora es un String por lo tanto le tenemos que pasar un puntero a dicha String
      results.push(line);
    }
  }
  results
}

// TEST UNITARIOS
#[cfg(test)]
mod tests {
  use super::*;

  // Test unitario para la funcion search()
  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    // Ponemos a prueba la funcion search() pasandole el query "duct"
    // y el contenido que simularia el contenido de un archivo.
    // Logicamente al pasarle la query 'duct' deberia devolver un vector con la linea "safe, fast, productive." ya que es la unica linea que contiene el query
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  // Otro test unitario para la funcion search() para ver que sea sensitive case, es decir que distinga entre mayusculas y minusculas
  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  // Otro test pero ahora para otra funcion search_case_insensitive() que es como
  // el search pero que no distingue entre mayusculas y minusculas
  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
  }
}