use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;

use crossbeam::utils::CachePadded;

const WAITING_LIST_SHRINK_SIZE: usize = 8;
const WAITING_LIST_MAX_CAPACITY: usize = 16;

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
    /// store hash value of the key and command ID which requires this key.
    /// (hash, commandID)
    waiting: VecDeque<Option<(u64, u64)>>,
}

impl Latch {
    fn new() -> Latch {
        Latch {
            waiting: VecDeque::new(),
        }
    }

    #[inline]
    fn wait_for_wake(&mut self, key_hash: u64, cid: u64) {
        self.waiting.push_back(Some((key_hash, cid)));
    }

    /// Find the first command ID in the queue whose hash value is equal to hash.
    fn get_first_req_by_hash(&self, hash: u64) -> Option<u64> {
        for (h, cid) in self.waiting.iter().flatten() {
            if *h == hash {
                return Some(*cid);
            }
        }
        None
    }

    fn pop_front(&mut self, key_hash: u64) -> Option<(u64, u64)> {
        // Maybe the first one is which we need.
        if let Some(item) = self.waiting.pop_front() {
            if let Some(x) = item.as_ref() {
                if x.0 == key_hash {
                    self.maybe_shrink();
                    return item;
                }
            }
            // If not the one we need, push it to old place.
            self.waiting.push_front(item);
        }
        for it in self.waiting.iter_mut() {
            if let Some(t) = it {
                if t.0 == key_hash {
                    return it.take();
                }
            }
        }
        None
    }

    /// For some hot keys, the waiting list maybe very long, so we should shrink the waiting
    /// VecDeque after pop.
    fn maybe_shrink(&mut self) {
        // Pop item which is none to make queue not too long.
        while let Some(item) = self.waiting.front() {
            if item.is_some() {
                break;
            }
            self.waiting.pop_front().unwrap();
        }
        if self.waiting.capacity() > WAITING_LIST_MAX_CAPACITY
            && self.waiting.len() < WAITING_LIST_SHRINK_SIZE
        {
            self.waiting.shrink_to_fit();
        }
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

    use crate::{Latch, Lock};

    #[test]
    fn test_lock() {
        let keys = vec!["hello", "world"];
        let lock = Lock::new(&keys);
        // or let lock = Lock::new(keys);
        assert!(!lock.acquired());
    }

    #[test]
    fn test_latch() {
        let mut l = Latch::new();
        l.wait_for_wake(1024, 1);
        l.wait_for_wake(1025, 2);
        l.wait_for_wake(1024, 9);
        l.wait_for_wake(1026, 3);
        assert_eq!(l.pop_front(1024), Some((1024, 1)));
        assert_eq!(l.pop_front(1024), Some((1024, 9)));
        assert_eq!(l.pop_front(1026), Some((1026, 3)));
        assert_eq!(l.pop_front(1025), Some((1025, 2)));
        assert_eq!(l.pop_front(1024),None);
        assert_eq!(l.pop_front(1025),None);
        assert_eq!(l.pop_front(1026),None);
    }
}
