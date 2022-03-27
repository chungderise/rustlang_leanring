fn main(){
  let mut my_string = String::from("My name is Chung IT");
  
  //length
  println!("Length : {}",my_string.len());

  // Is Empty
  println!("String is empty? {}",my_string.is_empty());

  for token in my_string.split_whitespace(){
    println!("{}",token);
  }

  println!("Does the string contain 'Chung'? {}",my_string.contains("Chung"));

  my_string.push_str(" Dep trai vai dai");
  println!("{}", my_string);

}