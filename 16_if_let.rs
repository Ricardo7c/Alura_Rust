#[allow(dead_code)]

fn ler_arquivo(_: String) -> Option<String> {
    Some(String::from("Conteudo do arquivo"))
}


fn main(){
    let conteudo = ler_arquivo(String::from("valor"));

    if let Some(valor) = conteudo{
        println!("{}", valor)
    }

}