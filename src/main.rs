use std::collections::HashMap;
use std::env;
use std::mem;
use std::io;
use std::str::FromStr;


fn substring_stupid(str: String, minInclusive: i32, maxExclusive: i32) -> String {

    let mut i: i32 = 0;

    let f = |acc: Vec<char>, x: char| {

        if (minInclusive <= i && i < maxExclusive) { // Capture min and max parameters in the closure

            let mut vec2 = acc.clone(); //Copy existing array into a mutable variable
            vec2.push(x); //Add single element to it
            let vec2 = vec2; //Re-bind it to be immutable

            i+=1; //Capture a mutable 'i' within the closure to keep track of index location within the vector
            return vec2
        } else {
            i+=1;
            return acc
        }

    };

    let char_iter = str.chars();

    let acc:Vec<char> = Vec::new(); // Initialize accumulator
    let str_vec:Vec<char> = char_iter.fold(acc, f);

    str_vec.into_iter().collect()

}

fn substring(str: String, minInclusive: i32, maxExclusive: i32) -> String {

    let mut i: i32 = 0;

    let f = |acc: String, x: char| {
        if minInclusive <= i && i < maxExclusive { // Capture min and max parameters in the closure
            i+=1; //Capture a mutable 'i' within the closure to keep track of index location within the vector
            acc+(&x.to_string())
            // String defines a concatenation operator that performs String+&str = new String
        } else {
            i+=1;
            acc
        }

    };

    let char_iter = str.chars();
    let acc:String = String::new(); // Initialize accumulator
    char_iter.fold(acc, f)

}



fn guessing_game() -> () {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input);

    let input = input.trim_end(); // re-bind input to not be mutable

    let target: String = String::from("Five");

    if (input==target) {
        println!("You guessed the right string!");
        ()
    }
    else {
        println!("{} is wrong, try again!", input);
        guessing_game();
    }
}

fn calculator() -> () {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input);
    let input: &str = input.trim_end();

    let mut op_flag: bool = false;

    //Look for how to re-write this with calling a lambda on string search

    //input.chars() returns an iterator over input characters
    for (i,c) in input.chars().enumerate() {
        if !c.is_ascii_digit() {
            let op = c;
            let s1 = &input[0..i];
            let s2 = &input[i+1..input.len()];
            let operand1: f32 = f32::from_str(s1).unwrap();
            let operand2: f32 = f32::from_str(s2).unwrap();

            println!(" {} {} {} ", operand1, c, operand2);

            match op {
                '+' => println!("= {}", operand1 + operand2),
                '-' => println!("= {}", operand1 - operand2),
                '*' => println!("= {}", operand1 * operand2),
                '/' => println!("= {}", operand1 / operand2),
                '^' => println!("= {}", operand1.powf(operand2)),
                _ => println!("Not a valid operand!")
            }

            op_flag = true;
        }
    }

    if (!op_flag) { println!("Add an operator you kablunkus")}

    calculator()
}


fn run_on_find_in_string<P:Fn(char)->bool, R:Fn(char, i32, String)->()>
    (predicate: P,
     run: R,
     str: String) {

    for (i,c) in str.chars().enumerate() {
        if (predicate(c)) {
            run(c, i as i32, str.clone());
        }
    }
}

fn calculator_HOF() -> () {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input);
    let input: &str = input.trim_end();

    let predicate = |c: char| -> bool {
        !c.is_ascii_digit()
    };

    let calculate = |op: char, i: i32, input: String| -> (){

        // Strings pass by reference by default
        // once you pass something by reference, you lose owernship of it to the function
        // i.e. ownership has been moved to the other function
        // So, I need to manually clone input in at least 2 of the 3 places I pass it to the function

        // &substring() is to convert from String -> &str
        let operand1: f32 = f32::from_str(&substring(input.clone(), 0, i)).unwrap(); //Clone input here

        let operand2: f32 = f32::from_str(&substring(input.clone(), i + 1, input.len() as i32)).unwrap();
        // Give up ownership here, 'move' ownership of input to substring but thats fine since I no longer need it.

        println!(" {} {} {} ", operand1, op, operand2);

        match op {
            '+' => println!("= {}", operand1 + operand2),
            '-' => println!("= {}", operand1 - operand2),
            '*' => println!("= {}", operand1 * operand2),
            '/' => println!("= {}", operand1 / operand2),
            '^' => println!("= {}", operand1.powf(operand2)),
            _ => println!("Not a valid operand!")
        }
    };

    run_on_find_in_string(predicate, calculate, String::from(input));

    calculator_HOF()
}


// fn evaluate(exp: Exp) -> i32 {
//     let e = scan(String::from("5+(6*7)^9"));
//
//     match e {
//
//     Exp::Parentheses(x) =>
//     println!("Inner expression: {}", x),
//
//     _ => println!("wonkus")
//
//     }
//
//     0
// }





fn tokenize(str: String)->Vec<String> {
    let mut tokens: Vec<String> = Vec::new();

    let mut operations: HashMap<char, Box<Fn(i32, i32)->i32>> =
    //I don't understand
    // why on earth
    // I need to wrap a box around this
    // I should just use the fucking std add operator, at this point it might be more straightforward
    // than whatever the hell this is.
        HashMap::from(
            [('+', fuck you),
                ('-', (|x, y| x-y)),
                ('*', (|x, y| x*y)),
                ('/', (|x, y| x/y)),
                ('^', (|x, y| x^y))
            ]
        );

    let mut num: String = String::new();

    let complete_num = || {
        tokens.push(num);
        num = String::new();
    };

    for (i,c) in str.chars.enumerate() {

        match operations.get(c) {
            Some(f) => {
                tokens.push(c);
                complete_num();
            },
            None =>
                match c {
                    '(' | ')' => {
                        tokens.push(c);
                        complete_num();
                    },
                    _ => {
                        if (c.is_digit(10)) {
                            num += c;
                        } // Otherwise its a character
                    }

                }
        }

    }

    tokens
}

fn calculator_PEMDAS() {

    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let input: &str = input.trim_end();

    let mut Queue: Vec<char> = Vec::new();
    let mut Stack: Vec<char> = Vec::new();


    let v: Vec<String> = tokenize(String::from(input));
    println!("tokens: {}", v.reduce(String::new(), |acc, x| acc+x));

    // for (i,c) in tokenize(String::from(input)).iter() {
    //
    // }



    let mut ArithmeticStack: Vec<i32> = Vec::new();

    
}

fn main() {

    //guessing_game();
    //calculator();
    //calculator_HOF();
    //evaluate(Exp::Parentheses(String::from("5+(6*7)")));
    calculator_PEMDAS();
}



//-> impl Fn(c: char) -> impl Fn(i: i32)

// Currying is giving me a stroke!
// TLDR: Don't use currying in this language lol
// fn calc(input: String) ->
//                        impl Fn( &dyn Fn(char)->bool ) ->
//                             Box<dyn Fn( &dyn Fn(char, i32, String)->() )> {
//
//     move |predicate| {
//         Box::new(
//             move |on_predicate| {
//                 for (i,c) in input.chars().enumerate() {
//                     if (predicate(c)) {
//                         on_predicate(c,i,input)
//                     }
//                 }
//             }
//         )
//     }
// }

