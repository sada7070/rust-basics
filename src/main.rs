// fn main() {
    
//     // numbers in rust
//     let x: u32 =  5;        // 'u' means unsigned integer i.e., it will always positive
//     let y: i8 = -2;        // 'i' unsigned integer i.e., it can be both +ve and -ve
//     let z = 0.5;       // 'f' - float

//     println!("x: {}, y: {}, z: {}", x, y, z);
//     println!("{}, {}, {}", x, y, z);

//     // boolean and conditional rendering(if else) in rust
//     let is_male = true;
//     let mut is_above_18 = false;                        // 'mut' to make the variable  mutable.

//     if is_male {
//         println!("You are male!");
//     } else if !is_male {
//         println!("You are female!");
//     }

//     if is_above_18 && is_male {
//         println!("You are not legle male!");
//     }

//     is_above_18 = true;

//     if is_above_18 && is_male {
//         println!("You are legle male!");
//     }

//     // string in rust

//      let greeting = String::from("hello world!");
//      println!("{}", greeting);

//      // to print a character of a string
//      let char1 = greeting.chars().nth(1);

//      println!("char1: {}", char1.unwrap());         // one way to print char using upwrap(). it gives error during run time if there is not char at requested index.(if unwrap() is not used it will give err now only)

//      //best approach to print
//      match char1 {
//          Some(c) => println!("char1: {}", c),
//          None => println!("No character at the requested index."),
//      }

//      // for loop in rust

//      for  i in 0..10 {
//         println!("{}", i);
//      }

//      // function
//      let sentence = String::from("rust basics from Harkirat");
//      let first_word = get_first_word(sentence);

//      println!("{}", first_word);

// }

// fn get_first_word(sentence: String) -> String {                 // we should also mention the type the return value with '->' symbol as shown
//     let mut ans = String::from("");
//     for char in sentence.chars() {
//         ans.push_str(char.to_string().as_str());

//         // ending loop immideatly when a white space occured so that we get onlt first word.
//         if char == ' ' {
//             break;
//         }
//     }
//     return  ans;
// }


// heap v/s stack
// fn main() {
//     stack_fn();
//     heap_fn();
//     update_fn();
// }

// fn stack_fn() {                                                             // just stored in stack
//     let a = 5;
//     let b = 2;
//     let ans = a + b;
//     println!("Sum: {}", ans);
// }

// fn heap_fn() {                                                              // stored in heap since string can be  changed during run time
//     let str1 = String::from("Hello");
//     let str2 = String::from("World!");

//     let concat_string = format!("{} {}", str1, str2);

//     println!("Concat string is: {}", concat_string);
// }

// fn update_fn() {
//     let mut s = String::from("Hello everyone!");
//     println!("Before update: {}", s);
//     println!("capacity: {}, length: {}, pointer: {:p}", s.capacity(), s.len(), s.as_ptr());

//     s.push_str(" How are you guys?");
//     println!("Updated str: {}", s);
//     println!("capacity: {}, length: {}, pointer: {:p}", s.capacity(), s.len(), s.as_ptr());

//     for _ in 1..1000 {
//         s.push_str(" How are you guys?");
//         //println!("Updated str: {}", s);
//         println!("capacity: {}, length: {}, pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
//     }

// }

// mutation and references
// fn main() {
//     let mut str = String::from("hello");
//     let s2 = &mut str;
//     //borrow_fn(&mut str);                                                      // once the variable got mutable reference, no other variable can get any type of references

//     //println!("{}", str);
//     println!("{}", s2);
// }

// fn borrow_fn(s: &mut String) {
//     s.push_str(" world");
// }

// // Structs in rust(similar to objects in js/ts)

// // first defining the object values
// struct User {                                       // example 1
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_sount: u64
// }

// struct Rect {                                       // example 2 with struct implimentation
//     height: u32,
//     width: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         self.height * self.width
//     }
// }

// fn main() {
//     let rect = Rect {                           // struct implementation
//         width: 10,
//         height: 5,
//     };

//     println!("Area of a rectangle: {}", rect.area());


//     // example 1
//     let user1 = User{
//         active: true,
//         username: String::from("Sada"),
//         email: String::from("sada@gmail.com"),
//         sign_in_sount: 10
//     };

//     println!("User1 UserName is: {}", user1.username);
// }



// // enum & patter matching in rust

// enum Shape {
//     Circle(f64),
//     Rectangle(f64, f64),
//     Square(f64),
// }

// fn calculate_area(shape: Shape) -> f64 {
//     match shape {
//         Shape::Circle(radius) => 3.14 * radius * radius,
//         Shape::Rectangle(height, width) => height * width,
//         Shape::Square(height) => height * height
//     }
// }

// fn main() {
//     let circle = Shape::Circle(5.0);
//     let rectangle = Shape::Rectangle(5.0, 10.0);
//     let square = Shape::Square(5.0);

//     println!("Area of Circle: {}", calculate_area(circle));
//     println!("Area of Reactangle: {}", calculate_area(rectangle));
//     println!("Area of Square: {}", calculate_area(square));
// }


// // error handlig in rust using enum
// use std::fs;

// fn main() {
//     let res = fs::read_to_string("example.txt");

//     // this is similar to try catch in js
//     match  res {
//         Ok(content) => {
//             println!("Content is: {}", content);
//         },
//         Err(err) => {
//             println!("{}", err);
//         }
//     }

//     println!("After error handling");
// }



// there is no null in rust, instead we use 'Option' which as 'Some' and 'None'

fn find_first_a(s: String) -> Option<i32> {
    for(index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

fn main() {
    let str = String::from("sada");

    match find_first_a(str) {
        Some(index) => println!("The first 'a' found at index: {}", index),
        None => println!("The string does not contain character 'a' in it."),               // without returning 'null' we should return something to avoid error which may cause.
    }
}