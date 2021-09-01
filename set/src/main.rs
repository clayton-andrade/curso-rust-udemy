use std::collections::HashSet;

fn main() {
    let mut conjunto: HashSet<i32> = HashSet::new();
    conjunto.insert(5);
    conjunto.insert(8);
    conjunto.insert(4);
    println!("{:?}", conjunto);

    for i in 0..10 {
        if conjunto.contains(&i) {
            continue;
        }
        if conjunto.insert(i) {
            println!("Adicionando o número {}...", i);
        } else {
            println!("Erro na inserção!");
        }
    }
    println!("{:?}", conjunto);

    if conjunto.remove(&15) {
        println!("Número removido");
    } else {
        println!("Número não encontrado");
    }
}
