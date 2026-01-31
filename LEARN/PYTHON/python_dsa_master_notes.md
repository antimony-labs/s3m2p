# Python + DSA Master Notes (One-Spot Revision Doc)
*Last updated: 2026-01-31*

> My single source of truth for Python fundamentals + DSA-ready patterns.
> Skim this daily, then drill with problems.

---

## Table of Contents
- [0) How to Use This Doc](#0-how-to-use-this-doc)
- [1) Python Mental Model](#1-python-mental-model)
- [2) Variables, Types & I/O](#2-variables-types--io)
- [3) Operators & Boolean Logic](#3-operators--boolean-logic)
- [4) Control Flow](#4-control-flow)
- [5) Functions](#5-functions)
- [6) Strings & Text Processing](#6-strings--text-processing)
- [7) Lists & Tuples](#7-lists--tuples)
- [8) Sets](#8-sets)
- [9) Dictionaries](#9-dictionaries)
- [10) Complexity + Memory Model](#10-complexity--memory-model)
- [11) OOP Basics](#11-oop-basics)
- [12) DSA Patterns Cheatsheet](#12-dsa-patterns-cheatsheet)
- [13) LeetCode Practice Tracker](#13-leetcode-practice-tracker)
- [14) Bug Journal](#14-bug-journal)
- [15) Cheat Sheets](#15-cheat-sheets)
- [16) Mini-Revision Drills (10–15 min)](#16-mini-revision-drills-1015-min)

---

## 0) How to Use This Doc
### Daily minimum (non-negotiable)
- [ ] 10 min revision (skim headings + examples)
- [ ] 1 LeetCode attempt (even if fail)
- [ ] 1 note in Bug Journal **or** 1 question added

### What “I understand it” means
- I can explain it in 3–5 lines **without** looking.
- I can apply it to a new problem variation.

---

## 1) Python Mental Model
Python is **high-level, interpreted, general-purpose**.

**Execution model**
- Code runs top-to-bottom
- Indentation defines blocks
- Expressions produce values, statements do work

```python
print("Hello, Python!")
3 + 2   # expression
```

---

## 2) Variables, Types & I/O
### Built-ins I should know cold
`print`, `len`, `type`, `int`, `float`, `str`, `min`, `max`, `sum`, `range`, `enumerate`, `sorted`

### Variables
A variable is a **name bound to an object** (a reference).

```python
name = "Ada"
age = 36
```

### Input (IMPORTANT)
`input()` always returns **string**.

```python
age = int(input("Age? "))
```

### Casting
```python
x = "10.5"
xf = float(x)   # 10.5
xi = int(xf)    # 10
```

---

## 3) Operators & Boolean Logic
### Arithmetic
`+  -  *  /  //  %  **`

### Comparison
```python
3 > 2   # True
3 == 2  # False
```

### Identity vs equality
- `==` compares values
- `is` compares identity

```python
a = [1,2]
b = [1,2]
a == b   # True
a is b   # False
```

---

## 4) Control Flow
```python
if x > 0:
    print("pos")
elif x == 0:
    print("zero")
else:
    print("neg")
```

```python
for i in range(5):
    print(i)
```

```python
while n > 0:
    n -= 1
```

---

## 5) Functions
```python
def add(a, b):
    return a + b
```

**Key ideas**
- `return` gives back a value
- `print` only displays
- default params are useful

```python
def greet(name="friend"):
    return f"Hello, {name}"
```

---

## 6) Strings & Text Processing
### Core facts
- Strings are **immutable**
- Slicing is powerful

```python
s = "Python"
s[0]     # P
s[-1]    # n
s[::-1]  # reverse
```

### Methods to memorize
- Clean: `strip`, `lstrip`, `rstrip`
- Case: `lower`, `upper`, `title`, `capitalize`
- Search: `find`, `index`, `count`
- Match: `startswith`, `endswith`
- Split/Join: `split`, `'sep'.join(...)`
- Replace: `replace`
- Check: `isalnum`, `isalpha`, `isdigit`, `isnumeric`

### Palindrome normalization
```python
def normalize(s):
    return "".join(ch.lower() for ch in s if ch.isalnum())
```

---

## 7) Lists & Tuples
### Lists (mutable)
```python
nums = [1,2,3]
nums.append(4)
```

### List methods
- `append(x)` add one
- `extend(iter)` add many
- `insert(i,x)` add at index
- `remove(x)` remove first match
- `pop(i)` remove by index
- `clear()` remove all
- `index(x)` find position
- `count(x)` count value
- `sort()` in place
- `reverse()` in place
- `copy()` shallow copy

### Tuples (immutable)
```python
point = (3,4)
```

### Tuple ops
`len`, `count`, `index`, unpacking (`a,b = t`)

### Copying lists (IMPORTANT)
```python
a = [1,2]
b = a          # alias
c = a.copy()   # new list
```

---

## 8) Sets
### Set facts
- Unordered, unique
- Fast membership

```python
st = set()
fruits = {"apple", "banana"}
```

### Set methods
- `add(x)`
- `update(iter)`
- `remove(x)` (error if missing)
- `discard(x)` (safe)
- `pop()` (arbitrary)
- `clear()`

### Set algebra
```python
A | B   # union
A & B   # intersection
A - B   # difference
A ^ B   # symmetric difference
```

---

## 9) Dictionaries
### Dict facts
- key → value mapping
- Fast lookup

```python
person = {"name": "Ada", "age": 36}
```

### Dict methods
- `get(k, d)` safe lookup
- `keys()`, `values()`, `items()`
- `update(...)`
- `pop(k)`, `popitem()`
- `setdefault(k, d)`

### Frequency pattern
```python
def freq_count(nums):
    counts = {}
    for x in nums:
        counts[x] = counts.get(x, 0) + 1
    return counts
```

### Two Sum pattern
```python
def two_sum(nums, target):
    seen = {}
    for i, x in enumerate(nums):
        need = target - x
        if need in seen:
            return [seen[need], i]
        seen[x] = i
```

---

## 10) Complexity + Memory Model
### Sanity checks
- `x in list` → O(n)
- `x in set` or `x in dict` → ~O(1)

### Mutability
- Immutable: `int, float, str, tuple`
- Mutable: `list, dict, set`

```python
def add_one(lst):
    lst.append(1)

x = []
add_one(x)
print(x)
```

---

## 11) OOP Basics
```python
class Player:
    def __init__(self, name):
        self.name = name
        self.score = 0

    def add(self, points):
        self.score += points
```

- `self` is the current instance
- Use classes when data + behavior belong together

---

## 12) DSA Patterns Cheatsheet
### Two pointers
- Sorted arrays, pair from ends

### Sliding window
- Subarray with sum/condition

### Prefix sums
- Range sum queries

### Hash map (dict)
- Count frequency, complement lookup

### Stack
- Valid parentheses, monotonic stack

---

## 13) LeetCode Practice Tracker
| #  | Problem                        | Topic    | Status | Key Idea                 | My Mistake | Retry Date |
| -- | ------------------------------ | -------- | ------ | ------------------------ | ---------- | ---------- |
| 1  | Length of Last Word            | strings  |        | split/scan               |            |            |
| 2  | Valid Palindrome               | strings  |        | normalize + two pointers |            |            |
| 3  | Reverse String                 | strings  |        | two pointers             |            |            |
| 4  | Running Sum                    | lists    |        | prefix running total     |            |            |
| 5  | Remove Duplicates Sorted Array | lists    |        | slow/fast pointers       |            |            |
| 6  | Valid Anagram                  | dict     |        | freq counts              |            |            |
| 7  | Find the Difference            | dict/XOR |        | freq or XOR              |            |            |
| 8  | Contains Duplicate             | set      |        | set length compare       |            |            |
| 9  | Intersection of Two Arrays     | set      |        | set intersection         |            |            |
| 10 | Two Sum                        | dict     |        | complement lookup        |            |            |
| 11 | Majority Element               | dict     |        | count max                |            |            |

---

## 14) Bug Journal
Use this format whenever I get stuck.

- Date:
- Problem:
- What I expected:
- What happened:
- Root cause:
- Fix:
- Rule for next time:

Example:
- Root cause: used `b = a` instead of `a.copy()`

---

## 15) Cheat Sheets
### If you see X, think Y
- “Unique elements” → `set`
- “Count frequency” → `dict`
- “Lookup complement / previously seen” → `dict`
- “Reverse / palindrome / pair from ends” → two pointers
- “Running total” → prefix sum idea

### Must-memorize built-ins
`len`, `type`, `range`, `enumerate`, `sorted`, `sum`, `min`, `max`

### Complexity reminders
- list membership → O(n)
- set/dict lookup → ~O(1)

---

## 16) Mini-Revision Drills (10–15 min)
### Drill A — Type + operator rules
- What is `type(3/2)`?
- What is `7//2`?
- What is `7%2`?

### Drill B — Strings
- Reverse a string (2 ways)
- Normalize for palindrome (alnum + lower)

### Drill C — Lists
- Copy vs alias
- Two-pointer remove duplicates pattern (concept)

### Drill D — Dict
- Write frequency counter
- Two Sum in 30 seconds

---

✅ End of master notes.
