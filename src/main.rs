extern crate prost;
#[macro_use]
extern crate prost_derive;

mod message;

fn main() {
    println!("Hello World!");

    let chat_message = message::create_chat_message(0, "".to_string());
    println!("{:?}", chat_message);
}