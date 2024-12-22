pub fn load_input(year: u16, day: u8) -> String {
    let profile = std::env::var("AOC_PROFILE").unwrap_or("default".to_string());
    let filename = format!("{}-{}-{:02}-input.txt", profile, year, day);
    let filepath = std::path::PathBuf::from(format!("../utils/.cache/{}", filename));
    std::fs::read_to_string(filepath.clone()).unwrap_or_else(|_| {
        panic!(
            "{}",
            format!("Could not read input file: {:?}", filepath).to_string()
        )
    })
}

pub fn pause() {
    println!("Paused...");
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Read line failed!");
}

// Trait aliases are experimental?
//trait Number = num_traits::PrimInt + num_traits::Signed;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T: num_traits::PrimInt + num_traits::Signed> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn min() -> Self {
        // Refactored and had x be T::max_value() here which caused all sorts of bizarre confusion,
        // especially the seemingly infinite looping on day 6.
        Self::new(T::min_value(), T::min_value())
    }

    pub fn max() -> Self {
        Self::new(T::max_value(), T::max_value())
    }

    pub fn manhattan_distance(&self, other: &Point2<T>) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BBox2<T> {
    pub min: Point2<T>,
    pub max: Point2<T>,
}

impl<T: num_traits::PrimInt + num_traits::Signed> Default for BBox2<T> {
    fn default() -> Self {
        Self {
            min: Point2::max(),
            max: Point2::min(),
        }
    }
}

impl<T: num_traits::PrimInt + num_traits::Signed> BBox2<T> {
    pub fn new(a: &Point2<T>, b: &Point2<T>) -> Self {
        Self {
            min: Point2::new(a.x.min(b.x), a.y.min(b.y)),
            max: Point2::new(a.x.max(b.x), a.y.max(b.y)),
        }
    }

    pub fn update(&mut self, p: &Point2<T>) {
        self.min.x = self.min.x.min(p.x);
        self.min.y = self.min.y.min(p.y);
        self.max.x = self.max.x.max(p.x);
        self.max.y = self.max.y.max(p.y);
    }

    pub fn contains(&self, p: &Point2<T>) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    pub fn rotate_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn rotate_left(&self) -> Self {
        self.rotate_right().rotate_right().rotate_right()
    }

    pub fn step<T: num_traits::PrimInt + num_traits::Signed>(&self, p: &Point2<T>) -> Point2<T> {
        match self {
            Self::North => Point2::new(p.x, p.y - T::one()),
            Self::East => Point2::new(p.x + T::one(), p.y),
            Self::South => Point2::new(p.x, p.y + T::one()),
            Self::West => Point2::new(p.x - T::one(), p.y),
        }
    }
}
