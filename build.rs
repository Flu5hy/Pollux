extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/message/message.proto"],
                                &["src/"]).unwrap();
}