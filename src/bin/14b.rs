// use core::time;
use ndarray::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
struct Robot {
    position: [i64; 2],
    velocity: [i64; 2],
}

impl Robot {
    fn position_after(&self, time: i64, room_size: [i64; 2]) -> [i64; 2] {
        [
            self.position_after_1d(0, time, room_size),
            self.position_after_1d(1, time, room_size),
        ]
    }
    fn position_after_1d(&self, axis: usize, time: i64, room_size: [i64; 2]) -> i64 {
        assert!(self.position[axis] >= 0);
        let p = self.position[axis] as u64;
        let mut v = self.velocity[axis];
        let r = room_size[axis] as u64;
        while v < 0 {
            v += r as i64;
        }
        let v = v as u64;
        ((p + v * time as u64) % r) as i64
    }
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = s.split_once(" ").unwrap();
        let parse_numbers = |s: &str| {
            let (_, s) = s.split_at(2);
            let (x, y) = s.split_once(",").unwrap();
            [x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()]
        };
        let position = parse_numbers(position);
        let velocity = parse_numbers(velocity);
        Ok(Robot { position, velocity })
    }
}

#[derive(Debug)]
struct Puzzle {
    robots: Vec<Robot>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let count = s.lines().count();
        let mut robots = Vec::with_capacity(count);
        for line in s.lines() {
            robots.push(line.parse::<Robot>().unwrap());
        }
        Ok(Puzzle { robots })
    }
}

// fn quadrant(position: [i64; 2], room_size: [i64; 2]) -> Option<usize> {
//     assert!(position[0] >= 0);
//     assert!(position[1] >= 0);
//     assert!(position[0] < room_size[0]);
//     assert!(position[1] < room_size[1]);
//     let half = room_size.map(|x| (x - 1) / 2);
//     match position[0].cmp(&half[0]) {
//         std::cmp::Ordering::Less => match position[1].cmp(&half[1]) {
//             std::cmp::Ordering::Less => Some(0),
//             std::cmp::Ordering::Equal => None,
//             std::cmp::Ordering::Greater => Some(1),
//         },
//         std::cmp::Ordering::Equal => None,
//         std::cmp::Ordering::Greater => match position[1].cmp(&half[1]) {
//             std::cmp::Ordering::Less => Some(2),
//             std::cmp::Ordering::Equal => None,
//             std::cmp::Ordering::Greater => Some(3),
//         },
//     }
// }

fn symmetry(room: ArrayView2<bool>) -> usize {
    let mut out = 0;
    for irow in 0..room.shape()[0] {
        if irow == 0 {
            continue;
        }
        for icol in 0..(room.shape()[1] - 1) / 2 {
            if icol == 0 {
                continue;
            }
            for drow in -1_i32..=1 {
                for dcol in -1_i32..=1 {
                    if let Some(val) =
                        room.get([(irow as i32 + drow) as usize, (icol as i32 + dcol) as usize])
                    {
                        if *val {
                            out += 1;
                        }
                    }
                }
            }
        }
    }
    out
}

fn draw_room(room: ArrayView2<bool>) {
    let shape = room.shape();
    for irow in 0..shape[0] {
        for icol in 0..shape[1] {
            if room[[irow, icol]] {
                print!("#")
            } else {
                print!(" ")
            }
        }
        println!("");
    }
}

impl Puzzle {
    // fn check(&self, time: i64, room_size: [i64; 2]) -> bool {
    //     let mut quadrant_robot_counts = [0; 4];
    //     for robot in &self.robots {
    //         if let Some(quadrant) = quadrant(robot.position_after(time, room_size), room_size) {
    //             quadrant_robot_counts[quadrant] += 1;
    //         }
    //     }
    //     quadrant_robot_counts[0] == quadrant_robot_counts[1]
    //         && quadrant_robot_counts[2] == quadrant_robot_counts[3]
    //         && quadrant_robot_counts[0] == quadrant_robot_counts[2]
    // }
    fn make_room(&self, time: i64, room_size: [i64; 2]) -> Array2<bool> {
        let shape = room_size.map(|x| x as usize);
        let mut room = Array2::from_elem(shape, false);
        for robot in &self.robots {
            *room
                .get_mut(robot.position_after(time, room_size).map(|x| x as usize))
                .unwrap() = true;
        }
        room
    }
    // fn match_start(&self, time: i64, room_size: [i64; 2]) -> bool {
    //     self.robots.iter().all(|r| {
    //         let p = r.position_after(time, room_size);
    //         p == r.position
    //     })
    // }
}

fn main() {
    let puzzle = include_str!("14.txt").parse::<Puzzle>().unwrap();
    let room_size = [101, 103];
    let mut max_symmetry = 0;
    let max_time = 10403;
    for time in 1..max_time {
        let room = puzzle.make_room(time, room_size);
        let symmetry = symmetry(room.view());
        if symmetry >= max_symmetry {
            println!("time: {time}, symmetry: {symmetry}");
            draw_room(room.view());
            max_symmetry = symmetry;
        }
        // if puzzle.match_start(time, room_size) {
        //     println!("looped: time: {time}");
        //     break;
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_position_after() {
        let position = [2, 4];
        let velocity = [2, -3];
        let robot = Robot { position, velocity };
        let room_size = [11, 7];
        assert_eq!(robot.position_after(0, room_size), position);
        assert_eq!(robot.position_after(1, room_size), [4, 1]);
        assert_eq!(robot.position_after(2, room_size), [6, 5]);
    }
}
