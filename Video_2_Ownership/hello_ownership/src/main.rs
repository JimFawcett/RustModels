#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    let x = 5;
    // x = 6;  // fails to compile, not mutable
    let mut y = 5;
    y = 6;
    {
        let ry = &mut y;
        *ry = *ry + 1;
        // y = y + 1;  
        //   compile fails, value of y borrowed
        print!("\n  value of ry = {}", *ry);
        // print!("\n  value of y  = {}", y);
        //   compile fails print! takes y by ref
        //   but already have mut ref of y
    }
    y = y + 1;  // ok, borrow dropped 
    print!("\n  value of y mutated to {}", y);

    println!("\n\n  Hello, ownership again!\n");
}
