use crate::tasks::task_3::calculate_eulers_function;
use crate::tasks::task_5::remainder;
use crate::tasks::utils;

use std::collections::HashMap;

pub struct OpenKey {
    exp: i64,
    n: i64
}

pub struct HiddenKey {
    exp: i64,
    n: i64
}

pub struct Alphabet {
    pairs: HashMap<char, i64>,
    reverse_pairs: HashMap<i64, char>
}

impl Alphabet {
    pub fn new() -> Self {
        Self {
            pairs: HashMap::new(),
            reverse_pairs: HashMap::new()
        }
    }

    pub fn insert(&mut self, symbol: char, value: i64) -> bool {
        if !self.pairs.contains_key(&symbol) && !self.reverse_pairs.contains_key(&value) {
           self.pairs.insert(symbol, value);
           self.reverse_pairs.insert(value, symbol);

           return true;
       }
       else {
           false
       }
    }

    pub fn get_char(&self, value: &i64) -> Option<&char> {
        self.reverse_pairs.get(value)
    }

    pub fn get_value(&self, symbol: &char) -> Option<&i64> {
        self.pairs.get(symbol)
    }
}

pub fn get_rsa_keys(p: i64, q: i64) -> (OpenKey, HiddenKey) {
    let n = p * q;
    let phi = (p - 1) * (q - 1); 
    let open_exp = phi - 1;
    let hidden_exp = utils::euclidean_algorithm(phi, open_exp).1;

    let open_key = OpenKey { exp: open_exp, n };
    let hidden_key = HiddenKey { exp: hidden_exp, n };

    (open_key, hidden_key)
}

pub fn rsa_encode(open_key: &OpenKey, text: &String, alphabet: &Alphabet) -> Vec<i64> {

    let mut encoded_text = Vec::with_capacity(text.len());
    let mut text_chars = text.chars().into_iter();
    
    for symbol in text_chars {
        let char_value = alphabet.get_value(&symbol).unwrap();
        let encoded_char_id = remainder(*char_value, open_key.exp, open_key.n).unwrap();
        encoded_text.push(encoded_char_id);
    }
    
    encoded_text
}

pub fn rsa_decode(hidden_key: &HiddenKey, text: &Vec<i64>, alphabet: &Alphabet) -> String {

    let mut decoded_text = String::with_capacity(text.len());
    
    for symbol in text.iter() {
        let decoded_char_id = remainder(*symbol, hidden_key.exp, hidden_key.n).unwrap();
        let decoded_char = alphabet.get_char(&decoded_char_id).unwrap();
        decoded_text.push(*decoded_char);
    }
    
    decoded_text
}
