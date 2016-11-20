use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

use super::Node;
use super::Searcher;
use ::blocksworld::world;

pub struct AStarSearcher {
    start_world: world::World,
    goal_world: world::World,
    goal_block_locations: HashMap<world::Entity, world::Location>,
    fringe: BinaryHeap<AStarNode>,
    explored: HashSet<Node>,
    start_to_node_cost: HashMap<Node, usize>,
}
impl AStarSearcher {
    pub fn new(start_world: world::World, goal_world: world::World) -> AStarSearcher {
        let mut goal_block_locations = HashMap::new();
        for x in 0..goal_world.get_grid_width() {
            for y in 0..goal_world.get_grid_height() {
                let location = world::Location::new(x as isize, y as isize);
                match goal_world.get_grid_location(&location) {
                    block @ world::Entity::Block(_) => goal_block_locations.insert(block, location),
                    _ => None,
                };
            }
        }
        AStarSearcher {
            start_world: start_world,
            goal_world: goal_world,
            goal_block_locations: goal_block_locations,
            fringe: BinaryHeap::new(),
            explored: HashSet::new(),
            start_to_node_cost: HashMap::new(),
        }
    }
    pub fn search(&mut self) -> Node {
        Searcher::search(self)
    }
    fn heuristic(&self, node: &Node) -> usize {
        let mut heuristic = 0;
        for x in 0..node.world.get_grid_width() {
            for y in 0..node.world.get_grid_height() {
                let location = world::Location::new(x as isize, y as isize);
                match node.world.get_grid_location(&location) {
                    block @ world::Entity::Block(_) => {
                        let goal_location = self.goal_block_locations.get(&block).unwrap();
                        heuristic += location.distance_to(goal_location);
                    }
                    _ => (),
                }
            }
        }
        heuristic
    }
    fn is_node_previously_visited(&self, node: &Node) -> bool {
        self.explored.contains(node)
    }
}
impl Searcher for AStarSearcher {
    fn get_start_world(&self) -> &world::World {
        &self.start_world
    }
    fn get_goal_world(&self) -> &world::World {
        &self.goal_world
    }
    fn fringe_push(&mut self, node: Node) {
        if self.is_node_previously_visited(&node) {
            return;
        }
        match &node.parent {
            &Some(ref parent_rc) => {
                let start_to_node_cost = *self.start_to_node_cost.get(&*parent_rc).unwrap() + 1;
                match self.start_to_node_cost.get(&node) {
                    Some(old_start_to_node_cost) => {
                        if start_to_node_cost >= *old_start_to_node_cost {
                            return;
                        }
                    }
                    None => (),
                }
                self.start_to_node_cost.insert(node.clone(), start_to_node_cost)
            }
            &None => self.start_to_node_cost.insert(node.clone(), 0),
        };
        let heuristic = self.heuristic(&node);
        let start_to_node_cost = *self.start_to_node_cost.get(&node).unwrap();
        self.fringe.push(AStarNode {
            start_to_node_to_goal_cost: start_to_node_cost + heuristic,
            node: node,
        });
    }
    fn fringe_pop(&mut self) -> Option<Node> {
        let next = self.fringe.pop().unwrap().into_node();
        self.explored.insert(next.clone());
        Some(next)
    }
}

struct AStarNode {
    start_to_node_to_goal_cost: usize,
    node: Node,
}
impl AStarNode {
    pub fn into_node(self) -> Node {
        self.node
    }
}
impl PartialEq for AStarNode {
    fn eq(&self, other: &AStarNode) -> bool {
        self.start_to_node_to_goal_cost == other.start_to_node_to_goal_cost
    }
}
impl Eq for AStarNode {}
impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &AStarNode) -> Option<Ordering> {
        other.start_to_node_to_goal_cost.partial_cmp(&self.start_to_node_to_goal_cost)
    }
}
impl Ord for AStarNode {
    fn cmp(&self, other: &AStarNode) -> Ordering {
        other.partial_cmp(self).unwrap()
    }
}
