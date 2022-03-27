fn main(){
  let numbers: [i32;5]= [1,2,3,4,5];

  let numbers1 = [2; 400];


  for n in numbers.iter(){
    println!("{}",n);
  }
  
  for n in 0..numbers.len(){
    println!("{}",numbers[n]);
  }

  for n in numbers1.iter(){
    println!("{}",n)
  }

}