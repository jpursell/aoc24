use ndarray::prelude::*;

#[derive(Clone, Copy, Debug)]
enum Token {
    X,
    M,
    A,
    S,
    O,
}

fn extract(str: &str) -> Array2<Token> {
    let nrows = str.lines().count();
    let ncols = str.lines().next().unwrap().chars().count();
    let shape = (nrows, ncols);
    let mut tokens = Vec::with_capacity(nrows * ncols);
    for line in str.lines() {
        for char in line.chars() {
            match char {
                'X' => tokens.push(Token::X),
                'M' => tokens.push(Token::M),
                'A' => tokens.push(Token::A),
                'S' => tokens.push(Token::S),
                _ => tokens.push(Token::O),
            }
        }
    }
    let tokens = Array2::from_shape_vec(shape, tokens).unwrap();
    tokens
}

// fn process(tokens: &[Token]) -> usize {
//     let mut out = 0;
//     let mut active = true;
//     for token in tokens {
//         match token {
//             Token::Do => {
//                 active = true;
//             }
//             Token::Dont => {
//                 active = false;
//             }
//             Token::Mul(val) => {
//                 if active {
//                     out += val
//                 }
//             }
//             _ => (),
//         }
//     }
//     out
// }
fn main() {
    let out = include_str!("04_test.txt");
    let out = extract(out);
    dbg!(&out);
    // let out = process(&out);
    // assert_eq!(out, 48);
}
