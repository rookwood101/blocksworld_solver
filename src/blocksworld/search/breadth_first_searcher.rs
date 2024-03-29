use std::collections::VecDeque;
use std::rc::Rc;

use super::BasicNode;
use super::Searcher;
use super::SearcherError;
use ::blocksworld::world;

pub struct BreadthFirstSearcher {
    start_world: world::World,
    goal_world: world::World,
    fringe: VecDeque<BasicNode>,
}
impl BreadthFirstSearcher {
    pub fn new(start_world: world::World, goal_world: world::World) -> BreadthFirstSearcher {
        BreadthFirstSearcher {
            start_world: start_world,
            goal_world: goal_world,
            fringe: VecDeque::new(),
        }
    }
    pub fn search(&mut self) -> Result<(BasicNode, u32), (SearcherError, u32)> {
        Searcher::search(self, None)
    }
}
impl Searcher for BreadthFirstSearcher {
    type NodeType = BasicNode;
    fn get_start_world(&self) -> &world::World {
        &self.start_world
    }
    fn get_goal_world(&self) -> &world::World {
        &self.goal_world
    }
    fn fringe_push(&mut self, node: Self::NodeType) {
        self.fringe.push_back(node);
    }
    fn fringe_pop(&mut self) -> Option<Self::NodeType> {
        self.fringe.pop_front()
    }
    fn new_node(&self,
                depth: u32,
                world: Box<world::World>,
                parent: Option<Rc<Self::NodeType>>)
                -> Self::NodeType {
        Self::NodeType::new(depth, world, parent)
    }
}