use std::collections::HashMap;
use std::iter;
use std::mem;

pub struct World {
    grid: Vec<Vec<Entity>>,
    width: usize,
    height: usize,
    agent_location: Location,
}

impl World {
    pub fn new(width: usize,
               height: usize,
               entity_starts: &HashMap<Entity, Location>)
               -> Result<World, WorldError> {
        try!(World::check_start_invariants(width, height, entity_starts));

        let mut grid = vec![vec![Entity::None; height]; width];
        let mut agent_location = Location::new(0, 0);

        for (entity, location) in entity_starts.iter() {
            match *entity {
                Entity::Agent => agent_location = location.clone(),
                _ => (),
            }

            grid[location.x as usize][location.y as usize] = entity.clone();
        }
        Ok(World {
            grid: grid,
            width: width,
            height: height,
            agent_location: agent_location,
        })
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

    pub fn clone_and_move_agent(&self, direction: Direction) -> Result<World, WorldError> {
        let new_agent_location = Location::new(self.agent_location.x +
                                               match direction {
                                                   Direction::Left => -1,
                                                   Direction::Right => 1,
                                                   _ => 0,
                                               },
                                               self.agent_location.y +
                                               match direction {
                                                   Direction::Up => -1,
                                                   Direction::Down => 1,
                                                   _ => 0,
                                               });

        try!(World::check_agent_location_invariants(&self, &new_agent_location));

        let mut clone_grid = World::clone_grid(&self.grid);

        World::swap_grid_locations(&mut clone_grid, (&self.agent_location, &new_agent_location));

        Ok(World {
            grid: clone_grid,
            agent_location: new_agent_location,
            width: self.width,
            height: self.height,
        })
    }

    fn check_agent_location_invariants(world: &World,
                                       new_agent_location: &Location)
                                       -> Result<(), WorldError> {
        if new_agent_location.x >= world.width as isize || new_agent_location.x <= 0 ||
           new_agent_location.y >= world.height as isize || new_agent_location.y <= 0 {
            return Err(WorldError::InvalidAgentMoveError);
        }

        Ok(())
    }
    fn check_start_invariants(grid_width: usize,
                              grid_height: usize,
                              entity_starts: &HashMap<Entity, Location>)
                              -> Result<(), WorldError> {
        let mut agent_count: u8 = 0;
        for (entity, location) in entity_starts.iter() {
            match *entity {
                Entity::Agent => agent_count += 1,
                _ => (),
            }
            if location.x >= grid_width as isize || location.x < 0 ||
               location.y >= grid_height as isize || location.y < 0 {
                return Err(WorldError::EntityOutOfBoundsError);
            }
            if agent_count > 1 {
                return Err(WorldError::InvalidNumberOfAgentsError);
            }
        }
        if agent_count == 0 {
            return Err(WorldError::InvalidNumberOfAgentsError);
        }
        // Todo: Do not allow multiple Entity s to exist in same location.
        Ok(())
    }
    fn clone_grid(grid: &Vec<Vec<Entity>>) -> Vec<Vec<Entity>> {
        grid.iter().map(|column| column.clone()).collect::<Vec<Vec<Entity>>>()
    }
    fn swap_grid_locations(grid: &mut Vec<Vec<Entity>>, locations: (&Location, &Location)) {
        if locations.0.x == locations.1.x {
            // If locations are in same Vec, use Vec::swap
            grid[locations.0.x as usize].swap(locations.0.y as usize, locations.1.y as usize);
        } else {
            if locations.0.x < locations.1.x {
                let mut grid_slices = grid.split_at_mut(locations.1.x as usize);
                mem::swap(&mut grid_slices.0[locations.0.x as usize][locations.0.y as usize],
                          &mut grid_slices.1[0][locations.1.y as usize]);
            } else {
                let mut grid_slices = grid.split_at_mut(locations.0.x as usize);
                mem::swap(&mut grid_slices.0[locations.1.x as usize][locations.1.y as usize],
                          &mut grid_slices.1[0][locations.0.y as usize]);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Entity {
    Agent,
    Block(char),
    None,
}

#[derive(Debug, Clone)]
pub struct Location {
    x: isize,
    y: isize,
}
impl Location {
    pub fn new(x: isize, y: isize) -> Location {
        Location { x: x, y: y }
    }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub enum WorldError {
    EntityOutOfBoundsError,
    InvalidNumberOfAgentsError,
    InvalidAgentMoveError,
}