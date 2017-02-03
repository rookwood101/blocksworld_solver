use blocksworld::world::World;
use blocksworld::search::BasicNode;
use blocksworld::search::Node;
use blocksworld::search::Searcher;

use std::collections::BTreeMap;
use std::rc::Rc;

pub fn solution_depth_difficulty(goal_world: World) -> Vec<(u8, World)> {
    let searcher = SolutionDepthSearcher::new(goal_world, 26);
    searcher.search()
}

// Uses a depth first style search to find problems of difficulties up to a certain difficulty.
// Each node generated has an A* search run on it to find its optimal path  size to the goal.
pub struct SolutionDepthSearcher {
    start_world: World,
    fringe: Option<BasicNode>,
    max_difficulty: u8,
    solutions: BTreeMap<u8, World>,
}
impl SolutionDepthSearcher {
    pub fn new(start_world: World, max_difficulty: u8) -> SolutionDepthSearcher {
        SolutionDepthSearcher {
            start_world: start_world,
            fringe: None,
            max_difficulty: max_difficulty,
            solutions: BTreeMap::new(),
        }
    }
    pub fn search(mut self) -> Vec<(u8, World)> {
        let _ = Searcher::search(&mut self, None);
        self.solutions.into_iter().collect::<Vec<(u8, World)>>()
    }
}
impl Searcher for SolutionDepthSearcher {
    type NodeType = BasicNode;
    fn get_start_world(&self) -> &World {
        &self.start_world
    }
    fn get_goal_world(&self) -> &World {
        &self.start_world
    }
    fn goal_reached(&self, _: &Self::NodeType) -> bool {
        self.solutions.len() > self.max_difficulty as usize
    }
    fn fringe_push(&mut self, node: Self::NodeType) {
        self.fringe = Some(node);
    }
    fn fringe_pop(&mut self) -> Option<Self::NodeType> {
        let node = match self.fringe.take() {
            Some(node) => node,
            None => return None,
        };
        let mut a_star_searcher =
            ::blocksworld::search::AStarSearcher::new(node.get_world().clone(),
                                                      self.get_goal_world().clone());
        let result = a_star_searcher.search().unwrap();
        // If we haven't already found a problem world at this depth, add it, to the Map
        if !self.solutions.contains_key(&(result.0.get_depth() as u8)) {
            self.solutions.insert(result.0.get_depth() as u8, node.get_world().clone());
            println!("New solution, depth {}", result.0.get_depth());
        }

        Some(node)
    }
    fn new_node(&self,
                depth: u32,
                world: Box<World>,
                parent: Option<Rc<Self::NodeType>>)
                -> Self::NodeType {
        Self::NodeType::new(depth, world, parent)
    }
}