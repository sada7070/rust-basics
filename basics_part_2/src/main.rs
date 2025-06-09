// // create function is_even to check weather the number is even or not.
// fn is_even(n: i32) -> String {
//     if n % 2 == 0 {
//         return "even".to_string();
//     } else {
//         return "odd".to_string();
//     }
// }


// // fibbonacci series function for given number
// fn fib(num: u32) -> u32{
//     let mut first = 0;
//     let mut second = 1; 

//     if num == 0 {
//         return first;
//     }
//     if num == 1 {
//         return second;
//     }

//     for _ in 0..num-1 {
//         let temp = second;
//         second = second + first;
//         first = temp;
//     }

//     return second;
// }


// length of the string
fn len_of_string(str: String) -> usize {
    return str.chars().count();
}

fn main() {
    //let num = 6;
    // println!("{} is {} an number", num, is_even(num));
    //println!("{}", fib(num));

    let str = String::from("Hello world!!!");
    println!("{}", len_of_string(str));
}
