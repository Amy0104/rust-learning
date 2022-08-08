use std::collections::HashMap;

fn base() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // for (key, value) in scores {
    //     println!("{},{}", key, value);
    // }
    println!("{:?}!", scores.get("Blue"));
}

fn biz() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<String, i32> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    for (key, value) in scores {
        println!("{},{}", key, value);
    }
}

fn entry() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(25);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

fn update() {
    let text = "Hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn main() {
    // base();
    // biz();
    // entry();
    // update();
    let v = vec![-5, 4, 1, -3, 2];
    let median = v.get(v.len() / 2);
    println!("{:?}", median.unwrap());
}
