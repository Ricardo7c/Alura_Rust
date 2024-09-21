use std::mem::size_of_val as Bytes;

fn main(){
    let inteiro = 300;
    println!("Inteiro = {}, tamanho = {} bytes", inteiro, Bytes(&inteiro));
    
    let real = 2.5;
    println!("Real = {}, tamanho = {} bytes", real, Bytes(&real));

    let mut booleano = false;
    println!("Boleano = {}, tamanho = {} bytes", booleano, Bytes(&booleano));

    booleano = true;
    println!("Boleano = {}, tamanho = {} bytes", booleano, Bytes(&booleano));

    let char = 'A';
    println!("Char = {}, tamanho = {} bytes", char, Bytes(&char));
}