

fn extract(str:&str) -> Vec<Vec<usize>> {
    let count = str.lines().count();
    let mut vecs = Vec::with_capacity(count);
    for line in str.lines() {
        let count = line.split_whitespace().count();
        let mut vec = Vec::with_capacity(count);
        for val in line.split_whitespace() {
            let val: usize = val.parse().unwrap();
            vec.push(val);
        }
        vecs.push(vec);
    }
    vecs
}

fn process_line(a: &Vec<usize>) -> bool {
    for (a, b) in a.chunks(2) {
        match a.abs_diff(b) {
            todo!()
        }

    }
}
fn process(vecs: Vec<Vec<usize>>) -> usize {
    let mut out = 0;
    for a in vecs.iter() {
        for (a, b) in a.chunks(2) {

        }
        out += a * counter[a];
    }
    out
}
fn main() {
    let out = include_str!("02_test.txt");
    let out = extract(out);
    let out = process(out);
    println!("{out:?}");
}