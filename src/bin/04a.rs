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
    Array2::from_shape_vec(shape, tokens).unwrap()
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

fn check_location_direction(
    tokens: ArrayView2<Token>,
    i: usize,
    j: usize,
    direction: &Direction,
) -> bool {
    let i = i as i64;
    let j = j as i64;
    let (di, dj) = match direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
        Direction::UpRight => (-1, 1),
        Direction::UpLeft => (-1, -1),
        Direction::DownRight => (1, 1),
        Direction::DownLeft => (1, -1),
    };
    let token_offsets = [(Token::M, 1), (Token::A, 2), (Token::S, 3)];
    for (match_token, offset) in &token_offsets {
        let row = usize::try_from(i + di * offset);
        if row.is_err() {
            return false;
        }
        let col = usize::try_from(j + dj * offset);
        if col.is_err() {
            return false;
        }
        match tokens.get([row.unwrap(), col.unwrap()]) {
            Some(token) => {
                if token != match_token {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }
    true
}

fn check_location(tokens: ArrayView2<Token>, i: usize, j: usize) -> usize {
    let directions = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::UpRight,
        Direction::UpLeft,
        Direction::DownRight,
        Direction::DownLeft,
    ];
    directions
        .iter()
        .filter(|&d| check_location_direction(tokens, i, j, d))
        .count()
}

fn process(tokens: ArrayView2<Token>) -> usize {
    let mut out = 0;

    for ((i, j), token) in tokens.indexed_iter() {
        if token == &Token::X {
            out += check_location(tokens, i, j);
        }
    }
    out
}

fn main() {
    let out = include_str!("04_test.txt");
    let out = extract(out);
    // dbg!(&out);
    let out = process(out.view());
    assert_eq!(out, 18);

    let out = include_str!("04.txt");
    let out = extract(out);
    let out = process(out.view());
    assert_eq!(out, 2344);
    println!("{out}");
}
