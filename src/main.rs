extern crate off_blockway;
#[macro_use]
extern crate serde_derive;

use std::io;
use std::io::prelude::*;
use std::process::Command;
use self::json::Parser;


mod styling;
mod json;


fn main() {

    // Get the input reader
    let input = io::stdin();
    print!( "{}\n", styling::HEADER );


    // Generate the operator 
    let mut operator = json::Operator::empty();
    operator.operate( json::Parser::parse_package( "json/dummy.json" ).expect(" Could not construct package") );
    println!( "{:?}", ( operator.uid, operator.chain ) );
    
    // Start the node server 
    Command::new( "forever" ).args( &[ "start", "js/server.js" ] ).output().expect( "Could not start server" );
    
    // While the input reader is not EOF
    for line in input.lock().lines()
    {

        let input = line.unwrap();
        
        //TODO: Get the cursor working with reading a line at a time
        //print!( "\n{}", styling::CURSOR );

        if input.clone() == "-h"
        {
            println!( "{}", styling::HELP );
        }
        else if input.clone() == "-q" || input.clone() == "quit"
        {
            println!("Goodbye!");
            Command::new( "forever" ).args( &["stop", "js/server.js"] ).output().expect( "Could not stop process" );
            break;
        }

    }

    
}


