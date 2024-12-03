fn extract_lists(str: &str) -> [Vec<u32>; 2] {
    let count = str.lines().count();
    let mut vecs = [Vec::with_capacity(count), Vec::with_capacity(count)];
    for line in str.lines() {
        let (a, b) = line.split_once(" ").unwrap();
        let a: u32 = a.parse().unwrap();
        let b: u32 = b.trim().parse().unwrap();
        vecs[0].push(a);
        vecs[1].push(b);
    }
    vecs
}

fn process_lists(mut vecs: [Vec<u32>; 2]) -> u32 {
    vecs[0].sort();
    vecs[1].sort();
    let mut diff = 0;
    for (a, b) in vecs[0].iter().zip(vecs[1].iter()) {
        diff += a.abs_diff(*b);
    }
    diff
}
fn main() {
    let test = include_str!("01.txt");
    let test = extract_lists(test);
    let test = process_lists(test);
    println!("hello: {test:?}");
}
