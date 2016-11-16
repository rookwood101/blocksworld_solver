mod blocksworld;

use blocksworld::world::*;
use std::collections::HashMap;

fn main() {
    let mut entity_starts = HashMap::new();
    entity_starts.insert(Entity::Block('A'), Location::new(0, 3));
    entity_starts.insert(Entity::Block('B'), Location::new(1, 3));
    entity_starts.insert(Entity::Block('C'), Location::new(2, 3));
    entity_starts.insert(Entity::Agent, Location::new(3, 3));

    let grid = World::new(4, 4, &entity_starts).unwrap();
    grid.pretty_print();
    let grid2 = grid.clone_and_move_agent(Direction::Up).unwrap();
    grid2.pretty_print();
}