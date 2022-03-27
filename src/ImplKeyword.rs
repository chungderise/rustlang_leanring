struct Rectangle{
  width: i32,
  height:i32
}

impl Rectangle {
  fn print_description(&self){
    println!("Rectangle: {} x {}", self.width, self.height);
  }
  fn in_quare(&self) -> bool {
    self.width == self.height
  }

  fn dientich(&self) {
    println!("Dien tich {}",&self.width * &self.height);
  }

}

fn main(){
  let my_rect = Rectangle { width:10,height:10};
  my_rect.print_description();
  println!("{}",my_rect.in_quare());
  my_rect.dientich();
}