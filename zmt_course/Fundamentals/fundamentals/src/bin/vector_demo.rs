struct Test {
    score: i32,
}

fn main() {
    let my_scores = vec![
        Test { score: 7 },
        Test { score: 9 },
        Test { score: 2 },
        Test { score: 10 },
        Test { score: 5 },
    ];

    for test in &my_scores {
        println!("score  = {:?}", test.score);
    }
}