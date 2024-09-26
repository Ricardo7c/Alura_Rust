struct Conta{
    titular: NomeCompleto,
    saldo: f64
}
struct  NomeCompleto{
    nome: String,
    sobrenome: String,
}

impl Conta {
    fn sacar(&mut self, valor:f64){
        self.saldo -= valor;
    }
    fn depositar(&mut self, valor:f64){
        self.saldo += valor;
    }
}

fn main(){
    let titular = NomeCompleto{
    nome: String::from("Ricardo"),
    sobrenome: String::from(" Silva")
    };

    let mut conta = Conta{
        titular: titular,
        saldo: 100.0
    };

    println!("Dados da conta: Titular = {} {}, Saldo = {}", conta.titular.nome, conta.titular.sobrenome, &conta.saldo);
    conta.sacar(50.0);
    println!("Dados da conta: Titular = {} {}, Saldo = {}", conta.titular.nome, conta.titular.sobrenome, &conta.saldo);
    conta.depositar(200.0);
    println!("Dados da conta: Titular = {} {}, Saldo = {}", conta.titular.nome, conta.titular.sobrenome, &conta.saldo)
    
    }