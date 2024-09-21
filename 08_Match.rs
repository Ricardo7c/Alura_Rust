fn main(){
    let linguagem = "PHP";
    let proposito = match linguagem {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Desconhecido"
    };
    println!("O proposito de {} é {}", linguagem, proposito);
}