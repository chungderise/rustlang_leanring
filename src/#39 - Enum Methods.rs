#![allow(dead_code)]

enum Day{
    Monday, Tuesday, Wednesday, Thursday, Friday, Sturday, Sunday
}

impl Day {
    fn is_weekday(&self) -> bool {
        match self {
            &Day::Sunday | &Day::Sturday => return false,
            _ => return true
        }
    }
}
fn main(){
    let d = Day::Sunday;
    
    println!("Is d a weekday? {}:", d.is_weekday());
}