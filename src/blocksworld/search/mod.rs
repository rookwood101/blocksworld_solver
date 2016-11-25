extern crate rand;

use ::blocksworld::world;
use std::rc::Rc;
use self::rand::{thread_rng, Rng};

mod breadth_first_searcher;
mod depth_first_searcher;
mod iterative_deepening_searcher;
mod a_star_searcher;
pub use self::breadth_first_searcher::BreadthFirstSearcher;
pub use self::depth_first_searcher::DepthFirstSearcher;
pub use self::iterative_deepening_searcher::IterativeDeepeningSearcher;
pub use self::a_star_searcher::AStarSearcher;

trait Searcher {
    type NodeType: Node;
    fn search(&mut self, max_depth: Option<u64>) -> Result<Self::NodeType, SearcherError> {
        let start_world_clone = self.get_start_world().clone();
        let root_node = self.new_node(0, Box::new(start_world_clone), None);
        self.fringe_push(root_node);

        let mut expanded_nodes = 0;
        let mut directions = world::Direction::directions_array();
        loop {
            let parent_rc = Rc::new(self.fringe_pop()
                .ok_or(SearcherError::GoalNotFoundError)?);
            if self.goal_reached(&*parent_rc) {
                println!("Expanded Nodes: {}", expanded_nodes);
                return match Rc::try_unwrap(parent_rc) {
                    Ok(node) => Ok(node),
                    Err(_) => unreachable!(),
                };
            }
            let child_depth = parent_rc.get_depth() + 1;
            match max_depth {
                Some(max_depth) => {
                    if child_depth >= max_depth {
                        continue;
                    }
                }
                None => (),
            }

            thread_rng().shuffle(&mut directions); // For depth first especially, add children in a random order to reduce looping
            for direction in directions.iter() {
                parent_rc.get_world()
                    .clone_and_move_agent(direction)
                    .and_then(|new_world| {
                        let new_node =
                            self.new_node(child_depth,
                                          Box::new(new_world),
                                          Some(parent_rc.clone()));
                        self.fringe_push(new_node);
                        Ok(())
                    })
                    .ok();
            }

            expanded_nodes += 1
        }
    }
    fn goal_reached(&self, node: &Self::NodeType) -> bool {
        node.get_world().eq_ignore_agent(self.get_goal_world())
    }

    fn new_node(&self,
                depth: u64,
                world: Box<world::World>,
                parent: Option<Rc<Self::NodeType>>)
                -> Self::NodeType;
    fn get_start_world(&self) -> &world::World;
    fn get_goal_world(&self) -> &world::World;
    fn fringe_push(&mut self, node: Self::NodeType);
    fn fringe_pop(&mut self) -> Option<Self::NodeType>;
}

pub trait Node {
    fn get_world(&self) -> &world::World;
    fn get_depth(&self) -> u64;
    fn get_parent(&self) -> Option<Rc<Self>>;
    fn print_tree(&self) {
        self.get_world().pretty_print();
        println!("{}", self.get_depth());
        let mut parent = self.get_parent();
        loop {
            match parent {
                Some(node_rc) => {
                    node_rc.get_world().pretty_print();
                    println!("{}", node_rc.get_depth());
                    parent = node_rc.get_parent();
                }
                None => break,
            }
        }
    }
}

pub struct BasicNode {
    depth: u64,
    world: Box<world::World>,
    parent: Option<Rc<BasicNode>>,
}
impl BasicNode {
    fn new(depth: u64, world: Box<world::World>, parent: Option<Rc<Self>>) -> Self {
        BasicNode {
            depth: depth,
            world: world,
            parent: parent,
        }
    }
}
impl Node for BasicNode {
    fn get_world(&self) -> &world::World {
        &*self.world
    }
    fn get_depth(&self) -> u64 {
        self.depth
    }
    fn get_parent(&self) -> Option<Rc<BasicNode>> {
        match &self.parent {
            &Some(ref parent_rc) => Some(parent_rc.clone()),
            &None => None,
        }
    }
}

#[derive(Debug)]
pub enum SearcherError {
    GoalNotFoundError,
}