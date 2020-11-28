use std::collections::HashMap;

/* TODO use a macro to generate CHAR_LIST and fn get_char_index()
     two_way_access!(CHAR_LIST, get_char_index, [a,b,c,f,...])
     expanding to:
        pub static CHAR_LIST: [char; 52] = [...]
        pub fn get_char_index(c: char) {
            match c {
                a => 0
                b => 1
                ...
            }
        }
*/

pub static CHAR_LIST: [char; 56] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', ' ', 'ó', '.', '0', '1', '2', '3', '4', '5', '6', '7', '8',
    '9', 'ù', 'à', 'é', 'è', ',', ':', ';', '_', '-', '+', '*', '/', '\\', '(', ')', '[', ']',
];

fn _compute_reversed_char_list() -> HashMap<char, i8> {
    CHAR_LIST
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, i as i8))
        .collect()
}

pub fn encode(s: &str) -> Vec<i8> {
    let reversed_char_list = _compute_reversed_char_list();
    s.chars()
        .map(|c| *reversed_char_list.get(&c).or(Some(&0)).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::encoder::simple_latin_symbols::encode;

    #[test]
    fn should_encode_string() {
        assert_eq!(
            encode("abcdefghijklmnopqrstuvwxyz"),
            vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25
            ]
        )
    }
}
