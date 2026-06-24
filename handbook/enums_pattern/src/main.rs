#[derive(Debug)]
enum IpAddrkind{
    V4,
    V6,
}

#[allow(non_camel_case_types,unused_variables,dead_code)]
#[derive(Debug)]
enum newIpKind{
    V4(String),
    V6(String),
}

#[allow(non_camel_case_types,unused_variables,dead_code,non_snake_case)]
#[derive(Debug)]
enum anotherIpAddr{
    V4(u8,u8,u8,u8),
    V6(String),
}

#[allow(dead_code)]
#[derive(Debug)]
struct IpAddr{
    kind: IpAddrkind,
    address: String,
}

#[allow(dead_code,unused_variables)]
fn route(ip_kind: IpAddrkind){

}

#[allow(non_camel_case_types,non_snake_case)]
fn main(){
    let home: IpAddr = IpAddr { 
        kind: IpAddrkind::V4, 
        address: String::from("127.0.0.1") 
    };

    let office: IpAddr = IpAddr{
        kind: IpAddrkind::V6,
        address: String::from("::1"),
    };

    let loopback: newIpKind = newIpKind::V4(String::from("127.0.0.1"));

    println!("{home:#?}");
    println!("{office:#?}");
    println!("{loopback:#?}");

    let newHome: anotherIpAddr = anotherIpAddr::V4(127, 0, 0, 1);
    let newOffice: anotherIpAddr = anotherIpAddr::V6(String::from("::1"));
    println!("{newHome:#?}");
    println!("{newOffice:#?}");
}