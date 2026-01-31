//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lessons.rs | PYTHON/src/lessons.rs
//! PURPOSE: Python fundamentals + DSA-ready curriculum
//! MODIFIED: 2026-01-31
//! ═══════════════════════════════════════════════════════════════════════════════

/// A single Python lesson
pub struct Lesson {
    pub id: usize,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub icon: &'static str,
    pub phase: &'static str,
    pub why_it_matters: &'static str,
    pub intuition: &'static str,
    pub content: &'static str,
    pub key_concepts: &'static [&'static str],
    pub key_takeaways: &'static [&'static str],
    pub dos_and_donts: &'static str,
    pub going_deeper: &'static str,
    pub common_mistakes: &'static str,
}

/// Glossary term with tooltip explanation
#[derive(Clone)]
pub struct Term {
    pub word: &'static str,
    pub short: &'static str,
    pub detail: &'static str,
}

/// Glossary of Python technical terms
pub static GLOSSARY: &[Term] = &[
    Term {
        word: "variable",
        short: "A name bound to an object",
        detail: "Variables are labels that point to objects in memory. Rebinding a name does not change the object itself.",
    },
    Term {
        word: "expression",
        short: "Produces a value",
        detail: "Expressions evaluate to a value, e.g., 3 + 2 or len(name).",
    },
    Term {
        word: "statement",
        short: "Performs an action",
        detail: "Statements do work, e.g., print(...), if/for/while, or assignment.",
    },
    Term {
        word: "indentation",
        short: "Defines code blocks",
        detail: "Python uses indentation (usually 4 spaces) to group statements into blocks.",
    },
    Term {
        word: "repl",
        short: "Interactive shell",
        detail: "REPL = Read–Eval–Print Loop, where you can run Python interactively.",
    },
    Term {
        word: "dynamic typing",
        short: "Types checked at runtime",
        detail: "In Python, variable types are determined when the program runs, not at compile time.",
    },
    Term {
        word: "casting",
        short: "Convert between types",
        detail: "Use int(), float(), str() to convert values between types.",
    },
    Term {
        word: "none",
        short: "The absence of a value",
        detail: "None represents “no value” and is often used as a default or placeholder.",
    },
    Term {
        word: "object",
        short: "A value with a type",
        detail: "Everything in Python is an object: it has a type, identity, and value.",
    },
    Term {
        word: "mutable",
        short: "Can be changed in place",
        detail: "Mutable objects (list, dict, set) can be modified without creating a new object.",
    },
    Term {
        word: "immutable",
        short: "Cannot be changed in place",
        detail: "Immutable objects (int, float, str, tuple) cannot be modified; new objects are created instead.",
    },
    Term {
        word: "list",
        short: "Ordered, mutable collection",
        detail: "Lists store items in order and can grow or shrink dynamically.",
    },
    Term {
        word: "tuple",
        short: "Ordered, immutable collection",
        detail: "Tuples are fixed-size sequences often used for safe grouping.",
    },
    Term {
        word: "set",
        short: "Unordered, unique collection",
        detail: "Sets store unique items with fast membership checks.",
    },
    Term {
        word: "dictionary",
        short: "Key → value mapping",
        detail: "Dictionaries (dict) map unique keys to values using hashing.",
    },
    Term {
        word: "hashing",
        short: "Map keys to indices",
        detail: "Hashing converts keys into numbers used to index hash tables.",
    },
    Term {
        word: "hash",
        short: "Numeric fingerprint of a value",
        detail: "Hash functions map keys to table indices; required for dict/set keys.",
    },
    Term {
        word: "slicing",
        short: "Subsequence extraction",
        detail: "Slicing selects parts of a sequence using [start:stop:step].",
    },
    Term {
        word: "comprehension",
        short: "Compact collection builder",
        detail: "Comprehensions build lists/sets/dicts in a single readable line.",
    },
    Term {
        word: "big-o",
        short: "Growth of time/space",
        detail: "Big-O describes how runtime or memory grows as input size increases.",
    },
    Term {
        word: "iterator",
        short: "Object that yields items one by one",
        detail: "Iterators implement __next__ and are consumed sequentially.",
    },
    Term {
        word: "generator",
        short: "Lazy iterator created with yield",
        detail: "Generators produce values on demand and save memory.",
    },
    Term {
        word: "exception",
        short: "Runtime error signal",
        detail: "Exceptions are raised on errors and handled with try/except.",
    },
    Term {
        word: "class",
        short: "Blueprint for objects",
        detail: "Classes define data and behavior; instances are the actual objects.",
    },
    Term {
        word: "instance",
        short: "A concrete object",
        detail: "An instance is a specific object created from a class.",
    },
    Term {
        word: "scope",
        short: "Where a name is visible",
        detail: "Local, enclosing, global, and built-in scopes determine name resolution (LEGB).",
    },
];

/// Curriculum phases
pub static PHASES: &[&str] = &[
    "Foundations",
    "Core Python",
    "Data Structures",
    "Problem Solving",
    "OOP + Practice",
];

/// All lessons
pub static LESSONS: &[Lesson] = &[
    Lesson {
        id: 0,
        title: "Welcome + Python Mental Model",
        subtitle: "How Python executes and why indentation matters",
        icon: "🐍",
        phase: "Foundations",
        why_it_matters: "A clean mental model prevents 80% of beginner bugs and makes problem solving faster.",
        intuition: "Think of Python as a smart calculator plus a notebook. Expressions create values, and names point to those values.",
        content: r#"
## What Python is
- High-level, interpreted, general-purpose
- Runs top-to-bottom, line-by-line (conceptually)
- Indentation defines blocks

## First 90 seconds
```python
print("Hello, Python!")
score = 3 + 2
print("score =", score)
```

## Mental model
- **Expression** → produces a value (`3 + 2`)
- **Statement** → does work (`print(...)`, `if ...`)
- **Name** → points to an object (`score = 5`)

## Quick check
- Q: What does `type(3/2)` return?
- A: `float`
- Q: Why does indentation matter?
- A: It defines code blocks (if/for/def)

## Mini drills
- Print your name and age on separate lines
- Evaluate `7 // 2` and `7 % 2`
- Try `"hi" * 3`
"#,
        key_concepts: &["Interpreter", "Expressions", "Statements", "Indentation", "REPL"],
        key_takeaways: &[
            "Python reads code top-to-bottom",
            "Indentation creates blocks",
            "Expressions produce values",
            "Names bind to objects",
        ],
        dos_and_donts: r#"
- **Do** keep indentation consistent (4 spaces)
- **Do** use the playground to test small ideas
- **Don't** mix tabs and spaces
"#,
        going_deeper: "",
        common_mistakes: r#"
- **Mistake:** Forgetting a colon after `if`/`for`/`def` → **Fix:** Add `:`
- **Mistake:** Inconsistent indentation → **Fix:** Use 4 spaces everywhere
- **Mistake:** Expecting `print()` to return a value → **Fix:** `print()` returns `None`
"#,
    },
    Lesson {
        id: 1,
        title: "Variables, Types, and I/O",
        subtitle: "Names, casting, and user input",
        icon: "🏷️",
        phase: "Foundations",
        why_it_matters: "Every bug you fix in Python starts with understanding types, names, and input/output.",
        intuition: "Variables are labels stuck on objects. You can re-label anytime, but the object stays the same.",
        content: r#"
## Variables are references
```python
age = 21
name = "Riya"
```

## Built-ins to know cold
`print`, `len`, `type`, `int`, `float`, `str`, `min`, `max`, `sum`, `range`, `enumerate`, `sorted`

## Input (always a string)
```python
age_str = input("How old are you? ")
age = int(age_str)
```

## Casting patterns
```python
x = "10.5"
xf = float(x)   # 10.5
xi = int(xf)    # 10
```

## Quick check
- Q: What type is `input()`?
- A: `str`
- Q: What happens with `int("10.5")`?
- A: Error; convert to float first

## Mini drills
- Ask for two numbers and print their sum
- Convert centimeters to meters using input
"#,
        key_concepts: &["Binding", "Dynamic typing", "Casting", "input()", "None"],
        key_takeaways: &[
            "Variables point to objects",
            "`input()` returns strings",
            "Cast explicitly before math",
            "Use `type()` to debug",
        ],
        dos_and_donts: "",
        going_deeper: "",
        common_mistakes: r#"
- **Mistake:** `age = input(...)` then `age + 1` → **Fix:** `age = int(input(...))`
- **Mistake:** Shadowing built-ins like `list = []` → **Fix:** Use better names
"#,
    },
    Lesson {
        id: 2,
        title: "Operators + Boolean Logic",
        subtitle: "Comparison, identity, and truth",
        icon: "⚖️",
        phase: "Foundations",
        why_it_matters: "Decision-making and loops rely on booleans. One wrong comparison breaks everything.",
        intuition: "Booleans are the steering wheel of your program—every decision uses them.",
        content: r#"
## Arithmetic operators
| Operator | Meaning |
| --- | --- |
| `+` | add |
| `-` | subtract |
| `*` | multiply |
| `/` | divide (float) |
| `//` | floor divide |
| `%` | remainder |
| `**` | power |

## Comparisons
```python
3 > 2   # True
3 == 2  # False
3 != 2  # True
```

## Identity vs equality
```python
a = [1, 2]
b = [1, 2]
a == b   # True
 a is b  # False
```

## Membership + logic
```python
"a" in "cat"      # True
3 in [1,2,3]      # True
(3 > 2) and True  # True
```

## Quick check
- Q: When should you use `is`?
- A: Identity checks (like `x is None`)

## Mini drills
- Check if a number is even using `%`
- Test if a character exists in a string
"#,
        key_concepts: &["Arithmetic", "Comparison", "Identity", "Membership", "Boolean logic"],
        key_takeaways: &[
            "`/` always returns float",
            "Use `is` for identity, `==` for values",
            "Booleans drive control flow",
        ],
        dos_and_donts: r#"
- **Do** use `x is None` for None checks
- **Don't** use `is` to compare strings or numbers
"#,
        going_deeper: "",
        common_mistakes: r#"
- **Mistake:** Using `is` instead of `==` → **Fix:** Use `==` for value equality
- **Mistake:** `if x = 3` → **Fix:** Use `==` in conditions
"#,
    },
    Lesson {
        id: 3,
        title: "Control Flow",
        subtitle: "if/elif/else, for, while",
        icon: "🧭",
        phase: "Core Python",
        why_it_matters: "Control flow is how you translate problem statements into steps.",
        intuition: "Conditionals choose paths; loops repeat work without copy-paste.",
        content: r#"
## If / elif / else
```python
x = 10
if x > 0:
    print("positive")
elif x == 0:
    print("zero")
else:
    print("negative")
```

## For loops + range
```python
for i in range(5):
    print(i)
```

## While loops
```python
count = 3
while count > 0:
    print(count)
    count -= 1
```

## Break / continue
```python
for x in [1,2,3,4,5]:
    if x == 3:
        continue
    if x == 5:
        break
    print(x)
```

## Quick check
- Q: When do you use `continue`?
- A: Skip the rest of the loop body and go to next iteration

## Mini drills
- Sum all even numbers from 1 to 20
- Print numbers 10 to 1 using a while loop
"#,
        key_concepts: &["if/elif/else", "for", "while", "range", "break/continue"],
        key_takeaways: &[
            "Indentation defines control blocks",
            "Use `range` for counted loops",
            "`break` exits the loop",
        ],
        dos_and_donts: "",
        going_deeper: "",
        common_mistakes: r#"
- **Mistake:** Infinite loop in `while` → **Fix:** update the loop variable
- **Mistake:** Off-by-one in `range` → **Fix:** remember `range` is end-exclusive
"#,
    },
    Lesson {
        id: 4,
        title: "Functions + Scope",
        subtitle: "Reusable logic and clean code",
        icon: "🧩",
        phase: "Core Python",
        why_it_matters: "Functions keep your logic reusable, testable, and readable.",
        intuition: "A function is a mini-program: input → processing → output.",
        content: r#"
## Defining a function
```python
def add(a, b):
    return a + b

print(add(2, 3))
```

## Default arguments
```python
def greet(name="friend"):
    return f"Hello, {name}!"
```

## *args and **kwargs
```python
def total(*nums):
    return sum(nums)

def show(**info):
    return info
```

## Scope basics
```python
x = 10

def demo():
    x = 5
    return x

print(demo())  # 5
print(x)       # 10
```

## Function patterns by data type
- **list**: return a new list (`[x*2 for x in nums]`)
- **dict**: build with `.get()` (`counts[x] = counts.get(x,0)+1`)
- **str**: normalize + compare (`s.lower()`)

## Quick check
- Q: What's the difference between `print` and `return`?
- A: `return` gives a value back; `print` just displays

## Mini drills
- Write `is_even(n)`
- Write `square_all(nums)` using a loop
"#,
        key_concepts: &["def", "return", "parameters", "defaults", "scope"],
        key_takeaways: &[
            "Functions make code reusable",
            "Prefer returning values over printing",
            "Default args simplify calls",
        ],
        dos_and_donts: r#"
- **Do** keep functions small and focused
- **Don't** use mutable default args like `def f(x=[])`
"#,
        going_deeper: "",
        common_mistakes: r#"
- **Mistake:** Forgetting to return a value → **Fix:** add `return`
- **Mistake:** Mutating a default list argument → **Fix:** use `None` and create inside
"#,
    },
    Lesson {
        id: 5,
        title: "Strings + Text Processing",
        subtitle: "Immutability, slicing, formatting, and methods",
        icon: "🧵",
        phase: "Data Structures",
        why_it_matters: "Most interview problems involve text. String mastery saves time.",
        intuition: "Strings are like arrays of characters, but you can’t change them in place.",
        content: r#"
## Indexing + slicing
```python
s = "Python"
s[0]      # 'P'
s[-1]     # 'n'
s[1:4]    # 'yth'
s[::-1]   # reverse
```

## String methods you must know
| Category | Methods | What they do |
| --- | --- | --- |
| Clean | `strip`, `lstrip`, `rstrip` | remove whitespace |
| Case | `lower`, `upper`, `title`, `capitalize` | change case |
| Search | `find`, `index`, `count` | locate and count |
| Match | `startswith`, `endswith` | prefix/suffix checks |
| Split/Join | `split`, `rsplit`, `splitlines`, `'sep'.join(...)` | tokenizing and joining |
| Replace | `replace` | substitution |
| Check | `isalnum`, `isalpha`, `isdigit`, `isnumeric`, `isspace` | validation |

```python
name = "  Ada Lovelace  "
print(name.strip().lower())
"data-science".split("-")
"-".join(["a", "b"])
```

## String building (performance)
```python
parts = ["A", "B", "C"]
result = "".join(parts)  # fast
```

## Formatting
```python
name = "Ada"
age = 36
print(f"{name} is {age} years old")

pi = 3.14159
print(f"pi = {pi:.2f}")
```

## Text processing pipeline
```python
raw = "  Hello, World!  "
clean = raw.strip().lower()
words = clean.split()     # ['hello,', 'world!']
```

## Palindrome normalization pattern
```python
def normalize(s: str) -> str:
    return "".join(ch.lower() for ch in s if ch.isalnum())
```

## Quick check
- Q: Are strings mutable?
- A: No

## Mini drills
- Reverse a string two ways
- Count vowels in a sentence
- Remove punctuation and lowercase
"#,
        key_concepts: &["Immutability", "Slicing", "String methods", "Join/split", "Formatting"],
        key_takeaways: &[
            "Strings are immutable",
            "Use `.join` for performance",
            "Normalize text before comparisons",
        ],
        dos_and_donts: r#"
- **Do** use `s.strip()` before comparisons
- **Don't** build strings with `+` in loops
"#,
        going_deeper: r#"
## Bytes vs str (advanced)
- `str` = text (Unicode)
- `bytes` = raw binary

```python
b = "hello".encode("utf-8")
text = b.decode("utf-8")
```
"#,
        common_mistakes: r#"
- **Mistake:** Trying to modify `s[0]` → **Fix:** build a new string
- **Mistake:** Using `+` in a loop → **Fix:** use `''.join(...)`
- **Mistake:** Using `find` then not checking `-1` → **Fix:** verify result
"#,
    },
    Lesson {
        id: 6,
        title: "Lists + Tuples",
        subtitle: "Ordered collections and essential methods",
        icon: "📦",
        phase: "Data Structures",
        why_it_matters: "Lists are the default container for most problems. Tuples lock data safely.",
        intuition: "Lists are flexible shelves; tuples are sealed boxes.",
        content: r#"
## List basics
```python
nums = [1, 2, 3]
nums.append(4)
nums[0] = 99
```

## List methods (complete essentials)
| Method | What it does | Example |
| --- | --- | --- |
| `append(x)` | add one item | `nums.append(5)` |
| `extend(iter)` | add many | `nums.extend([6,7])` |
| `insert(i,x)` | add at index | `nums.insert(1, 42)` |
| `remove(x)` | remove first match | `nums.remove(2)` |
| `pop(i)` | remove by index | `nums.pop()` |
| `clear()` | remove all | `nums.clear()` |
| `index(x)` | find position | `nums.index(3)` |
| `count(x)` | count value | `nums.count(3)` |
| `sort()` | sort in place | `nums.sort()` |
| `reverse()` | reverse in place | `nums.reverse()` |
| `copy()` | shallow copy | `nums.copy()` |

## Sorting patterns
```python
nums = [3, 1, 4]
nums.sort()              # in-place
sorted_nums = sorted(nums)  # new list

words = ["aaa", "b", "cc"]
words.sort(key=len)
```

## List slicing
```python
nums = [1,2,3,4,5]
nums[1:4]   # [2,3,4]
nums[::-1]  # reversed copy
```

## Comprehensions
```python
squares = [x*x for x in range(5)]
```

## List as stack / queue
```python
stack = []
stack.append(1)
stack.pop()
```

For fast queue operations use `collections.deque`:
```python
from collections import deque
q = deque([1,2,3])
q.append(4)
q.popleft()
```

## Copy vs alias (important)
```python
a = [1, 2, 3]
b = a          # alias
c = a.copy()   # new list
```

## Nested list trap
```python
grid = [[0] * 3] * 3  # same row repeated
```

Correct:
```python
grid = [[0] * 3 for _ in range(3)]
```

## Tuples (immutable)
```python
point = (3, 4)
```

### Tuple operations
- `len(t)`
- `t.count(x)`
- `t.index(x)`
- unpacking: `a, b = (1, 2)`

## Quick check
- Q: What does `b = a` create?
- A: An alias (same list)

## Mini drills
- Sum a list without `sum()`
- Remove duplicates from a sorted list (concept)
- Insert a value at index 2
"#,
        key_concepts: &["List ops", "Tuple immutability", "Copying", "Comprehensions", "Sorting"],
        key_takeaways: &[
            "Lists are mutable and ordered",
            "Tuples are immutable and safe",
            "`b = a` shares the same list",
            "Use `sorted` for a new list",
        ],
        dos_and_donts: r#"
- **Do** use `append` for single items and `extend` for many
- **Don't** assume `sort()` returns a new list
- **Don't** build nested lists with `[[x]*n]*m`
"#,
        going_deeper: r#"
## Complexity snapshot
- Index access: O(1)
- Append: O(1) amortized
- Insert/remove at front: O(n)
- Membership: O(n)
"#,
        common_mistakes: r#"
- **Mistake:** Modifying a list while iterating → **Fix:** iterate over a copy or use indices
- **Mistake:** Confusing alias with copy → **Fix:** use `list(a)` or `a.copy()`
- **Mistake:** `new = nums.sort()` gives `None` → **Fix:** use `sorted(nums)`
"#,
    },
    Lesson {
        id: 7,
        title: "Sets",
        subtitle: "Uniqueness + fast membership",
        icon: "🧩",
        phase: "Data Structures",
        why_it_matters: "Sets remove duplicates and give fast membership checks—perfect for many DSA problems.",
        intuition: "A set is a bag with no duplicates. Order doesn’t matter.",
        content: r#"
## Set basics
```python
st = set()
fruits = {"apple", "banana", "apple"}
```

## Core methods
| Method | What it does | Example |
| --- | --- | --- |
| `add(x)` | add one | `st.add(3)` |
| `update(iter)` | add many | `st.update([4,5])` |
| `remove(x)` | remove; error if missing | `st.remove(2)` |
| `discard(x)` | remove safely | `st.discard(2)` |
| `pop()` | remove arbitrary | `st.pop()` |
| `clear()` | remove all | `st.clear()` |
| `copy()` | shallow copy | `st.copy()` |

## Set algebra
```python
A = {1, 2, 3}
B = {3, 4, 5}
A | B   # union
A & B   # intersection
A - B   # difference
A ^ B   # symmetric difference
```

## Subset / superset checks
```python
A <= B   # subset
A <  B   # proper subset
A >= B   # superset
```

## Set comprehension
```python
squares = {x*x for x in range(5)}
```

## Frozenset (immutable set)
```python
fs = frozenset([1,2,3])
```

## Quick check
- Q: Is `{}` a set?
- A: No, it’s a dict

## Mini drills
- Count unique words in a sentence
- Check if a list has duplicates
"#,
        key_concepts: &["Uniqueness", "Membership", "Union/Intersection", "Frozenset"],
        key_takeaways: &[
            "Sets ignore duplicates",
            "Membership is fast on average",
            "Use `discard` to avoid errors",
        ],
        dos_and_donts: r#"
- **Do** use sets for membership checks
- **Don't** rely on set order
"#,
        going_deeper: r#"
## Complexity snapshot
- Membership: ~O(1) average
- Set algebra: O(n)
"#,
        common_mistakes: r#"
- **Mistake:** Using `{}` for empty set → **Fix:** use `set()`
- **Mistake:** Expecting order in a set → **Fix:** use list if order matters
"#,
    },
    Lesson {
        id: 8,
        title: "Dictionaries",
        subtitle: "Key-value mapping, methods, and internals",
        icon: "🗺️",
        phase: "Data Structures",
        why_it_matters: "Hash maps solve a huge class of problems in O(1) average time.",
        intuition: "A dictionary is a fast lookup table: key → value.",
        content: r#"
## Dict basics
- Keys are **unique** and **hashable**
- Values can be any type
- Insertion order is preserved (Python 3.7+)

```python
person = {"name": "Ada", "age": 36}
print(person["name"])
```

## Creating dictionaries
```python
d1 = {"a": 1, "b": 2}
d2 = dict(a=1, b=2)
d3 = dict([("a", 1), ("b", 2)])
d4 = dict.fromkeys(["x", "y"], 0)
d5 = {k: k*k for k in range(5)}  # comprehension
```

## Add / update elements (beginner essentials)
```python
d = {}
d["count"] = 1              # add new key
d["count"] = 2              # update existing key
d.update({"x": 10, "y": 20})
d.setdefault("z", 0)        # add only if missing
```

### Merge (Python 3.9+)
```python
a = {"x": 1}
b = {"y": 2}
c = a | b        # new dict
a |= b           # in-place merge
```

## Access (safe vs unsafe)
```python
d["x"]           # KeyError if missing
d.get("x")       # None if missing
d.get("x", 0)    # default value
"x" in d         # membership checks keys
```

## Remove elements
```python
d.pop("x")       # remove key and return value
d.popitem()      # remove last inserted key-value
del d["x"]       # delete key (KeyError if missing)
d.clear()        # remove everything
```

## Core dict methods (must know)
| Method | What it does | Example |
| --- | --- | --- |
| `get(k, d)` | safe lookup | `d.get("x", 0)` |
| `keys()` | view of keys | `list(d.keys())` |
| `values()` | view of values | `list(d.values())` |
| `items()` | view of pairs | `list(d.items())` |
| `update(...)` | merge/update | `d.update({"x":1})` |
| `pop(k)` | remove key | `d.pop("x")` |
| `popitem()` | remove last pair | `d.popitem()` |
| `setdefault(k, d)` | get or set | `d.setdefault("x", 0)` |
| `copy()` | shallow copy | `d.copy()` |
| `fromkeys(keys, v)` | init | `dict.fromkeys(keys, 0)` |

## Iteration patterns
```python
for k in d:              # keys
    pass

for k, v in d.items():   # pairs
    pass

for k in sorted(d):      # ordered by key
    pass
```

## Frequency pattern (very common)
```python
def freq_count(items):
    counts = {}
    for x in items:
        counts[x] = counts.get(x, 0) + 1
    return counts
```

## Grouping pattern
```python
groups = {}
for item in items:
    groups.setdefault(item[0], []).append(item)
```

## Two Sum pattern
```python
def two_sum(nums, target):
    seen = {}
    for i, x in enumerate(nums):
        need = target - x
        if need in seen:
            return [seen[need], i]
        seen[x] = i
```

## Time complexity (average)
- `get`, `set`, `in`, `pop` → ~O(1)
- Worst case O(n) if many collisions

## Quick check
- Q: What does `dict.get(k)` return if missing?
- A: `None` (or default if provided)

## Mini drills
- Count characters in a string
- Find the first repeated number
- Build a word → count dictionary
"#,
        key_concepts: &[
            "Key-value",
            "Hashing",
            "Views (keys/values/items)",
            "Merging",
            "Frequency maps",
        ],
        key_takeaways: &[
            "Dicts offer fast lookup",
            "Use `get` to avoid KeyError",
            "Keys must be hashable and unique",
            "Views update as dict changes",
        ],
        dos_and_donts: r#"
- **Do** use `in d` to test keys
- **Do** prefer `get` for optional access
- **Don't** mutate a dict while iterating over it
- **Don't** use unhashable keys (list, dict, set)
"#,
        going_deeper: r#"
## How dictionaries work (implementation theory)
- **Hash table**: keys are hashed to an index in a table
- **Collisions**: different keys can map to the same index
- **Resolution**: CPython uses open addressing with probing (fast on average)
- **Load factor**: as the table fills, Python resizes to keep lookups fast

### Hashability rules
- Hashable: `int`, `float`, `str`, `tuple` (if elements hashable), `frozenset`
- Unhashable: `list`, `dict`, `set`

### Order guarantee
- Python 3.7+ preserves insertion order (official language guarantee)
"#,
        common_mistakes: r#"
- **Mistake:** Accessing missing keys with `[]` → **Fix:** use `get()`
- **Mistake:** Overwriting values unintentionally → **Fix:** inspect keys before assignment
- **Mistake:** Using a list as a key → **Fix:** use tuple or frozenset
- **Mistake:** Assuming `dict.keys()` is a list → **Fix:** wrap with `list(...)` when needed
"#,
    },
    Lesson {
        id: 9,
        title: "Complexity + Memory Model",
        subtitle: "Big-O, mutability, and copying",
        icon: "🧠",
        phase: "Problem Solving",
        why_it_matters: "Speed and space are what separate accepted from TLE solutions.",
        intuition: "Think about how many times you touch data and whether you mutate it in place.",
        content: r#"
## Time complexity sanity checks
- list membership `x in list` → O(n)
- set/dict lookup `x in set` → ~O(1) average

## Python memory model
- Variables store **references**
- Mutable: list, dict, set
- Immutable: int, float, str, tuple

## Mutation gotcha
```python
def add_one(lst):
    lst.append(1)

x = []
add_one(x)
print(x)  # [1]
```

## Shallow copy vs deep copy
```python
import copy

nested = [[1], [2]]
shallow = nested.copy()
deep = copy.deepcopy(nested)
```

## Quick check
- Q: Which is faster for membership: list or set?
- A: set

## Mini drills
- Identify which types are mutable
- Explain why `b = a` can be dangerous
"#,
        key_concepts: &["Big-O", "Mutability", "References", "Copying"],
        key_takeaways: &[
            "Choose data structures for speed",
            "Mutation changes shared references",
            "Use `.copy()` or `deepcopy` when needed",
        ],
        dos_and_donts: "",
        going_deeper: "",
        common_mistakes: r#"
- **Mistake:** Assuming `b = a` copies data → **Fix:** use `a.copy()` or `list(a)`
- **Mistake:** Mutating input in a function unintentionally → **Fix:** document or copy
"#,
    },
    Lesson {
        id: 10,
        title: "DSA Patterns + OOP + Practice",
        subtitle: "Two pointers, prefix sums, and clean design",
        icon: "🏗️",
        phase: "OOP + Practice",
        why_it_matters: "Patterns plus structure turn random coding into reliable problem solving.",
        intuition: "Patterns are reusable blueprints; OOP organizes state and behavior together.",
        content: r#"
## Core DSA patterns (templates)

### Two pointers
```python
l, r = 0, len(arr) - 1
while l < r:
    # move l or r based on condition
    l += 1
```

### Sliding window
```python
l = 0
for r in range(len(arr)):
    # expand window with r
    # shrink from left when condition fails
    if condition:
        l += 1
```

### Prefix sum
```python
prefix = [0]
for x in arr:
    prefix.append(prefix[-1] + x)
```

### Stack pattern
```python
stack = []
for ch in s:
    if stack and ch == stack[-1]:
        stack.pop()
    else:
        stack.append(ch)
```

## OOP basics
```python
class Player:
    def __init__(self, name):
        self.name = name
        self.score = 0

    def add(self, points):
        self.score += points
```

- **Class** defines a blueprint
- **Instance** is a concrete object
- `self` refers to the current object

## Practice system

### Bug journal (use every time you get stuck)
- Date:
- Problem:
- What I expected:
- What happened:
- Root cause:
- Fix:
- Rule for next time:

### LeetCode tracker (sample)
| # | Problem | Topic | Status | Key Idea | Mistake | Retry |
| - | ------- | ----- | ------ | -------- | ------- | ----- |
| 1 | Two Sum | dict | | complement lookup | | |
| 2 | Valid Palindrome | strings | | normalize + two pointers | | |

## Quick check
- Q: When should you use a class?
- A: When state and behavior belong together

## Mini drills
- Implement two-pointer reverse
- Write a tiny class with two methods
"#,
        key_concepts: &["Two pointers", "Sliding window", "Prefix sum", "Stack", "Classes"],
        key_takeaways: &[
            "Patterns are reusable solution blueprints",
            "Prefix sums enable fast range queries",
            "OOP groups data + behavior",
            "Track mistakes to improve faster",
        ],
        dos_and_donts: r#"
- **Do** write small templates you can reuse
- **Do** keep classes simple and focused
- **Don't** over-engineer when a function is enough
"#,
        going_deeper: "",
        common_mistakes: r#"
- **Mistake:** Forgetting to update pointers in loops → **Fix:** always move at least one pointer
- **Mistake:** Overusing classes for simple data → **Fix:** start with functions, refactor later
"#,
    },
    Lesson {
        id: 11,
        title: "Python Core (Advanced)",
        subtitle: "Iterators, generators, exceptions, files, modules",
        icon: "🧰",
        phase: "Core Python",
        why_it_matters: "This is the real-world toolbox that makes Python productive at scale.",
        intuition: "Iterables let you stream data, exceptions handle failure, modules organize code.",
        content: r#"
## Iterables, iterators, generators
```python
nums = [1, 2, 3]
itr = iter(nums)
next(itr)  # 1
```

### Generator
```python
def gen():
    for i in range(3):
        yield i
```

### Generator expression
```python
squares = (x*x for x in range(5))
```

## Exceptions
```python
try:
    x = int("abc")
except ValueError as e:
    print("bad input")
else:
    print("success")
finally:
    print("always runs")
```

## File I/O
```python
with open("data.txt", "w") as f:
    f.write("hello")

with open("data.txt", "r") as f:
    text = f.read()
```

## Modules + packages
```python
import math
from collections import deque
```

## Standard library essentials
- `collections`: `deque`, `defaultdict`, `Counter`
- `itertools`: `accumulate`, `product`, `permutations`
- `math`, `statistics`, `random`

## Typing hints
```python
def add(a: int, b: int) -> int:
    return a + b
```

## Dataclasses
```python
from dataclasses import dataclass

@dataclass
class Point:
    x: int
    y: int
```

## Quick check
- Q: Why use `with open(...)`?
- A: It closes the file automatically

## Mini drills
- Write a generator that yields even numbers
- Catch a KeyError safely
- Read a file and count lines
"#,
        key_concepts: &["Iterators", "Generators", "Exceptions", "File I/O", "Modules"],
        key_takeaways: &[
            "Generators save memory",
            "Use try/except for safe failures",
            "`with open` manages resources",
            "Modules keep code organized",
        ],
        dos_and_donts: r#"
- **Do** use generators for large data streams
- **Do** catch specific exceptions
- **Don't** leave files open
- **Don't** swallow exceptions silently
"#,
        going_deeper: r#"
## Custom iteration
```python
class CounterIter:
    def __init__(self, n):
        self.n = n
        self.i = 0

    def __iter__(self):
        return self

    def __next__(self):
        if self.i >= self.n:
            raise StopIteration
        val = self.i
        self.i += 1
        return val
```
"#,
        common_mistakes: r#"
- **Mistake:** Using `except:` everywhere → **Fix:** catch specific errors
- **Mistake:** Forgetting `yield` in generators → **Fix:** use `yield` to stream values
- **Mistake:** Reading huge files with `read()` → **Fix:** iterate line by line
"#,
    },
];
