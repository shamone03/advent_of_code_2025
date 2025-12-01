use crate::day1::Rotation;

mod day1 {
    fn get_input() -> &'static str {
        include_str!("../input/day1.txt")
        //         "L68
        // L30
        // R48
        // L5
        // R60
        // L55
        // L1
        // L99
        // R14
        // L82"
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Rotation(pub u32);

    #[derive(Clone, Copy, Debug)]
    pub enum Direction {
        Left(u32),
        Right(u32),
    }

    impl Rotation {
        pub fn left(mut self, n: u32) -> Self {
            if n > self.0 {
                self.0 = (100 - ((n - self.0) % 100)) % 100;
            } else {
                self.0 -= n;
                self.0 %= 100;
            }
            self
        }

        pub fn right(mut self, n: u32) -> Self {
            self.0 += n;
            self.0 %= 100;
            self
        }

        pub fn rotate(self, direction: Direction) -> Self {
            let ret = match direction {
                Direction::Left(amount) => self.left(amount),
                Direction::Right(amount) => self.right(amount),
            };
            assert!(self.0 <= 99);
            ret
        }
    }

    pub fn part1() -> u32 {
        let mut rotate = Rotation(50);
        get_input()
            .lines()
            .map(|line| {
                let amount = line
                    .chars()
                    .skip(1)
                    .collect::<String>()
                    .parse::<u32>()
                    .expect("Invalid input");

                match line.chars().next().expect("Invalid input") {
                    'L' => Direction::Left(amount),
                    'R' => Direction::Right(amount),
                    _ => panic!("Invalid input"),
                }
            })
            .map(|direction| {
                dbg!(rotate, direction);
                rotate = rotate.rotate(direction);
                rotate
            })
            .filter(|rotation| rotation.0 == 0)
            .count()
            .try_into()
            .unwrap()
    }
}

fn main() {
    println!("{:?}", day1::part1());
    println!("{:?}", Rotation(52).left(552))
}
