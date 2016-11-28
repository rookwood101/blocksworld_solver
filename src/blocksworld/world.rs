use bidir_map::BidirMap;

use std::iter;

#[derive(Debug, Clone, PartialEq)]
pub struct World {
    pub entities: BidirMap<Entity, Location>,
    width: isize,
    height: isize,
}

impl World {
    pub fn new(width: usize,
               height: usize,
               entity_starts: &BidirMap<Entity, Location>)
               -> Result<World, WorldError> {
        let width = width as isize;
        let height = height as isize;
        World::check_start_invariants(width, height, entity_starts)?;

        Ok(World {
            entities: entity_starts.clone(),
            width: width,
            height: height,
        })
    }
    pub fn pretty_print(&self) {
        let wall_char = '*';
        let agent_char = '@';
        let none_char = ' ';
        let padding_char = ' ';

        let horizontal_wall = iter::repeat(format!("{}{}", wall_char, padding_char))
            .take(self.width as usize + 2)
            .collect::<String>();

        println!("{}", horizontal_wall);
        for y in 0..self.height {
            print!("{}{}", wall_char, padding_char);
            for x in 0..self.width {
                match self.get_grid_location(&Location::new(x, y)).unwrap() {
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

    pub fn clone_and_move_agent(&self, direction: &Direction) -> Result<World, WorldError> {
        let old_agent_location = self.entities.get_by_first(&Entity::Agent).unwrap();
        let new_agent_location = Location::new(old_agent_location.x +
                                               match *direction {
                                                   Direction::Left => -1,
                                                   Direction::Right => 1,
                                                   _ => 0,
                                               },
                                               old_agent_location.y +
                                               match *direction {
                                                   Direction::Up => -1,
                                                   Direction::Down => 1,
                                                   _ => 0,
                                               });

        Self::check_location_invariants(self.width, self.height, &new_agent_location)?;

        let mut clone_world = self.clone();

        let new_agent_location_entity = clone_world.get_grid_location(&new_agent_location).unwrap();
        match new_agent_location_entity {
            Entity::None => (),
            _ => clone_world.set_entity_location(new_agent_location_entity, old_agent_location.clone()),
        }
        clone_world.set_entity_location(Entity::Agent, new_agent_location);

        Ok(clone_world)
    }
    pub fn get_grid_location(&self, location: &Location) -> Result<Entity, WorldError> {
        Self::check_location_invariants(self.width, self.height, location)?;
        Ok(self.entities.get_by_second(location).map(|ent| ent.clone()).unwrap_or(Entity::None))
    }
    pub fn get_entity_location(&self, entity: &Entity) -> Result<&Location, WorldError> {
        self.entities.get_by_first(entity).ok_or(WorldError::NonExistentEntityError)
    }
    pub fn set_entity_location(&mut self, entity: Entity, location: Location) {
        self.entities.insert(entity, location).unwrap();
    }
    pub fn eq_ignore_agent(&self, other: &World) -> bool {
        if self.width != other.width || self.height != other.height {
            return false;
        }
        if self.entities.len() != other.entities.len() {
            return false;
        }
        self.entities
            .iter()
            .filter(|&&(ref ent, _)| *ent != Entity::Agent)
            .all(|&(ref ent, ref loc)| other.entities.get_by_first(ent) == Some(loc))
    }

    fn check_location_invariants(width: isize,
                                 height: isize,
                                 location: &Location)
                                 -> Result<(), WorldError> {
        if location.x >= width || location.x < 0 || location.y >= height as isize ||
           location.y < 0 {
            return Err(WorldError::EntityOutOfBoundsError);
        }

        Ok(())
    }
    fn check_start_invariants(grid_width: isize,
                              grid_height: isize,
                              entity_starts: &BidirMap<Entity, Location>)
                              -> Result<(), WorldError> {
        let mut agent_count: u8 = 0;
        for &(ref entity, ref location) in entity_starts.iter() {
            match *entity {
                Entity::Agent => agent_count += 1,
                _ => (),
            }
            Self::check_location_invariants(grid_width, grid_height, &location)?;
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
}


#[derive(Clone, PartialEq, Hash, Eq, Debug)]
pub enum Entity {
    Agent,
    Block(char),
    None,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Location {
    x: isize,
    y: isize,
}
impl Location {
    pub fn new(x: isize, y: isize) -> Location {
        Location { x: x, y: y }
    }
    pub fn distance_to(&self, other: &Location) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn directions_array() -> [Direction; 4] {
        static DIRECTIONS: [Direction; 4] =
            [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        DIRECTIONS.clone()
    }
}

#[derive(Debug)]
pub enum WorldError {
    EntityOutOfBoundsError,
    InvalidNumberOfAgentsError,
    NonExistentEntityError,
}