fn main(){
    let idade = 17;
    let autorizacao = true;
    if idade >= 18{
        println!("Pode entrar na balada");
    }else if idade > 16 && autorizacao{
        println!("Pode entrar porque é maior de 16 e responsavel autorizou");
    }else{
        println!("Não pode entrar na balada");
    }

    let condicao = if idade >= 18 {"maior"} else {"menor"};
    println!("É {} de idade", condicao)
}
