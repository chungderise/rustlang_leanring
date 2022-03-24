enum Direction{
  _Up,
  _Down,
  _Left,
  _Right

}

fn main(){
  let player_direction:Direction = Direction::_Right;

  match player_direction{
    Direction::_Up => println!("We are heading up!"),
    Direction::_Down => println!("We are heading Down!"),
    Direction::_Right => println!("We are heading Right!"),
    Direction::_Left => println!("We are heading Left!"),
  }

}