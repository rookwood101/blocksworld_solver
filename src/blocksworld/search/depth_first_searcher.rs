use std::collections::VecDeque;

use super::Node;
use super::Searcher;
use ::blocksworld::world;

pub struct DepthFirstSearcher {
    start_world: world::World,
    goal_world: world::World,
    fringe: VecDeque<Node>,
}
impl DepthFirstSearcher {
    pub fn new(start_world: world::World, goal_world: world::World) -> DepthFirstSearcher {
        DepthFirstSearcher {
            start_world: start_world,
            goal_world: goal_world,
            fringe: VecDeque::new(),
        }
    }
    pub fn search(&mut self) -> Node {
        Searcher::search(self)
    }
}
impl Searcher for DepthFirstSearcher {
    fn get_start_world(&self) -> &world::World {
        &self.start_world
    }
    fn get_goal_world(&self) -> &world::World {
        &self.goal_world
    }
    fn fringe_push(&mut self, node: Node) {
        self.fringe.push_back(node);
    }
    fn fringe_pop(&mut self) -> Option<Node> {
        self.fringe.pop_back()
    }
}