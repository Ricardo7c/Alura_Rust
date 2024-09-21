fn rouba (string: &mut String){ //Inves de receber uma string, recebe uma referencia de string
    string.push_str(" Dias");
    println!("{}", string);
}

fn main(){
    let mut uma_string = String::from("Vinicius");
    rouba(&mut uma_string); // Invés de passar o valor, passamos a referencia do valor.
    println!("{}",uma_string); // uma_string continua funcionando porque não foi movida.
} 