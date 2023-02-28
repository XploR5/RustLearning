// //This is a code for gussing game

// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;
// //decorations
// use colorful::Color;
// use colorful::Colorful;

// fn main() {
//     println!("{}", "Guess the number!".gradient(Color::LightMagenta));
//     let secret_number: u8 = rand::thread_rng().gen_range(1..=100);
//     let mut count: u8 = 0;

//     loop {
//         println!("Enter guess");
//         // Reading User Input
//         let mut guess:String = String::new();
//         io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");
//         let guess: u8 = guess.trim().parse().expect("Please type a NUMBER!");

//         // Counting the number of guesses
//         count += 1;
//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("\nToo small!"),
//             Ordering::Greater => println!("\nToo big!"),
//             Ordering::Equal => {
//                 println!("\nYou win! in {} guesses", count);
//                 break;
//             }
//         }
//     }
// }

// References
// fn main() {
// let user_input: &str = "Rohan is Awesome";
//     seperate_words(user_input);
// }

// fn seperate_words(user_input: &str) {

//     for c in user_input.chars() {

//         if c == ' ' {
//             println!("");
//         }
//         else {
//             print!("{}", c);
//         }
//     }
// }

// // STRUCTS

// #[derive(Debug)]
// struct RightTriangle {
//     base: u32,
//     height: u32,
// }

// impl RightTriangle  {
//     fn area(&self)-> u32{
//         (self.base * self.height) / 2
//     }
//     fn can_hold(&self, other: &RightTriangle) -> bool{
//         self.base > other.base && self.height > other.height
//     }
// }

// fn main() {
//     let triangle1 = RightTriangle {
//         base: 4,
//         height: 5,
//     };

//     let triangle2 = RightTriangle { base: 2, height: 4 };
//     let triangle3 = RightTriangle { base: 7, height: 10 };

//     println!(
//         "The area of the {:?} is {} square pixels.", triangle1,
//         triangle1.area()
//     );

//     println!("{:?} can hold {:?} : {}", triangle1, triangle2, triangle1.can_hold(&triangle2));
//     println!("{:?} can hold {:?} : {}", triangle2, triangle3, triangle2.can_hold(&triangle3));
//     println!("{:?} can hold {:?} : {}", triangle3, triangle2, triangle3.can_hold(&triangle2));

// }

// #[derive(Debug)]
// enum Direction {
//     East,
//     West,
//     North(String),
//     South(String),
// }

// fn main() {
//     let travel_direction = Direction::North(String::from("North-East"));

//     match travel_direction {
//         Direction::East => println!("East"),
//         Direction::West => println!("West"),
//         Direction::North(sub_direction) => println!("North -> {}", sub_direction),
//         Direction::South(sub_direction) => println!("South -> {}", sub_direction),
//     }
// }

// fn main() {
//     let mut count = 0;
//     let mut sum = 0;
//     let numbers: Vec<i8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//     for number in numbers.iter() {
//         count += 1;
//         sum += number;
//     }
//     let average = sum / count;
//     println!("The average of the numbers is {}", average);
// }

// // Enum, Struct and pattern matching

// #[derive(Debug)]
// struct File {
//     name: String,
//     size: u64,
// }

// #[derive(Debug)]
// enum FileStatus {
//     Open(File),
//     Closed(File),
// }

// fn main() {
//     let file = File {
//         name: String::from("myfile.txt"),
//         size: 1024,
//     };

//     let status = FileStatus::Closed(file);

//     println!("{:#?}", status);

//     match status {
//         FileStatus::Open(f) => {
//             println!("The file {} is open", f.name);
//         }
//         FileStatus::Closed(f) => {
//             println!("The file {} is closed", f.name);
//         }
//     }
// }

// // HASHMAPS

// use std::collections::HashMap;

// fn main(){
//     let some_string: String = String::from("Hello Rohan How are you doing? Rohan you are doing great");

//     let word_collection: Vec<&str> = some_string.split(" ").collect();

//     let mut word_map: HashMap<&str, u8> = HashMap::new();

//     for word in word_collection.iter() {
//         let count = word_map.entry(word).or_insert(0);
//         *count += 1;
//     }

//     println!("{:#?}", word_map);
// }

// fn main() {
//     let apple: bool = true;

//     if apple {
//         println!("Apple is true");
//     }
//     else {
//         println!("Apple is false");
//     }

//     let result = if apple { "Apple is true" } else { "Apple is false" };
//     print!("res: {}", result)

//     match apple {
//         true => println!("Apple is true"),
//         false => println!("Apple is false"),
//     }
// }

// // GENERICS

// fn main() {
//     let num_vector: Vec<i32> = vec![1, 2, 3, 4, 16, 55, 6, 7, 29, 8, 9];
//     let char_vector: Vec<char> = vec![
//         'q', 'r', 'u', 'c', 'd', 'a', 'b', 's', 't', 'k', 'l', 'm', 'n',
//     ];

//     println!("The largest number is {}", find_largest(&num_vector));
//     println!("The largest char is {}", find_largest(&char_vector));
// }

// fn find_largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// TRIATS

// struct Tweet {
//     auther: String,
//     content: String,
//     share: bool,
// }

// impl ContentSize for Tweet {
//     fn content_size(&self) -> u32 {
//         self.auther.len() as u32 + self.content.len() as u32
//     }
// }

// struct Book {
//     auther: String,
//     title: String,
//     content: String,
// }

// impl ContentSize for Book {
//     fn content_size(&self) -> u32 {
//         self.auther.len() as u32 + self.title.len() as u32 + self.content.len() as u32
//     }
// }

// trait ContentSize {
//     fn content_size(&self) -> u32;
// }

// fn main() {

//     let some_book = Book {
//         auther: String::from("Rohan"),
//         title: String::from("Rohan's Book"),
//         content: String::from("This is a book written by Rohan"),
//     };

//     let some_tweet = Tweet {
//         auther: String::from("Rohan"),
//         content: String::from("This is a tweet written by Rohan"),
//         share: true,
//     };

//     println!("The size of the book is {}", some_book.content_size());
//     println!("The size of the tweet is {}", some_tweet.content_size());

// }



// struct Tweet {
//     auther: String,
//     content: String,
//     share: bool,
// }

// impl Tweet {
//     fn content_size(&self) -> u32 {
//         self.auther.len() as u32 + self.content.len() as u32
//     }
// }

// struct Book {
//     auther: String,
//     title: String,
//     content: String,
// }

// impl Book {
//     fn content_size(&self) -> u32 {
//         self.auther.len() as u32 + self.title.len() as u32 + self.content.len() as u32
//     }
// }

// // trait ContentSize {
// //     fn content_size(&self) -> u32;
// // }

// fn main() {

//     let some_book = Book {
//         auther: String::from("Rohan"),
//         title: String::from("Rohan's Book"),
//         content: String::from("This is a book written by Rohan"),
//     };

//     let some_tweet = Tweet {
//         auther: String::from("Rohan"),
//         content: String::from("This is a tweet written by Rohan"),
//         share: true,
//     };

//     println!("The size of the book is {}", some_book.content_size());
//     println!("The size of the tweet is {}", some_tweet.content_size());

// }


// // RUST LIFETIMES

fn main(){
    let sentence1: String = String::from("I am Rohan");
    let sentence2: String = String::from("Rohan is Awesome!");

    let longest: &str = longest(&sentence1, &sentence2.as_str());
    println!("The longest sentence is: {}", longest);
}

fn longest<'a>(x: &'a str,y:  &'a str) -> &'a str {

    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

// Two ways for string literals

// fn print_string(s: &str) {
//     println!("{}", s);
// }

// fn main() {
//     let my_string = String::from("Hello, world!");
//     print_string(my_string.as_str()); // print one
//     print_string(&my_string); // print two
// }

// fn main(){

// }