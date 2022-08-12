fn push() {
    let mut s1 = String::from("foo");
    let s2 = String::from("bar");
    s1.push_str(&s2);
    println!("{}", s2);
}
fn main() {
    push();

    let mut s1 = String::from("hello, ");
    let s2 = String::from("world");
    // let s2 = "world";
    // s1.push_str(s2);
    let s3 = s1 + &s2; // s1 has been removed

    // println!("s1 is {}", s1);
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);

    let hello = "Здравствуйте";
    let answer = &hello[0..4];

    println!("answer is {}", answer);

    let s = &hello[0..2];

    // for c in "नमस्ते".chars() {
    //     println!("{}", c);
    // }

    // for b in "नमस्ते".bytes() {
    //     println!("{}", b);
    // }
}
