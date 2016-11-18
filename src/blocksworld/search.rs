use ::blocksworld::world;
use std::collections::VecDeque;
use std::rc::Rc;

trait Searcher {
    fn search(&mut self) -> Node {
        let start_world_clone = self.get_start_world().clone();
        self.fringe_push(Node {
            depth: 0,
            world: Box::new(start_world_clone),
            parent: None,
        });

        loop {
            let next_node = self.fringe_pop().unwrap();
            if self.goal_reached(&next_node) {
                return next_node;
            }


            let child_depth = next_node.depth + 1;
            let parent_rc = Rc::new(next_node);
            for direction in world::Direction::iter() {
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

pub struct BreadthFirstSearcher {
    start_world: world::World,
    goal_world: world::World,
    fringe: VecDeque<Node>,
}
impl BreadthFirstSearcher {
    pub fn new(start_world: world::World, goal_world: world::World) -> BreadthFirstSearcher {
        BreadthFirstSearcher {
            start_world: start_world,
            goal_world: goal_world,
            fringe: VecDeque::new(),
        }
    }
    pub fn search(&mut self) -> Node {
        Searcher::search(self)
    }
}
impl Searcher for BreadthFirstSearcher {
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
        self.fringe.pop_front()
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