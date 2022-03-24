fn main() {
//Data Types
////Tuple : là một dạng dữ liệu kết hợp với nhiều kiểu dữ liệu trong một tuple;
let tup:(&str,i32) = ("hello",100_000);
println!("tup= {:?}",tup);
println!("tup= {:#?}",tup);

let (_string,_integer) = tup;
let _integer:i32 = tup.1;
println!("_string={}",_integer);
println!("_integer={}", _string);

}
