#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

pub fn struct_example(){
    let rec1: Rectangle = Rectangle{
        width: 30,
        height: 40,
    };
    println!("{rec1:#?}");
    // prints to stderr console stream
    dbg!(&rec1);
}