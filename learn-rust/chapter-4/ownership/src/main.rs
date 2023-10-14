fn main() {
    let s1 = String::from("hello world");

    let word_count = count_words(&s1);
    println!("Number of words: {}", word_count);

    let w = count_words1(&s1);
    print!("统计单词数量：{}", w)
}

fn count_words(s: &String) -> usize {
    s.split_whitespace().count()
}

fn count_words1(s: &String) -> usize {
    let mut word_count = 0;
    let mut in_word = false;

    for c in s.chars() {
        if c.is_whitespace() {
            if in_word {
                in_word = false;
            }
        } else {
            if !in_word {
                in_word = true;
                word_count += 1;
            }
        }
    }

    word_count
}
