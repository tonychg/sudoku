use crate::board::Board;
use crate::rng;
use rand::seq::SliceRandom;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Node<T> {
    board: T,
    visited: bool,
}

impl<T> Node<T>
where
    T: Clone + Board,
{
    pub fn new(board: T) -> Self {
        Self {
            board,
            visited: false,
        }
    }

    pub fn adjacent_nodes(&mut self, y_range: &[usize], x_range: &[usize]) -> Vec<Node<T>> {
        let mut neighbors = Vec::new();

        let position = if y_range.is_empty() && y_range.is_empty() {
            self.board.next_empty()
        } else {
            self.board.next_empty_random(y_range, x_range)
        };

        if let Some((x, y)) = position {
            for num in 1..=9u8 {
                if self.board.can_be_placed(x, y, num) {
                    let mut next_board = self.board.clone();
                    next_board.set(x, y, num);
                    neighbors.push(Self::new(next_board));
                }
            }
        }
        self.visited = true;
        neighbors
    }
}

pub fn solve_dfs<T: Board + Clone>(
    board: T,
    seed: Option<u64>,
    limit: Option<usize>,
    max_i: Option<usize>,
) -> Vec<T> {
    let mut count: usize = 0;
    let mut iterations: usize = 0;
    let mut stack: VecDeque<Node<T>> = VecDeque::new();
    let mut solutions: Vec<T> = Vec::new();
    let root = Node::new(board.clone());
    let mut x_range = Vec::new();
    let mut y_range = Vec::new();

    if let Some(seed) = seed {
        let mut rng = rng::rng_from_seed(seed);
        x_range = (0..board.size()).collect::<Vec<usize>>();
        y_range = x_range.clone();
        x_range.shuffle(&mut rng);
        y_range.shuffle(&mut rng);
    }

    stack.push_front(root);

    while let Some(mut node) = stack.pop_front() {
        if let Some(max_i) = max_i {
            if iterations >= max_i {
                tracing::info!("Maximum iterations reached");
                if let Some(node) = stack.pop_back() {
                    let mut node = node.clone();
                    node.visited = false;
                    iterations = 0;
                    stack.clear();
                    stack.push_front(node);
                }
            }
        }
        if let Some(limit) = limit {
            if count >= limit {
                return solutions;
            }
        }
        if node.board.next_empty().is_none() {
            count += 1;
            solutions.push(node.board.clone());
        }
        if !node.visited {
            for node in node.adjacent_nodes(&y_range, &x_range) {
                if !node.visited {
                    stack.push_front(node.clone())
                }
            }
        }
        iterations += 1;
    }
    solutions
}
