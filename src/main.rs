use rand::Rng;
use rand::seq::SliceRandom;

struct Set {
    characters: String,
    length: u32,
}

impl Set {
    /// Creates a new Set.
    fn new(characters: String, length: u32) -> Set {
        Set { characters: characters, length: length }
    }
}

fn main(){
    let password: String = create_pass(32, true, true);
    println!("{}", password)
}

fn retreive(character_set: &Set) -> char {
    // returns the character at a random index
    let index = rand::thread_rng().gen_range(0..character_set.length);

    let mut choice: char = ' ';
    loop {
        let mut count: u32 = 1;
        for element in character_set.characters.chars() {
            if count == index {
                choice = element;
                break;
            }
            count += 1;
        }

        // decied weather to choose the currently selected item or not
        let choose: bool = rand::random();
        if choose == true {
            return choice;
        }
    }
}

fn create_pass(length: u32, put_numbers: bool, put_symbols: bool) -> String {
    // lower and upper cases are by default
    let lower_alphabets = String::from("abcdefghijklmnopqrstuvwxyz");
    let upper_alphabets = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let numbers = String::from("0123456789");
    let symbols = String::from("!&\";#$%&'()*+,-./:;<=>?@[\\]^_`{|}~");

    let mut options = vec![
        Set::new(lower_alphabets, 26),
        Set::new(upper_alphabets, 26),
    ];
    if put_numbers {
        options.push(Set::new(numbers, 10));
    }
    if put_symbols {
        options.push(Set::new(symbols, 34));
    }

    let mut password = String::from("");
    for _i in 1..length {
        let set: &Set = options.choose(&mut rand::thread_rng()).unwrap();
        let character: char = retreive(set);
        password.push_str(&character.to_string());
    }

    return password;
}