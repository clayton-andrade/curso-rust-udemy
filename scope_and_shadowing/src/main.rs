fn main() {
    let a = 123;

    {
        let b = 456;
        println!("inside new scope, b = {}", b);
        let a = 789;
        println!("inside new scope, a = {}", a);
    }

    println!("a = {}", a);

    let a = 888;
    println!("a = {}", a);

    // println!("b = {}", b); // Não imprime por estar fora de escopo
}
