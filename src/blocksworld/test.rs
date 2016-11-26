use blocksworld::world::World;
use blocksworld::search::AStarSearcher;
use blocksworld::search::BreadthFirstSearcher;
use blocksworld::search::DepthFirstSearcher;
use blocksworld::search::IterativeDeepeningSearcher;

fn all_searcher_tester(start_world: World, goal_world: World) {
    let mut a_star_searcher = AStarSearcher::new(start_world.clone(), goal_world.clone());
    let a_star_goal_node = a_star_searcher.search();
    a_star_goal_node.unwrap();

    let mut breadth_first_searcher = BreadthFirstSearcher::new(start_world.clone(),
                                                               goal_world.clone());
    let breadth_first_goal_node = breadth_first_searcher.search();
    breadth_first_goal_node.unwrap();

    let mut depth_first_searcher = DepthFirstSearcher::new(start_world.clone(), goal_world.clone());
    let depth_first_goal_node = depth_first_searcher.search();
    depth_first_goal_node.unwrap();

    let mut iterative_deepening_searcher = IterativeDeepeningSearcher::new(start_world, goal_world);
    let iterative_deepening_goal_node = iterative_deepening_searcher.search();
    iterative_deepening_goal_node.unwrap();
}