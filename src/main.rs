mod how_you_hold_data_for_operations;
use how_you_hold_data_for_operations::user_defined::structs::structures;

/*mod greetings;
mod how_you_hold_data_for_operations;

extern crate hello_world_lib;
//mod how_you_hold_data_for_operations;

///Optionally oad each member of greetings
use greetings::default_greeting;
use greetings::spanish;
use greetings::french;*/
///Alternatively, use * to load them all
//use greetings::*; 
///Alternatively, load them in one line
/// 
/*use how_you_hold_data_for_operations::primitives::scalar::boolean;
use how_you_hold_data_for_operations::primitives::compound::tuple;
use how_you_hold_data_for_operations::primitives::literals::literal;
use hello_world_lib::{greeting_from_lib,};

use greetings::{english, spanish, french};

fn main() {
    literal();
    return;
    println!("Hello, world!");
    boolean();
    tuple();

    println!("{}", english::default_greeting());
    println!("{}", english::default_greeting2());
    println!("{}", spanish::default_greeting());
    println!("{}", french::default_greeting());
    println!("{}", hello_world_lib::greeting_from_lib());

}*/

fn main() {
    structures();
    return;
    
    let mut product = 1.0;
    let array: [f32; 5] = [1.0, 2.0, 3.0, 4.0, 5.0]; 

    for &n in array.iter() {
        product *= n;
    }

    println!("Product: {}", product);
}




