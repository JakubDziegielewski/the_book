use std::io;

fn main() {
    let mut input = String::new();
        println!("Enter a number: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

    let vector: Vec<u64> = input.trim().split(" ").map(|x| x.parse().unwrap()).collect();
    let mut first: u64 = 0;
    let mut second: u64 = 0;

    for v in vector{
        if v > first{
            second = first;
            first = v;
        }
        else if v > second {
            second = v;
        }
    }

    let res = first * second;
    println!("{res}")
}
