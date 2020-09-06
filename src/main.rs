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
    //this build method now consolidates the initial state (people names), how to great them, and also
    //a timer to indicate when to execute the greeting. By putting this all into a plugin it's a self-contained aspect of our app: Greeting people
    fn build(&self, app: &mut AppBuilder) {
        // the reason we call from_seconds with the true flag is to make the timer repeat itself
        app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            //Startup System is only ran once - at startup
            .add_startup_system(add_people.system())
            //normal systems run continuously once event looped
            .add_system(greet_people.system());
    }
}

//In comparison to its prior form this method takes in a Time Resource a kind of "global" state value that we will use to meter when
//it's time to print another hello message (whereas without it we were printing at the speed of the CPU)
//Notes from docs:
//Res and ResMut pointers provide read and write access (respectively) to resources.
//Note that resources must come before components or your function will not be convertible to a System.
//Time (as in Res<Time>) comes from the default plugins' Time Resource
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, _person: &Person, name: &Name) {
    // update our timer with the time elapsed since the last update
    timer.0.tick(time.delta_seconds);

    // check to see if the timer has finished. if it has, print our message
    if timer.0.finished {
        println!("hello {}!", name.0);
    }

    //^ this has a bug. greet_people is ran for each entities that has person and name but only the entity that is being evaluated at the
    //crossover of the timer is getting greated, not the others. "Queries" solve this.
}

//A component to hold a timer interval that will accumulate delta time and indicate when it is time to fire off again
//This needs to be a distinct type so it is can be refenced as a unique type for Resource consumption
struct GreetTimer(Timer);

// fn greet_people(_person: &Person, name: &Name) {
//     println!("hello {}!", name.0);
// }

//A person component
struct Person;

//We could put name under person, but what if we want to put names to other things e.g. a pet
//So ECS practice is to separate Name to its own component so it can be composed into mutiple entities
struct Name(String);

//Initializing a set of People components to our App
fn add_people(mut commands: Commands) {
    commands
        //spawning Entities that have a Person and Name components
        .spawn((Person, Name("Elaina Proctor".to_string())))
        .spawn((Person, Name("Renzo Hume".to_string())))
        .spawn((Person, Name("Zayna Nieves".to_string())));
}
