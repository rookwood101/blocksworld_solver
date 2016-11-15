use std::collections::HashMap;

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
        println!("test!");
    }


    fn check_start_invariants(width: usize,
                              height: usize,
                              entity_starts: &HashMap<Entity, Location>)
                              -> Result<(), GridError> {
        for (_, location) in entity_starts.iter() {
            if location.x >= width || location.y >= height {
                return Err(GridError::StartInvariantError);
            }
        }
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
    StartInvariantError,
}