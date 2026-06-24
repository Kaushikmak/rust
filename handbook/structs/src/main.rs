mod struct_example;
mod methods;

#[derive(Debug)]
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[allow(dead_code)]
fn build_user(email: String, username: String) -> User {
    User { 
        active: true, 
        username, 
        email, 
        sign_in_count: 1 
    }
}

#[derive(Debug)]
struct Color(i32,i32,i32);

struct AlwaysEqual;

fn main() {
    let mut user1: User = User{
        active: true,
        username: String::from("kaushik"),
        email: String::from("kaushikmak35@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("kaushikgk1@gmail.com");

    println!("{:#?}",user1);

    let user2: User = User{
        active: user1.active,
        username: user1.username,
        email: String::from("tastytaco@gmail.com"),
        sign_in_count: user1.sign_in_count,
    };

    println!("{user2:#?}");

    let user3: User = User{
        ..user2
    };

    println!("{user3:#?}");
    println!("{}",user2.sign_in_count);

    // tuple structs
    let black: Color = Color(0,0,0);
    println!("{black:#?}");

    let subject: AlwaysEqual = AlwaysEqual;

    struct_example::struct_example();

    methods::methods();


}
