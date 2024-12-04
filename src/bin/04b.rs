use ndarray::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
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

fn check_location(tokens: ArrayView2<Token>, i: usize, j: usize) -> bool {
    let ul = tokens[[i - 1, j - 1]];
    let ur = tokens[[i - 1, j + 1]];
    let ll = tokens[[i + 1, j - 1]];
    let lr = tokens[[i + 1, j + 1]];
    match ul {
        Token::M => {
            if lr != Token::S {
                return false;
            }
        }
        Token::S => {
            if lr != Token::M {
                return false;
            }
        }
        _ => {
            return false;
        }
    }
    match ur {
        Token::M => {
            if ll != Token::S {
                return false;
            }
        }
        Token::S => {
            if ll != Token::M {
                return false;
            }
        }
        _ => {
            return false;
        }
    }
    true
}

fn process(tokens: ArrayView2<Token>) -> usize {
    let mut out = 0;

    let shape = tokens.shape();
    for ((i, j), token) in tokens.indexed_iter() {
        if i == 0 || j == 0 || i == shape[0] - 1 || j == shape[1] - 1 {
            continue;
        }
        if token == &Token::A {
            if check_location(tokens, i, j) {
                out += 1;
            }
        }
    }
    out
}

fn main() {
    let out = include_str!("04_test.txt");
    let out = extract(out);
    // dbg!(&out);
    let out = process(out.view());
    assert_eq!(out, 9);

    let out = include_str!("04.txt");
    let out = extract(out);
    let out = process(out.view());
    println!("{out}");
}
