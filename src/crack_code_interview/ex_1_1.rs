use std::collections::HashMap;

fn main() {
    let string_all_unique     = "abcdefghij";
    let string_not_all_unique = "abcdeaghij";
    println!("{}", all_unique(&string_all_unique));
    println!("{}", all_unique(&string_not_all_unique));

    let string_permut_ok_1 = "abaccd";
    let string_permut_ok_2 = "aabdcc";
    let string_permut_ok_3 = "atbdcc";
    let string_permut_ok_4 = "aabdcca";
    println!("{}",permutation(string_permut_ok_1, string_permut_ok_2));
    println!("{}",permutation(string_permut_ok_1, string_permut_ok_3));
    println!("{}",permutation(string_permut_ok_1, string_permut_ok_4));
}

// no extra data structure allowed.
fn all_unique(string: &str) -> bool {
    let bytes = string.as_bytes();
    let len = string.len();
    let mut letter: u8;
    for i in 0..len{
        letter = bytes[i];
        for j in i+1..len{
            if letter == bytes[j]{
                return false
            }
        }
    }
    true  
}

fn permutation(str1: &str, str2: &str) -> bool {
    if str1.len() != str2.len() {
        return false
    }

    let mut str1_letters = HashMap::new();
    let mut str2_letters = HashMap::new();
    
    let b_1 = str1.as_bytes();
    let b_2 = str2.as_bytes();

    for i in 0..str1.len() {
       let count = str1_letters.entry(b_1[i]).or_insert(0);
       *count += 1; 
        let count = str2_letters.entry(b_2[i]).or_insert(0);
       *count += 1; 
    }
   
    for (key, value) in str1_letters.into_iter() {
        if str1_letters.get(&key) != value {
            return false
        }
    }
    true
}
