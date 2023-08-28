use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut s_map: HashMap<char, i32> = HashMap::new();
    let mut t_map = HashMap::new();
    for char in s.chars() {
        let count = s_map.get(&char);
        if let Some(count) = count {
            s_map.insert(char, count + 1);
        } else {
            s_map.insert(char, 1);
        }
    };

    for char in t.chars() {
        let count = t_map.get(&char);
        if let Some(count) = count {
            t_map.insert(char, count + 1);
        } else {
            t_map.insert(char, 1);
        }
    };

    s_map == t_map
}

fn main() {
    let is_anagram = is_anagram("anagram".to_string(), "nagarfam".to_string());
    print!("{is_anagram}");
}