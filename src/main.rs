use rand::Rng;
use std::io;

enum Fond {
    Fond(),
    Hide(),
}

struct Hengman_char {
    char: char,
    fond: Fond,
}

fn hengman_char(c : char) -> Hengman_char {
    let a = Hengman_char {
        char: c,
        fond: Fond::Hide,
        
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let word_list = ["pome", "patate"];
    let x = rng.gen_range(0, word_list.len()-1);

    let mut word :Vec<> = vec!();
    for i in word_list[x].chars() {
        word.push(i);
        print!("*");
    }
    println!("");
    
    loop {
         println!("enter your guest");
         let mut input = String::new();
        let input = match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input.clone()
            }
            Err(_) => panic!("input panic"),
        };
        // a blank input have a len of 2 invisible char
        //a input of 1 char have 2 invisible char + the real input
        if input.len() == 3 {
           
        }
        else if input.len() > 3 {
            println!("pls, do not put more then 1 char par guess");
            continue
        }
        else {
            println!("pls, enter a char");
            continue
        }
        
        let mut input_vec: Vec<char>= vec!();
        for i in input.chars() {
            input_vec.push(i)
        }

        let match_fond = false;

        for letter in word.clone() {
            if input_vec[0] == letter {
                
            }
        }
    }
}