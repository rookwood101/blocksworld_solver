use blocksworld::world::World;
use blocksworld::world::Entity;
use blocksworld::world::Location;

use bidir_map::BidirMap;

use blocksworld::search::Searcher;
use blocksworld::search::Node;
use blocksworld::search::AStarSearcher;
use blocksworld::search::BreadthFirstSearcher;
use blocksworld::search::DepthFirstSearcher;
use blocksworld::search::IterativeDeepeningSearcher;

mod problem_generators;

pub fn test() {
    let mut entity_goal_positions = BidirMap::new();
    entity_goal_positions.insert(Entity::Block('A'), Location::new(1, 1));
    entity_goal_positions.insert(Entity::Block('B'), Location::new(1, 2));
    entity_goal_positions.insert(Entity::Block('C'), Location::new(1, 3));
    entity_goal_positions.insert(Entity::Agent, Location::new(3, 3));
    let goal_world = World::new(4, 4, &entity_goal_positions).unwrap();
    let solutions = problem_generators::solution_depth_difficulty(goal_world.clone());
    for &(ref difficulty, ref world) in solutions.iter() {
        world.pretty_print();
        println!("Difficulty:\t{}", difficulty);
        let averages = run_all_searchers_average(world, &goal_world, *difficulty);
        println!("A* Search Average Expanded Nodes:\t{}", averages[0]);
        println!("Depth First Search Average Expanded Nodes:\t{}",
                 averages[1]);
        println!("Breadth First Search Average Expanded Nodes:\t{}",
                 averages[2]);
        println!("Iterative Deepening Search Average Expanded Nodes:\t{}",
                 averages[3]);

    }
}

fn run_all_searchers_average(start_world: &World, goal_world: &World, difficulty: u8) -> Vec<u64> {
    let try_runs = 100;
    let mut runs = vec![0; 4];

    let mut totals = vec![0; 4];

    for run in 0..(try_runs + 1) {
        runs[0] = run;
        totals[0] +=
            AStarSearcher::new(start_world.clone(), goal_world.clone()).search().unwrap().1;
        runs[1] = run;
        totals[1] +=
            DepthFirstSearcher::new(start_world.clone(), goal_world.clone()).search().unwrap().1;
        if difficulty <= 14 {
            runs[2] = run;
            totals[2] += BreadthFirstSearcher::new(start_world.clone(), goal_world.clone())
                .search()
                .unwrap()
                .1;
        }
        if difficulty <= 14 {
            runs[3] = run;
            totals[3] += IterativeDeepeningSearcher::new(start_world.clone(), goal_world.clone())
                .search()
                .unwrap()
                .1;
        }

    }

    for (i, total) in totals.iter_mut().enumerate() {
        if *total == 0 {
            continue;
        }
        *total /= runs[i];
    }

    totals
}
