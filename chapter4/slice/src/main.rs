fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    println!("word: {}!", word);

    s.clear();
    println!("s now: {}!", s);


    let s1 = String::from("hello world");

    let word = first_word1(&s1);

    // s1.clear();

    println!("first wordld: {}", word);



    let my_string = String::from("hello world");
    let my_string1 = String::from("hello world");
    let my_string_literal = "hello world";


    let word2 = first_word2(&my_string);

    let word3 = first_word2(&my_string1[..]);

    let word4 = first_word2(&my_string_literal[..]);

    let word5 = first_word2(my_string_literal);


    println!("word2: {}", word2);
    println!("word3: {}", word3);
    println!("word4: {}", word4);
    println!("word5: {}", word5);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word1(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}