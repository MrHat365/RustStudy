use std::cell::Cell;

fn main() {
    // structs
    struct Point{
        x: i32,
        y: i32,
    }

    let point = Point{x: 0, y: 0};

    println!("Point's x =  {}, y = {}", point.x, point.y);


    // tuple structs

    struct Color (u8, u8, u8);
    let android_green = Color(0xa4,0xc6,0x39);
    println!("android's color is {}, {}, {}", android_green.0, android_green.1, android_green.2);
 
    // A tuple struct's constructors can be used as functions
    struct Digit(i32);
    let v = vec![0, 1, 2];
    let d: Vec<Digit> = v.into_iter().map(Digit).collect();

    // newtype: a tuple struct with only one element
    struct Inches(i32);
    let length =Inches(10);
    let Inches(integer_length)= length;

    // unit-like structs
    struct Null;
    let empty =Null;


    #[derive(Default)]
    struct Point3d{
        x: i32,
        y: i32,
        z: i32,
    }

    // 一个包含..的struct可以用来从其它结构体拷贝一些值或者在解构时忽略一些域：
    let origin = Point3d:: default();
    println!("origin detail is x = {}, y = {}, z = {}", origin.x, origin.y, origin.z);
    // origin detail is x = 0, y = 0, z = 0
    let point = Point3d{y: 1, ..origin};
    println!("point detail is x = {}, y = {}, z = {}", point.x, point.y, point.z);
    // point detail is x = 0, y = 1, z = 0
    let Point3d{x: x0, y: y0, ..} = point;
    println!("point changed is x = {}, y = {}, z = {}", point.x, point.y, point.z);
    // point changed is x = 0, y = 1, z = 0


    struct Pointx {
        x: i32,
        y: Cell<i32>,
    }
    let mut pointx = Pointx{x: 5, y: Cell::new(6)};
    println!("point detail is x = {}, y = {:?}", pointx.x, pointx.y);
    pointx.y.set(7);
    println!("point detail is x = {}, y = {:?}", pointx.x, pointx.y);
}
