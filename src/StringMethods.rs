fn main(){
    /*Replace*/
    {
        let my_string =  String::from("Rust is fantastic!");
        println!("After replace: {}" , my_string.replace("fantastic","great"));
    }

    /*Lines*/

    {
        let my_string = String::from("The weather is \nnice\noutside mate!");

        for elem in my_string.lines() {
            println!("[{}]",elem);
        }
    }

    /*Split*/
    {
        let my_string =  String::from("Con+cac+dai+20;cm");
        let tokens: Vec<&str> = my_string.split("+").collect();

        println!("{}", my_string);
        println!("At index 2: {}", tokens[2]);

    }

    /*Trim */
    {
        let my_string = String::from("   My name is concac  \n\r");
        println!("Before trim:{}", my_string);
        println!("After trim:{}", my_string.trim());        
    }

    /*Chars*/
    {
        let my_string = String::from("dcode on Youtube");

        /*Get character at　index*/
        match my_string.chars().nth(4){
            Some(c) => println!("Character at index4:{}",c),
            None => println!("No character at index 4.")
        }

    }


}