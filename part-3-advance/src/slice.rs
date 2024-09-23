pub fn main() {
    let name = String::from("naresh rao");

    let ans = first_word(name);
    println!("{}",ans);
    main2();
}

fn first_word(name: String) -> String {
    let mut ans = String::from("");

    for i in name.chars() {
        if i == ' ' {
            break;
        }
        ans.push_str(&i.to_string());
    }
    return ans;
}


pub fn main2() {
    let word  = String::from("hellow orld");

    let word2 = first_word2(&word);
    
    println!("{}",word2);
    
}

fn first_word2(word: &str)-> &str{

    let mut index = 0;
    for i in word.chars() {
        if i == ' '{
            break;
        }
        index = index + 1;
    }

    return &word[0..index];
}