//! # Hello
//!
//! Hi,
//!
//! The purpose of this project is to practice Rust and DSA.
//!
//! ## Development Note
//!
//! Please follow the rules below to contribute to this project.
//!
//! - Use chrome and [Clip LeetCode](https://chrome.google.com/webstore/detail/clip-leetcode/cnghimckckgcmhbdokjielmhkmnagdcp/related)
//!   extension to maintain documentation of each method of [`Solution`]
//!
//! ## TODO
//!
//! - [ ] Implement constructor of [`SinglyLinkedList`] by macro
//! - [ ] Implement command line program to be able to use all methods of [`Solution`] by `nom` and `clap`
//! - [ ] Make sure to use [Clip LeetCode](https://chrome.google.com/webstore/detail/clip-leetcode/cnghimckckgcmhbdokjielmhkmnagdcp/related)
//!       for all methods of [`Solution`]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SinglyLinkedList {
    head: Option<Box<ListNode>>,
}

/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// LeetCode solutions provided by Shieh, Yueh-chang <ttboma@gmail.com>
/// Each of the methods for Solution struct are placed under its own module
pub struct Solution {}
mod all_paths_source_target;
mod all_possible_fbt;
mod binary_tree_paths;
mod can_place_flowers;
mod contains_duplicate;
mod count_bits;
mod count_max_or_subsets;
mod count_odds;
mod count_pairs;
mod count_squares;
mod count_substrings;
mod count_vowel_strings;
mod decode_string;
mod diff_ways_to_compute;
mod distribute_cookies;
mod divisor_game;
mod fib;
mod find_ball;
mod find_difference;
mod find_kth_bit;
mod find_kth_positive;
mod find_max_average;
mod find_the_winner;
mod gcd_of_strings;
mod generate;
mod generate_parenthesis;
mod get_kth;
mod get_maximum_generated;
mod get_row;
mod has_path_sum;
mod is_anagram;
mod is_isomorphic;
mod is_palindrome;
mod is_power_of_four;
mod is_power_of_three;
mod is_power_of_two;
mod is_subsequence;
mod jump;
mod kids_with_candies;
mod kth_grammar;
mod largest_altitude;
mod longest_palindrome;
mod majority_element;
mod max_profit;
mod max_sum_after_partitioning;
mod merge;
mod merge_alternately;
mod merge_two_lists;
mod middle_node;
mod min_cost_climbing_stairs;
mod min_eating_speed;
mod min_non_zero_product;
mod min_score;
mod move_zeroes;
mod num_tile_possibilities;
mod permute;
mod pivot_index;
mod predict_the_winner;
mod product_except_self;
mod read_binary_watch;
mod remove_duplicates;
mod remove_element;
mod remove_elements;
mod remove_nodes;
mod reorder_list;
mod reverse_list;
mod reverse_vowels;
mod reverse_words;
mod rotate;
mod running_sum;
mod search;
mod ship_within_days;
mod stone_game;
mod subset_xor_sum;
mod subsets;
mod swap_pairs;
mod total_fruit;
mod tribonacci;
mod two_egg_drop;
mod zero_filled_subarray;

// parser utilities
pub mod parse_util;
