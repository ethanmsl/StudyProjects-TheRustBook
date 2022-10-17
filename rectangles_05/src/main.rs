#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "(Raw) The area of the rectangle is {} square pixels.",
        area_raw(width1, height1)
    );

//--------------------------
    let rectupl1 = (30, 50);
    println!(
        "(Tuple) The area of the rectangle is {} square pixels.",
        area_tuple(rectupl1)
    );

//----------------------------
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "(Struct) The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    println!("derived debug display of rect1 is: {:?}", rect1);
    println!("derived debug display of rect1 is: {:#?}", rect1);
//--------------------------
// looking at debug macro (dbg!)
    let scale = 3;
    println!("---'dbg!(30 * scale)'---");
    let rect2 = Rectangle {
        width: dbg!(30 * scale),  // *
        height: 50,
    };
    // NOTE: 'dbg!' macro takes and then gives ownership
    println!("---'dbg!(&rect1)'---");
    dbg!(&rect1);  // *
}

fn area_raw(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
