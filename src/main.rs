mod routes; 

fn main() {
    let a = 1; 
    let b = 2;
    let d = routes::stockRoute::foo(a, b);
    println!("Hello from Rust {}", d);
}
