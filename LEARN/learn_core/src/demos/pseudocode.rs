//! ===============================================================================
//! FILE: pseudocode.rs | LEARN/learn_core/src/demos/pseudocode.rs
//! PURPOSE: Pseudocode display and highlighting for data structure demos
//! MODIFIED: 2026-01-08
//! LAYER: LEARN -> learn_core -> demos
//! ===============================================================================

/// A line of pseudocode with optional indentation
#[derive(Clone, Debug)]
pub struct CodeLine {
    pub text: &'static str,
    pub indent: usize,
}

impl CodeLine {
    pub const fn new(text: &'static str, indent: usize) -> Self {
        Self { text, indent }
    }
}

/// Pseudocode state for visualization
#[derive(Clone, Debug, Default)]
pub struct Pseudocode {
    /// The operation being performed
    pub operation: &'static str,
    /// Lines of pseudocode
    pub lines: &'static [CodeLine],
    /// Currently highlighted line (0-indexed), None if no highlight
    pub current_line: Option<usize>,
}

impl Pseudocode {
    pub const fn new(operation: &'static str, lines: &'static [CodeLine]) -> Self {
        Self {
            operation,
            lines,
            current_line: None,
        }
    }

    pub fn set_line(&mut self, line: usize) {
        self.current_line = Some(line);
    }

    pub fn clear(&mut self) {
        self.operation = "";
        self.lines = &[];
        self.current_line = None;
    }
}

// ===============================================================================
// ARRAY PSEUDOCODE
// ===============================================================================

pub mod array {
    use super::CodeLine;

    pub static ACCESS: &[CodeLine] = &[
        CodeLine::new("function access(arr, index):", 0),
        CodeLine::new("if index < 0 or index >= size:", 1),
        CodeLine::new("return ERROR", 2),
        CodeLine::new("return arr[index]  // O(1)", 1),
    ];

    pub static INSERT: &[CodeLine] = &[
        CodeLine::new("function insert(arr, index, value):", 0),
        CodeLine::new("if size >= capacity:", 1),
        CodeLine::new("return ERROR  // array full", 2),
        CodeLine::new("// Shift elements right", 1),
        CodeLine::new("for i from size-1 down to index:", 1),
        CodeLine::new("arr[i+1] = arr[i]", 2),
        CodeLine::new("arr[index] = value", 1),
        CodeLine::new("size = size + 1", 1),
    ];

    pub static DELETE: &[CodeLine] = &[
        CodeLine::new("function delete(arr, index):", 0),
        CodeLine::new("if index >= size:", 1),
        CodeLine::new("return ERROR", 2),
        CodeLine::new("// Shift elements left", 1),
        CodeLine::new("for i from index to size-2:", 1),
        CodeLine::new("arr[i] = arr[i+1]", 2),
        CodeLine::new("size = size - 1", 1),
    ];
}

// ===============================================================================
// LINKED LIST PSEUDOCODE
// ===============================================================================

pub mod linked_list {
    use super::CodeLine;

    pub static INSERT_HEAD: &[CodeLine] = &[
        CodeLine::new("function insertHead(value):", 0),
        CodeLine::new("newNode = createNode(value)", 1),
        CodeLine::new("newNode.next = head", 1),
        CodeLine::new("head = newNode", 1),
        CodeLine::new("size = size + 1", 1),
    ];

    pub static INSERT_TAIL: &[CodeLine] = &[
        CodeLine::new("function insertTail(value):", 0),
        CodeLine::new("newNode = createNode(value)", 1),
        CodeLine::new("if head is null:", 1),
        CodeLine::new("head = newNode", 2),
        CodeLine::new("else:", 1),
        CodeLine::new("current = head", 2),
        CodeLine::new("while current.next != null:", 2),
        CodeLine::new("current = current.next", 3),
        CodeLine::new("current.next = newNode", 2),
        CodeLine::new("size = size + 1", 1),
    ];

    pub static DELETE_HEAD: &[CodeLine] = &[
        CodeLine::new("function deleteHead():", 0),
        CodeLine::new("if head is null:", 1),
        CodeLine::new("return ERROR", 2),
        CodeLine::new("temp = head", 1),
        CodeLine::new("head = head.next", 1),
        CodeLine::new("delete temp", 1),
        CodeLine::new("size = size - 1", 1),
    ];

    pub static SEARCH: &[CodeLine] = &[
        CodeLine::new("function search(value):", 0),
        CodeLine::new("current = head", 1),
        CodeLine::new("index = 0", 1),
        CodeLine::new("while current != null:", 1),
        CodeLine::new("if current.value == value:", 2),
        CodeLine::new("return index  // Found!", 3),
        CodeLine::new("current = current.next", 2),
        CodeLine::new("index = index + 1", 2),
        CodeLine::new("return NOT_FOUND", 1),
    ];
}

// ===============================================================================
// STACK PSEUDOCODE
// ===============================================================================

pub mod stack {
    use super::CodeLine;

    pub static PUSH: &[CodeLine] = &[
        CodeLine::new("function push(value):", 0),
        CodeLine::new("if size >= capacity:", 1),
        CodeLine::new("return OVERFLOW", 2),
        CodeLine::new("top = top + 1", 1),
        CodeLine::new("stack[top] = value", 1),
        CodeLine::new("size = size + 1", 1),
    ];

    pub static POP: &[CodeLine] = &[
        CodeLine::new("function pop():", 0),
        CodeLine::new("if isEmpty():", 1),
        CodeLine::new("return UNDERFLOW", 2),
        CodeLine::new("value = stack[top]", 1),
        CodeLine::new("top = top - 1", 1),
        CodeLine::new("size = size - 1", 1),
        CodeLine::new("return value", 1),
    ];

    pub static PEEK: &[CodeLine] = &[
        CodeLine::new("function peek():", 0),
        CodeLine::new("if isEmpty():", 1),
        CodeLine::new("return ERROR", 2),
        CodeLine::new("return stack[top]  // O(1)", 1),
    ];
}

// ===============================================================================
// QUEUE PSEUDOCODE
// ===============================================================================

pub mod queue {
    use super::CodeLine;

    pub static ENQUEUE: &[CodeLine] = &[
        CodeLine::new("function enqueue(value):", 0),
        CodeLine::new("if size >= capacity:", 1),
        CodeLine::new("return OVERFLOW", 2),
        CodeLine::new("rear = (rear + 1) % capacity", 1),
        CodeLine::new("queue[rear] = value", 1),
        CodeLine::new("size = size + 1", 1),
    ];

    pub static DEQUEUE: &[CodeLine] = &[
        CodeLine::new("function dequeue():", 0),
        CodeLine::new("if isEmpty():", 1),
        CodeLine::new("return UNDERFLOW", 2),
        CodeLine::new("value = queue[front]", 1),
        CodeLine::new("front = (front + 1) % capacity", 1),
        CodeLine::new("size = size - 1", 1),
        CodeLine::new("return value", 1),
    ];

    pub static PEEK: &[CodeLine] = &[
        CodeLine::new("function peek():", 0),
        CodeLine::new("if isEmpty():", 1),
        CodeLine::new("return ERROR", 2),
        CodeLine::new("return queue[front]  // O(1)", 1),
    ];
}

// ===============================================================================
// BINARY TREE PSEUDOCODE
// ===============================================================================

pub mod binary_tree {
    use super::CodeLine;

    pub static INSERT: &[CodeLine] = &[
        CodeLine::new("function insert(root, value):", 0),
        CodeLine::new("newNode = createNode(value)", 1),
        CodeLine::new("if root is null:", 1),
        CodeLine::new("return newNode", 2),
        CodeLine::new("queue.enqueue(root)", 1),
        CodeLine::new("while not queue.isEmpty():", 1),
        CodeLine::new("current = queue.dequeue()", 2),
        CodeLine::new("if current.left is null:", 2),
        CodeLine::new("current.left = newNode", 3),
        CodeLine::new("return root", 3),
        CodeLine::new("else queue.enqueue(current.left)", 2),
        CodeLine::new("if current.right is null:", 2),
        CodeLine::new("current.right = newNode", 3),
        CodeLine::new("return root", 3),
        CodeLine::new("else queue.enqueue(current.right)", 2),
    ];

    pub static PREORDER: &[CodeLine] = &[
        CodeLine::new("function preorder(node):", 0),
        CodeLine::new("if node is null:", 1),
        CodeLine::new("return", 2),
        CodeLine::new("visit(node)        // Root first", 1),
        CodeLine::new("preorder(node.left)  // Then left", 1),
        CodeLine::new("preorder(node.right) // Then right", 1),
    ];

    pub static INORDER: &[CodeLine] = &[
        CodeLine::new("function inorder(node):", 0),
        CodeLine::new("if node is null:", 1),
        CodeLine::new("return", 2),
        CodeLine::new("inorder(node.left)   // Left first", 1),
        CodeLine::new("visit(node)          // Then root", 1),
        CodeLine::new("inorder(node.right)  // Then right", 1),
    ];

    pub static POSTORDER: &[CodeLine] = &[
        CodeLine::new("function postorder(node):", 0),
        CodeLine::new("if node is null:", 1),
        CodeLine::new("return", 2),
        CodeLine::new("postorder(node.left)  // Left first", 1),
        CodeLine::new("postorder(node.right) // Then right", 1),
        CodeLine::new("visit(node)           // Root last", 1),
    ];

    pub static LEVELORDER: &[CodeLine] = &[
        CodeLine::new("function levelorder(root):", 0),
        CodeLine::new("if root is null: return", 1),
        CodeLine::new("queue.enqueue(root)", 1),
        CodeLine::new("while not queue.isEmpty():", 1),
        CodeLine::new("node = queue.dequeue()", 2),
        CodeLine::new("visit(node)", 2),
        CodeLine::new("if node.left: queue.enqueue(node.left)", 2),
        CodeLine::new("if node.right: queue.enqueue(node.right)", 2),
    ];
}

// ===============================================================================
// BST PSEUDOCODE
// ===============================================================================

pub mod bst {
    use super::CodeLine;

    pub static INSERT: &[CodeLine] = &[
        CodeLine::new("function insert(root, value):", 0),
        CodeLine::new("if root is null:", 1),
        CodeLine::new("return createNode(value)", 2),
        CodeLine::new("if value < root.value:", 1),
        CodeLine::new("root.left = insert(root.left, value)", 2),
        CodeLine::new("else if value > root.value:", 1),
        CodeLine::new("root.right = insert(root.right, value)", 2),
        CodeLine::new("return root", 1),
    ];

    pub static SEARCH: &[CodeLine] = &[
        CodeLine::new("function search(root, value):", 0),
        CodeLine::new("if root is null:", 1),
        CodeLine::new("return NOT_FOUND", 2),
        CodeLine::new("if value == root.value:", 1),
        CodeLine::new("return root  // Found!", 2),
        CodeLine::new("if value < root.value:", 1),
        CodeLine::new("return search(root.left, value)", 2),
        CodeLine::new("else:", 1),
        CodeLine::new("return search(root.right, value)", 2),
    ];
}

// ===============================================================================
// HEAP PSEUDOCODE
// ===============================================================================

pub mod heap {
    use super::CodeLine;

    pub static INSERT: &[CodeLine] = &[
        CodeLine::new("function insert(value):", 0),
        CodeLine::new("heap.append(value)", 1),
        CodeLine::new("index = size - 1", 1),
        CodeLine::new("// Bubble up", 1),
        CodeLine::new("while index > 0:", 1),
        CodeLine::new("parent = (index - 1) / 2", 2),
        CodeLine::new("if heap[index] > heap[parent]:", 2),
        CodeLine::new("swap(heap[index], heap[parent])", 3),
        CodeLine::new("index = parent", 3),
        CodeLine::new("else: break", 2),
    ];

    pub static EXTRACT: &[CodeLine] = &[
        CodeLine::new("function extractMax():", 0),
        CodeLine::new("if isEmpty(): return ERROR", 1),
        CodeLine::new("max = heap[0]", 1),
        CodeLine::new("heap[0] = heap[size-1]", 1),
        CodeLine::new("size = size - 1", 1),
        CodeLine::new("// Sink down", 1),
        CodeLine::new("index = 0", 1),
        CodeLine::new("while hasChildren(index):", 1),
        CodeLine::new("largest = getLargestChild(index)", 2),
        CodeLine::new("if heap[index] < heap[largest]:", 2),
        CodeLine::new("swap(heap[index], heap[largest])", 3),
        CodeLine::new("index = largest", 3),
        CodeLine::new("else: break", 2),
        CodeLine::new("return max", 1),
    ];
}

// ===============================================================================
// HASH TABLE PSEUDOCODE
// ===============================================================================

pub mod hash_table {
    use super::CodeLine;

    pub static INSERT: &[CodeLine] = &[
        CodeLine::new("function insert(key, value):", 0),
        CodeLine::new("index = hash(key) % numBuckets", 1),
        CodeLine::new("// Search bucket for existing key", 1),
        CodeLine::new("for entry in buckets[index]:", 1),
        CodeLine::new("if entry.key == key:", 2),
        CodeLine::new("entry.value = value  // Update", 3),
        CodeLine::new("return", 3),
        CodeLine::new("// Key not found, add new entry", 1),
        CodeLine::new("buckets[index].append((key, value))", 1),
        CodeLine::new("size = size + 1", 1),
    ];

    pub static SEARCH: &[CodeLine] = &[
        CodeLine::new("function search(key):", 0),
        CodeLine::new("index = hash(key) % numBuckets", 1),
        CodeLine::new("// Search bucket chain", 1),
        CodeLine::new("for entry in buckets[index]:", 1),
        CodeLine::new("if entry.key == key:", 2),
        CodeLine::new("return entry.value  // Found!", 3),
        CodeLine::new("return NOT_FOUND", 1),
    ];
}

// ===============================================================================
// GRAPH PSEUDOCODE
// ===============================================================================

pub mod graph {
    use super::CodeLine;

    pub static BFS: &[CodeLine] = &[
        CodeLine::new("function BFS(start):", 0),
        CodeLine::new("visited = set()", 1),
        CodeLine::new("queue = [start]", 1),
        CodeLine::new("visited.add(start)", 1),
        CodeLine::new("while queue not empty:", 1),
        CodeLine::new("vertex = queue.dequeue()", 2),
        CodeLine::new("process(vertex)", 2),
        CodeLine::new("for neighbor in adjacent(vertex):", 2),
        CodeLine::new("if neighbor not in visited:", 3),
        CodeLine::new("visited.add(neighbor)", 4),
        CodeLine::new("queue.enqueue(neighbor)", 4),
    ];

    pub static DFS: &[CodeLine] = &[
        CodeLine::new("function DFS(start):", 0),
        CodeLine::new("visited = set()", 1),
        CodeLine::new("stack = [start]", 1),
        CodeLine::new("while stack not empty:", 1),
        CodeLine::new("vertex = stack.pop()", 2),
        CodeLine::new("if vertex not in visited:", 2),
        CodeLine::new("visited.add(vertex)", 3),
        CodeLine::new("process(vertex)", 3),
        CodeLine::new("for neighbor in adjacent(vertex):", 3),
        CodeLine::new("if neighbor not in visited:", 4),
        CodeLine::new("stack.push(neighbor)", 5),
    ];
}

// ===============================================================================
// AVL TREE PSEUDOCODE
// ===============================================================================

pub mod avl {
    use super::CodeLine;

    pub static INSERT: &[CodeLine] = &[
        CodeLine::new("function insert(root, value):", 0),
        CodeLine::new("// Standard BST insert", 1),
        CodeLine::new("if root is null:", 1),
        CodeLine::new("return createNode(value)", 2),
        CodeLine::new("if value < root.value:", 1),
        CodeLine::new("root.left = insert(root.left, value)", 2),
        CodeLine::new("else:", 1),
        CodeLine::new("root.right = insert(root.right, value)", 2),
        CodeLine::new("// Update height", 1),
        CodeLine::new("root.height = 1 + max(height(left), height(right))", 1),
        CodeLine::new("// Check balance factor", 1),
        CodeLine::new("balance = height(left) - height(right)", 1),
        CodeLine::new("// Left-Left case", 1),
        CodeLine::new("if balance > 1 and value < root.left.value:", 1),
        CodeLine::new("return rotateRight(root)", 2),
        CodeLine::new("// Right-Right case", 1),
        CodeLine::new("if balance < -1 and value > root.right.value:", 1),
        CodeLine::new("return rotateLeft(root)", 2),
        CodeLine::new("// Left-Right case", 1),
        CodeLine::new("if balance > 1 and value > root.left.value:", 1),
        CodeLine::new("root.left = rotateLeft(root.left)", 2),
        CodeLine::new("return rotateRight(root)", 2),
        CodeLine::new("// Right-Left case", 1),
        CodeLine::new("if balance < -1 and value < root.right.value:", 1),
        CodeLine::new("root.right = rotateRight(root.right)", 2),
        CodeLine::new("return rotateLeft(root)", 2),
        CodeLine::new("return root", 1),
    ];

    pub static ROTATE_RIGHT: &[CodeLine] = &[
        CodeLine::new("function rotateRight(y):", 0),
        CodeLine::new("x = y.left", 1),
        CodeLine::new("T2 = x.right", 1),
        CodeLine::new("// Perform rotation", 1),
        CodeLine::new("x.right = y", 1),
        CodeLine::new("y.left = T2", 1),
        CodeLine::new("// Update heights", 1),
        CodeLine::new("y.height = max(height(y.left), height(y.right)) + 1", 1),
        CodeLine::new("x.height = max(height(x.left), height(x.right)) + 1", 1),
        CodeLine::new("return x  // New root", 1),
    ];

    pub static ROTATE_LEFT: &[CodeLine] = &[
        CodeLine::new("function rotateLeft(x):", 0),
        CodeLine::new("y = x.right", 1),
        CodeLine::new("T2 = y.left", 1),
        CodeLine::new("// Perform rotation", 1),
        CodeLine::new("y.left = x", 1),
        CodeLine::new("x.right = T2", 1),
        CodeLine::new("// Update heights", 1),
        CodeLine::new("x.height = max(height(x.left), height(x.right)) + 1", 1),
        CodeLine::new("y.height = max(height(y.left), height(y.right)) + 1", 1),
        CodeLine::new("return y  // New root", 1),
    ];
}
