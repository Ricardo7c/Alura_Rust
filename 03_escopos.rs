fn main(){
    let a = 123;
    let b = 789;

    {
        let b = 456;   // Shadowing = b interno está sobrepondo o valor de b externo apenas para o escopo interno
        println!("Dentro: a = {}", a);
        println!("Dentro: b = {}", b);
    }

    println!("Fora: a = {}", a);
    println!("Fora: b = {}", b);
}