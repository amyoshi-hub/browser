// src/mode_s.rs
pub fn mode_select() -> u32 {
    println!("Select mode:");
    println!("1: GUI Mode");
    println!("2: CUI Mode");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().unwrap_or(0)
}
