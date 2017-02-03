use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::rc::Rc;

use super::Node;
use super::Searcher;
use super::SearcherError;
use ::blocksworld::world;

pub struct AStarSearcher {
    start_world: world::World,
    goal_world: world::World,
    fringe: BinaryHeap<AStarNode>,
}
impl AStarSearcher {
    pub fn new(start_world: world::World, goal_world: world::World) -> AStarSearcher {
        AStarSearcher {
            start_world: start_world,
            goal_world: goal_world,
            fringe: BinaryHeap::new(),
        }
    }
    pub fn search(&mut self) -> Result<(AStarNode, u32), (SearcherError, u32)> {
        Searcher::search(self, None)
    }
    fn heuristic(&self, world: &world::World) -> usize {
        world.entities
            .iter()
            .filter(|&&(ref ent, _)| *ent != world::Entity::Agent)
            .map(|&(ref ent, ref loc)| {
                loc.distance_to(self.get_goal_world().get_entity_location(&ent).unwrap())
            })
            .sum::<usize>()
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
        if self.is_node_unoptimal(&node) {
            return;
        }
        self.fringe.push(node);
    }
    fn fringe_pop(&mut self) -> Option<Self::NodeType> {
        self.fringe.pop()
    }
    fn new_node(&self,
                depth: u32,
                world: Box<world::World>,
                parent: Option<Rc<Self::NodeType>>)
                -> Self::NodeType {
        let heuristic = self.heuristic(&*world);
        let start_to_self_cost = match &parent {
            &Some(ref parent_rc) => parent_rc.start_to_self_cost + 1,
            &None => 0,
        };
        AStarNode::new(depth, world, parent, start_to_self_cost, heuristic)
    }
}

#[derive(Clone)]
pub struct AStarNode {
    depth: u32,
    world: Box<world::World>,
    parent: Option<Rc<AStarNode>>,
    start_to_self_cost: usize,
    heuristic: usize,
}
impl AStarNode {
    fn new(depth: u32,
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
    fn get_depth(&self) -> u32 {
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