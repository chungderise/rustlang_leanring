fn main() {
    //Shadowing
      let outer:i32 = 10;
      {
        let inner:i32 = 200;
        println!("inner= {}",inner);
        let outer:i32 = 300;
        println!("outer ={}",outer);
      }
      println!("outer = {}",outer);
    }
    