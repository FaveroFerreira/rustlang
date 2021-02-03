/**
 * Strings são estranhas no Rust.
 * 
 * - string (primitivo): Imutáveis e de tamanho fixo, alocadas em algum lugar na memória
 * - String: Mutável, alocada na heap
 */

pub fn run() {

    let ola_mundo = "Olá mundo"; // primitivo
    let mut ola = String::from("Olá mundo sou mutável"); // heap string

    println!("Valor de ola_mundo = {}", ola_mundo);
    println!("Valor de ola = {}", ola);

    // .push() adiciona char a uma string existente
    ola.push('\u{1F600}');

    // push_str() adiciona string a string existente
    ola.push_str(" mais adição de campos a string");

    println!("O novo valor de olá é = {}", ola);

    // tamanho
    println!("ola lenght: {}", ola.len());

    // capacidade
    println!("ola capacity: {}", ola.capacity());

    // is_empty()
    println!("ola is empty: {}", ola.is_empty());

    // iterar sobre palavras
    for word in ola.split_whitespace() {
        println!("{}", word);
    }

}