fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let value = vec.get(10).unwrap(); // This will panic if the index is out of bounds
    println!("Value: {}", value);
}