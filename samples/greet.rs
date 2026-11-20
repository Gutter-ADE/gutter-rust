/// Says hello. Hover `greet` below to see this comment.
fn greet(name: &str) -> String {
    format!("Hello, {name}")
}

fn main() {
    println!("{}", greet("Gutter"));
}
