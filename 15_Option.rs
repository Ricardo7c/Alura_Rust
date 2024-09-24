#[allow(dead_code)]

fn ler_arquivo(_caminho: String) -> Option<String> {
    Some(String::from("Conteudo do arquivo"))
}


fn main(){
    let conteudo = ler_arquivo(String::from("valor"));
    match &conteudo {
        Some(valor) => println!("{}", valor),
        None => println!("Arquivo não existe")
    };
}