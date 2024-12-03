use counter::Counter;

fn extract_lists(str: &str) -> [Vec<usize>; 2] {
    let count = str.lines().count();
    let mut vecs = [Vec::with_capacity(count), Vec::with_capacity(count)];
    for line in str.lines() {
        let (a, b) = line.split_once(" ").unwrap();
        let a: usize = a.parse().unwrap();
        let b: usize = b.trim().parse().unwrap();
        vecs[0].push(a);
        vecs[1].push(b);
    }
    vecs
}

fn process_lists(vecs: [Vec<usize>; 2]) -> usize {
    let counter = vecs[1].iter().collect::<Counter<_>>();
    let mut out = 0;
    for a in vecs[0].iter() {
        out += a * counter[a];
    }
    out
}
fn main() {
    let test = include_str!("01.txt");
    let test = extract_lists(test);
    let test = process_lists(test);
    println!("hello: {test:?}");
}
