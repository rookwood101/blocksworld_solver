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

    let start_world = World::new(4, 4, &hard_entity_start_positions).unwrap();
    let goal_world = World::new(4, 4, &entity_goal_positions).unwrap();

    println!("Beginning A* Search!");
    let mut a_star_searcher = AStarSearcher::new(start_world.clone(), goal_world.clone());
    let goal_node = a_star_searcher.search();
    goal_node.unwrap().print_tree();
    println!("Beginning Iterative Deepening Search!");
    let mut iterative_deepening_searcher = IterativeDeepeningSearcher::new(start_world.clone(),
                                                                           goal_world.clone());
    let goal_node = iterative_deepening_searcher.search();
    goal_node.unwrap().print_tree();
    println!("Beginning Depth First Search!");
    let mut depth_searcher = DepthFirstSearcher::new(start_world.clone(), goal_world.clone());
    let goal_node = depth_searcher.search();
    goal_node.unwrap().print_tree();
    println!("Beginning Breadth First Search!");
    let mut breadth_searcher = BreadthFirstSearcher::new(start_world.clone(), goal_world.clone());
    let goal_node = breadth_searcher.search();
    goal_node.unwrap().print_tree();
}