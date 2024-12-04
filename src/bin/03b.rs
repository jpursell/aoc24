use regex::Regex;
#[derive(Clone, Copy, Debug)]
enum Token {
    Do,
    Dont,
    Mul(usize),
    None,
}
fn extract(str: &str) -> Vec<Token> {
    let mul_re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    let mut tokens = vec![Token::None; str.len()];
    for cap in mul_re.captures_iter(str) {
        let loc = cap.get(0).unwrap().start();
        let (_, [a, b]) = cap.extract();
        let a: usize = a.parse().unwrap();
        let b: usize = b.parse().unwrap();
        let val = a * b;
        tokens[loc] = Token::Mul(val);
    }
    for cap in do_re.captures_iter(str) {
        let loc = cap.get(0).unwrap().start();
        tokens[loc] = Token::Do;
    }
    for cap in dont_re.captures_iter(str) {
        let loc = cap.get(0).unwrap().start();
        tokens[loc] = Token::Dont;
    }
    tokens
}

fn process(tokens: &[Token]) -> usize {
    let mut out = 0;
    let mut active = true;
    for token in tokens {
        match token {
            Token::Do => {
                active = true;
            }
            Token::Dont => {
                active = false;
            }
            Token::Mul(val) => {
                if active {
                    out += val
                }
            }
            _ => (),
        }
    }
    out
}
fn main() {
    let out = include_str!("03_testb.txt");
    let out = extract(out);
    // dbg!(&out);
    let out = process(&out);
    assert_eq!(out, 48);

    let out = include_str!("03.txt");
    let out = extract(out);
    let out = process(&out);
    println!("{out:?}");
}
