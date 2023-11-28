use rand::seq::SliceRandom;
use std::collections::HashMap;

/// # [380. Insert Delete GetRandom O(1)](https://leetcode.com/problems/insert-delete-getrandom-o1/description/?envType=study-plan-v2&envId=top-interview-150)
///
/// Implement the `RandomizedSet` class:
///
/// - `RandomizedSet()` Initializes the `RandomizedSet` object.
/// - `bool insert(int val)` Inserts an item `val` into the set if not present. Returns `true` if the item was not present, `false` otherwise.
/// - `bool remove(int val)` Removes an item `val` from the set if present. Returns `true` if the item was present, `false` otherwise.
/// - `int getRandom()` Returns a random element from the current set of elements (it's guaranteed that at least one element exists when this method is called). Each element must have the <b>same probability</b> of being returned.
///
/// You must implement the functions of the class such that each function works in**average** `O(1)`time complexity.
///
/// **Example 1:**
///
/// ```txt
/// Input
///
/// ["RandomizedSet", "insert", "remove", "insert", "getRandom", "remove", "insert", "getRandom"]
/// [[], [1], [2], [2], [], [1], [2], []]
/// Output
///
/// [null, true, false, true, 2, true, false, 2]
///
/// Explanation
///
/// RandomizedSet randomizedSet = new RandomizedSet();
/// randomizedSet.insert(1); // Inserts 1 to the set. Returns true as 1 was inserted successfully.
/// randomizedSet.remove(2); // Returns false as 2 does not exist in the set.
/// randomizedSet.insert(2); // Inserts 2 to the set, returns true. Set now contains [1,2].
/// randomizedSet.getRandom(); // getRandom() should return either 1 or 2 randomly.
/// randomizedSet.remove(1); // Removes 1 from the set, returns true. Set now contains [2].
/// randomizedSet.insert(2); // 2 was already in the set, so return false.
/// randomizedSet.getRandom(); // Since 2 is the only number in the set, getRandom() will always return 2.
/// ```
///
/// **Constraints:**
///
/// - `-2^31 <= val <= 2^31 - 1`
/// - At most `2 *``10^5` calls will be made to `insert`, `remove`, and `getRandom`.
/// - There will be **at least one**  element in the data structure when `getRandom` is called.
#[derive(Default)]
pub struct RandomizedSet {
    inserted: HashMap<i32, usize>,
    items: Vec<i32>,
}

impl RandomizedSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, val: i32) -> bool {
        if self.inserted.contains_key(&val) {
            return false;
        }
        self.inserted.insert(val, self.items.len());
        self.items.push(val);
        true
    }

    pub fn remove(&mut self, val: i32) -> bool {
        match self.inserted.remove(&val) {
            None => false,
            Some(i) => {
                self.items.swap_remove(i);
                if i < self.items.len() {
                    self.inserted.insert(self.items[i], i);
                }
                true
            }
        }
    }

    pub fn get_random(&self) -> i32 {
        *self.items.choose(&mut rand::thread_rng()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut obj = RandomizedSet::new();
        assert_eq!(obj.insert(1), true); // Inserts 1 to the set. Returns true as 1 was inserted successfully.
        assert_eq!(obj.remove(2), false); // Returns false as 2 does not exist in the set.
        assert_eq!(obj.insert(2), true); // Inserts 2 to the set, returns true. Set now contains [1,2].
        assert_eq!(obj.remove(1), true); // Removes 1 from the set, returns true. Set now contains [2].
        assert_eq!(obj.insert(2), false); // 2 was already in the set, so return false.
        obj.get_random(); // Since 2 is the only number in the set, getRandom() will always return 2.
    }
}