use aoc_runner_derive::aoc;
use nalgebra::DMatrix;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let (grid, x, y, dir) = parse_input(input);
    let visited = get_visited_cells(&grid, x, y, dir);
    visited.iter().filter(|&&v| v).count()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let (mut grid, x, y, dir) = parse_input(input);
    let visited = get_visited_cells(&grid, x, y, dir);
    let mut looping_blocks = 0;
    for block_y in 0..grid.nrows() {
        for block_x in 0..grid.ncols() {
            if !visited[(block_y, block_x)] || (block_y, block_x) == (y, x) {
                continue;
            }
            grid[(block_y, block_x)] = Cell::Wall;
            {
                let mut visited_with_dir =
                    DMatrix::from_fn(grid.nrows(), grid.ncols(), |_, _| [false; 4]);
                let mut x = x as isize;
                let mut y = y as isize;
                let mut dir = dir;
                loop {
                    let (new_x, new_y) = match dir {
                        Direction::Up => (x, y - 1),
                        Direction::Down => (x, y + 1),
                        Direction::Left => (x - 1, y),
                        Direction::Right => (x + 1, y),
                    };
                    if new_x < 0
                        || new_x >= grid.nrows() as isize
                        || new_y < 0
                        || new_y >= grid.ncols() as isize
                    {
                        break;
                    }
                    let cell = grid[(new_y as usize, new_x as usize)];
                    if cell == Cell::Wall {
                        dir = dir.turn_right();
                    } else {
                        x = new_x;
                        y = new_y;
                    }
                    let already_visited =
                        &mut visited_with_dir[(y as usize, x as usize)][dir.as_ordinal()];
                    if *already_visited {
                        looping_blocks += 1;
                        break;
                    }
                    *already_visited = true;
                }
            }
            grid[(block_y, block_x)] = Cell::Empty;
        }
    }

    looping_blocks
}

fn get_visited_cells(
    grid: &DMatrix<Cell>,
    x: usize,
    y: usize,
    mut dir: Direction,
) -> DMatrix<bool> {
    let mut visited = DMatrix::from_fn(grid.nrows(), grid.ncols(), |_, _| false);
    let mut x = x as isize;
    let mut y = y as isize;
    loop {
        let (new_x, new_y) = match dir {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };
        if new_x < 0
            || new_x >= grid.nrows() as isize
            || new_y < 0
            || new_y >= grid.ncols() as isize
        {
            break;
        }
        let cell = grid[(new_y as usize, new_x as usize)];
        if cell == Cell::Wall {
            dir = dir.turn_right();
        } else {
            x = new_x;
            y = new_y;
        }
        visited[(y as usize, x as usize)] = true;
    }
    visited
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_ordinal(self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
}

fn parse_input(input: &str) -> (DMatrix<Cell>, usize, usize, Direction) {
    let input = input.as_bytes();
    let width = input.iter().take_while(|&&c| c != b'\n').count();
    let height = input.len() / width;
    let mut grid = DMatrix::from_fn(height, width, |_, _| Cell::Empty);
    let mut guard_x = 0;
    let mut guard_y = 0;
    let mut guard_dir = Direction::Up;
    for (y, line) in input.chunks(width + 1).enumerate() {
        for (x, c) in line.iter().enumerate().take(width) {
            match *c {
                b'#' => grid[(y, x)] = Cell::Wall,
                b'.' => grid[(y, x)] = Cell::Empty,
                b'^' => {
                    guard_x = x;
                    guard_y = y;
                    guard_dir = Direction::Up;
                }
                b'v' => {
                    guard_x = x;
                    guard_y = y;
                    guard_dir = Direction::Down;
                }
                b'<' => {
                    guard_x = x;
                    guard_y = y;
                    guard_dir = Direction::Left;
                }
                b'>' => {
                    guard_x = x;
                    guard_y = y;
                    guard_dir = Direction::Right;
                }
                c => panic!("invalid character {c}"),
            }
        }
    }
    (grid, guard_x, guard_y, guard_dir)
}
