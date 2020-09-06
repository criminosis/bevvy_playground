use bevy::prelude::*;

fn main() {
    //Being able to turn a method into a system is done via "trait extension"
    //http://xion.io/post/code/rust-extension-traits.html
    App::build()
        //adds default things like Windowing, Input Controls, and Core Plugin (an event loop)
        .add_default_plugins()
        .add_plugin(HelloPlugin)
        .run();
}

//Plugins represent the unit of modularity into Brevvy, portions of functionality that can be assembled together (or sliced out and replaced)
//This is demonstrating moving our hello logic into such a plugin
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        //Startup System is only ran once - at startup
        app.add_startup_system(add_people.system())
            //normal systems run continuously once event looped?
            .add_system(hello_world.system())
            .add_system(greet_people.system());
    }
}

fn hello_world() {
    println!("hello world!");
}

fn greet_people(_person: &Person, name: &Name) {
    println!("hello {}!", name.0);
}

//A person component
struct Person;

//We could put name under person, but what if we want to put names to other things e.g. a pet
//So ECS practice is to separate Name to its own component so it can be composed into mutiple entities
struct Name(String);

//Initializing a set of People components to our App
fn add_people(mut commands: Commands) {
    commands
        .spawn((Person, Name("Elaina Proctor".to_string())))
        .spawn((Person, Name("Renzo Hume".to_string())))
        .spawn((Person, Name("Zayna Nieves".to_string())));
}
