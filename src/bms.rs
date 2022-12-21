//! Breadh-first search algorithm.
//!
use std::collections::{VecDeque, HashMap, HashSet};
use std::hash::Hash;
use std::borrow::Borrow;

pub fn build_path<'a, T, U, F, I>(start: &U, goal: &U, neighbours: F) -> Option<Vec<T>>
where
    T: Hash + Eq + Clone + Borrow<U> + 'a,
    U: Hash + Eq + ToOwned<Owned=T> + 'a + ?Sized,
    F: Fn(&U) -> I,
    I: IntoIterator<Item=&'a T>,
{
    let mut result = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut seen = HashSet::new();
    seen.insert(start);
    while let Some(p) = queue.pop_front() {
        if p == goal {
            return Some(rebuild_path(result, goal))
        }
        for next in neighbours(p) {
            let next = next.borrow();
            if seen.contains(&next) {
                continue
            }
            seen.insert(next);
            result.insert(next, p);
            queue.push_back(next);
        }
    }
    None
}

fn rebuild_path<T, U>(pathes: HashMap<&U, &U>, goal: &U) -> Vec<T>
where
    T: Clone + Eq + Hash + Borrow<U>,
    U: Hash + Eq + ToOwned<Owned=T> + ?Sized,
{
    let mut result: Vec<T> = Vec::new();
    result.insert(0, goal.to_owned());
    let mut prev = goal;
    while let Some(&p) = pathes.get(&prev) {
        result.insert(0, p.to_owned());
        prev = p;
    }
    result
}
