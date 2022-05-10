struct Rectangle {
    width: u8,
    height: u8
}

impl Rectangle {
    fn is_sqaure(&self) -> bool {
        self.width == self.height
    }
}

fn main(){

}

fn give_two() -> i32 {
    2
}

// #[cfg(test)]
mod dcode_test{
    #[test]
    #[should_panic]
    fn test_basic() {
        assert!(1==1);
       panic!("Oh no!");
    }

    #[test]
    #[ignore]
    fn test_equals(){
        assert_eq!(super::give_two(),1+1);

        assert_ne!(super::give_two(),1+2);
    }

    #[test]
    #[should_panic]
    fn test_structs(){
        let r = super::Rectangle{
            width:50,
            height:25
        };

        assert!(r.is_sqaure());

    }

}