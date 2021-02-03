/**
 *  - Variáveis são block scoped, similar ao Java.
 *  - Variáveis são por padrão imutáveis
 *  - Variáveis guardam dados primitivos ou referências
 * 
 *  - Podem ser declaradas de algumas formas:
 *      - let nome_da_variavel --- definição de variável comum, imutável por padrão, sempre snake_case
 *      - let mut nome_da_variavel --- variável mutável, definida com a keyword "mut", sempre snake_case
 *      - const nome_da_variavel --- variáveis que nunca, jamais serão alteradas, definidas com a keyword "const", exigem tipagem explícita, sempre UPPER CASE
 * */

pub fn run() {
    let nome = "Guilherme";
    let idade = 25;

    println!("Meu nome é {} e eu tenho {} anos", nome, idade);    
    
    // idade = 26; -- erro em compile time, por padrão todas as variáveis são imutáveis

    let mut idade_mutavel = 25; // mut -- keyword para permitir que uma variável seja mutável 
    
    println!("No ano de 2020, {} tinha {} anos", nome, idade_mutavel);
    
    idade_mutavel = 26;

    println!("Dia 17/01, no ano de 2021 {} fez {} anos", nome, idade_mutavel);

    const CPF: i64 = 02405474134;
    println!("{}, programador de {} anos, de CPF: {}, está aprendendo Rust, e esse não é meu CPF de verdade ok  ...", nome, idade, CPF);

    // Definição de multiplas variáveis
    let ( profissao, nivel ) = ( "Engenheiro de Software", "Sênior" );
    println!("{} atua como {} de nível {}", nome, profissao, nivel);
}