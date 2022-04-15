use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::<&str, i32>::new();

    //Add value
    marks.insert("Rust Programmung", 96);
    marks.insert("Web Development", 94);
    marks.insert("UX Design", 75);
    marks.insert("Professional Computing Studies", 45);
    marks.insert("C++ Programming", 23);

    println!("How many subjects have you studies? {}",marks.len());

    match marks.get("Web Development"){
        Some(mark) => println!("You got {} for Web Dev!", mark),
        None => println!("You didnt study Web Development")
    }

    //Remove a value
    marks.remove("UX Design");

    for (subject, mark) in &marks {
        println!("For {} you got {} %", subject,mark);
    }

    //check for value
    println!("Did you study C++ {}", marks.contains_key("C++ Programming"));
}