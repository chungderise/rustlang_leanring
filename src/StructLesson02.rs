#[derive(Debug)]
struct hinhchunhat{
  dai: u32,
  rong: u32,
}

impl hinhchunhat{
  fn dien_tich(&self) -> u32 {
    self.dai * self.rong
  }

  fn chua(&self, hinhchunhatkhac: &hinhchunhat) -> bool {
    self.dai > hinhchunhatkhac.dai && self.rong > hinhchunhatkhac.rong
  }
}

impl hinhchunhat {
  fn hinhvuong(kichthuoc: u32) -> hinhchunhat {
    hinhchunhat{
      dai:kichthuoc,
      rong: kichthuoc,
    }

  }
}

fn main(){
  let kichthuoc: hinhchunhat = hinhchunhat {dai:30, rong:50};
  println!("dien tich hinh chu nhat = {}", kichthuoc.dien_tich());

  let kichthuoc2: hinhchunhat = hinhchunhat {dai:20, rong:40};
  let kichthuoc3: hinhchunhat = hinhchunhat {dai:40, rong:60};

  println!("hinh chu co the chua hinh 2 = {}",kichthuoc.chua(&kichthuoc2));
  println!("hinh chu co the chua hinh 3 = {}",kichthuoc.chua(&kichthuoc3));

  let kichthuoc4: hinhvuong = hinhvuong {10};
  println!("hinh chu co the chua hinh 3 = {}",hinhchunhat::hinhvuong(&kichthuoc4));


}
