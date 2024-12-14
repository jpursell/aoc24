use core::time;
use std::{str::FromStr, thread::sleep};
use ndarray::prelude::*;

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

impl Puzzle {
    fn process(&mut self, time: i64, room_size: [i64; 2]) {
        let shape = room_size.map(|x| x as usize);
        let mut room = Array2::from_elem(shape, false);
        for robot in &self.robots {
            *room.get_mut( robot.position_after(time, room_size).map(|x| x as usize)).unwrap() = true;
        }
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
}

fn main() {
    let mut puzzle = include_str!("14.txt").parse::<Puzzle>().unwrap();
    let room_size = [101, 103];
    for time in 0..100 {
        println!("time: {time}");
        puzzle.process(time, room_size);
        sleep(time::Duration::from_millis(200));
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