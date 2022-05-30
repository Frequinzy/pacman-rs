use std::collections::HashSet;

type Position = (usize, usize);

#[derive(Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug)]
struct GameState {
    pacman: Position,
    direction: Direction,
    fruits: HashSet<Position>,
    power_ups: HashSet<Position>,
    walls: HashSet<Position>,
    score: usize,
}

impl GameState {
    fn new() -> Self {
        let mut pacman = (0, 0);
        let mut fruits = HashSet::new();
        let mut power_ups = HashSet::new();
        let mut walls = HashSet::new();
        let direction = Direction::West;
        let score = 0;

        for r in 0..START_BOARD.len() {
            for c in 0..START_BOARD[r].len() {
                match START_BOARD[r][c] {
                    'w' => { walls.insert((r, c)); },
                    '.' => { fruits.insert((r, c)); },
                    '*' => { power_ups.insert((r, c)); },
                    'p' => pacman = (r, c),
                    ' ' => (),
                    _ => panic!("Not a valid character"),
                }
            }
        }

        Self {
            pacman,
            direction,
            fruits,
            power_ups,
            walls,
            score,
        }
    }

    fn tick(&mut self) {
        self.update_pacman();
    }

    fn update_pacman(&mut self) {
        let next_square = match &self.direction {
            Direction::North => (self.pacman.0 - 1, self.pacman.1),
            Direction::East => (self.pacman.0, self.pacman.1 + 1),
            Direction::South => (self.pacman.0 + 1, self.pacman.1),
            Direction::West => (self.pacman.0, self.pacman.1 - 1),
        };
        println!("Pacman.0 = {}, Pacman.1 = {}", self.pacman.0, self.pacman.1);
        println!("Next: {:?}, Direction: {:?}", next_square, self.direction);

        if self.walls.contains(&next_square) { return; }


        self.pacman = next_square;

        if self.fruits.remove(&next_square) { self.score += 1; return; }
        
        if self.power_ups.remove(&next_square) { self.score += 10; return; }
    }
}

const START_BOARD: [[char; 28]; 31] = [
    ['w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w'],
    ['w', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'w', 'w', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'w'],
    ['w', '.', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', '.', 'w'],
    ['w', '*', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', '*', 'w'],
    ['w', '.', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', '.', 'w'],
    ['w', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'w'],
    ['w', '.', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', '.', 'w'],
    ['w', '.', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', '.', 'w'],
    ['w', '.', '.', '.', '.', '.', '.', 'w', 'w', '.', '.', '.', '.', 'w', 'w', '.', '.', '.', '.', 'w', 'w', '.', '.', '.', '.', '.', '.', 'w'],
    ['w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', ' ', 'w', 'w', ' ', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w'],
    ['w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', ' ', 'w', 'w', ' ', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w'],
    ['w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w'],
    ['w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', ' ', 'w', 'w', 'w', ' ', ' ', 'w', 'w', 'w', ' ', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w'],
    ['w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', ' ', 'w', ' ', ' ', ' ', ' ', ' ', ' ', 'w', ' ', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w'],
    [' ', ' ', ' ', ' ', ' ', ' ', '.', ' ', ' ', ' ', 'w', ' ', ' ', ' ', ' ', ' ', ' ', 'w', ' ', ' ', ' ', '.', ' ', ' ', ' ', ' ', ' ', ' '],
    ['w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', ' ', 'w', ' ', ' ', ' ', ' ', ' ', ' ', 'w', ' ', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w'],
    ['w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', ' ', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', ' ', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w'],
    ['w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w'],
    ['w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', ' ', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', ' ', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w'],
    ['w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', ' ', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', ' ', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w'], 
    ['w', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'w', 'w', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'w'],
    ['w', '.', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', '.', 'w'],
    ['w', '.', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', 'w', 'w', '.', 'w'],
    ['w', '*', '.', '.', 'w', 'w', '.', '.', '.', '.', '.', '.', '.', ' ', 'p', '.', '.', '.', '.', '.', '.', '.', 'w', 'w', '.', '.', '*', 'w'],
    ['w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w'],
    ['w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w'],
    ['w', '.', '.', '.', '.', '.', '.', 'w', 'w', '.', '.', '.', '.', 'w', 'w', '.', '.', '.', '.', 'w', 'w', '.', '.', '.', '.', '.', '.', 'w'],
    ['w', '.', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', '.', 'w'],
    ['w', '.', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', '.', 'w', 'w', '.', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', '.', 'w'],
    ['w', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'w'],
    ['w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w'],
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_deconstruct_board() {
        let gs = GameState::new();

        let mut test_board: [[char; 28]; 31] = [[' '; 28]; 31];

        for fruit in gs.fruits {
            test_board[fruit.0][fruit.1] = '.';
        }

        for wall in gs.walls {
            test_board[wall.0][wall.1] = 'w';
        }

        for power_up in gs.power_ups {
            test_board[power_up.0][power_up.1] = '*';
        }

        test_board[gs.pacman.0][gs.pacman.1] = 'p';

        for r in test_board {
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
}
