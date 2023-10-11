fn main() {
    let s = sum(12, 12);
    println!("{}", s);
    println!("{}", sum1(1, 2));
}

fn sum(i1: i32, i2: i32) -> i32 {
    let s = i1 * i2;
    s
}

fn sum1(i1: i32, i2: i32) -> i32 {
    i1 * i2
}
