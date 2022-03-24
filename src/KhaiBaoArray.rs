fn main() {
//Data Types
////Array: Là một danh sách có kích thước cố đinh, và các dữ liệu trong đó đồng nhất 1 dữ liệu.

let number: [i32;3] = [100,200,300];
let get_number1: i32 = number[0];//100
let get_number2: i32 = number[1];//200
let get_number3: i32 = number[2];//300

println!("get_number1 = {}", get_number1);
println!("get_number2 = {}", get_number2);
println!("get_number3 = {}", get_number3);

let _hashing: [i32;32]=[0;32];
println!("hashing = {:?}", _hashing);

for i in _hashing.iter(){
  print!("{}",i);
}



}
