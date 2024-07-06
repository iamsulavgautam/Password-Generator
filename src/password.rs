use rand::Rng;
use rand::seq::SliceRandom;
use std::{
    sync::{Arc, Mutex},
    thread,
};

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


pub fn create_pass(length: u32, put_numbers: bool, put_symbols: bool) -> String {
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

fn retreive(character_set: &Set) -> char {
    let set = &character_set.characters;
    let length: u32 = character_set.length;

    let index = Arc::new(Mutex::new(0));
    let choice = Arc::new(Mutex::new(false));

    
    loop {
        let index_clone = Arc::clone(&index);
        let choice_clone = Arc::clone(&choice);

        let choosing = thread::spawn( move || {
            let num = rand::thread_rng().gen_range(0..=length);
            let mut value = index_clone.lock().unwrap();
            *value = num;
        });
        
        let deciding = thread::spawn( move || {
            let choose: bool = rand::random();
            let mut value = choice_clone.lock().unwrap();
            *value = choose;
        });
        
        choosing.join().unwrap();
        deciding.join().unwrap();
        
        let index = *index.lock().unwrap();
        let choice = *choice.lock().unwrap();

        if choice {
            return itereate(set, index);
        }
    }
}

fn itereate(list: &String, index: u32) -> char {
    let mut count: u32 = 0;
    let mut element: char = ' ';

    for item in list.chars() {
        element = item;
        if count == index { break; }
        count += 1;
    }
    return element;
}