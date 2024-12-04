use regex::Regex;
#[derive(Debug)]
struct Mul {
    a: usize,
    b: usize,
}
fn extract(str: &str) -> Vec<Mul> {
    let mul_re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let mut out = Vec::new();
    let mut active = true;
    do_re.
    for i in 0..str.len() {
        if do_re.is_match_at(str, i) {
            active = true;
        } else if dont_re.is_match_at(str, i) {
            active = false;
        } else if active && mul_re.is_match_at(str, i) {
            let cap = mul_re.captures_at(str, i).unwrap();
            let (_, [a, b]) = cap.extract();
            let a = a.parse().unwrap();
            let b = b.parse().unwrap();
            dbg!(&i);
            let mul = Mul{a, b};
            dbg!(&mul);
            out.push(mul);
        }
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
    let out = include_str!("03_test.txt");
    let out = extract(out);
    // dbg!(&out);
    let out = process(out);
    assert_eq!( out , 48);

    // let out = include_str!("03.txt");
    // let out = extract(out);
    // let out = process(out);
    // assert!(out > 1552138);
    // println!("{out:?}");
}
