use std::collections::HashMap;
use std::iter;

pub struct Grid {
    grid: Vec<Vec<Entity>>,
}

impl Grid {
    pub fn new(width: usize,
               height: usize,
               entity_starts: &HashMap<Entity, Location>)
               -> Result<Grid, GridError> {
        try!(Grid::check_start_invariants(width, height, entity_starts));

        let mut grid = vec![vec![Entity::None; height]; width];
        for (entity, location) in entity_starts.iter() {
            grid[location.x][location.y] = entity.clone();
        }
        Ok(Grid { grid: grid })
    }
    pub fn pretty_print(&self) {
        let width = self.grid.len();
        let height = self.grid[0].len();

        let wall_char = '*';
        let agent_char = '@';
        let none_char = ' ';
        let padding_char = ' ';

        let horizontal_wall = iter::repeat(format!("{}{}", wall_char, padding_char))
            .take(width + 2)
            .collect::<String>();

        println!("{}", horizontal_wall);
        for y in 0..height {
            print!("{}{}", wall_char, padding_char);
            for x in 0..width {
                match self.grid[x][y] {
                    Entity::Agent => print!("{}", agent_char),
                    Entity::Block(block_char) => print!("{}", block_char),
                    Entity::None => print!("{}", none_char),
                }
                print!("{}", padding_char);
            }
            print!("{}\n", wall_char);
        }
        println!("{}", horizontal_wall);
    }


    fn check_start_invariants(width: usize,
                              height: usize,
                              entity_starts: &HashMap<Entity, Location>)
                              -> Result<(), GridError> {
        let mut agent_count: u8 = 0;
        for (entity, location) in entity_starts.iter() {
            match entity {
                &Entity::Agent => agent_count += 1,
                _ => (),
            }
            if location.x >= width || location.y >= height {
                return Err(GridError::EntityOutOfBoundsError);
            }
            if agent_count > 1 {
                return Err(GridError::MultipleAgentsError);
            }
        }
        // Todo: Do not allow multiple Entity s to exist in same location.
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Entity {
    Agent,
    Block(char),
    None,
}

#[derive(Debug)]
pub struct Location {
    x: usize,
    y: usize,
}
impl Location {
    pub fn new(x: usize, y: usize) -> Location {
        Location { x: x, y: y }
    }
}

#[derive(Debug)]
pub enum GridError {
    EntityOutOfBoundsError,
    MultipleAgentsError,
}