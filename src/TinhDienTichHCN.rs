fn main(){

  let hinh_chu_nhat = (30,50);
  println!("Dien tich hinh chu nhat {}",dien_tich(hinh_chu_nhat));

}

fn dien_tich(kichthuoc: (u32,u32)) -> u32{
  kichthuoc.0 * kichthuoc.1
}