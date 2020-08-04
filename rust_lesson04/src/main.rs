fn diverges()->!{
    panic!("This function never returns!");
}

fn main() {
    let x: i32 = diverges();
    let y: String = diverges();

    // println!("x in panic! is {}", x);
    // println!("y in panic! is {}", y);
}