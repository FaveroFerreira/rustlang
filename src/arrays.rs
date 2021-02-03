/**
 * Arrays: possuem tamanho fixo e tipo dos elementos é único, ao contrário de um touple
 */
pub fn run() {

    // let nome_da_var: [tipo;tamanho]
    let numeros: [i32;5] = [1, 2, 3, 4, 5];

    // printar todos os elementos usando debug trait
    println!("{:?}", numeros);

    // printar valor específico
    println!("{}", numeros[0]);

}