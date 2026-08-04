fn main() {
    let number = 7;
    println!("{}", number);

    for i in 1..=10{
        println!("{} * {} = {}", number, i, number * i);
    }
}