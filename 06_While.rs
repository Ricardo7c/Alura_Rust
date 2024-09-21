fn main(){
    let multiplicador = 5;
    let mut cont = 0;
    while cont < 10 {
        cont += 1;
        println!("{} x {} = {}", multiplicador, cont, multiplicador*cont);
    }

    let mut cont = 0;
    loop{
        cont += 1;
        println!("{} x {} = {}", multiplicador, cont, multiplicador*cont);
        if cont == 10{
            break;
        }
    }
}