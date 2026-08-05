use std::io;

fn main() {
    let mut input = String::new();
    println!("Digite um numero para ver sua tabuada");

    io::stdin().read_line(&mut input).unwrap();

    let number: i32 = input.trim().parse().unwrap();
    
    println!("{}", number);

    for i in 1..=10{
        println!("{} * {} = {}", number, i, number * i);
    }
}
