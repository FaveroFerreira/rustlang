/**
 * Tipos primitivos
 * 
 * Integer: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 depende da quantidade de bits a guardar na memória, (u) unsigned não registra negativos
 * Float: f32, f64
 * Boolean: bool
 * Charactes: char
 * Tuples: lista de objetos divergentes
 * Arrays: tamanho fixo
 * 
 * Rust é uma linguagem tipada, mas o compilador vai entender conforme a declaração & uso, portanto não é necessário tipar todas a variáveis
 * 
 */
pub fn run() {

    let x = 1; // automáticamente o compilador entende que x é i32

    let y = 2.5; // automáticamente o compilador entende que u é f64

    let z: i64 = 128371623; // tipo declarando explicitamente

    let printar_numericos: bool = true;

    let maior_que = 10 > 5;

    let chr = 'a';

    let emoji: char = '\u{1F600}'; // emojis são chars, em unicode

    println!("Variáveis criadas {:?}", (x, y, z, printar_numericos, maior_que, chr, emoji));

    if printar_numericos {
        print_numeric_types_min_max();
    }
}

fn print_numeric_types_min_max() {
    println!("\n --------------- TAMANHOS NUMÉRICOS ------------------ \n");

    println!("u8 MIN: {}, u8 MAX: {}", std::u8::MIN, std::u8::MAX);
    println!("i8 MIN: {}, i8 MAX: {}", std::i8::MIN, std::i8::MAX);

    println!("u16 MIN: {}, u16 MAX: {}", std::u16::MIN, std::u16::MAX);
    println!("i16 MIN: {}, i16 MAX: {}", std::i16::MIN, std::i16::MAX);
    
    println!("u32 MIN: {}, u32 MAX: {}", std::u32::MIN, std::u32::MAX);
    println!("i32 MIN: {}, i32 MAX: {}", std::i32::MIN, std::i32::MAX);
    
    println!("u64 MIN: {}, u64 MAX: {}", std::u64::MIN, std::u64::MAX);
    println!("i64 MIN: {}, i64 MAX: {}", std::i64::MIN, std::i64::MAX);

    println!("u128 MIN: {}, u128 MAX: {}", std::u128::MIN, std::u128::MAX);
    println!("i128 MIN: {}, i128 MAX: {}", std::i128::MIN, std::i128::MAX);

    println!("f32 MIN: {}, f32 MAX: {}", std::f32::MIN, std::f32::MAX);
    println!("f64 MIN: {}, f64 MAX: {}", std::f64::MIN, std::f64::MAX);

}