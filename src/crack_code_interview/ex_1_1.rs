
fn main() {
    let string_all_unique     = "abcdefghij";
    let string_not_all_unique = "abcdeaghij";
    println!("{}", all_unique(&string_all_unique));
    println!("{}", all_unique(&string_not_all_unique));
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
