use std::env;

fn main(){
  let args: Vec<String> = env::args().collect();

for elem in args.iter() {
  println!("{}",elem);
}

println!("{}",args[1]);

}