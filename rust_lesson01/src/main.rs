fn main() {
    //  boolen type
    let t = true;
    let f = false;

    println!("t = {}", t);
    println!("f = {}", f);

    // char type
    let c = 'c';
    println!("char c = {}", c);

    // numberc type
    let x = 42;
    let y: u16 = 12;
    let z: u32 = 123_456;
    let w: f64 = 1.23e+2;
    let zero = w.min(123.4);
    let bin = 0b1111_0000;
    let otc = 0o7320_1546;
    let hex = 0o7320_1346;
    
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
    println!("w = {}", w);
    println!("zero = {}", zero);
    println!("bin = {}", bin);
    println!("otc = {}", otc);
    println!("hex = {}", hex);

    // string types
    let str = "Hello World!";
    println!("str = {}", str);
    let mut string = str.to_string();
    println!("string = {}", string);

    // array and slices
    let a = [0, 1, 2, 3, 4];
    println!("a's lenth is {}", a.len());
    let middle = &a[1..4];
    println!("middle's lenth is {}", middle.len());
    let mut ten_zeros: [i64; 10] = [0; 10];
    println!("ten_zeros's lenth is {}", ten_zeros.len())

    // tuples
    let tuple: (i32, &str) = (50, "hello");
    let (fifty, _) = tuple;
    let hello = tuple.1;

    // raw pointers
    let x = 5;
    let raw = &x as *const i32;
    ler points_at = unsafe { *raw };

    //funtion
    fn foo(x: i32) -> i32 { x };
    let bar: fn(i32) -> i32 = foo;

}
