extern crate reqwest;

fn main(){
    match reqwest::get("http://localhost/concacgi/"){
        Ok(mut response) => {
            //Check if 200 OK
            if response.status() == reqwest::StatusCode::Ok {
                match response.text() {
                    Ok(text) => println!("Response Text: {}", text),
                    Err(_) => println!("Could not read reponse text!")
                }
            } else {
                println!("Respone was not 200 OK.");
            }
        }
        Err(_) => println!("Could not make the request!")
    }
}