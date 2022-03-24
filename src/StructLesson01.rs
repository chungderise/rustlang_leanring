#[derive(Debug)]
struct Member{
  username: String,
  email: String,
  age: u64,
  active: bool,
}

fn main(){
  let mut member1: Member = Member {
    email: String::from("chungit@gmail.com"),
    username: String::from("chungit"),
    age:33,
    active:true,
  };

let name: String =  member1.username;
println!("name = {}",name);

member1.username = String::from("Dep trai vai");
println!("member = {}",member1.username);

let member2:Member = create_new_member(
  String::from("Chungdeptrai"), 
  String::from("tvchung5@gmail.com"),
  30,
);
println!("member2 = {:#?}",member2);

let member3:Member = Member{
  username:  String::from("Concac"),
  ..member2
};
println!("member3= {:#?}",member3);

}

fn create_new_member(username:String, email:String,age:u64) -> Member{
  Member{
    username : username,
    email : email,
    age : age,
    active : true,
  }
}