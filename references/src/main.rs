fn show(s:&String) {
    print!("\n  {}", s);
}
fn main() {
    let mut t = String::from("another string");
    show(&t);
    let rt = &mut t;
    rt.push('-');
    show(&rt);
    //let rt2 = &mut t;
    drop(rt);
    let rt2 = &mut t;
    rt2.push_str(" with more stuff");
    show(&rt2);
    show(&t);
    println!("\n  That's all folks!\n");
}
