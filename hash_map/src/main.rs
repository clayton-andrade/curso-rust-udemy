use std::collections::HashMap;

fn main() {
    let mut shapes = HashMap::<String, i32>::new();
    shapes.insert(String::from("quadrado"), 4);
    shapes.insert(String::from("triângulo"), 3);

    println!("Um quadrado tem {} lados", shapes["quadrado"]);

    for (k, v) in &shapes {
        println!("{}: {}", k, v);
    }

    let sides = shapes.entry("hexágono".to_owned()).or_insert(6);
    println!("{}", sides);
    println!("{:?}", shapes);
    let sides = shapes.entry("quadrado".to_string()).or_insert(6);
    println!("{}", sides);
    println!("{:?}", shapes);

    let chars = "quantas vezes eu estive cara à cara com a pior metade".chars();
    let mut map = HashMap::new();
    for c in chars {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}