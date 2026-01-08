//! ===============================================================================
//! FILE: lessons.rs | DATA_STRUCTURES/src/lessons.rs
//! PURPOSE: Data Structures lesson definitions - from arrays to balanced trees
//! MODIFIED: 2026-01-07
//! LAYER: LEARN -> DATA_STRUCTURES
//! ===============================================================================
//!
//! Curriculum covering fundamental CS data structures with interactive visualizations.
//! Each lesson starts with intuition, then builds to formal concepts and complexity analysis.

/// Technical term that can have a popup explanation
#[derive(Clone)]
pub struct Term {
    pub word: &'static str,
    pub short: &'static str,
    pub detail: &'static str,
}

/// Glossary of data structure technical terms
pub static GLOSSARY: &[Term] = &[
    Term {
        word: "Big O",
        short: "Upper bound on algorithm complexity",
        detail: "Describes worst-case growth rate. O(n) means time grows linearly with input size. \
                 O(1) is constant time, O(log n) is logarithmic, O(n^2) is quadratic.",
    },
    Term {
        word: "O(1)",
        short: "Constant time - independent of input size",
        detail: "The holy grail of efficiency. Array index access and hash table lookup (average) \
                 are O(1). The operation takes the same time whether you have 10 or 10 million items.",
    },
    Term {
        word: "O(n)",
        short: "Linear time - grows with input size",
        detail: "Searching an unsorted list, traversing all elements. If you double the input, \
                 you double the time. Still efficient for most practical purposes.",
    },
    Term {
        word: "O(log n)",
        short: "Logarithmic time - halves problem each step",
        detail: "Binary search, balanced tree operations. Incredibly efficient - searching 1 billion \
                 items takes only ~30 steps. The result of divide-and-conquer strategies.",
    },
    Term {
        word: "amortized",
        short: "Average cost over many operations",
        detail: "Some operations are expensive but rare. Dynamic array resize is O(n), but happens \
                 so rarely that the average insert is still O(1) amortized.",
    },
    Term {
        word: "pointer",
        short: "Memory address reference to another location",
        detail: "Instead of storing data directly, store its address. Enables linked structures \
                 where elements can be anywhere in memory. Fundamental to dynamic data structures.",
    },
    Term {
        word: "node",
        short: "Basic unit containing data and references",
        detail: "In linked structures, a node holds the actual data plus pointers to other nodes. \
                 A linked list node has data + next pointer. A tree node has data + child pointers.",
    },
    Term {
        word: "contiguous",
        short: "Adjacent memory locations",
        detail: "Arrays store elements contiguously - one after another in memory. This enables \
                 O(1) index access but makes insertion expensive (must shift elements).",
    },
    Term {
        word: "dynamic allocation",
        short: "Memory allocated at runtime",
        detail: "Unlike fixed-size arrays, linked structures grow by allocating new nodes as needed. \
                 More flexible but adds overhead for memory management.",
    },
    Term {
        word: "head",
        short: "First element of a list",
        detail: "The entry point to a linked list. All operations start from the head and follow \
                 pointers to find other elements.",
    },
    Term {
        word: "tail",
        short: "Last element of a list",
        detail: "The final node in a linked list (its next pointer is null). Some implementations \
                 keep a tail pointer for O(1) append operations.",
    },
    Term {
        word: "root",
        short: "Top node of a tree",
        detail: "The starting point for tree traversal. Unlike lists, trees branch out from the \
                 root to multiple children, creating a hierarchical structure.",
    },
    Term {
        word: "leaf",
        short: "Node with no children",
        detail: "Terminal nodes at the bottom of a tree. In a binary tree, a leaf has no left or \
                 right child. Leaves are where most data operations terminate.",
    },
    Term {
        word: "edge",
        short: "Connection between nodes",
        detail: "In graphs, edges connect vertices (nodes). Can be directed (one-way) or undirected \
                 (two-way). May have weights representing distance, cost, etc.",
    },
    Term {
        word: "vertex",
        short: "Node in a graph",
        detail: "A point in a graph, connected to other vertices via edges. Can represent cities, \
                 web pages, people, states - any entity with relationships.",
    },
    Term {
        word: "collision",
        short: "Multiple keys mapping to same bucket",
        detail: "Hash tables use a hash function to map keys to array indices. When two different \
                 keys hash to the same index, we have a collision that must be resolved.",
    },
    Term {
        word: "chaining",
        short: "Collision resolution using linked lists",
        detail: "Each hash table bucket stores a linked list. Colliding keys are added to the list. \
                 Simple but uses extra memory for pointers.",
    },
    Term {
        word: "probing",
        short: "Collision resolution by searching for empty slots",
        detail: "On collision, search for the next available slot (linear, quadratic, or double \
                 hashing). No extra memory, but can suffer from clustering.",
    },
    Term {
        word: "load factor",
        short: "Ratio of items to buckets in a hash table",
        detail: "load_factor = n / capacity. Higher load factor means more collisions. Typically \
                 resize when load factor exceeds 0.7 to maintain O(1) performance.",
    },
    Term {
        word: "rotation",
        short: "Tree rebalancing operation",
        detail: "Restructures a tree to maintain balance without changing the ordering property. \
                 Left and right rotations are the building blocks of AVL and Red-Black trees.",
    },
    Term {
        word: "traversal",
        short: "Visiting all nodes in a structure",
        detail: "Different orders reveal different information. In-order gives sorted sequence, \
                 pre-order for serialization, post-order for deletion, level-order for BFS.",
    },
    Term {
        word: "height",
        short: "Longest path from root to leaf",
        detail: "Determines tree efficiency. Balanced trees maintain O(log n) height. Unbalanced \
                 trees can degrade to O(n) height, losing their advantage over lists.",
    },
    Term {
        word: "balance factor",
        short: "Height difference between subtrees",
        detail: "In AVL trees: balance_factor = height(left) - height(right). Must be -1, 0, or 1 \
                 for every node. Violations trigger rotations to restore balance.",
    },
];

/// Single lesson in the curriculum
pub struct Lesson {
    pub id: usize,
    pub icon: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub why_it_matters: &'static str,
    pub intuition: &'static str,
    pub demo_explanation: &'static str,
    pub key_takeaways: &'static [&'static str],
    pub going_deeper: &'static str,
    pub math_details: &'static str,
    pub implementation: &'static str,
}

/// All lessons organized by phase
pub static LESSONS: &[Lesson] = &[
    // =========================================================================
    // PHASE 1: LINEAR STRUCTURES (0-3)
    // =========================================================================
    Lesson {
        id: 0,
        icon: "[]",
        title: "Arrays",
        subtitle: "The Foundation - Contiguous Memory in Action",
        why_it_matters: "Arrays are the most fundamental data structure in computing. Every other \
            data structure is either built on arrays or designed to overcome their limitations. \
            Understanding arrays means understanding how computers actually store and access data.",
        intuition: r#"Imagine a row of numbered lockers in a hallway. Each locker has a number (index) painted on it, and you can instantly walk to locker #47 because you know exactly where it is - you don't need to check lockers 1-46 first.

**That's an array.** A contiguous block of memory where each slot has a fixed position.

The magic: because elements are stored side-by-side, accessing any element by its index is instant - O(1). The computer calculates: base_address + (index * element_size).

The limitation: if you want to insert a new element in the middle, everything after it must shift over - O(n). Like inserting a book on a tightly packed shelf."#,
        demo_explanation: "Watch how array operations work: index access is instant (highlighted in green), \
            but insertions and deletions cause a ripple effect as elements shift.",
        key_takeaways: &[
            "Index access is O(1) - the defining strength of arrays",
            "Insertion/deletion at arbitrary positions is O(n) due to shifting",
            "Memory is contiguous - great for cache performance",
            "Size is typically fixed; dynamic arrays amortize resize costs",
        ],
        going_deeper: "Arrays shine when you need random access and rarely insert/delete in the middle. \
            Dynamic arrays (Vec, ArrayList) handle resizing by doubling capacity when full - this \
            makes the amortized cost of append still O(1). Arrays are also cache-friendly: accessing \
            sequential elements is fast because they're loaded into CPU cache together.",
        math_details: r#"<p><strong>Time Complexity:</strong></p>
$$\text{Access: } O(1)$$
$$\text{Search (unsorted): } O(n)$$
$$\text{Insert at end: } O(1) \text{ amortized}$$
$$\text{Insert at index } i\text{: } O(n-i)$$

<p><strong>Address Calculation:</strong></p>
$$\text{address}[i] = \text{base} + i \times \text{sizeof(element)}$$"#,
        implementation: r#"<h4>Array in Rust</h4>
<pre><code>// Fixed-size array
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let third = arr[2]; // O(1) access

// Dynamic array (Vec)
let mut vec = Vec::new();
vec.push(1); // O(1) amortized
vec.insert(0, 0); // O(n) - shifts all elements</code></pre>

<h4>Key Operations</h4>
<ul>
<li><code>arr[i]</code> - O(1) random access</li>
<li><code>vec.push(x)</code> - O(1) amortized append</li>
<li><code>vec.insert(i, x)</code> - O(n) insertion</li>
<li><code>vec.remove(i)</code> - O(n) removal</li>
</ul>"#,
    },
    Lesson {
        id: 1,
        icon: "->",
        title: "Linked Lists",
        subtitle: "Pointers and Dynamic Memory",
        why_it_matters: "Linked lists teach you to think in pointers - a fundamental concept in \
            computer science. While often slower than arrays in practice, they unlock the door to \
            understanding trees, graphs, and memory management.",
        intuition: r#"Imagine a scavenger hunt. Each clue tells you where to find the next clue. You can't jump to clue #5 - you must follow the chain from the start.

**That's a linked list.** Each node contains data and a pointer to the next node.

The trade-off is inverted from arrays:
- **Insert/delete at a known position: O(1)** - just rewire the pointers
- **Find a position by index: O(n)** - must walk the chain

When you need to frequently insert/delete at the beginning or after a known node, linked lists win. When you need random access, arrays win."#,
        demo_explanation: "Watch nodes link together through pointers. Insertion only requires changing \
            two pointers - no shifting. But notice how finding a node requires traversing from the head.",
        key_takeaways: &[
            "Insert/delete at known position is O(1) - just change pointers",
            "Access by index is O(n) - must traverse from head",
            "No wasted space, but extra memory per node for pointers",
            "Good for frequent insertions at the front or after known nodes",
        ],
        going_deeper: "Doubly-linked lists add a 'prev' pointer, enabling O(1) deletion when you have \
            a reference to the node (no need to find the previous node). Circular lists connect tail \
            to head, useful for round-robin scheduling. In practice, arrays often beat linked lists \
            due to cache locality, but lists remain important for certain algorithms and teaching pointers.",
        math_details: r#"<p><strong>Time Complexity:</strong></p>
$$\text{Access by index: } O(n)$$
$$\text{Insert at head: } O(1)$$
$$\text{Insert after node: } O(1)$$
$$\text{Delete node (given prev): } O(1)$$
$$\text{Search: } O(n)$$

<p><strong>Space:</strong> Each node stores data + pointer(s)</p>
$$\text{Memory per node: } \text{sizeof(T)} + \text{sizeof(pointer)}$$"#,
        implementation: r#"<h4>Linked List Node</h4>
<pre><code>struct Node&lt;T&gt; {
    data: T,
    next: Option&lt;Box&lt;Node&lt;T&gt;&gt;&gt;,
}

// Insert at head: O(1)
fn push_front(head: &mut Option&lt;Box&lt;Node&lt;T&gt;&gt;&gt;, data: T) {
    let new_node = Box::new(Node {
        data,
        next: head.take(),
    });
    *head = Some(new_node);
}</code></pre>

<h4>When to Use</h4>
<ul>
<li>Frequent insertions/deletions at front</li>
<li>Unknown size, memory-constrained</li>
<li>Building block for stacks, queues</li>
<li>Understanding pointer manipulation</li>
</ul>"#,
    },
    Lesson {
        id: 2,
        icon: "[]",
        title: "Stacks",
        subtitle: "Last In, First Out (LIFO)",
        why_it_matters: "Stacks are everywhere: function calls, undo/redo, expression parsing, \
            backtracking algorithms. Understanding stacks means understanding how programs execute \
            and how to solve problems that require 'remembering' and 'undoing'.",
        intuition: r#"Think of a stack of plates in a cafeteria. You can only add or remove from the top. The last plate placed is the first one taken.

**Last In, First Out (LIFO).**

This simple constraint is incredibly powerful:
- **Function calls**: When function A calls B, B's context goes on top. When B returns, it's popped off, and A continues.
- **Undo**: Each action pushes onto the stack. Undo pops the last action.
- **Matching parentheses**: Push '(', pop when you see ')'. If balanced, stack is empty at the end.

All operations are O(1) - you only ever touch the top."#,
        demo_explanation: "Push adds elements to the top with a satisfying 'drop'. Pop removes from the top. \
            Peek shows the top without removing. Watch the stack grow and shrink.",
        key_takeaways: &[
            "Push, pop, and peek are all O(1)",
            "LIFO order - last in, first out",
            "Used in function call stacks, expression evaluation, backtracking",
            "Can be implemented with array or linked list",
        ],
        going_deeper: "The call stack is why recursive functions work - each call's local variables are \
            pushed onto the stack, and popped when returning. Stack overflow happens when recursion goes \
            too deep. Stacks are also key to converting infix to postfix notation (shunting-yard algorithm) \
            and evaluating postfix expressions in a single pass.",
        math_details: r#"<p><strong>Time Complexity:</strong></p>
$$\text{Push: } O(1)$$
$$\text{Pop: } O(1)$$
$$\text{Peek: } O(1)$$
$$\text{Search: } O(n)$$

<p><strong>Space: </strong>O(n) for n elements</p>"#,
        implementation: r#"<h4>Stack Operations</h4>
<pre><code>let mut stack = Vec::new();

stack.push(1);  // Push onto top
stack.push(2);
stack.push(3);  // Stack: [1, 2, 3] (3 is top)

let top = stack.pop();  // Returns Some(3)
let peek = stack.last(); // Returns Some(&2)</code></pre>

<h4>Classic Applications</h4>
<ul>
<li>Balanced parentheses checking</li>
<li>Function call stack</li>
<li>Undo/redo functionality</li>
<li>Expression evaluation</li>
<li>DFS traversal</li>
</ul>"#,
    },
    Lesson {
        id: 3,
        icon: "<>",
        title: "Queues",
        subtitle: "First In, First Out (FIFO)",
        why_it_matters: "Queues model waiting lines, task scheduling, and breadth-first exploration. \
            Whenever order of arrival matters, you need a queue. They're fundamental to operating \
            systems, networking, and graph algorithms.",
        intuition: r#"Think of a line at a coffee shop. First person in line gets served first. New customers join at the back.

**First In, First Out (FIFO).**

Where stacks are about undoing and backtracking, queues are about fairness and order:
- **Print queue**: Documents print in the order submitted
- **Task scheduler**: Processes run in order of arrival (round-robin)
- **BFS**: Explore nodes level by level, visiting earlier-discovered nodes first

Enqueue (add to back) and dequeue (remove from front) are both O(1)."#,
        demo_explanation: "Elements enter from the right (enqueue) and exit from the left (dequeue). \
            Watch how the first element in is always the first element out.",
        key_takeaways: &[
            "Enqueue and dequeue are O(1)",
            "FIFO order - first in, first out",
            "Used in BFS, task scheduling, buffering",
            "Circular buffer is an efficient array-based implementation",
        ],
        going_deeper: "Double-ended queues (deques) allow insertion/removal at both ends - useful for \
            sliding window algorithms. Priority queues (covered later with heaps) are 'unfair' queues \
            where importance trumps arrival order. Circular buffers implement queues efficiently in \
            fixed-size arrays by wrapping around.",
        math_details: r#"<p><strong>Time Complexity:</strong></p>
$$\text{Enqueue: } O(1)$$
$$\text{Dequeue: } O(1)$$
$$\text{Peek front: } O(1)$$
$$\text{Search: } O(n)$$

<p><strong>Circular Buffer:</strong></p>
$$\text{front} = (\text{front} + 1) \mod \text{capacity}$$
$$\text{rear} = (\text{rear} + 1) \mod \text{capacity}$$"#,
        implementation: r#"<h4>Queue with VecDeque</h4>
<pre><code>use std::collections::VecDeque;

let mut queue = VecDeque::new();

queue.push_back(1);  // Enqueue
queue.push_back(2);
queue.push_back(3);  // Queue: [1, 2, 3]

let front = queue.pop_front();  // Returns Some(1)
// Queue is now [2, 3]</code></pre>

<h4>Classic Applications</h4>
<ul>
<li>BFS graph traversal</li>
<li>Task/job scheduling</li>
<li>Print spooler</li>
<li>Buffer for streaming data</li>
</ul>"#,
    },

    // =========================================================================
    // PHASE 2: TREE STRUCTURES (4-6)
    // =========================================================================
    Lesson {
        id: 4,
        icon: "/\\",
        title: "Binary Trees",
        subtitle: "Hierarchical Data Organization",
        why_it_matters: "Trees are the natural structure for hierarchical data: file systems, \
            HTML documents, organization charts, decision trees. Binary trees are the foundation \
            for BSTs, heaps, and expression parsing.",
        intuition: r#"Imagine a family tree, but each person has at most two children (left and right). Start from a single root ancestor.

**That's a binary tree.** Each node has at most two children.

Trees unlock new ways to organize data:
- **Hierarchy**: Natural for parent-child relationships
- **Recursion**: Every subtree is itself a tree - problems decompose beautifully
- **Multiple paths**: Unlike linear structures, trees branch

Traversals visit nodes in different orders:
- **Pre-order**: Process node, then children (serialization)
- **In-order**: Left child, node, right child (sorted order in BST)
- **Post-order**: Children first, then node (deletion)
- **Level-order**: Layer by layer using a queue (BFS)"#,
        demo_explanation: "Build a tree and watch different traversals highlight nodes in order. \
            Notice how in-order visits nodes left-to-right, while level-order goes top-to-bottom.",
        key_takeaways: &[
            "Each node has at most 2 children (left and right)",
            "Height determines efficiency - balanced is O(log n), unbalanced can be O(n)",
            "Four traversal orders: pre-order, in-order, post-order, level-order",
            "Foundation for BSTs, heaps, expression trees",
        ],
        going_deeper: "A complete binary tree fills each level before starting the next (except possibly \
            the last level). A full binary tree has every node with 0 or 2 children. A perfect binary tree \
            is both complete and full. These distinctions matter for heap implementations and tree balancing.",
        math_details: r#"<p><strong>Tree Properties:</strong></p>
$$\text{Max nodes at level } k\text{: } 2^k$$
$$\text{Max nodes in tree of height } h\text{: } 2^{h+1} - 1$$
$$\text{Height of complete tree with } n \text{ nodes: } \lfloor \log_2 n \rfloor$$

<p><strong>Traversal Time:</strong> O(n) - visits each node once</p>"#,
        implementation: r#"<h4>Binary Tree Node</h4>
<pre><code>struct TreeNode&lt;T&gt; {
    data: T,
    left: Option&lt;Box&lt;TreeNode&lt;T&gt;&gt;&gt;,
    right: Option&lt;Box&lt;TreeNode&lt;T&gt;&gt;&gt;,
}

// In-order traversal (recursive)
fn inorder(node: Option&lt;&Box&lt;TreeNode&lt;T&gt;&gt;&gt;) {
    if let Some(n) = node {
        inorder(n.left.as_ref());
        process(n.data);
        inorder(n.right.as_ref());
    }
}</code></pre>

<h4>Traversal Orders</h4>
<ul>
<li><strong>Pre-order</strong>: Node -> Left -> Right</li>
<li><strong>In-order</strong>: Left -> Node -> Right</li>
<li><strong>Post-order</strong>: Left -> Right -> Node</li>
<li><strong>Level-order</strong>: BFS with queue</li>
</ul>"#,
    },
    Lesson {
        id: 5,
        icon: "< >",
        title: "Binary Search Trees",
        subtitle: "Ordered Retrieval in O(log n)",
        why_it_matters: "BSTs combine the fast lookup of arrays with the flexible insertion of linked \
            lists. They're the foundation for sets, maps, and databases. Understanding BSTs is key \
            to understanding balanced trees and efficient searching.",
        intuition: r#"Remember the number guessing game? "Is it higher or lower?" Each answer eliminates half the possibilities.

**A BST is that game in tree form.** Every node follows one rule: left children are smaller, right children are larger.

To find a value:
1. Start at root
2. If value < node, go left. If value > node, go right.
3. Repeat until found or reach null.

Each step eliminates half the tree - that's O(log n)... **if the tree is balanced**. If you insert sorted data, you get a linked list and O(n). That's why balanced trees exist."#,
        demo_explanation: "Watch the search path light up as we find a value - notice how each comparison \
            eliminates half the remaining tree. Try inserting values in different orders to see how \
            the tree shape changes.",
        key_takeaways: &[
            "Left subtree < node < right subtree (BST property)",
            "Search, insert, delete are O(h) where h is height",
            "Balanced tree: h = O(log n). Unbalanced: h = O(n)",
            "In-order traversal gives sorted sequence",
        ],
        going_deeper: "BST deletion has three cases: (1) leaf - just remove, (2) one child - replace with \
            child, (3) two children - replace with in-order successor or predecessor. Self-balancing \
            trees (AVL, Red-Black) add rotations to maintain O(log n) height regardless of insertion order.",
        math_details: r#"<p><strong>Time Complexity (height h):</strong></p>
$$\text{Search: } O(h)$$
$$\text{Insert: } O(h)$$
$$\text{Delete: } O(h)$$
$$\text{Min/Max: } O(h)$$

<p><strong>Height bounds:</strong></p>
$$\text{Balanced: } h = O(\log n)$$
$$\text{Unbalanced: } h = O(n)$$"#,
        implementation: r#"<h4>BST Search</h4>
<pre><code>fn search(node: Option&lt;&Box&lt;BstNode&gt;&gt;, key: i32)
    -> bool
{
    match node {
        None => false,
        Some(n) => {
            if key < n.data {
                search(n.left.as_ref(), key)
            } else if key > n.data {
                search(n.right.as_ref(), key)
            } else {
                true  // Found!
            }
        }
    }
}</code></pre>

<h4>Deletion Cases</h4>
<ul>
<li><strong>Leaf</strong>: Remove directly</li>
<li><strong>One child</strong>: Replace with child</li>
<li><strong>Two children</strong>: Replace with successor</li>
</ul>"#,
    },
    Lesson {
        id: 6,
        icon: "/\\",
        title: "Heaps & Priority Queues",
        subtitle: "Always Get the Best First",
        why_it_matters: "When you need the maximum (or minimum) element quickly and repeatedly, heaps \
            are the answer. They power priority queues, heap sort, and algorithms like Dijkstra's \
            shortest path.",
        intuition: r#"Imagine a tournament bracket where the winner always rises to the top. After each game, the better team moves up.

**A heap is like that.** In a max-heap, every parent is greater than its children. The root is always the maximum.

The clever trick: store it in an array! For node at index i:
- Left child: 2i + 1
- Right child: 2i + 2
- Parent: (i - 1) / 2

Insert: Add at end, then "bubble up" - swap with parent while larger.
Extract max: Remove root, put last element at root, then "sink down" - swap with larger child while smaller.

Both operations are O(log n) - just the height of the tree."#,
        demo_explanation: "Watch elements bubble up after insertion and sink down after extraction. \
            Notice how the heap property is restored with at most O(log n) swaps.",
        key_takeaways: &[
            "Root is always max (max-heap) or min (min-heap)",
            "Insert and extract are O(log n)",
            "Peek at max/min is O(1)",
            "Stored in an array using index formulas",
        ],
        going_deeper: "Heapify - building a heap from an unsorted array - can be done in O(n) by sinking \
            down from the middle. This is faster than inserting n elements (O(n log n)). Heap sort uses \
            this: heapify, then repeatedly extract max. Priority queues in Rust use BinaryHeap.",
        math_details: r#"<p><strong>Time Complexity:</strong></p>
$$\text{Insert: } O(\log n)$$
$$\text{Extract max/min: } O(\log n)$$
$$\text{Peek: } O(1)$$
$$\text{Heapify (build): } O(n)$$

<p><strong>Array Index Formulas:</strong></p>
$$\text{left}(i) = 2i + 1$$
$$\text{right}(i) = 2i + 2$$
$$\text{parent}(i) = \lfloor(i-1)/2\rfloor$$"#,
        implementation: r#"<h4>Heap Operations</h4>
<pre><code>use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(3);
heap.push(1);
heap.push(4);  // Heap: max at top

let max = heap.pop();  // Returns Some(4)
let peek = heap.peek(); // Returns Some(&3)</code></pre>

<h4>When to Use</h4>
<ul>
<li>Priority scheduling</li>
<li>Finding k largest/smallest</li>
<li>Dijkstra's shortest path</li>
<li>Heap sort</li>
<li>Median maintenance</li>
</ul>"#,
    },

    // =========================================================================
    // PHASE 3: ADVANCED STRUCTURES (7-9)
    // =========================================================================
    Lesson {
        id: 7,
        icon: "#",
        title: "Hash Tables",
        subtitle: "O(1) Average Lookup",
        why_it_matters: "Hash tables achieve the holy grail: O(1) average-case lookup, insert, and delete. \
            They're behind every dictionary, set, cache, and database index. Understanding hashing \
            is essential for efficient algorithms.",
        intuition: r#"Imagine a huge library where books are shelved by the first letter of the author's name. Finding "Knuth" means going straight to the 'K' section - no searching through everything.

**A hash table works the same way**, but smarter. A hash function converts any key into an array index:

hash("Knuth") → 42 → store at index 42

The magic: this conversion is O(1). You compute the index directly instead of searching.

The catch: **collisions**. Two different keys might hash to the same index. Solutions:
- **Chaining**: Each bucket is a linked list of colliding items
- **Open addressing**: Find the next empty slot

With a good hash function and low load factor, operations stay O(1) on average."#,
        demo_explanation: "Enter a key and watch the hash function compute an index. See how collisions \
            are resolved through chaining. Notice how lookup goes directly to the bucket.",
        key_takeaways: &[
            "Average O(1) insert, lookup, delete",
            "Requires a good hash function for even distribution",
            "Collisions are inevitable - must be handled",
            "Load factor affects performance - resize when too full",
        ],
        going_deeper: "A good hash function: deterministic, uniform distribution, fast to compute. \
            Cryptographic hashes (SHA-256) are overkill for hash tables - simpler functions like \
            FNV or xxHash suffice. Robin Hood hashing and cuckoo hashing are advanced collision \
            strategies with better worst-case behavior.",
        math_details: r#"<p><strong>Time Complexity (average case):</strong></p>
$$\text{Insert: } O(1)$$
$$\text{Lookup: } O(1)$$
$$\text{Delete: } O(1)$$

<p><strong>Worst case (many collisions):</strong> O(n)</p>

<p><strong>Load Factor:</strong></p>
$$\alpha = n / \text{capacity}$$
<p>Resize when $\alpha > 0.7$ typically</p>"#,
        implementation: r#"<h4>HashMap in Rust</h4>
<pre><code>use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("key", 42);       // O(1) avg
let val = map.get("key");    // O(1) avg
map.remove("key");           // O(1) avg

// Iteration is O(n)
for (k, v) in &map {
    println!("{}: {}", k, v);
}</code></pre>

<h4>When to Use</h4>
<ul>
<li>Dictionary/map (key-value store)</li>
<li>Set membership testing</li>
<li>Caching and memoization</li>
<li>Counting frequencies</li>
</ul>"#,
    },
    Lesson {
        id: 8,
        icon: "o-o",
        title: "Graphs",
        subtitle: "Connections Everywhere",
        why_it_matters: "Graphs model relationships: social networks, road maps, dependencies, state \
            machines. BFS finds shortest paths, DFS explores all possibilities. Graphs are the \
            universal language of connected data.",
        intuition: r#"Think of a map of cities connected by roads. Each city is a **vertex** (node), each road is an **edge** (connection).

Graphs can be:
- **Directed**: One-way streets (Twitter follows)
- **Undirected**: Two-way roads (Facebook friends)
- **Weighted**: Roads with distances

**Two representations:**
1. **Adjacency matrix**: 2D array where matrix[i][j] = 1 if edge exists. Fast lookup O(1), but O(V^2) space.
2. **Adjacency list**: Each vertex stores a list of its neighbors. Space-efficient O(V+E).

**Two traversals:**
- **BFS**: Explore level by level, like ripples in a pond. Finds shortest path (unweighted).
- **DFS**: Go as deep as possible, then backtrack. Detects cycles, topological sort."#,
        demo_explanation: "Build a graph by adding vertices and edges. Run BFS to see the level-by-level \
            exploration (blue wave). Run DFS to see the depth-first path (follows one branch fully).",
        key_takeaways: &[
            "Vertices (nodes) + Edges (connections)",
            "Adjacency list is usually more space-efficient",
            "BFS finds shortest path, uses a queue",
            "DFS explores deeply, uses a stack (or recursion)",
        ],
        going_deeper: "BFS time is O(V+E) - visit every vertex and edge once. Dijkstra's algorithm extends \
            BFS for weighted graphs using a priority queue. DFS can detect cycles (back edges), find \
            connected components, and topologically sort DAGs. Many classic problems (shortest path, \
            minimum spanning tree, max flow) are graph problems.",
        math_details: r#"<p><strong>Space Complexity:</strong></p>
$$\text{Adjacency Matrix: } O(V^2)$$
$$\text{Adjacency List: } O(V + E)$$

<p><strong>Time Complexity:</strong></p>
$$\text{BFS/DFS: } O(V + E)$$
$$\text{Check edge (matrix): } O(1)$$
$$\text{Check edge (list): } O(\text{degree})$$"#,
        implementation: r#"<h4>Graph Representations</h4>
<pre><code>// Adjacency List
let mut graph: Vec&lt;Vec&lt;usize&gt;&gt; = vec![vec![]; n];
graph[0].push(1);  // Edge 0 -> 1
graph[1].push(0);  // Edge 1 -> 0 (undirected)

// BFS from vertex 0
let mut visited = vec![false; n];
let mut queue = VecDeque::from([0]);
while let Some(v) = queue.pop_front() {
    if visited[v] { continue; }
    visited[v] = true;
    for &neighbor in &graph[v] {
        queue.push_back(neighbor);
    }
}</code></pre>

<h4>Classic Algorithms</h4>
<ul>
<li><strong>BFS</strong>: Shortest path (unweighted)</li>
<li><strong>DFS</strong>: Cycle detection, topological sort</li>
<li><strong>Dijkstra</strong>: Shortest path (weighted)</li>
<li><strong>Kruskal/Prim</strong>: Minimum spanning tree</li>
</ul>"#,
    },
    Lesson {
        id: 9,
        icon: "/=\\",
        title: "Balanced Trees",
        subtitle: "Self-Correcting for Performance",
        why_it_matters: "Regular BSTs can degrade to O(n). Balanced trees guarantee O(log n) by \
            automatically rebalancing after insertions and deletions. AVL and Red-Black trees are \
            the workhorses behind most language standard libraries.",
        intuition: r#"Remember how a BST can become a linked list if you insert sorted data? Balanced trees prevent this by restructuring after operations.

**AVL Trees**: After every insert/delete, check if any node's subtrees differ in height by more than 1. If so, **rotate** to fix it.

Rotations are the magic move:
- **Right rotation**: When left subtree is too tall
- **Left rotation**: When right subtree is too tall
- **Double rotations**: For zig-zag cases

The result: height is always O(log n), so all operations stay O(log n) guaranteed.

**Red-Black Trees** use a different trick: color nodes red or black with rules that limit how unbalanced the tree can get. Less strictly balanced than AVL, but fewer rotations on average."#,
        demo_explanation: "Insert values and watch the tree automatically rebalance with rotations. \
            Notice how the height stays logarithmic even with sequential insertions.",
        key_takeaways: &[
            "Guarantee O(log n) operations regardless of insertion order",
            "Rotations are O(1) operations that preserve BST property",
            "AVL: stricter balance (height difference <= 1)",
            "Red-Black: looser balance, fewer rotations",
        ],
        going_deeper: "AVL trees are faster for lookups (more balanced), Red-Black trees are faster for \
            insertions/deletions (fewer rotations). Red-Black trees are used in C++ std::map, Java TreeMap, \
            Linux kernel. B-trees generalize this for disk storage - each node holds many keys, minimizing \
            disk reads.",
        math_details: r#"<p><strong>AVL Balance Factor:</strong></p>
$$\text{BF}(n) = \text{height}(n.\text{left}) - \text{height}(n.\text{right})$$
$$\text{Must be } \in \{-1, 0, 1\}$$

<p><strong>Time Complexity (guaranteed):</strong></p>
$$\text{Search: } O(\log n)$$
$$\text{Insert: } O(\log n)$$
$$\text{Delete: } O(\log n)$$

<p><strong>Height bounds:</strong></p>
$$\text{AVL: } h < 1.44 \log_2(n+2)$$
$$\text{Red-Black: } h \leq 2 \log_2(n+1)$$"#,
        implementation: r#"<h4>Rotation Concept</h4>
<pre><code>// Right rotation when left subtree too tall
//       y                x
//      / \              / \
//     x   C   --->     A   y
//    / \                  / \
//   A   B                B   C

fn rotate_right(y: Node) -> Node {
    let x = y.left.take();
    y.left = x.right;
    x.right = Some(y);
    x
}</code></pre>

<h4>When to Use</h4>
<ul>
<li>Need guaranteed O(log n) performance</li>
<li>Data arrives in sorted/nearly-sorted order</li>
<li>Implementing maps, sets, priority queues</li>
<li>Database indexes</li>
</ul>"#,
    },
];
