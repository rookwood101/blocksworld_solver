mod blocksworld;

use blocksworld::world::*;
use blocksworld::search::*;
use std::collections::HashMap;

fn main() {
    let mut hard_entity_start_positions = HashMap::new();
    hard_entity_start_positions.insert(Entity::Block('A'), Location::new(0, 3));
    hard_entity_start_positions.insert(Entity::Block('B'), Location::new(1, 3));
    hard_entity_start_positions.insert(Entity::Block('C'), Location::new(2, 3));
    hard_entity_start_positions.insert(Entity::Agent, Location::new(3, 3));

    let mut easy_entity_start_positions = HashMap::new();
    easy_entity_start_positions.insert(Entity::Block('A'), Location::new(1, 1));
    easy_entity_start_positions.insert(Entity::Block('B'), Location::new(1, 2));
    easy_entity_start_positions.insert(Entity::Block('C'), Location::new(0, 3));
    easy_entity_start_positions.insert(Entity::Agent, Location::new(2, 3));

    let mut entity_goal_positions = HashMap::new();
    entity_goal_positions.insert(Entity::Block('A'), Location::new(1, 1));
    entity_goal_positions.insert(Entity::Block('B'), Location::new(1, 2));
    entity_goal_positions.insert(Entity::Block('C'), Location::new(1, 3));
    entity_goal_positions.insert(Entity::Agent, Location::new(3, 3));

    let start_world = World::new(4, 4, &easy_entity_start_positions).unwrap();
    let goal_world = World::new(4, 4, &entity_goal_positions).unwrap();

    println!("Beginning Search!");

    let searcher = BreadthFirstSearcher::new(start_world, goal_world);
    let goal_node = searcher.search();
    goal_node.print_tree();
}