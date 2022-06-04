use std::collections::{HashSet, HashMap};

//7 a type name for usize, usize
type Position = (usize, usize);

/// An enum representing one of the four coordial directions
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
enum Direction {
    East,
    South,
    West,
    North,
}

const DIRECTIONS: [Direction; 4]= [Direction::North, Direction::West, Direction::South, Direction::East];

/// An enum containing the different modes a ghost can be in
#[derive(Debug, PartialEq)]
enum Mode {
    Chase,
    Scatter,
    Frightened,
}

#[derive(Debug)]
enum Personality {
    Blinky,
    Pinky,
    Inky,
    Clyde,
}

/// Struct containing information about a ghost
#[derive(Debug)]
struct Pacman {
    pos: Position,
    current_dir: Direction,
    desired_dir: Direction,
}

impl Pacman {
    fn new(pos: Position) -> Self {
        let current_dir = Direction::West;
        let desired_dir = Direction::West;
        Self {
            pos,
            current_dir,
            desired_dir,
        }
    }
}

#[derive(Debug)]
struct Ghost {
    pos: Position,
    dir: Direction,
    mode: Mode,
}

impl Ghost {
    fn new(pos: Position, mode: Mode) -> Self {
        let dir = Direction::North;
        Self {
            pos,
            dir,
            mode,
        }
    }
}

/// Struct containing crucial information regarding game state
#[derive(Debug)]
struct GameState {
    // Pacman related unfo
    pacman: Pacman,

    // Ghost info
    ghosts: HashMap<Personality, Ghost>,
    intersections: HashSet<Position>,
    special_intersections: HashSet<Position>,

    // Map related info
    fruits: HashSet<Position>,
    power_ups: HashSet<Position>,
    walls: HashSet<Position>,

    // Other info
    score: usize,
}

impl GameState {
    /// Function for creating a new game.
    /// Pacman, fruits, power fruits, and walls are all loaded from the START_TABLE array
    fn new() -> Self {
        let mut pacman_pos = (0, 0);
        let mut intersections = HashSet::new();
        let mut special_intersections = HashSet::new();
        let mut fruits = HashSet::new();
        let mut power_ups = HashSet::new();
        let mut walls = HashSet::new();
        let score = 0;
        let ghosts = HashMap::new();

        for r in 0..START_BOARD.len() {
            for c in 0..START_BOARD[r].len() {
                match START_BOARD[r][c] {
                    'w' => { walls.insert((r, c)); },
                    '.' => { fruits.insert((r, c)); },
                    '*' => { power_ups.insert((r, c)); },
                    'p' => pacman_pos = (r, c),
                    'i' => { intersections.insert((r, c)); fruits.insert((r, c)); },
                    'j' => { intersections.insert((r, c)); }
                    'o' => { special_intersections.insert((r, c)); intersections.insert((r, c)); fruits.insert((r, c)); },
                    'k' => { special_intersections.insert((r, c)); intersections.insert((r, c)); },
                    ' ' => (),
                    _ => panic!("Not a valid character"),
                }
            }
        }

        let pacman = Pacman::new(pacman_pos);

        Self {
            pacman,
            ghosts,
            intersections,
            special_intersections,
            fruits,
            power_ups,
            walls,
            score,
        }
    }

    /// Main game loop function handling everything that happens each frame/tick of the game
    pub fn tick(&mut self) {
        self.update_pacman();
    }

    fn update_ghosts(&mut self) {
        for (personality, ghost) in &mut self.ghosts {
            if self.intersections.contains(&ghost.pos) {
                let target_square: Position = match (personality, &ghost.mode) {
                    (Personality::Blinky, Mode::Chase) => {todo!()},
                    (Personality::Blinky, Mode::Scatter) => {todo!()},
                    (Personality::Pinky, Mode::Chase) => {todo!()},
                    (Personality::Pinky, Mode::Scatter) => {todo!()},
                    (Personality::Inky, Mode::Chase) => {todo!()},
                    (Personality::Inky, Mode::Scatter) => {todo!()},
                    (Personality::Clyde, Mode::Chase) => {todo!()},
                    (Personality::Clyde, Mode::Scatter) => {todo!()},
                    (_, Mode::Frightened) => {(0, 0)},
                };

                let dirs = match self.special_intersections.contains(&ghost.pos) {
                    true =>  check_dirs(&self.walls, ghost, true),
                    false => check_dirs(&self.walls, ghost, false),
                };
                let mut min_dist = usize::MAX;
                let mut available_choices = Vec::<(Direction, usize)>::with_capacity(3);
                for dir in dirs {
                    let next_square = get_next_square(&ghost.pos, &dir);
                    let dist = target_square.0.abs_diff(next_square.0).pow(2) +
                               target_square.1.abs_diff(next_square.1).pow(2);
                    if dist < min_dist { min_dist = dist };
            
                    available_choices.push((dir, dist));
                }
                let choice = available_choices.into_iter()
                            .filter(|_choice| _choice.1 == min_dist)
                            .max_by_key(|_choice| _choice.0)
                            .unwrap();
                ghost.dir = choice.0;
            }

            let next_square = get_next_square(&ghost.pos, &ghost.dir);
            if self.walls.contains(&next_square) { 
                ghost.dir = check_dirs(&self.walls, ghost, false)[0];
                let next_square = get_next_square(&ghost.pos, &ghost.dir);
            }

            ghost.pos = next_square;
        }
    }

    /// Responsivle for handling pacmans movement each tick
    fn update_pacman(&mut self) {
        // Get the next square based on pacmans desired_dir
        let next_square = get_next_square(&self.pacman.pos, &self.pacman.desired_dir);

        // Check if the new square is a wall.
        if self.walls.contains(&next_square) {
            // Try the direction pacman was previously moving in as well. 
            let next_square = get_next_square(&self.pacman.pos, &self.pacman.current_dir);

            if self.walls.contains(&next_square) { return; }
        } else {
            // If there isn't a wall pacman begins to move in the desired direction
            match self.pacman.desired_dir {
                Direction::North => self.pacman.current_dir = Direction::North,
                Direction::East => self.pacman.current_dir = Direction::East,
                Direction::South => self.pacman.current_dir = Direction::South,
                Direction:: West => self.pacman.current_dir = Direction::West,
            }
        }

        // Update pacmans position
        self.pacman.pos = next_square;

        // Removes fruit if there is one in the new square and increments score
        if self.fruits.remove(&next_square) { self.score += 1; return; }
        
        // Removes power- fruit if there is one in the new square and increments score
        if self.power_ups.remove(&next_square) { self.score += 10; return; }
    }
}

fn check_dirs(walls: &HashSet<Position>, ghost: &Ghost, special: bool) -> Vec<Direction> {
    let opp_dir = opposite_direction(&ghost.dir);
    let mut dirs = Vec::with_capacity(3);
    for dir in DIRECTIONS {
        if dir == opp_dir { continue; }

        if special { if dir == Direction::North { continue; } }

        let next_square = get_next_square(&ghost.pos, &dir);
        if walls.contains(&next_square) { continue; }

        dirs.push(dir);
    }
    dirs
}

fn get_next_square(pos: &Position, dir: &Direction) -> Position {
    match dir {
        Direction::North => (pos.0 - 1, pos.1),
        Direction::East => (pos.0, pos.1 + 1),
        Direction::South => (pos.0 + 1, pos.1),
        Direction::West => (pos.0, pos.1 - 1),
    }
}

fn opposite_direction(dir: &Direction) -> Direction {
    match dir {
        Direction::North => Direction::South,
        Direction::West => Direction::East,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
    }
}

/// Layout of the classic pacman boardx
/// Used for initialization of the in game board representation and not for anything else
/// Char meaning:
/// * 'w' = wall
/// * '.' = fruit
/// * '*' = power fruit
/// * 'p' = pacman
/// * 'i' = intersection whre ghost ai will need to make decision + fruit
/// * 'j' = intersection whre ghost ai will need to make decision
/// * 'o' = special intersection + fruit
/// * 'k' = special intersection
/// * ' ' = empty square
const START_BOARD: [[char; 28]; 36] = [
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w'],
    ['w', '.', '.', '.', '.', '.', 'i', '.', '.', '.', '.', '.', '.', 'w', 'w', '.', '.', '.', '.', '.', '.', 'i', '.', '.', '.', '.', '.', 'w'],
    ['w', '.', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', '.', 'w'],
    ['w', '*', 'w', ' ', ' ', 'w', '.', 'w', ' ', ' ', ' ', 'w', '.', 'w', 'w', '.', 'w', ' ', ' ', ' ', 'w', '.', 'w', ' ', ' ', 'w', '*', 'w'],
    ['w', '.', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', '.', 'w'],
    ['w', 'i', '.', '.', '.', '.', 'i', '.', '.', 'i', '.', '.', 'i', '.', '.', 'i', '.', '.', 'i', '.', '.', 'i', '.', '.', '.', '.', 'i', 'w'],
    ['w', '.', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', '.', 'w'],
    ['w', '.', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', '.', 'w'],
    ['w', '.', '.', '.', '.', '.', 'i', 'w', 'w', '.', '.', '.', '.', 'w', 'w', '.', '.', '.', '.', 'w', 'w', 'i', '.', '.', '.', '.', '.', 'w'],
    ['w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', ' ', 'w', 'w', ' ', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w'],
    [' ', ' ', ' ', ' ', ' ', 'w', '.', 'w', 'w', 'w', 'w', 'w', ' ', 'w', 'w', ' ', 'w', 'w', 'w', 'w', 'w', '.', 'w', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', 'w', '.', 'w', 'w', ' ', ' ', ' ', 'k', ' ', ' ', 'k', ' ', ' ', ' ', 'w', 'w', '.', 'w', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', 'w', '.', 'w', 'w', ' ', 'w', 'w', 'w', ' ', ' ', 'w', 'w', 'w', ' ', 'w', 'w', '.', 'w', ' ', ' ', ' ', ' ', ' '],
    ['w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', ' ', 'w', ' ', ' ', ' ', ' ', ' ', ' ', 'w', ' ', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w'],
    [' ', ' ', ' ', ' ', ' ', ' ', 'i', ' ', ' ', 'j', 'w', ' ', ' ', ' ', ' ', ' ', ' ', 'w', 'j', ' ', ' ', 'i', ' ', ' ', ' ', ' ', ' ', ' '],
    ['w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', ' ', 'w', ' ', ' ', ' ', ' ', ' ', ' ', 'w', ' ', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w'],
    [' ', ' ', ' ', ' ', ' ', 'w', '.', 'w', 'w', ' ', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', ' ', 'w', 'w', '.', 'w', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', 'w', '.', 'w', 'w', 'j', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'j', 'w', 'w', '.', 'w', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', 'w', '.', 'w', 'w', ' ', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', ' ', 'w', 'w', '.', 'w', ' ', ' ', ' ', ' ', ' '],
    ['w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', ' ', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', ' ', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w'], 
    ['w', '.', '.', '.', '.', '.', 'i', '.', '.', 'i', '.', '.', '.', 'w', 'w', '.', '.', '.', 'i', '.', '.', 'i', '.', '.', '.', '.', '.', 'w'],
    ['w', '.', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', '.', 'w'],
    ['w', '.', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', '.', 'w'],
    ['w', '*', '.', '.', 'w', 'w', 'i', '.', '.', 'i', '.', '.', 'o', ' ', 'p', 'o', '.', '.', 'i', '.', '.', 'i', 'w', 'w', '.', '.', '*', 'w'],
    ['w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w'],
    ['w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w'],
    ['w', '.', '.', 'i', '.', '.', '.', 'w', 'w', '.', '.', '.', '.', 'w', 'w', '.', '.', '.', '.', 'w', 'w', '.', '.', '.', 'i', '.', '.', 'w'],
    ['w', '.', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', '.', 'w'],
    ['w', '.', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', '.', 'w'],
    ['w', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'i', '.', '.', 'i', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'w'],
    ['w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w'],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_deconstruct_board() {
        let gs = GameState::new();

        let mut test_board: [[char; 28]; 36] = [[' '; 28]; 36];

        for fruit in gs.fruits {
            test_board[fruit.0][fruit.1] = '.';
        }

        for wall in gs.walls {
            test_board[wall.0][wall.1] = 'w';
        }

        for power_up in gs.power_ups {
            test_board[power_up.0][power_up.1] = '*';
        }

        for intersection in gs.intersections {
            if test_board[intersection.0][intersection.1] == ' ' {
                test_board[intersection.0][intersection.1] = 'j';
            } else {
                test_board[intersection.0][intersection.1] = 'i';
            }
        }

        for sp_intersection in gs.special_intersections {
            if test_board[sp_intersection.0][sp_intersection.1] == 'j' {
                test_board[sp_intersection.0][sp_intersection.1] = 'k';
            } else {
                test_board[sp_intersection.0][sp_intersection.1] = 'o';
            }
        }

        test_board[gs.pacman.pos.0][gs.pacman.pos.1] = 'p';

        for r in test_board {
            println!("{:?}", r);
        }

        for r in START_BOARD {
            println!("{:?}", r);
        }

        assert_eq!(test_board, START_BOARD);
    }

    #[test]
    fn tick_test() {
        let mut gs = GameState::new();

        for i in 0..4 {
            gs.tick();
            println!("Score: {:?}, Pacman: {:?}", gs.score, gs.pacman);
        }

        assert_eq!(gs.score, 3);
    }

    #[test]
    fn direction_ordering() {
        assert!(Direction::North > Direction::West && Direction::West > Direction::South && Direction::South > Direction::East)
    }
}
