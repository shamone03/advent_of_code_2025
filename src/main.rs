mod day1 {
    fn get_input() -> &'static str {
        include_str!("../input/day1.txt")
    }

    #[derive(Clone, Copy)]
    struct Rotation(u32);

    impl Rotation {
        fn left(mut self, n: u32) -> Self {
            self.0 -= n;
            self
        }
        fn right(mut self, n: u32) -> Self {
            self.0 += n;
            self
        }
    }

    pub fn part1() -> u32 {
        let rotate = Rotation(50);
        get_input()
            .lines()
            .filter_map(|line| match line.chars().next()? {
                'L' => Some(
                    rotate.left(
                        line.chars()
                            .skip(1)
                            .collect::<String>()
                            .parse::<u32>()
                            .ok()?,
                    ),
                ),
                'R' => Some(
                    rotate.right(
                        line.chars()
                            .skip(1)
                            .collect::<String>()
                            .parse::<u32>()
                            .ok()?,
                    ),
                ),
                _ => None,
            })
            .filter(|rotation| rotation.0 == 0)
            .count()
            .try_into()
            .unwrap()
    }
}

fn main() {
    println!("day 1 part 1: {}", day1::part1());
}
