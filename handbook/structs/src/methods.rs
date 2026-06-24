#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

// associated functions
impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn width(&self) -> bool{
        self.width > 0
    }

    fn get_width(&self) -> u32{
        self.width
    }

    fn set_width(&mut self,new_width: u32) -> u32{
        self.width = new_width;
        self.width
    }

    fn can_hold(&self,other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

pub fn methods(){
    let mut rec1: Rectangle = Rectangle{
        width: 40,
        height: 30,
    };
    println!("{rec1:#?}");
    println!("area: {}",rec1.area());

    // field
    println!("width: {}",rec1.width);
    // method
    println!("width: {}",rec1.width());
    // getter
    println!("get width: {}",rec1.get_width());
    // setter
    println!("set width: {}",rec1.set_width(3));
}