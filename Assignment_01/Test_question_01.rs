fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let name = "Sparsh";
    let greeting = greet(name);
    println!("{}", greeting);
}
