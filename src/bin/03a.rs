use regex::Regex;
struct Mul {
    a: usize,
    b: usize,
}
fn extract(str: &str) -> Vec<Mul> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9])+\)").unwrap();
    let mut out = Vec::new();
    for (_, [a, b]) in re.captures_iter(str).map( |c|c.extract()) {
        out.push(Mul{a, b});
    }
    out
}

// fn process_line(a: &Vec<usize>, increasing: bool) -> bool {
//     for chunk in a.windows(2) {
//         let d = if increasing {
//             chunk[1] as i64 - chunk[0] as i64
//         } else {
//             chunk[0] as i64 - chunk[1] as i64
//         };
//         let ok = match d {
//             1 | 2 | 3 => true,
//             _ => false,
//         };
//         if !ok {
//             return false;
//         }
//     }
//     true
// }
// fn process_line_at(a: &[usize], loc: usize) -> bool {
//     let a: Vec<usize> = a
//         .iter()
//         .enumerate()
//         .filter(|(i, _x)| i != &loc)
//         .map(|(_i, x)| *x)
//         .collect();
//     process_line(&a, true) || process_line(&a, false)
// }
// fn process(vecs: Vec<Vec<usize>>) -> usize {
//     let mut out = 0;
//     for a in vecs.iter() {
//         for i in 0..a.len() {
//             let ok = process_line_at(a, i);
//             if ok {
//                 out += 1;
//                 break;
//             }
//         }
//     }
//     out
// }
fn main() {
    let out = include_str!("03_test.txt");
    let out = extract(out);
    let out = process(out);
    println!("{out:?}");
}
