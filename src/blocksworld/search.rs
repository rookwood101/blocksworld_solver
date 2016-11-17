use ::blocksworld::world;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct BreadthFirstSearcher {
    start_world: world::World,
    goal_world: world::World,
}

impl BreadthFirstSearcher {
    pub fn new(start_world: world::World, goal_world: world::World) -> BreadthFirstSearcher {
        BreadthFirstSearcher {
            start_world: start_world,
            goal_world: goal_world,
        }
    }
    pub fn search(&self) -> Node {
        let mut fringe_queue = VecDeque::new();
        fringe_queue.push_back(Node {
            depth: 0,
            world: Box::new(self.start_world.clone()),
            parent: None,
        });

        loop {
            let next_node = fringe_queue.pop_front().unwrap();
            if self.goal_reached(&next_node) {
                return next_node;
            }


            let child_depth = next_node.depth + 1;
            let parent_rc = Rc::new(next_node);
            for direction in world::Direction::iter() {
                parent_rc.world
                    .clone_and_move_agent(direction)
                    .and_then(|new_world| {
                        fringe_queue.push_back(Node {
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
        *node.world == self.goal_world
    }
}

pub struct Node {
    depth: u64,
    world: Box<world::World>,
    parent: Option<Rc<Node>>,
}
impl Node {
    pub fn print_tree(&self) {
        match &self.parent {
            &Some(ref node_rc) => node_rc.print_tree(),
            &None => (),
        }
        self.world.pretty_print();
        println!("Depth: {}", self.depth);
    }
}