fn main() {
    const INPUT: &str = include_str!("../../inputs/day_1/input");

    let mut safe = SafePart2::new(50);
    let mut password = 0;

    for line in INPUT.lines() {
        let turn_left = line.starts_with('L');
        let amount = line.chars().skip(1).collect::<String>().parse::<i32>().unwrap();

        let zero_count = if turn_left {
            safe.turn_left(amount)
        } else {
            safe.turn_right(amount)
        };

        println!("Adding {}", zero_count);
        password += zero_count;

        // if safe.is_points_to_zero() {
        //     password += 1;
        // }
    }

    println!("The password is {}!", password);
}

pub struct SafePart1 {
    current: i32
}

impl SafePart1 {
    const MIN: i32 = 0;
    const MAX: i32 = 99;

    pub fn new(start: i32) -> Self {
        if Self::MIN >= Self::MAX {
            panic!("MIN should be less than MAX");
        }

        if !(Self::MIN..=Self::MAX).contains(&start) {
            panic!("start should be in range from {} to {}", Self::MIN, Self::MAX);
        }

        Self { current: start }
    }

    pub fn turn_left(&mut self, amount: i32) {
        print!("Turning left on {}. ", amount);

        self.current -= amount % (Self::MAX + 1);

        if self.current < Self::MIN {
            self.current = Self::MAX - (self.current.abs()) + 1;
        }

        println!("Result: {}", self.current);
    }

    pub fn turn_right(&mut self, amount: i32) {
        print!("Turning right on {}. ", amount);

        self.current += amount % (Self::MAX + 1);
        self.current %= Self::MAX + 1;

        println!("Result: {}", self.current);
    }

    pub fn is_points_to_zero(&self) -> bool {
        self.current == 0
    }
}

pub struct SafePart2 {
    current: i32
}

impl SafePart2 {
    const MIN: i32 = 0;
    const MAX: i32 = 99;

    pub fn new(start: i32) -> Self {
        if Self::MIN >= Self::MAX {
            panic!("MIN should be less than MAX");
        }

        if !(Self::MIN..=Self::MAX).contains(&start) {
            panic!("start should be in range from {} to {}", Self::MIN, Self::MAX);
        }

        Self { current: start }
    }

    pub fn turn_left(&mut self, amount: i32) -> i32 {
        let max_plus_one: i32 = Self::MAX + 1;

        print!("Turning left on {}. ", amount);

        let mut zero_count = 0;
        let mut actual_amount = amount;

        if self.is_points_to_zero() {
            zero_count -= 1;
        }

        while actual_amount > Self::MAX {
            zero_count += 1;
            actual_amount -= max_plus_one;
        }

        self.current -= actual_amount;

        if self.current < Self::MIN {
            zero_count += 1;
            self.current = Self::MAX - (self.current.abs()) + 1;
        } else if self.is_points_to_zero() {
            zero_count += 1;
        }

        assert!(self.current >= Self::MIN && self.current <= Self::MAX);
        print!("Result: {}. ", self.current);
        zero_count
    }

    pub fn turn_right(&mut self, amount: i32) -> i32 {
        let max_plus_one: i32 = Self::MAX + 1;

        print!("Turning right on {}. ", amount);

        let mut zero_count = 0;
        let mut actual_amount = amount;

        while actual_amount > Self::MAX {
            zero_count += 1;
            actual_amount -= max_plus_one;
        }

        self.current += actual_amount;

        if self.current > Self::MAX {
            zero_count += 1;
            self.current = Self::MIN + (self.current - max_plus_one);
        }

        assert!(self.current >= Self::MIN && self.current <= Self::MAX);
        print!("Result: {}. ", self.current);
        zero_count
    }

    pub fn is_points_to_zero(&self) -> bool {
        self.current == 0
    }
}