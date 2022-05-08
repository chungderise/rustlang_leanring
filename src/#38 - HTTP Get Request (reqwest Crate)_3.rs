extern crate reqwest;

fn main(){
    let response_text = reqwest::get("http://localhost/concacgi/")
        .expect("Couldn't make request")
        .text().expect("Could not read respone text!");
    println!("Respone Text: {}", response_text);
}