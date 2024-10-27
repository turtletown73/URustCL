pub mod interpreter;

fn main() {
    println!("{:?}", interpreter::interpret(String::from("./fibonacci.urcl")));
}