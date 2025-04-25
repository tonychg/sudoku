use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter;

pub fn dfs<T, ID>(
    start: Vec<T>,
    id_fn: impl Fn(&T) -> ID,
    complete_fn: impl Fn(&T) -> bool,
    neighbors_fn: impl Fn(T) -> Vec<T>,
) -> impl Iterator<Item = T>
where
    ID: Hash + Eq + Debug,
{
    let mut stack: VecDeque<T> = VecDeque::from(start);
    let mut visited: HashSet<ID> = HashSet::new();
    iter::from_fn(move || {
        loop {
            let c = match stack.pop_front() {
                Some(node) => node,
                None => return None,
            };
            if complete_fn(&c) {
                return Some(c);
            }
            let id = id_fn(&c);
            if visited.contains(&id) {
                continue;
            }
            for node in neighbors_fn(c) {
                if !visited.contains(&id_fn(&node)) {
                    stack.push_front(node);
                }
            }
            visited.insert(id);
        }
    })
}

pub fn dfs_with_max_depth<T, ID>(
    start: Vec<T>,
    id_fn: impl Fn(&T) -> ID,
    complete_fn: impl Fn(&T) -> bool,
    neighbors_fn: impl Fn(T) -> Vec<T>,
    max_depth: usize,
) -> impl Iterator<Item = T>
where
    ID: Hash + Eq + Debug,
{
    let mut stack: VecDeque<T> = VecDeque::from(start);
    let mut visited: HashSet<ID> = HashSet::new();
    let mut i = 0;
    iter::from_fn(move || {
        loop {
            let c = match stack.pop_front() {
                Some(node) => node,
                None => return None,
            };
            if i >= max_depth {
                tracing::debug!(max_depth, "Maximum depth reached");
                if let Some(node) = stack.pop_back() {
                    visited.remove(&id_fn(&node));
                    stack.clear();
                    stack.push_front(node);
                    i = 0;
                }
            }
            if complete_fn(&c) {
                return Some(c);
            }
            let id = id_fn(&c);
            if visited.contains(&id) {
                continue;
            }
            for node in neighbors_fn(c) {
                stack.push_front(node);
            }
            visited.insert(id);
            i += 1;
        }
    })
}
