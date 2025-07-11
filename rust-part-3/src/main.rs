// fn is_even(num: i32)-> bool {
//     if num % 2 == 0 {
//         return true;
//     }
//     return false;
// }

// fn main() {
//     println!("{}", is_even(50));
// }

// fn fibonacci(num: i32) -> i32 {
//     let mut first = 0;
//     let mut second = 1;

//     if num == 0 {
//         return first;
//     }

//     if num == 1 {
//         return second;
//     }

//     for _ in 0..(num - 1){
//         let temp = second;
//         second = second + first;
//         first = temp
//     }

//     return second;
// }

// fn main() {
//     println!("{}", fibonacci(4));
// }

// fn get_str_len(s: &str) -> usize {
//     s.chars().count()
// }

// fn main() {
//     let my_string = String::from("Hello world!!");
//     println!("{}", get_str_len(&my_string));
// }

//struct

// struct User {
//     first_name: String,
//     last_name: String,
//     age: i32,
// }

// fn main() {
//     let user = User{
//         first_name: String::from("Sada"),
//         last_name: String::from("shiva"),
//         age: 23,
//     };

//     println!("User details, firstName: {}, lastName: {}, age: {}", user.first_name, user.last_name, user.age);
// }

struct Rect {
    height: u32,
    width: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let rect = Rect {
        height: 3,
        width: 6
    };
        
    println!("Area of rect is: {}", rect.area());
}
