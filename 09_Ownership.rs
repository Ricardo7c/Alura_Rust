fn rouba (string: &String){ //Inves de receber uma string, recebe uma referencia de string
    println!("{}", string);
}

fn main(){
    let uma_string = String::from("Vinicius");
    rouba(&uma_string); // Invés de passar o valor, passamos a referencia do valor.
    println!("{}",uma_string); // uma_string continua funcionando porque não foi movida.

} 