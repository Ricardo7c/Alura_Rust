
fn main(){

    let notas:[f32; 4] = [10.0, 8.0, 9.5, 6.0];
    let lista_de_zeros:[f32; 4] = [0.0; 4];

    let matriz = [
        notas,
        lista_de_zeros
    ];

    println!("------------Por itens-----------");
    for nota in notas {
        println!("A nota é = {}", nota);
    }

    println!("------------Por indice-----------");
    for indice in 0..notas.len(){
        println!("A nota é = {}", notas[indice]);
    }

    println!("---------Lista de zeros-----------");
    for indice in 0..lista_de_zeros.len(){
        print!("{} ", lista_de_zeros[indice]);
    }
    println!();

    println!("---------Matriz-----------");
    for linha in matriz{
        for item in linha{
            print!("{}", item);
        }   
}
}