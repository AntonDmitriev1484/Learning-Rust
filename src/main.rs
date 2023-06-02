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


enum Exp {
    Parentheses(String),
    Exponent(String, String),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Division(String, String),
    Num(String)
}

fn scan(str: String) -> Exp {

    let mut hasParens: Option<Exp> = None;
    let mut hasExpon: Option<Exp> = None;
    let mut hasAdd: Option<Exp> = None;
    let mut hasSub: Option<Exp> = None;
    let mut hasMul: Option<Exp> = None;
    let mut hasDiv: Option<Exp> = None;
    let mut isNum: Option<Exp> = None;


    let mut parensCounter:i32 = 0;
    let mut startParenIndex:i32 = 0;
    let mut endParenIndex:i32 = 0;

    let len: i32 = str.chars().count() as i32;

    for (i,c) in str.chars().enumerate() {
        let j = i as i32;

        let l_string = substring(str.clone(), 0, j);
        let r_string = substring(str.clone(), j+1, len);

        //Re-do this with a map lol
        match c {
            '+' => hasAdd = Some(Exp::Add(l_string, r_string)),
            '-' => hasSub = Some(Exp::Subtract(l_string, r_string)),
            '*' => hasMul = Some(Exp::Multiply(l_string, r_string)),
            '/' => hasDiv = Some(Exp::Division(l_string, r_string)),
            '^' => hasExpon = Some(Exp::Exponent(l_string, r_string)),
            '(' => {
                parensCounter += 1;
                startParenIndex = i as i32 + 1;
            }
            ')' => {
                parensCounter -= 1;
                if (parensCounter == 0) {
                    endParenIndex = i as i32;
                    hasParens = Some(Exp::Parentheses(substring(str.clone(), startParenIndex, endParenIndex)));
                }
            }
            _ => isNum = Some(Exp::Num(str.clone()))
        };

    };

    match hasParens {
        Some(exp) => exp,
        None =>
            match hasExpon {
                Some(exp) => exp,
                None =>
                    match hasMul {
                        Some(exp) => exp,
                        None =>
                            match hasDiv {
                                Some(exp) => exp,
                                None =>
                                    match hasAdd {
                                        Some(exp) => exp,
                                        None =>
                                            match hasSub {
                                                Some(exp) => exp,
                                                None =>
                                                    match isNum {
                                                        Some(exp) => exp,
                                                        None => Exp::Num(String::from(""))
                                                    }
                                            }
                                    }
                            }
                    }
            }
    }



}

fn evaluate(exp: Exp) -> i32 {
    let e = scan(String::from("5+(6*7)^9"));

    match e {

    Exp::Parentheses(x) =>
    println!("Inner expression: {}", x),

    _ => println!("wonkus")

    }

    0
}

fn calculator_PEMDAS() {
    
}

fn main() {

    //guessing_game();
    //calculator();
    //calculator_HOF();
    evaluate(Exp::Parentheses(String::from("5+(6*7)")));
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

