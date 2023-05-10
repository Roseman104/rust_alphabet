fn main() {
    // println!("Hello, world!");

    //println!("Let's Increment a to b: {}", increment_alphabet("a"));
    //println!("Now Let's do the rest!");
    //loop_alphabet();
    //loop_alphabet_new();
    //loop_alphabet_03();
    loop_alphabet_04();
    // let mut s = String::from("a");
    // for _ in 0..26 {
    //     println!("{}", s);
    //     s = increment_alphabet(&s);
    // }
}

fn loop_alphabet_01() {
    let mut s = String::from("a");
    for _ in 0..26 {
        println!("{}", s);
        s = increment_alphabet_01(&s);
    }
}

fn loop_alphabet_02() {
    let mut s = String::from("a");
    for _ in 0..26 * 26 * 26 + 1 {
        println!("{}", s);
        s = increment_alphabet_02(&s);
    }
}

fn loop_alphabet_03() {
    let mut s = String::from("a");
    for _ in 0..26 * 26 + 1 {
        println!("{}", s);
        s = increment_alphabet_03(&s);
    }
}

fn loop_alphabet_04() {
    let mut s = String::from("a");
    for _ in 0..26 * 26 * 26 + 1 {
        println!("{}", s);
        s = increment_alphabet_04(&s);
    }
}

fn increment_alphabet_01(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    // Increment the last character if it's not 'z'
    if chars[len - 1] != 'z' {
        chars[len - 1] = (chars[len - 1] as u8 + 1) as char;
    } else {
        // If the last character is 'z', carry over the increment to the previous characters
        let mut carry = true;
        for i in (0..len).rev() {
            if carry {
                if chars[i] == 'z' {
                    chars[i] = 'a';
                } else {
                    chars[i] = (chars[i] as u8 + 1) as char;
                    carry = false;
                }
            }
        }
        // If we need to carry over the increment to the beginning of the string, add a new 'a' character
        if carry {
            chars.insert(0, 'a');
        }
    }
    chars.iter().collect()
}

fn increment_alphabet_02(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    // Increment the last character if it's not 'z'
    if chars[len - 1] != 'z' {
        chars[len - 1] = (chars[len - 1] as u8 + 1) as char;
    } else {
        // If the last character is 'z', carry over the increment to the previous characters
        let mut carry = true;
        for i in (0..len).rev() {
            if carry {
                if chars[i] == 'z' {
                    chars[i] = 'a';
                } else {
                    chars[i] = (chars[i] as u8 + 1) as char;
                    carry = false;
                }
            }
        }
        // If we need to carry over the increment to the beginning of the string, add a new 'a' character
        if carry {
            chars.insert(0, 'a');
        }
    }

    chars.iter().collect()
}

fn increment_alphabet_03(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    // Increment the last character if it's not 'z'
    if chars[len - 1] != 'z' {
        chars[len - 1] = (chars[len - 1] as u8 + 1) as char;
    } else {
        // If the last character is 'z', carry over the increment to the previous characters
        let mut carry = true;
        for i in (0..len - 1).rev() {
            if carry {
                if chars[i] == 'z' {
                    chars[i] = 'a';
                } else {
                    chars[i] = (chars[i] as u8 + 1) as char;
                    carry = false;
                }
            }
        }
        // Handle the last character separately
        if carry {
            let last_char = chars[len - 1];
            if last_char == 'z' {
                chars[len - 1] = 'a';
                chars.push('a');
            } else {
                chars[len - 1] = (last_char as u8 + 1) as char;
            }
        }
    }

    chars.iter().collect()
}

fn increment_alphabet_04(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    // Increment the last character if it's not 'z'
    if chars[len - 1] != 'z' {
        chars[len - 1] = (chars[len - 1] as u8 + 1) as char;
    } else {
        // If the last character is 'z', carry over the increment to the previous characters
        let mut carry = true;
        for i in (0..len - 1).rev() {
            if carry {
                if chars[i] == 'z' {
                    chars[i] = 'a';
                } else {
                    chars[i] = (chars[i] as u8 + 1) as char;
                    carry = false;
                }
            }
        }
        // Handle the last character separately
        if carry {
            if len == 1 {
                chars[len - 1] = 'a';
                chars.push('a');
            } else {
                let last_char = chars[len - 2];
                if last_char == 'z' {
                    chars[len - 2] = 'a';
                    chars[len - 1] = 'a';
                    chars.push('a');
                } else {
                    chars[len - 2] = (last_char as u8 + 1) as char;
                    chars[len - 1] = 'a';
                }
            }
        }
    }

    chars.iter().collect()
}
