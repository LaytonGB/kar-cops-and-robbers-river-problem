use std::collections::*;

use cops_and_robbers::CopsAndRobbers;
mod boat_position;
mod cops_and_robbers;
mod journey;

fn main() {
    let mut queue = BinaryHeap::new();
    let mut visited = Vec::new();

    let mut depth = 0;
    let mut expansions = 0;

    let mut final_state: CopsAndRobbers = CopsAndRobbers::new();
    let mut final_path: Vec<CopsAndRobbers> = vec![final_state.clone()];

    queue.push((
        3 - final_state.robbers,
        final_state.clone(),
        depth,
        final_path.clone(),
    ));

    while let Some((_, current_car, current_depth, current_path)) = queue.pop() {
        if visited.contains(&current_car) {
            continue;
        }

        visited.push(current_car.clone());

        if current_car.cops == 0 && current_car.robbers == 0 {
            // Found the goal state
            final_state = current_car;
            final_path = current_path;
            break;
        }

        // Generate next states and enqueue them
        let next_states = current_car.get_valid_moves();
        for next_car in next_states {
            expansions += 1;

            let next_car = CopsAndRobbers::from(next_car);
            let heuristic_value = 3 - next_car.robbers;

            let mut next_path = current_path.clone(); // Clone the current path
            next_path.push(next_car.clone()); // Add the next state to the path

            queue.push((heuristic_value, next_car, current_depth + 1, next_path));
        }

        depth = current_depth + 1;
    }

    println!("Depth: {}", depth);
    println!("State: {:?}", final_state);
    println!("Path: {:?}", final_path);
    println!("Expansions: {}", expansions);
    println!("Visited: {}", visited.len());
}
