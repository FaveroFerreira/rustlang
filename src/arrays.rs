/**
 * Arrays: possuem tamanho fixo e tipo dos elementos é único, ao contrário de um touple
 * 
 * São alocados na stack
 */

use std::mem;

pub fn run() {

    // let nome_da_var: [tipo;tamanho]
    let mut numeros: [i32;5] = [1, 2, 3, 4, 5];

    // printar todos os elementos usando debug trait
    println!("{:?}", numeros);

    // printar valor específico
    println!("primeiro elemento do array é {}", numeros[0]);

    // alterar valor
    numeros[0] = 25;

    println!("novo valor do primeiro elemento do array é {}", numeros[0]);

    // memória ocupada pelo array
    println!("numeros ocupa {} bytes", mem::size_of_val(&numeros));

    // slices, o bound to é exclusivo e não inclusivo
    let slice: &[i32] = &numeros[0..2];

    println!("slice 0 a 2 = {:?}", slice);
}