pub mod mods;

fn do_add() {
    use mods::add::add1;
    use mods::add::add2;
    use mods::add::add3;
    println!("{}", add1(Some(10), Some(100)));
    println!("{}", add1(None, Some(100)));
    println!("{}", add1(Some(10), None));
    println!("{}", add1(None, None));

    println!("{}", add2!(10,100));
    println!("{}", add2!(10));
    println!("{}", add2!());

    println!("{}", add3((10, 100)));
    println!("{}", add3(10));
    println!("{}", add3(()));
}


fn main() {
    do_add()
}

