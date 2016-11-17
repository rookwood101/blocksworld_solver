use ::blocksworld::world;

struct BreadthFirstSearcher {
    start_state: world::World,
    goal_state: world::World,
}

impl BreadthFirstSearcher {
    pub fn new(start_state: world::World, goal_state: world::World) -> BreadthFirstSearcher {
        BreadthFirstSearcher {
            start_state: start_state,
            goal_state: goal_state,
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