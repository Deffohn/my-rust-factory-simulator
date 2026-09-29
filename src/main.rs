pub mod core;
// temporary TODO replace by trait & implement usage for parsers methods
pub mod parsers;

use crate::core::simulation::simulation::{Simulation, initialize_simulation};

use std::io;

fn main() -> io::Result<()> {
    println!("-------------------------------------");
    println!("--- Welcome to Factory Simulation ---");
    println!("-------------------------------------");
    println!("");

    let _simulation: Simulation = match initialize_simulation() {
      Ok(simulation) => simulation,
      Err(err) => {
        return Err(io::Error::new(err.kind(), format!(
          "Simulation initialization failed with error : \n{error}",
          error = err.to_string()
        )));
      }
    };

    return Ok(());
}
