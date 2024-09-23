fn eh_fim_de_semana(dia_da_semana: DiaDaSemana) -> bool{
    match dia_da_semana {
        DiaDaSemana::Sabado | DiaDaSemana::Domingo => true,
        _ => false
    }
}

#[allow(dead_code)]
enum DiaDaSemana {
    Domingo,
    Segunda,
    Terça,
    Quarta,
    Quinta,
    Sexta,
    Sabado
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), 
    CymkColor{cyan: u8, magenta: u8, yellow:u8, black:u8 } //Valores nomeados.
}



fn main(){
    println!("É fim de semana? {}", eh_fim_de_semana(DiaDaSemana::Sabado));


    let cor = Color::CymkColor{cyan: 100, magenta: 50, yellow:70, black:255 };
    let exibircor = match cor {
        Color::Red => "Vermelho",
        Color::Green => "Verder",
        Color::Blue => "Blue",
        Color::RgbColor(0, 0, 0 ) 
            | Color::CymkColor { cyan:_, magenta:_, yellow:_, black:255 } => "Preta",
        Color::RgbColor(_, _, _ ) => "RGB desconhecido",
            
        Color::CymkColor { cyan:_, magenta:_, yellow:_, black:_ } => "CYMK Desconhecido"
    };

    println!("Cor = {}", exibircor);

}