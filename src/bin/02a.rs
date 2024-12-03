fn extract(str: &str) -> Vec<Vec<usize>> {
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

fn process_line(a: &Vec<usize>, increasing: bool) -> bool {
    for chunk in a.windows(2) {
        let d = if increasing {
            chunk[1] as i64 - chunk[0] as i64
        } else {
            chunk[0] as i64 - chunk[1] as i64
        };
        let ok = match d {
            1 | 2 | 3 => true,
            _ => false,
        };
        if !ok {
            return false;
        }
    }
    true
}
fn process(vecs: Vec<Vec<usize>>) -> usize {
    let mut out = 0;
    for a in vecs.iter() {
        let ok = process_line(a, true) || process_line(a, false);
        if ok {
            out += 1;
        }
    }
    out
}
fn main() {
    let out = include_str!("02.txt");
    let out = extract(out);
    let out = process(out);
    println!("{out:?}");
}
