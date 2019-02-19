use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failure: read_line (1)");
    /* ^^^ We don't need this. ^^^ */
    
    let mut ns = String::new();
    io::stdin().read_line(&mut ns).expect("Failure: read_line (2)");
    
    let mut ns: Vec<u32> = ns
        .split_whitespace().map(|t| t.parse().unwrap())
        .collect();
    
    while let Some(top) = ns.pop() {
        print!("{} ", top);
    }
}
