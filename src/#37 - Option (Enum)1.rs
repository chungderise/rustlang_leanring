fn main(){
    let name = String::from("Domenic");
    //In ra ky tu so 1 trong chuoi name
    println!("Character at index 8: {}", match name.chars().nth(1) {
        Some(c) => c.to_string(),
        None => "No character at index 8!".to_string()
    });
}