//mod intro_example;
//mod breakout_example;
mod my_breakout;

//To do use crate::vec3_extension::*; in my_breakout we must first "mod" in vec3_extension here
//main.rs and lib.rs have special rules for "bringing" in things though somehow we're able to "use" Bevy in without a mod here
mod vec3_extension;

fn main() {
    //intro_example::run();
    //breakout_example::run();
    my_breakout::run();
}
