use bevy::prelude::*;

fn main() {
    //Being able to turn a method into a system is done via "trait extension"
    //http://xion.io/post/code/rust-extension-traits.html
    App::build().add_system(hello_world.system()).run();
}

fn hello_world() {
    println!("hello world!");
}
