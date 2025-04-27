use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter;

pub fn bfs<T, ID>(
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
                    stack.push_back(node);
                }
            }
            visited.insert(id);
        }
    })
}
