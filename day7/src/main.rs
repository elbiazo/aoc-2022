struct Dir {
    name: String,
    size: usize,
    children: Vec<Box<Dir>>,
}

fn main() {
    println!("Hello, world!");
}
