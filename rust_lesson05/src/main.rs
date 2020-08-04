fn main() {
    let x = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for i in x.iter(){
        println!("i = {}", i);
    }
    /*
        这里是loop循环，因为有警告所以注释
    */
    // 'outer: loop{
    //     println!("Entered the outer loop");
    //     'inner: loop{
    //         println!("Entered the inner loop");
    //         break 'outer;
    //     }
    //     println!("This point will never be reached");
    // }
    // println!("Exited the outer loop");

    let day = 8;
    match day{
        0 | 6=> println!("weekend"),
        1 ... 5 => println!("weekend"),
        _ => println!("invalid"),
    }
    let x = 1;
    match x {
        e @ 1 ... 5 => println!("got a tang element {}", e),
        _ => println!("anything"),
    }

    let y = 5;
    let mut z = 5;
    match y {
        // the 'r' inside the match has the type '&i32'
        ref r => println!("Got a reference to {}", r),
    }
    match z {
        // the 'mr' inside the match has the type '&i32' and is mutable
        ref mut mr => println!("Got a Mutable reference to {}", mr),
    }

    let pair = (0, -2);
    match pair{
        (0, y) => println!("x is '0' and 'y' is '{:?}'", y),
        (x, 0) => println!("'x' is '{:?}' and 'y' is '0'", x),
        _ => println!("It doesn't matter what they are"),
    }

    struct Point{
        x: i32,
        y: i32,
    }
    let origin = Point {x: 2, y: 5};

    match origin {
        Point {x, ..} => println!("x is {}", x),
    }

    enum OptionalInt{
        Value(i32),
        Missing,
    }
    let x = OptionalInt::Value(5);

    match x{
        OptionalInt::Value(i) if i > 5 => println!("Got an int bingger than five!"),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck"),
    }

    let number = Some(7);
    let mut optional = Some(0);

    if let Some(i) = number {
        println!("Matched {:?}", i);
    }else {
        println!("Didn't match a number!");
    }

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        }else{
            println!("'i' is '{:?}. Try again.'", i);
            optional = Some(i + 1);
        }
    }
}
