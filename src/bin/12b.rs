use ndarray::prelude::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

fn label_image(map: ArrayView2<char>) -> (Array2<usize>, usize) {
    let mut out = Array2::zeros(map.raw_dim());
    let mut label = 0;
    let shape = out.shape().to_vec();
    // init rows with 1 1d label images
    for irow in 0..shape[0] {
        label += 1;
        out[[irow, 0]] = label;
        for icol in 1..shape[1] {
            if map[[irow, icol]] != map[[irow, icol - 1]] {
                label += 1;
            }
            out[[irow, icol]] = label;
        }
    }
    // connect regions
    let mut label_region = BTreeMap::new();
    let mut regions = Vec::new();
    for icol in 0..shape[1] {
        for irow in 1..shape[0] {
            let above_label = out[[irow - 1, icol]];
            let above_char = map[[irow - 1, icol]];
            let current_label = out[[irow, icol]];
            let current_char = map[[irow, icol]];
            let mut add_key = |key| {
                label_region.entry(key).or_insert_with(|| {
                    regions.push(BTreeSet::from([key]));
                    regions.len() - 1
                });
            };
            add_key(above_label);
            add_key(current_label);
            if above_char != current_char {
                continue;
            }
            let above_region = label_region[&above_label];
            let current_region = label_region[&current_label];
            // already merged?
            if above_region == current_region {
                continue;
            }
            // merge
            for key in &regions[current_region] {
                *label_region.get_mut(key).unwrap() = above_region;
            }
            // want to do this: regions[above_region].append(&mut regions[current_region]);
            // ...but need to use split_at_mut to avoid double mutable ref
            if above_region < current_region {
                let (left_regions, right_regions) = regions.split_at_mut(current_region);
                left_regions[above_region].append(&mut right_regions[0]);
            } else {
                let (left_regions, right_regions) = regions.split_at_mut(above_region);
                right_regions[0].append(&mut left_regions[current_region]);
            }
        }
    }
    // remove empty regions
    let regions = regions
        .into_iter()
        .filter(|r| !r.is_empty())
        .collect::<Vec<_>>();
    for (iregion, region) in regions.iter().enumerate() {
        for key in region {
            *label_region.get_mut(key).unwrap() = iregion;
        }
    }
    out.iter_mut().for_each(|x| *x = label_region[x]);
    (out, regions.len())
}

struct Region {
    area: usize,
    sides: usize,
}

#[derive(Debug)]
struct Puzzle {
    map: Array2<char>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nrows = s.lines().count();
        let ncols = s.lines().next().unwrap().chars().count();
        let mut map = Vec::with_capacity(nrows * ncols);
        for line in s.lines() {
            for c in line.chars() {
                map.push(c);
            }
        }
        let map = Array2::from_shape_vec((nrows, ncols), map).unwrap();
        Ok(Puzzle { map })
    }
}

impl Puzzle {
    fn process(&mut self) -> usize {
        let (labels, num_regions) = label_image(self.map.view());
        let mut regions = Vec::with_capacity(num_regions);
        for _ in 0..num_regions {
            regions.push(Region {
                area: 0,
                sides: 0,
            });
        }
        for &label in labels.iter() {
            (*regions.get_mut(label).unwrap()).area += 1;
        }
        let shape = labels.shape().to_vec();
        // check vertical sides
        // check hoizontal sides
        regions.iter().map(|r| r.area * r.sides).sum()
    }
}

fn main() {
    let mut puzzle = include_str!("12.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    println!("{out}");
    assert_eq!(out, 1456082);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let mut out = include_str!("12_test_a.txt").parse::<Puzzle>().unwrap();
        let out = out.process();
        assert_eq!(out, 80);
    }

    #[test]
    fn test_b() {
        let mut out = include_str!("12_test_b.txt").parse::<Puzzle>().unwrap();
        let out = out.process();
        assert_eq!(out, 436);
    }

    #[test]
    fn test_e() {
        let mut out = include_str!("12_test_e.txt").parse::<Puzzle>().unwrap();
        let out = out.process();
        assert_eq!(out, 236);
    }

    #[test]
    fn test_m() {
        let mut out = include_str!("12_test_m.txt").parse::<Puzzle>().unwrap();
        let out = out.process();
        assert_eq!(out, 368);
    }

    #[test]
    fn test_c() {
        let mut out = include_str!("12_test_c.txt").parse::<Puzzle>().unwrap();
        let out = out.process();
        assert_eq!(out, 1206);
    }
}
