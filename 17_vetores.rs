fn main(){
    //let mut notas: Vec<f32> = Vec::with_capacity(4);
    let mut notas: Vec<f32> = Vec::new();
    notas.push(10.0);
    notas.push(8.8);
    notas.push(6.5);

    println!("{:?}", notas);
    notas.push(7.3);
    println!("{:?}", notas);

    println!("-------------------------------");
    let mut notas2 : Vec<f32> = vec![2.3, 5.4, 6.4];
    println!("Capacidade = {}", notas2.capacity());
    println!("{:?}", notas2);

    notas2.push(7.3);
    println!("Capacidade = {}", notas2.capacity());
    println!("Nota 1 = {}", notas2[0]);

    
    println!("Nota 5 = {}", match notas.get(5) {
        Some(n) => *n,
        None => 0.0
    });

    println!("-------------POP--------------");
    while let Some(nota) = notas2.pop(){
        println!("Valor removido = {}", nota);
        println!("{:?}", notas2)
    }

    for nota in &notas{
        println!("Nota = {}", nota);
    }
    println!("{:?}", notas)

    
}