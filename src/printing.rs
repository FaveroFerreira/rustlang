pub fn run() {
    // Printar para o console
    println!("Olá, vindo do arquivo printing.rs");

    // Print também aceita uma expressão
    println!("A soma de 10 + 10 é igual a {}", 10 + 10);

    // Print com formataçao basica de output com placeholders
    println!("Primeiro param: {}, segundo param: {}, terceiro param: {}", "Brasil", "Argentina", "Irlanda");

    // Print com formatação com argumentos posicionais, definindo a ordem dos params no placeholder
    println!("{0} está aprendendo {1}. O {0} está achando muito legal", "Guilherme", "Rust");

    // Print com formataçao usando argumentos nomeados
    println!("{nome} acha {lang} uma ferramenta incrível, porém, complexa. {lang} é extremamente divertido!", lang = "Rust", nome = "Guilherme");

    // Print de traits de um argumento, por exemplo, o valor inteiro 10, sendo possível printar a sua "trait" binária (seu valor binário 1010)
    println!("O valor binário de 10 é {:b}, o valor hexadecimal de 10 é {:x}, o valor octal de 10 é {:o}", 10, 10, 10);

    // Print com a trait de debug {:?}, extremamente útil, podendo receber N argumentos, facilita do debugging e troubleshooting
    println!("Debug de várias variáveis, com diferentes tipos. Todas serão printadas aqui -> {:?}", (12, 34, 9493, 123, true, "Hello", false, 'A'));
}