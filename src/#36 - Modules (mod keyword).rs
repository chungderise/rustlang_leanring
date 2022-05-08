mod dcode {
    fn chicken(){
        println!("Chicken!");
    }

    pub fn print_message(){
        chicken();
        println!("How's it going");
    }

    pub mod water{
        pub fn print_message() {
            println!("con cac");
        }
    }

}

fn main(){
    dcode::print_message();
    dcode::water::print_message();
}