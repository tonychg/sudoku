use super::heuristics;

#[derive(Debug, Clone)]
pub struct Node {
    pub grid: Vec<Vec<u8>>,
    pub size: usize,
    pub visited: bool,
    pub neighbors: Vec<Node>,
}

impl Node {
    pub fn new(grid: &[Vec<u8>], size: usize) -> Self {
        Self {
            grid: grid.to_vec(),
            size,
            visited: false,
            neighbors: Vec::new(),
        }
    }

    pub fn add_adjacent_nodes(&mut self) {
        if let Some((x, y)) = heuristics::next_empty_top_left(&self.grid, self.size) {
            for num in 1..=9u8 {
                if heuristics::can_be_placed(&self.grid, self.size, x, y, num) {
                    let mut grid = self.grid.clone();
                    grid[y][x] = num;
                    self.neighbors.push(Node::new(&grid, self.size));
                }
            }
        }
    }
}

pub fn dfs(root: Node, limit: Option<usize>) -> Vec<Node> {
    let mut stack = Vec::new();
    let mut count: usize = 0;
    let mut solutions: Vec<Node> = vec![];

    stack.push(root);
    while let Some(mut node) = stack.pop() {
        if let Some(limit) = limit {
            if count >= limit {
                return solutions;
            }
        }
        if heuristics::next_empty_top_left(&node.grid, node.size).is_none() {
            count += 1;
            solutions.push(node.clone())
        }
        if !node.visited {
            node.visited = true;
            node.add_adjacent_nodes();
            for node in node.neighbors.iter() {
                if !node.visited {
                    stack.push(node.clone());
                }
            }
        }
    }
    solutions
}
