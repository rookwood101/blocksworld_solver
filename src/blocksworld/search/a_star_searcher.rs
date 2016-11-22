use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::cmp::Ordering;
use std::rc::Rc;

use super::Node;
use super::Searcher;
use ::blocksworld::world;

pub struct AStarSearcher {
    start_world: world::World,
    goal_world: world::World,
    goal_block_locations: HashMap<world::Entity, world::Location>,
    fringe: BinaryHeap<AStarNode>,
    explored_states: HashSet<world::World>,
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
            explored_states: HashSet::new(),
        }
    }
    pub fn search(&mut self) -> AStarNode {
        Searcher::search(self)
    }
    fn heuristic(&self, world: &world::World) -> usize {
        let mut heuristic = 0;
        for x in 0..world.get_grid_width() {
            for y in 0..world.get_grid_height() {
                let location = world::Location::new(x as isize, y as isize);
                match world.get_grid_location(&location) {
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
    fn is_node_previously_visited(&self, node: &AStarNode) -> bool {
        self.explored_states.contains(node.get_world())
    }
    fn is_node_unoptimal(&self, node: &AStarNode) -> bool {
        self.fringe
            .iter()
            .filter(|n| *n.get_world() == *node.get_world())
            .any(|n| node.start_to_self_cost >= n.start_to_self_cost)
    }
}
impl Searcher for AStarSearcher {
    type NodeType = AStarNode;
    fn get_start_world(&self) -> &world::World {
        &self.start_world
    }
    fn get_goal_world(&self) -> &world::World {
        &self.goal_world
    }
    fn fringe_push(&mut self, node: Self::NodeType) {
        if self.is_node_previously_visited(&node) || self.is_node_unoptimal(&node) {
            return;
        }
        self.fringe.push(node);
    }
    fn fringe_pop(&mut self) -> Option<Self::NodeType> {
        match self.fringe.pop() {
            Some(next) => {
                self.explored_states.insert(next.get_world().clone());
                Some(next)
            }
            None => None,
        }
    }
    fn new_node(&self,
                depth: u64,
                world: Box<world::World>,
                parent: Option<Rc<Self::NodeType>>)
                -> Self::NodeType {
        let heuristic = self.heuristic(&*world);
        let start_to_self_cost = match &parent {
            &Some(ref parent_rc) => parent_rc.start_to_self_cost + 1,
            &None => 0,
        };
        AStarNode::new(depth, world, parent, heuristic, start_to_self_cost)
    }
}

#[derive(Clone)]
pub struct AStarNode {
    depth: u64,
    world: Box<world::World>,
    parent: Option<Rc<AStarNode>>,
    start_to_self_cost: usize,
    heuristic: usize,
}
impl AStarNode {
    fn new(depth: u64,
           world: Box<world::World>,
           parent: Option<Rc<Self>>,
           start_to_self_cost: usize,
           heuristic: usize)
           -> Self {
        AStarNode {
            depth: depth,
            world: world,
            parent: parent,
            start_to_self_cost: start_to_self_cost,
            heuristic: heuristic,
        }
    }
}
impl Node for AStarNode {
    fn get_world(&self) -> &world::World {
        &*self.world
    }
    fn get_depth(&self) -> u64 {
        self.depth
    }
    fn get_parent(&self) -> Option<Rc<Self>> {
        match &self.parent {
            &Some(ref parent_rc) => Some(parent_rc.clone()),
            &None => None,
        }
    }
}
impl PartialEq for AStarNode {
    fn eq(&self, other: &AStarNode) -> bool {
        (self.start_to_self_cost + self.heuristic) == (other.start_to_self_cost + other.heuristic)
    }
}
impl Eq for AStarNode {}
impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &AStarNode) -> Option<Ordering> {
        (other.start_to_self_cost + other.heuristic)
            .partial_cmp(&(self.start_to_self_cost + self.heuristic))
    }
}
impl Ord for AStarNode {
    fn cmp(&self, other: &AStarNode) -> Ordering {
        other.partial_cmp(self).unwrap()
    }
}
impl Hash for AStarNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_world().hash(state)
    }
}