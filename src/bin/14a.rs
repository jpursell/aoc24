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

fn quadrant(position: [i64; 2], room_size: [i64; 2]) -> Option<usize> {
    assert!(position[0] >= 0);
    assert!(position[1] >= 0);
    assert!(position[0] < room_size[0]);
    assert!(position[1] < room_size[1]);
    let half = room_size.map(|x| (x - 1) / 2);
    match position[0].cmp(&half[0]) {
        std::cmp::Ordering::Less => match position[1].cmp(&half[1]) {
            std::cmp::Ordering::Less => Some(0),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => Some(1),
        },
        std::cmp::Ordering::Equal => None,
        std::cmp::Ordering::Greater => match position[1].cmp(&half[1]) {
            std::cmp::Ordering::Less => Some(2),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => Some(3),
        },
    }
}

impl Puzzle {
    fn process(&mut self, time: i64, room_size: [i64; 2]) -> usize {
        let mut quadrant_robot_counts = [0; 4];
        for robot in &self.robots {
            if let Some(quadrant) = quadrant(robot.position_after(time, room_size), room_size) {
                quadrant_robot_counts[quadrant] += 1;
            }
        }
        quadrant_robot_counts.iter().product::<usize>()
    }
}

fn main() {
    let mut puzzle = include_str!("14.txt").parse::<Puzzle>().unwrap();
    let time = 100;
    let room_size = [101, 103];
    let out = puzzle.process(time, room_size);
    println!("{out}");
    assert_eq!(out, 211773366);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut out = include_str!("14_test.txt").parse::<Puzzle>().unwrap();
        let time = 100;
        let room_size = [11, 7];
        let out = out.process(time, room_size);
        assert_eq!(out, 12);
    }
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
