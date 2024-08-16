fn main() {
    // println!("Hello, world!");  // Print hello world

    // // --------------------------------1. NUMBERS---------------------------------- 
    // let x : i32 = 1;
    // let a  = 3;
    // let y : u32 = 4; // by default, if we dont give the type, it automatically take as i32
    // let z : f32 = 10000.001;
    // println!("x: {}, a: {}, y: {}, z: {}",x,a,y,z);





    // // --------------------------------2. BOOLEAN-----------------------------------
    // let is_male = false;
    // let is_above_18 = true;
    // if is_male{
    //     println!("The person is male.")
    // } else {
    //     println!("The person is not a male.")
    // }
    // if is_male && is_above_18 {
    //     println!("The person is a legal male")
    // } else {
    //     println!("Ther person is either not legal or is not a male or both.")
    // }




    // //--------------------------------- 3. STRINGS---------------------------------
    // let str = "hello, world";   // Declaring string this way gives the type as &str
    //     // don't have fixed type - means it can change its size at runtime.
    // println!("Greetings: {}", str);

    // let new_str = String::from("Hello World");
    // println!("{}", new_str);
    
    // let letter = new_str.chars().nth(1);

    // // print!("{}", letter);
    // // In the above cases, It will show error with the type for letter as Option<char>,  as rust is  strict and doesn't know if the char at nth exists or not.
    // match letter {
    //     Some(c) => println!("{}",c),
    //     None => println!("No character at index 1000"),
    // }



    // ----------------------------------FUNCTION-------------------------------------

    let a = 10;
    let b = 20;
    let sum = add(a, b);
    println!("Sum of {} and {} is {}",a,b,sum);


    fn add(a:i32, b:i32) -> i32 {
        return a+b;
    }

    //  we can define a function both inside and outside
    //  If we define the same function both outside and inside the main function, the inner one will take precedence.






    // //--------------------------CONDITIONAL AND LOOPS-------------------------------
    
    // //IF-ELSE 
    // let is_even = true;
    // if is_even {
    //     println!("The number is even");
    // } else if !is_even {
    //     println!("The number is odd");
    // }

    // //LOOPS
    // for i in 0..10{
    //     println!("Itearation with for loop {}", i);
    // }

    // // WHILE LOOP 
    // let mut i = 10;
    // while i>0 {
    //     println!("Iteration with while loop {}", i);
    //     i-=1;
    // }


    // // Iterating a string
    // let sentence = String::from("What are you doing.");
    // let first_word = get_first_word(sentence);
    // println!("The first word is {}", first_word);




}

// fn get_first_word(sentence:String) -> String {
//     let mut word = String::from("");
//     for char in sentence.chars(){
//         word.push_str(char.to_string().as_str());
//         if char == ' '{
//             break;
//         }
//     }
//     return word;
// }

