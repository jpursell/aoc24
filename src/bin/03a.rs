use regex::Regex;
#[derive(Debug)]
struct Mul {
    a: usize,
    b: usize,
}
fn extract(str: &str) -> Vec<Mul> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut out = Vec::new();
    for (_, [a, b]) in re.captures_iter(str).map(|c| c.extract()) {
        let a = a.parse().unwrap();
        let b = b.parse().unwrap();
        out.push(Mul { a, b });
    }
    out
}

fn process(vecs: Vec<Mul>) -> usize {
    let mut out = 0;
    for a in vecs.iter() {
        out += a.a * a.b;
    }
    out
}
fn main() {
    // let out = include_str!("03_test.txt");
    // let out = extract(out);
    // let out = process(out);
    // assert_eq!( out , 161);

    let out = include_str!("03.txt");
    let out = extract(out);
    let out = process(out);
    assert!(out > 1552138);
    println!("{out:?}");
}
