/**
 * Tuple: é um grupo de elementos com tipagem diferente
 * Máximo de 12 elementos
 */
pub fn run() {
   let pessoa: (&str, &str, i8) = ("Guilherme", "Passo Fundo", 25);

   println!("{}, de {} anos nasceu em {} - RS", pessoa.0, pessoa.2, pessoa.1);
}