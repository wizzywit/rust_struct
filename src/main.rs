// an example to calculate the area of a rectangle

//// using the basic normal way

// fn main() {
//     let width = 30;
//     let height = 50;
//     println!("The area of the rectangle is {} square pixels.", area(width, height));
// }

// fn area(width: u32, height: u32) -> u32{
//     width * height
// }

//// refactoring by making use of tuples

// fn main() {
//     let rect = (30,50);
//     println!("The area of the rectangle is {} square pixels.", area(rect));
// }

// fn area(dimensions: (u32, u32)) -> u32{
//     dimensions.0 * dimensions.1
// }

//// refactoring using struct
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32
// }

// // use {:?} to display a struct instance or {:#?} to properly format it
// // must include the #[derive(Debug)] just above the struct definition
// fn main() {
//     let rect = Rectangle {
//         width: 30,
//         height: 50
//     };
//     println!("rect instance is {:#?}",rect);
//     println!("The area of the rectangle is {} square pixels.", area(&rect));
// }

// fn area(rectangle: &Rectangle) -> u32{
//     rectangle.width * rectangle.height
// }

//// refactoring using methods implementation of a struct definition
/// 
/// 
/// Methods are defined within the context of a struct (inside an impl(implementation block))
/// alway have a first parameter called &self(representing the instance of the struct)
/// 
/// 
/// methods that dont take in the self params are refered to as associated functions and not methods
/// because they are associated with the struct itself and not an instance of the struct
/// 
/// 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // an associated function definition
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };
    let rect1 = Rectangle {
        width: 10,
        height: 40
    };

    let rect2 = Rectangle {
        width: 60,
        height: 45
    };

    let sq = Rectangle::square(4);
    println!("Can rect hold rect1? {}",rect.can_hold(&rect1));
    println!("Can rect hold rect2? {}",rect.can_hold(&rect2));
    println!("The area of the rectangle is {} square pixels.", rect.area());
    println!("square instance is {:#?}",sq);
}

