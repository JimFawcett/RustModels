/////////////////////////////////////////////////////////////
// main.rs - executes named module                         //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 24 Feb 2020  //
/////////////////////////////////////////////////////////////

mod point_traits;
mod probe_struct;
mod probe_traits;

fn main() {

    point_traits::run();
    //probe_struct::run();
    //probe_traits::run();
    print!("\n\n");
}
