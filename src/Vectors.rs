fn main(){
  let mut my_vector =  vec![1,2,3,4];

  my_vector.push(49);
  my_vector.remove(1);

  for number in my_vector.iter(){
    println!("{}",number);
  }
  
  for i in &mut my_vector{
    println!("{}",i);
  }

  let mut v = Vec::new();
  v.push(33);
  v.push(44);  
  v.push(55);

  for i in &mut v{
    println!("{}",i);
  }

}