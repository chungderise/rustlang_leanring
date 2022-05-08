extern crate reqwest;

fn main(){
    // doc du lieu text tu mot trang website
    match reqwest::get("http://localhost/concac/") {
        Ok(mut response) => {
            //Check if 200 ok
            if response.status() == reqwest::StatusCode::Ok {
                match response.text() {
                    Ok(text) => println!("Response Text: {}", text),
                    Err(_) => println!("Could not read response text!")
                }
            } else {
                println!("Response was not 200 ok");
            }
        }
        Err(_) => println!("Could not make the request!")

    }
}