fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let s1 = String::from("apple");
    let result;

    {
        let s3 = String::from("cherry");
        result = longest(s1.as_str(), &s3);
        println!("The longest string is: {}", result);
    }

    println!("The longest string is: {}", result);
}
