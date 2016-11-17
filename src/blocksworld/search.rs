use ::blocksworld::world;

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
    pub fn search<'a>() -> Node<'a> {
        unimplemented!();
    }
}

struct Node<'a> {
    depth: u64,
    world: Box<world::World>,
    parent: Option<&'a Node<'a>>,
}