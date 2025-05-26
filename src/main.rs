use argh::FromArgs;

#[derive(FromArgs)]
/// TSP solution with dynamic programming.
struct FileArgs {

    /// path to .txt file containing the adjacency matrix, defaults to "./test/1.txt"
    #[argh(option, long = "file-path", default="String::from(\"1.txt\")")]
    file_path: String,

}

fn text_parser(file_path: &str) -> Vec<Vec<i32>> {
    let content = std::fs::read_to_string(file_path).expect("Failed to read file");
    content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn solve(
    current_node: usize,
    visited_mask: usize,
    adjacency_matrix: &Vec<Vec<i32>>,
    n: usize,
    saved: &mut Vec<Vec<SavedState>>
) -> i32 {

    // Base case: if all nodes have been visited, return the cost to return to the starting node
    if visited_mask == (1 << n) - 1 {
        return adjacency_matrix[current_node][0];
    }

    // If already computed, return the stored value
    if saved[current_node][visited_mask].accumulated_cost != i32::MAX {
        return saved[current_node][visited_mask].accumulated_cost;
    }

    let mut min_cost = i32::MAX;
    let mut best_next_node = -1;

    for next_node in 0..n {
        if (visited_mask & (1 << next_node)) == 0 { // If next_node has not been visited
            let new_visited_mask = visited_mask | (1 << next_node);
            let cost = adjacency_matrix[current_node][next_node]
                + solve(next_node, new_visited_mask, adjacency_matrix, n, saved);
            if cost < min_cost {
                min_cost = cost;
                best_next_node = next_node as i32;
            }
        }
    }
    
    saved[current_node][visited_mask].accumulated_cost = min_cost;
    saved[current_node][visited_mask].next_node = best_next_node;
 
    min_cost
}

#[derive(Clone)]
struct SavedState {
    accumulated_cost: i32,
    next_node: i32,
}

fn get_travel_order(saved: &Vec<Vec<SavedState>>, n: usize) -> Vec<usize> {
    let mut travel_order = Vec::new();
    let mut current_node = 0;
    let mut visited_mask = 1; // Start with the first node visited

    travel_order.push(current_node);
    while visited_mask != (1 << n) - 1 {
        let next_node = saved[current_node][visited_mask].next_node as usize;
        travel_order.push(next_node);
        visited_mask |= 1 << next_node; // Mark the next node as visited
        current_node = next_node;
    }
    travel_order.push(0); // Return to the starting node
    travel_order
}

fn tsp(adjacency_matrix: &Vec<Vec<i32>>) -> (i32, Vec<usize>) {
    let n = adjacency_matrix.len();
    let mut saved: Vec<Vec<SavedState>> = vec![vec![SavedState {
        accumulated_cost: i32::MAX,
        next_node: -1,
    }; 1 << n]; n];

    let result = solve(0, 1, adjacency_matrix, n, &mut saved);
    let travel_order = get_travel_order(&saved, n);

    (result, travel_order)
}

fn main() {
    let args: FileArgs = argh::from_env();
    let adjacency_matrix = text_parser(&format!("./test/{}", args.file_path));
    
    let (result, travel_order) = tsp(&adjacency_matrix);
    println!("Minimum cost of TSP: {}", result);
    println!("Travel order: {:?}", travel_order);
}