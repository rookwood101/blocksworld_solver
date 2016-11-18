extern crate rand;

use ::blocksworld::world;
use std::rc::Rc;
use self::rand::{thread_rng, Rng};

mod breadth_first_searcher;
mod depth_first_searcher;
pub use self::breadth_first_searcher::BreadthFirstSearcher;
pub use self::depth_first_searcher::DepthFirstSearcher;

trait Searcher {
    fn search(&mut self) -> Node {
        let start_world_clone = self.get_start_world().clone();
        self.fringe_push(Node {
            depth: 0,
            world: Box::new(start_world_clone),
            parent: None,
        });
        let mut directions = world::Direction::directions_array();

        loop {
            let parent_rc = Rc::new(self.fringe_pop().unwrap());
            if self.goal_reached(&parent_rc) {
                return Rc::try_unwrap(parent_rc).unwrap();
            }


            let child_depth = parent_rc.depth + 1;
            thread_rng().shuffle(&mut directions); // For depth first especially, add children in a random order to reduce looping
            for direction in directions.iter() {
                parent_rc.world
                    .clone_and_move_agent(direction)
                    .and_then(|new_world| {
                        self.fringe_push(Node {
                            depth: child_depth,
                            world: Box::new(new_world),
                            parent: Some(parent_rc.clone()),
                        });
                        Ok(())
                    })
                    .ok();
            }
        }
    }
    fn goal_reached(&self, node: &Node) -> bool {
        *node.world == *self.get_goal_world()
    }

    fn get_start_world(&self) -> &world::World;
    fn get_goal_world(&self) -> &world::World;
    fn fringe_push(&mut self, node: Node);
    fn fringe_pop(&mut self) -> Option<Node>;
}

#[derive(Debug)]
pub struct Node {
    depth: u64,
    world: Box<world::World>,
    parent: Option<Rc<Node>>,
}
impl Node {
    pub fn print_tree(self) {
        let mut parent = &Some(Rc::new(self));
        loop {
            match parent {
                &Some(ref node_rc) => {
                    node_rc.world.pretty_print();
                    parent = &node_rc.parent;
                }
                &None => break,
            }
        }
    }
}