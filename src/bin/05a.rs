
// fn extract(str: &str) -> Array2<Token> {
//     let nrows = str.lines().count();
//     let ncols = str.lines().next().unwrap().chars().count();
//     let shape = (nrows, ncols);
//     let mut tokens = Vec::with_capacity(nrows * ncols);
//     for line in str.lines() {
//         for char in line.chars() {
//             match char {
//                 'X' => tokens.push(Token::X),
//                 'M' => tokens.push(Token::M),
//                 'A' => tokens.push(Token::A),
//                 'S' => tokens.push(Token::S),
//                 _ => tokens.push(Token::O),
//             }
//         }
//     }
//     Array2::from_shape_vec(shape, tokens).unwrap()
// }

// fn process(tokens: ArrayView2<Token>) -> usize {
//     let mut out = 0;

//     let shape = tokens.shape();
//     for ((i, j), token) in tokens.indexed_iter() {
//         if i == 0 || j == 0 || i == shape[0] - 1 || j == shape[1] - 1 {
//             continue;
//         }
//         if token == &Token::A && check_location(tokens, i, j) {
//             out += 1;
//         }
//     }
//     out
// }

fn main() {
    let _out = include_str!("05_test.txt");
    // let out = extract(out);
    // dbg!(&out);
    // let out = process(out.view());
    // assert_eq!(out, 143);

    // let out = include_str!("04.txt");
    // let out = extract(out);
    // let out = process(out.view());
    // assert_eq!(out, 1815);
    // println!("{out}");
}
