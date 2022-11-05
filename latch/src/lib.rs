use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;

use crossbeam::utils::CachePadded;

/// Lock required for a command
#[derive(Clone)]
pub struct Lock {
    pub required_hashes: Vec<u64>,
    pub owned_count: usize,
}

impl Lock {
    pub fn new<'a, K, I>(keys: I) -> Lock
    where
        K: Hash + ?Sized + 'a,
        I: IntoIterator<Item = &'a K>,
    {
        let mut required_hashes: Vec<u64> = keys
            .into_iter()
            .map(|key| {
                let mut s = DefaultHasher::new();
                key.hash(&mut s);
                s.finish()
            })
            .collect();
        required_hashes.sort_unstable();
        required_hashes.dedup();
        Lock {
            required_hashes,
            owned_count: 0,
        }
    }

    pub fn acquired(&self) -> bool {
        self.required_hashes.len() == self.owned_count
    }

    pub fn is_write_lock(&self) -> bool {
        !self.required_hashes.is_empty()
    }
}

#[derive(Clone)]
struct Latch {
    // store hash value of the key and command ID which requires this key.
    waiting: VecDeque<Option<(u64, u64)>>,
}

impl Latch {
    fn new() -> Latch {
        Latch { waiting: VecDeque::new() }
    }
}

pub struct Latches {
    slots: Vec<CachePadded<Mutex<Latch>>>,
    size: usize,
}

impl Latches {

    /// Creates latches
    /// 
    /// The size will be rounded up to the power of 2.
    pub fn new(size: usize) -> Latches {
        let size = size.next_power_of_two();
        let mut slots = Vec::with_capacity(size);
        (0..size).for_each(|_| slots.push(Mutex::new(Latch::new()).into()));
        Latches { slots, size }
    }

    // TODO: acquire
    // TODO: release
    // TODO: lock_latch
}

#[cfg(test)]
mod tests {
    use crate::Lock;

    #[test]
    fn test_lock() {
        let mut keys = vec!["hello", "workd"];
        let lock = Lock::new(&keys);
        assert!(!lock.acquired());
        // keys.push("valuae");
    }
}
