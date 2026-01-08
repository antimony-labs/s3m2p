//! ===============================================================================
//! FILE: heap_problems.rs | LEARN/learn_core/src/demos/problems/heap_problems.rs
//! PURPOSE: Heap/Top-K algorithm visualizations
//! MODIFIED: 2026-01-08
//! LAYER: LEARN -> learn_core -> demos -> problems
//! ===============================================================================

use crate::Demo;
use crate::demos::pseudocode::{Pseudocode, CodeLine};

// Static pseudocode for each variant
static KTH_LARGEST_CODE: &[CodeLine] = &[
    CodeLine::new("min_heap = []", 0),
    CodeLine::new("for num in nums:", 0),
    CodeLine::new("heappush(min_heap, num)", 1),
    CodeLine::new("if len(min_heap) > k:", 1),
    CodeLine::new("heappop(min_heap)", 2),
    CodeLine::new("return min_heap[0]  # kth largest", 0),
];

static MERGE_K_LISTS_CODE: &[CodeLine] = &[
    CodeLine::new("min_heap = []", 0),
    CodeLine::new("# Add first node from each list", 0),
    CodeLine::new("for i, list in enumerate(lists):", 0),
    CodeLine::new("if list: heappush(heap, (list.val, i))", 1),
    CodeLine::new("result = []", 0),
    CodeLine::new("while min_heap:", 0),
    CodeLine::new("val, i = heappop(min_heap)", 1),
    CodeLine::new("result.append(val)", 1),
    CodeLine::new("if lists[i].next:", 1),
    CodeLine::new("heappush(heap, (next.val, i))", 2),
    CodeLine::new("return result", 0),
];

static TOP_K_FREQUENT_CODE: &[CodeLine] = &[
    CodeLine::new("count = Counter(nums)", 0),
    CodeLine::new("min_heap = []", 0),
    CodeLine::new("for num, freq in count.items():", 0),
    CodeLine::new("heappush(min_heap, (freq, num))", 1),
    CodeLine::new("if len(min_heap) > k:", 1),
    CodeLine::new("heappop(min_heap)", 2),
    CodeLine::new("return [num for freq, num in min_heap]", 0),
];

static MEDIAN_STREAM_CODE: &[CodeLine] = &[
    CodeLine::new("max_heap = []  # lower half", 0),
    CodeLine::new("min_heap = []  # upper half", 0),
    CodeLine::new("def addNum(num):", 0),
    CodeLine::new("heappush(max_heap, -num)", 1),
    CodeLine::new("heappush(min_heap, -heappop(max_heap))", 1),
    CodeLine::new("if len(min_heap) > len(max_heap):", 1),
    CodeLine::new("heappush(max_heap, -heappop(min_heap))", 2),
    CodeLine::new("def findMedian():", 0),
    CodeLine::new("if len(max_heap) > len(min_heap):", 1),
    CodeLine::new("return -max_heap[0]", 2),
    CodeLine::new("return (-max_heap[0] + min_heap[0]) / 2", 1),
];

/// Animation state for heap problems
#[derive(Clone, Debug, Default)]
pub struct HeapProblemsDemo {
    /// Input array
    pub arr: Vec<i32>,
    /// Min heap
    pub min_heap: Vec<i32>,
    /// Max heap (for median)
    pub max_heap: Vec<i32>,
    /// K value
    pub k: usize,
    /// Current position in input
    pub pos: usize,
    /// Result
    pub result: Vec<i32>,
    /// Current median (for median stream)
    pub current_median: f32,
    /// Frequency map (for top k frequent)
    pub freq_map: Vec<(i32, i32)>, // (number, frequency)
    /// Current step
    pub step: usize,
    /// Whether complete
    pub complete: bool,
    /// Status message
    pub message: String,
    /// Pseudocode
    pub pseudocode: Pseudocode,
    /// Timer
    pub timer: f32,
    /// Problem variant
    pub variant: HeapProblemVariant,
    /// Highlight indices
    pub highlights: Vec<usize>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum HeapProblemVariant {
    #[default]
    KthLargest,
    MergeKSortedLists,
    TopKFrequent,
    MedianFromStream,
}

impl HeapProblemsDemo {
    pub fn new(variant: HeapProblemVariant) -> Self {
        let mut demo = Self {
            variant,
            ..Default::default()
        };
        demo.reset(42);
        demo
    }

    fn setup_kth_largest(&mut self) {
        self.arr = vec![3, 2, 1, 5, 6, 4];
        self.k = 2;
        self.min_heap.clear();
        self.pos = 0;
        self.pseudocode = Pseudocode::new("Kth Largest Element", KTH_LARGEST_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = format!("Find {}th largest element", self.k);
    }

    fn setup_merge_k_lists(&mut self) {
        // Representing 3 sorted lists: [1,4,5], [1,3,4], [2,6]
        self.arr = vec![1, 4, 5, 1, 3, 4, 2, 6];
        self.min_heap.clear();
        self.result.clear();
        self.pos = 0;
        self.pseudocode = Pseudocode::new("Merge K Sorted Lists", MERGE_K_LISTS_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = "Merge 3 sorted lists".to_string();
    }

    fn setup_top_k_frequent(&mut self) {
        self.arr = vec![1, 1, 1, 2, 2, 3];
        self.k = 2;
        self.min_heap.clear();
        self.freq_map = vec![(1, 3), (2, 2), (3, 1)]; // Pre-computed frequencies
        self.pos = 0;
        self.pseudocode = Pseudocode::new("Top K Frequent Elements", TOP_K_FREQUENT_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = format!("Find top {} frequent elements", self.k);
    }

    fn setup_median_stream(&mut self) {
        self.arr = vec![2, 3, 4, 1, 5, 6];
        self.min_heap.clear();
        self.max_heap.clear();
        self.pos = 0;
        self.current_median = 0.0;
        self.pseudocode = Pseudocode::new("Find Median from Data Stream", MEDIAN_STREAM_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = "Maintain running median".to_string();
    }

    fn heap_push(heap: &mut Vec<i32>, val: i32) {
        heap.push(val);
        // Bubble up
        let mut i = heap.len() - 1;
        while i > 0 {
            let parent = (i - 1) / 2;
            if heap[i] < heap[parent] {
                heap.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    fn heap_pop(heap: &mut Vec<i32>) -> Option<i32> {
        if heap.is_empty() {
            return None;
        }
        let result = heap[0];
        let last = heap.pop().unwrap();
        if !heap.is_empty() {
            heap[0] = last;
            // Bubble down
            let mut i = 0;
            loop {
                let left = 2 * i + 1;
                let right = 2 * i + 2;
                let mut smallest = i;
                if left < heap.len() && heap[left] < heap[smallest] {
                    smallest = left;
                }
                if right < heap.len() && heap[right] < heap[smallest] {
                    smallest = right;
                }
                if smallest != i {
                    heap.swap(i, smallest);
                    i = smallest;
                } else {
                    break;
                }
            }
        }
        Some(result)
    }

    fn max_heap_push(heap: &mut Vec<i32>, val: i32) {
        heap.push(val);
        let mut i = heap.len() - 1;
        while i > 0 {
            let parent = (i - 1) / 2;
            if heap[i] > heap[parent] {
                heap.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    fn max_heap_pop(heap: &mut Vec<i32>) -> Option<i32> {
        if heap.is_empty() {
            return None;
        }
        let result = heap[0];
        let last = heap.pop().unwrap();
        if !heap.is_empty() {
            heap[0] = last;
            let mut i = 0;
            loop {
                let left = 2 * i + 1;
                let right = 2 * i + 2;
                let mut largest = i;
                if left < heap.len() && heap[left] > heap[largest] {
                    largest = left;
                }
                if right < heap.len() && heap[right] > heap[largest] {
                    largest = right;
                }
                if largest != i {
                    heap.swap(i, largest);
                    i = largest;
                } else {
                    break;
                }
            }
        }
        Some(result)
    }

    pub fn step_algorithm(&mut self) {
        if self.complete {
            return;
        }

        match self.variant {
            HeapProblemVariant::KthLargest => self.step_kth_largest(),
            HeapProblemVariant::MergeKSortedLists => self.step_merge_k_lists(),
            HeapProblemVariant::TopKFrequent => self.step_top_k_frequent(),
            HeapProblemVariant::MedianFromStream => self.step_median_stream(),
        }

        self.step += 1;
    }

    fn step_kth_largest(&mut self) {
        if self.pos >= self.arr.len() {
            self.complete = true;
            self.pseudocode.current_line = Some(5);
            let result = self.min_heap.first().copied().unwrap_or(0);
            self.message = format!("{}th largest = {}", self.k, result);
            return;
        }

        let num = self.arr[self.pos];
        Self::heap_push(&mut self.min_heap, num);
        self.pseudocode.current_line = Some(2);
        self.message = format!("Push {} to heap: {:?}", num, self.min_heap);

        if self.min_heap.len() > self.k {
            let removed = Self::heap_pop(&mut self.min_heap).unwrap();
            self.pseudocode.current_line = Some(4);
            self.message = format!("Heap size > {}, pop {}: {:?}", self.k, removed, self.min_heap);
        }

        self.pos += 1;
    }

    fn step_merge_k_lists(&mut self) {
        // Simplified: just show merging concept
        if self.pos >= self.arr.len() {
            self.complete = true;
            self.pseudocode.current_line = Some(10);
            self.message = format!("Merged: {:?}", self.result);
            return;
        }

        // Simulate: add elements in sorted order
        let candidates: Vec<i32> = self.arr[self.pos..].iter()
            .take(3)
            .copied()
            .collect();

        if let Some(&min_val) = candidates.iter().min() {
            self.result.push(min_val);
            self.pseudocode.current_line = Some(7);
            self.message = format!("Pop min {} from heap, result: {:?}", min_val, self.result);

            // Find and mark as processed
            if let Some(idx) = self.arr[self.pos..].iter().position(|&x| x == min_val) {
                self.arr[self.pos + idx] = i32::MAX; // Mark as used
            }
        }

        self.pos += 1;
    }

    fn step_top_k_frequent(&mut self) {
        if self.pos >= self.freq_map.len() {
            self.complete = true;
            self.pseudocode.current_line = Some(6);
            let result: Vec<i32> = self.min_heap.clone();
            self.message = format!("Top {} frequent: {:?}", self.k, result);
            return;
        }

        let (num, freq) = self.freq_map[self.pos];
        self.min_heap.push(num);
        self.pseudocode.current_line = Some(3);
        self.message = format!("Add {} (freq {}), heap: {:?}", num, freq, self.min_heap);

        if self.min_heap.len() > self.k {
            self.min_heap.remove(0); // Simplified removal
            self.pseudocode.current_line = Some(5);
        }

        self.pos += 1;
    }

    fn step_median_stream(&mut self) {
        if self.pos >= self.arr.len() {
            self.complete = true;
            self.pseudocode.current_line = Some(10);
            self.message = format!("Final median: {}", self.current_median);
            return;
        }

        let num = self.arr[self.pos];

        // Add to max heap first
        Self::max_heap_push(&mut self.max_heap, num);
        self.pseudocode.current_line = Some(3);

        // Balance: move max of max_heap to min_heap
        if let Some(max_val) = Self::max_heap_pop(&mut self.max_heap) {
            Self::heap_push(&mut self.min_heap, max_val);
        }

        // Rebalance if min_heap is larger
        if self.min_heap.len() > self.max_heap.len() {
            if let Some(min_val) = Self::heap_pop(&mut self.min_heap) {
                Self::max_heap_push(&mut self.max_heap, min_val);
            }
        }

        // Calculate median
        if self.max_heap.len() > self.min_heap.len() {
            self.current_median = self.max_heap[0] as f32;
        } else if !self.max_heap.is_empty() && !self.min_heap.is_empty() {
            self.current_median = (self.max_heap[0] + self.min_heap[0]) as f32 / 2.0;
        }

        self.pseudocode.current_line = Some(8);
        self.message = format!(
            "Add {}, max_heap: {:?}, min_heap: {:?}, median: {}",
            num, self.max_heap, self.min_heap, self.current_median
        );

        self.pos += 1;
    }

    pub fn get_heap(&self) -> &[i32] {
        &self.min_heap
    }
}

impl Demo for HeapProblemsDemo {
    fn reset(&mut self, _seed: u64) {
        self.step = 0;
        self.complete = false;
        self.timer = 0.0;
        self.min_heap.clear();
        self.max_heap.clear();
        self.result.clear();
        self.pos = 0;
        self.current_median = 0.0;
        self.highlights.clear();

        match self.variant {
            HeapProblemVariant::KthLargest => self.setup_kth_largest(),
            HeapProblemVariant::MergeKSortedLists => self.setup_merge_k_lists(),
            HeapProblemVariant::TopKFrequent => self.setup_top_k_frequent(),
            HeapProblemVariant::MedianFromStream => self.setup_median_stream(),
        }
    }

    fn step(&mut self, dt: f32) {
        self.timer += dt;
    }

    fn set_param(&mut self, _name: &str, _value: f32) -> bool {
        false
    }

    fn params() -> &'static [crate::demo::ParamMeta] {
        &[]
    }
}
