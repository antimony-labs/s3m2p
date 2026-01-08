//! ===============================================================================
//! FILE: mod.rs | LEARN/learn_core/src/demos/problems/mod.rs
//! PURPOSE: Algorithm problem definitions for Practice section
//! MODIFIED: 2026-01-08
//! LAYER: LEARN -> learn_core -> demos -> problems
//! ===============================================================================

/// Difficulty level for algorithm problems
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn label(&self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            Difficulty::Easy => "#00d4aa",    // Teal
            Difficulty::Medium => "#ffc107",  // Amber
            Difficulty::Hard => "#ff6b6b",    // Red
        }
    }
}

/// Pattern/technique category for the problem
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pattern {
    TwoPointers,
    SlidingWindow,
    FastSlowPointers,
    BinarySearch,
    StackQueue,
    TreeTraversal,
    HeapTopK,
    GraphTraversal,
    DynamicProgramming,
}

impl Pattern {
    pub fn label(&self) -> &'static str {
        match self {
            Pattern::TwoPointers => "Two Pointers",
            Pattern::SlidingWindow => "Sliding Window",
            Pattern::FastSlowPointers => "Fast/Slow Pointers",
            Pattern::BinarySearch => "Binary Search",
            Pattern::StackQueue => "Stack/Queue",
            Pattern::TreeTraversal => "Tree BFS/DFS",
            Pattern::HeapTopK => "Heap/Top-K",
            Pattern::GraphTraversal => "Graph",
            Pattern::DynamicProgramming => "Dynamic Programming",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Pattern::TwoPointers => "⇆",
            Pattern::SlidingWindow => "⬜",
            Pattern::FastSlowPointers => "🐢🐇",
            Pattern::BinarySearch => "🔍",
            Pattern::StackQueue => "📚",
            Pattern::TreeTraversal => "🌳",
            Pattern::HeapTopK => "△",
            Pattern::GraphTraversal => "◉",
            Pattern::DynamicProgramming => "📊",
        }
    }
}

/// An algorithm problem definition
#[derive(Clone, Debug)]
pub struct Problem {
    pub id: usize,
    pub title: &'static str,
    pub pattern: Pattern,
    pub difficulty: Difficulty,
    pub description: &'static str,
    pub hint: &'static str,
    pub time_complexity: &'static str,
    pub space_complexity: &'static str,
}

/// All 40 algorithm problems
pub const PROBLEMS: &[Problem] = &[
    // =====================================================
    // TWO POINTERS (5 problems)
    // =====================================================
    Problem {
        id: 0,
        title: "Two Sum II (Sorted Array)",
        pattern: Pattern::TwoPointers,
        difficulty: Difficulty::Easy,
        description: "Given a sorted array, find two numbers that add up to a target sum. Return their indices.",
        hint: "Use two pointers starting from both ends. If sum is too small, move left pointer right. If too large, move right pointer left.",
        time_complexity: "O(n)",
        space_complexity: "O(1)",
    },
    Problem {
        id: 1,
        title: "Remove Duplicates",
        pattern: Pattern::TwoPointers,
        difficulty: Difficulty::Easy,
        description: "Remove duplicates from a sorted array in-place. Return the new length.",
        hint: "Use a slow pointer to track the position for unique elements, fast pointer to scan.",
        time_complexity: "O(n)",
        space_complexity: "O(1)",
    },
    Problem {
        id: 2,
        title: "Container With Most Water",
        pattern: Pattern::TwoPointers,
        difficulty: Difficulty::Medium,
        description: "Given heights of vertical lines, find two lines that form a container with maximum water.",
        hint: "Start pointers at both ends. Move the shorter line inward - keeping the shorter line can never give a larger area.",
        time_complexity: "O(n)",
        space_complexity: "O(1)",
    },
    Problem {
        id: 3,
        title: "3Sum",
        pattern: Pattern::TwoPointers,
        difficulty: Difficulty::Medium,
        description: "Find all unique triplets in an array that sum to zero.",
        hint: "Sort the array. For each element, use two pointers on the remaining elements to find pairs that sum to its negative.",
        time_complexity: "O(n²)",
        space_complexity: "O(1)",
    },
    Problem {
        id: 4,
        title: "Trapping Rain Water",
        pattern: Pattern::TwoPointers,
        difficulty: Difficulty::Hard,
        description: "Given elevation heights, calculate how much rain water can be trapped.",
        hint: "Water at each position = min(max_left, max_right) - height. Use two pointers to track max heights from both sides.",
        time_complexity: "O(n)",
        space_complexity: "O(1)",
    },

    // =====================================================
    // SLIDING WINDOW (5 problems)
    // =====================================================
    Problem {
        id: 5,
        title: "Max Sum Subarray of Size K",
        pattern: Pattern::SlidingWindow,
        difficulty: Difficulty::Easy,
        description: "Find the maximum sum of any contiguous subarray of size K.",
        hint: "Maintain a window of size K. Slide by subtracting the leftmost and adding the rightmost element.",
        time_complexity: "O(n)",
        space_complexity: "O(1)",
    },
    Problem {
        id: 6,
        title: "Longest Substring Without Repeating",
        pattern: Pattern::SlidingWindow,
        difficulty: Difficulty::Medium,
        description: "Find the length of the longest substring without repeating characters.",
        hint: "Expand window by adding characters. When duplicate found, shrink from left until no duplicates.",
        time_complexity: "O(n)",
        space_complexity: "O(min(n, alphabet))",
    },
    Problem {
        id: 7,
        title: "Minimum Window Substring",
        pattern: Pattern::SlidingWindow,
        difficulty: Difficulty::Hard,
        description: "Find the minimum window in S that contains all characters of T.",
        hint: "Expand to include all chars of T, then contract from left while still valid. Track minimum window found.",
        time_complexity: "O(n + m)",
        space_complexity: "O(alphabet)",
    },
    Problem {
        id: 8,
        title: "Permutation in String",
        pattern: Pattern::SlidingWindow,
        difficulty: Difficulty::Medium,
        description: "Check if s2 contains any permutation of s1.",
        hint: "Use a fixed-size window equal to s1's length. Compare character frequencies in the window with s1.",
        time_complexity: "O(n)",
        space_complexity: "O(1)",
    },
    Problem {
        id: 9,
        title: "Sliding Window Maximum",
        pattern: Pattern::SlidingWindow,
        difficulty: Difficulty::Medium,
        description: "Return the max value in each sliding window of size K.",
        hint: "Use a deque to maintain indices of useful elements. Remove elements outside window and smaller than current.",
        time_complexity: "O(n)",
        space_complexity: "O(k)",
    },

    // =====================================================
    // FAST/SLOW POINTERS (4 problems)
    // =====================================================
    Problem {
        id: 10,
        title: "Linked List Cycle Detection",
        pattern: Pattern::FastSlowPointers,
        difficulty: Difficulty::Easy,
        description: "Determine if a linked list has a cycle.",
        hint: "Use slow (1 step) and fast (2 steps) pointers. If they meet, there's a cycle.",
        time_complexity: "O(n)",
        space_complexity: "O(1)",
    },
    Problem {
        id: 11,
        title: "Find Middle of Linked List",
        pattern: Pattern::FastSlowPointers,
        difficulty: Difficulty::Easy,
        description: "Find the middle node of a linked list in one pass.",
        hint: "When fast reaches the end, slow is at the middle.",
        time_complexity: "O(n)",
        space_complexity: "O(1)",
    },
    Problem {
        id: 12,
        title: "Linked List Cycle Start",
        pattern: Pattern::FastSlowPointers,
        difficulty: Difficulty::Medium,
        description: "Find where a cycle begins in a linked list.",
        hint: "After fast and slow meet, reset one to head. Move both one step at a time - they meet at cycle start.",
        time_complexity: "O(n)",
        space_complexity: "O(1)",
    },
    Problem {
        id: 13,
        title: "Happy Number",
        pattern: Pattern::FastSlowPointers,
        difficulty: Difficulty::Easy,
        description: "Determine if a number is 'happy' (sum of squared digits eventually equals 1).",
        hint: "The sequence either reaches 1 or enters a cycle. Use fast/slow to detect cycle.",
        time_complexity: "O(log n)",
        space_complexity: "O(1)",
    },

    // =====================================================
    // BINARY SEARCH (5 problems)
    // =====================================================
    Problem {
        id: 14,
        title: "Binary Search",
        pattern: Pattern::BinarySearch,
        difficulty: Difficulty::Easy,
        description: "Find target in a sorted array. Return -1 if not found.",
        hint: "Compare with middle element. Search left half if target is smaller, right half if larger.",
        time_complexity: "O(log n)",
        space_complexity: "O(1)",
    },
    Problem {
        id: 15,
        title: "Search in Rotated Sorted Array",
        pattern: Pattern::BinarySearch,
        difficulty: Difficulty::Medium,
        description: "Search in a sorted array that has been rotated at an unknown pivot.",
        hint: "One half is always sorted. Determine which half, then decide which half to search.",
        time_complexity: "O(log n)",
        space_complexity: "O(1)",
    },
    Problem {
        id: 16,
        title: "Find First and Last Position",
        pattern: Pattern::BinarySearch,
        difficulty: Difficulty::Medium,
        description: "Find the starting and ending position of a target value in a sorted array.",
        hint: "Do two binary searches: one to find leftmost occurrence, one for rightmost.",
        time_complexity: "O(log n)",
        space_complexity: "O(1)",
    },
    Problem {
        id: 17,
        title: "Search a 2D Matrix",
        pattern: Pattern::BinarySearch,
        difficulty: Difficulty::Medium,
        description: "Search for a value in a row-wise and column-wise sorted matrix.",
        hint: "Treat the 2D matrix as a sorted 1D array, or start from top-right corner.",
        time_complexity: "O(log(m*n))",
        space_complexity: "O(1)",
    },
    Problem {
        id: 18,
        title: "Median of Two Sorted Arrays",
        pattern: Pattern::BinarySearch,
        difficulty: Difficulty::Hard,
        description: "Find the median of two sorted arrays in O(log(m+n)) time.",
        hint: "Binary search on the smaller array to find the correct partition. Median is based on max of left partition and min of right.",
        time_complexity: "O(log(min(m,n)))",
        space_complexity: "O(1)",
    },

    // =====================================================
    // STACK/QUEUE (5 problems)
    // =====================================================
    Problem {
        id: 19,
        title: "Valid Parentheses",
        pattern: Pattern::StackQueue,
        difficulty: Difficulty::Easy,
        description: "Check if a string of brackets is valid (properly opened and closed).",
        hint: "Push opening brackets onto stack. For closing brackets, pop and check if they match.",
        time_complexity: "O(n)",
        space_complexity: "O(n)",
    },
    Problem {
        id: 20,
        title: "Evaluate Reverse Polish Notation",
        pattern: Pattern::StackQueue,
        difficulty: Difficulty::Medium,
        description: "Evaluate an arithmetic expression in Reverse Polish Notation.",
        hint: "Push numbers onto stack. When operator encountered, pop two operands, apply operator, push result.",
        time_complexity: "O(n)",
        space_complexity: "O(n)",
    },
    Problem {
        id: 21,
        title: "Daily Temperatures",
        pattern: Pattern::StackQueue,
        difficulty: Difficulty::Medium,
        description: "For each day, find how many days until a warmer temperature.",
        hint: "Use a monotonic decreasing stack. When a higher temp is found, pop all smaller ones and calculate differences.",
        time_complexity: "O(n)",
        space_complexity: "O(n)",
    },
    Problem {
        id: 22,
        title: "Implement Queue using Stacks",
        pattern: Pattern::StackQueue,
        difficulty: Difficulty::Easy,
        description: "Implement a FIFO queue using only two stacks.",
        hint: "Use one stack for push, one for pop. Transfer elements between stacks when needed.",
        time_complexity: "O(1) amortized",
        space_complexity: "O(n)",
    },
    Problem {
        id: 23,
        title: "Largest Rectangle in Histogram",
        pattern: Pattern::StackQueue,
        difficulty: Difficulty::Hard,
        description: "Find the largest rectangular area in a histogram.",
        hint: "Use a monotonic increasing stack. When a shorter bar is found, calculate areas for all taller bars in stack.",
        time_complexity: "O(n)",
        space_complexity: "O(n)",
    },

    // =====================================================
    // TREE BFS/DFS (5 problems)
    // =====================================================
    Problem {
        id: 24,
        title: "Binary Tree Level Order Traversal",
        pattern: Pattern::TreeTraversal,
        difficulty: Difficulty::Medium,
        description: "Return nodes level by level from left to right.",
        hint: "Use BFS with a queue. Process all nodes at current level before moving to next.",
        time_complexity: "O(n)",
        space_complexity: "O(n)",
    },
    Problem {
        id: 25,
        title: "Maximum Depth of Binary Tree",
        pattern: Pattern::TreeTraversal,
        difficulty: Difficulty::Easy,
        description: "Find the maximum depth (longest path from root to leaf).",
        hint: "DFS: return 1 + max(depth(left), depth(right)). Base case: null node has depth 0.",
        time_complexity: "O(n)",
        space_complexity: "O(h)",
    },
    Problem {
        id: 26,
        title: "Validate Binary Search Tree",
        pattern: Pattern::TreeTraversal,
        difficulty: Difficulty::Medium,
        description: "Check if a binary tree is a valid BST.",
        hint: "Each node must be within a valid range. Pass min/max bounds down the recursion.",
        time_complexity: "O(n)",
        space_complexity: "O(h)",
    },
    Problem {
        id: 27,
        title: "Lowest Common Ancestor",
        pattern: Pattern::TreeTraversal,
        difficulty: Difficulty::Medium,
        description: "Find the lowest common ancestor of two nodes in a binary tree.",
        hint: "If current node is either p or q, return it. Otherwise, recurse left and right. LCA is where both sides return non-null.",
        time_complexity: "O(n)",
        space_complexity: "O(h)",
    },
    Problem {
        id: 28,
        title: "Serialize and Deserialize Binary Tree",
        pattern: Pattern::TreeTraversal,
        difficulty: Difficulty::Hard,
        description: "Design an algorithm to serialize and deserialize a binary tree.",
        hint: "Use preorder traversal with markers for null nodes. Deserialize by reading tokens and building tree recursively.",
        time_complexity: "O(n)",
        space_complexity: "O(n)",
    },

    // =====================================================
    // HEAP/TOP-K (4 problems)
    // =====================================================
    Problem {
        id: 29,
        title: "Kth Largest Element",
        pattern: Pattern::HeapTopK,
        difficulty: Difficulty::Medium,
        description: "Find the kth largest element in an unsorted array.",
        hint: "Use a min-heap of size k. The root will be the kth largest element.",
        time_complexity: "O(n log k)",
        space_complexity: "O(k)",
    },
    Problem {
        id: 30,
        title: "Merge K Sorted Lists",
        pattern: Pattern::HeapTopK,
        difficulty: Difficulty::Hard,
        description: "Merge k sorted linked lists into one sorted list.",
        hint: "Use a min-heap to always get the smallest head among all lists. Add next node from that list to heap.",
        time_complexity: "O(n log k)",
        space_complexity: "O(k)",
    },
    Problem {
        id: 31,
        title: "Top K Frequent Elements",
        pattern: Pattern::HeapTopK,
        difficulty: Difficulty::Medium,
        description: "Return the k most frequent elements in an array.",
        hint: "Count frequencies with a hash map, then use a min-heap of size k based on frequency.",
        time_complexity: "O(n log k)",
        space_complexity: "O(n)",
    },
    Problem {
        id: 32,
        title: "Find Median from Data Stream",
        pattern: Pattern::HeapTopK,
        difficulty: Difficulty::Hard,
        description: "Design a data structure that supports adding numbers and finding median.",
        hint: "Use two heaps: max-heap for lower half, min-heap for upper half. Balance sizes after each insert.",
        time_complexity: "O(log n) insert, O(1) median",
        space_complexity: "O(n)",
    },

    // =====================================================
    // GRAPH TRAVERSAL (4 problems)
    // =====================================================
    Problem {
        id: 33,
        title: "Number of Islands",
        pattern: Pattern::GraphTraversal,
        difficulty: Difficulty::Medium,
        description: "Count the number of islands in a 2D grid of '1's (land) and '0's (water).",
        hint: "DFS/BFS from each unvisited '1', marking all connected '1's as visited. Count number of DFS calls needed.",
        time_complexity: "O(m*n)",
        space_complexity: "O(m*n)",
    },
    Problem {
        id: 34,
        title: "Clone Graph",
        pattern: Pattern::GraphTraversal,
        difficulty: Difficulty::Medium,
        description: "Create a deep copy of an undirected graph.",
        hint: "Use BFS/DFS with a hash map to track already cloned nodes. Clone node, then recursively clone neighbors.",
        time_complexity: "O(V + E)",
        space_complexity: "O(V)",
    },
    Problem {
        id: 35,
        title: "Course Schedule",
        pattern: Pattern::GraphTraversal,
        difficulty: Difficulty::Medium,
        description: "Determine if you can finish all courses given prerequisites (detect cycle in directed graph).",
        hint: "Use topological sort. If cycle exists, not all courses can be finished. Track in-progress nodes during DFS.",
        time_complexity: "O(V + E)",
        space_complexity: "O(V + E)",
    },
    Problem {
        id: 36,
        title: "Word Ladder",
        pattern: Pattern::GraphTraversal,
        difficulty: Difficulty::Hard,
        description: "Find the shortest transformation sequence from beginWord to endWord.",
        hint: "BFS where each word is a node, edges connect words differing by one letter. Track visited words.",
        time_complexity: "O(n * m²)",
        space_complexity: "O(n * m)",
    },

    // =====================================================
    // DYNAMIC PROGRAMMING (3 problems)
    // =====================================================
    Problem {
        id: 37,
        title: "Climbing Stairs",
        pattern: Pattern::DynamicProgramming,
        difficulty: Difficulty::Easy,
        description: "Count ways to climb n stairs taking 1 or 2 steps at a time.",
        hint: "ways(n) = ways(n-1) + ways(n-2). It's the Fibonacci sequence!",
        time_complexity: "O(n)",
        space_complexity: "O(1)",
    },
    Problem {
        id: 38,
        title: "Longest Increasing Subsequence",
        pattern: Pattern::DynamicProgramming,
        difficulty: Difficulty::Medium,
        description: "Find the length of the longest strictly increasing subsequence.",
        hint: "dp[i] = longest subsequence ending at i. For each j < i where nums[j] < nums[i], dp[i] = max(dp[i], dp[j] + 1).",
        time_complexity: "O(n²) or O(n log n)",
        space_complexity: "O(n)",
    },
    Problem {
        id: 39,
        title: "Coin Change",
        pattern: Pattern::DynamicProgramming,
        difficulty: Difficulty::Medium,
        description: "Find the minimum number of coins needed to make a given amount.",
        hint: "dp[amount] = min coins to make amount. For each coin, dp[i] = min(dp[i], dp[i - coin] + 1).",
        time_complexity: "O(amount * coins)",
        space_complexity: "O(amount)",
    },
];

/// Get problems filtered by pattern
pub fn problems_by_pattern(pattern: Pattern) -> impl Iterator<Item = &'static Problem> {
    PROBLEMS.iter().filter(move |p| p.pattern == pattern)
}

/// Get problems filtered by difficulty
pub fn problems_by_difficulty(difficulty: Difficulty) -> impl Iterator<Item = &'static Problem> {
    PROBLEMS.iter().filter(move |p| p.difficulty == difficulty)
}

/// Get all unique patterns in order
pub const ALL_PATTERNS: &[Pattern] = &[
    Pattern::TwoPointers,
    Pattern::SlidingWindow,
    Pattern::FastSlowPointers,
    Pattern::BinarySearch,
    Pattern::StackQueue,
    Pattern::TreeTraversal,
    Pattern::HeapTopK,
    Pattern::GraphTraversal,
    Pattern::DynamicProgramming,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_count() {
        assert_eq!(PROBLEMS.len(), 40);
    }

    #[test]
    fn test_pattern_distribution() {
        let two_pointers = problems_by_pattern(Pattern::TwoPointers).count();
        let sliding_window = problems_by_pattern(Pattern::SlidingWindow).count();
        let fast_slow = problems_by_pattern(Pattern::FastSlowPointers).count();
        let binary_search = problems_by_pattern(Pattern::BinarySearch).count();
        let stack_queue = problems_by_pattern(Pattern::StackQueue).count();
        let tree = problems_by_pattern(Pattern::TreeTraversal).count();
        let heap = problems_by_pattern(Pattern::HeapTopK).count();
        let graph = problems_by_pattern(Pattern::GraphTraversal).count();
        let dp = problems_by_pattern(Pattern::DynamicProgramming).count();

        assert_eq!(two_pointers, 5);
        assert_eq!(sliding_window, 5);
        assert_eq!(fast_slow, 4);
        assert_eq!(binary_search, 5);
        assert_eq!(stack_queue, 5);
        assert_eq!(tree, 5);
        assert_eq!(heap, 4);
        assert_eq!(graph, 4);
        assert_eq!(dp, 3);
    }

    #[test]
    fn test_difficulty_distribution() {
        let easy = problems_by_difficulty(Difficulty::Easy).count();
        let medium = problems_by_difficulty(Difficulty::Medium).count();
        let hard = problems_by_difficulty(Difficulty::Hard).count();

        assert_eq!(easy, 11);
        assert_eq!(medium, 21);
        assert_eq!(hard, 8);
    }

    #[test]
    fn test_unique_ids() {
        let mut ids: Vec<_> = PROBLEMS.iter().map(|p| p.id).collect();
        ids.sort();
        let expected: Vec<_> = (0..40).collect();
        assert_eq!(ids, expected);
    }
}
