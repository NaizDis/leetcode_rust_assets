# AGENTS.md - LeetCode Rust Solutions

## Project Overview

This is a Rust project containing LeetCode problem solutions organized by algorithm category.
Solutions are modular, with each category in its own file under `src/lib/`.

## Build Commands

```bash
# Build the project
cargo build

# Run the binary
cargo run

# Run all tests
cargo test

# Run a single test (by test name)
cargo test <test_name>

# Run tests matching a pattern
cargo test <pattern>

# Check formatting
cargo fmt -- --check

# Format code
cargo fmt

# Lint with clippy
cargo clippy -- -D warnings

# Full check (fmt + clippy + test)
cargo fmt && cargo clippy -- -D warnings && cargo test
```

## Project Structure

```
src/
├── main.rs          # Binary entry point, imports and calls lib functions
└── lib/
    ├── mod.rs       # Module declarations (pub mod ...)
    ├── array_strings.rs
    ├── binary_search.rs
    ├── dp.rs
    ├── graphs.rs
    ├── hashmap.rs
    ├── heaps.rs
    ├── linklist.rs
    ├── recursive_backtracking.rs
    ├── sliding_win.rs
    ├── stack.rs
    ├── trees.rs
    └── two_pointer.rs
```

## Code Style Guidelines

### Naming Conventions

- **Functions**: `snake_case` (e.g., `max_profit`, `is_balanced`)
- **Types/Structs**: `PascalCase` (e.g., `TreeNode`, `LRUCache`, `Trie`)
- **Variables**: `snake_case` (e.g., `max_len`, `node_val`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_DEPTH`)
- **Modules**: `snake_case` (e.g., `array_strings`, `two_pointer`)

### Function Declarations

- Use `pub fn` for all solution functions (required for external testing)
- Prefer explicit return types over type inference for public APIs
- Use descriptive names that match LeetCode problem names when applicable
- Keep functions focused on a single algorithm problem

### Imports

- Group std imports separately from other imports
- Use nested imports for deeply nested paths:
  ```rust
  use std::{
      cmp::Ordering,
      collections::HashMap,
      rc::Rc,
      cell::RefCell,
  };
  ```
- Common imports in this codebase:
  - `std::collections::{HashMap, VecDeque, BinaryHeap}`
  - `std::rc::Rc`
  - `std::cell::RefCell`
  - `std::cmp::{self, Ordering}`
  - `std::mem`

### Data Structures

#### Binary Trees (as defined in `trees.rs`)
```rust
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

type TreeLink = Option<Rc<RefCell<TreeNode>>>;
```

#### Linked Lists
```rust
// Use Option<Box<ListNode>> pattern
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
```

### Error Handling

- Use `Option<T>` when a value may or may not exist
- Use `Result<T, E>` for fallible operations
- Prefer `unwrap()` only in test code or when failure is impossible
- Use `?` operator for propagation
- Default to panicking only for truly unrecoverable states

### Patterns

#### Early returns
```rust
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    // ...
}
```

#### Using `if let` for Option matching
```rust
if let Some(node) = root {
    let node = node.borrow();
    // ...
}
```

#### Using `match` for exhaustive handling
```rust
match val.cmp(&p) {
    Ordering::Greater => /* ... */,
    Ordering::Less => /* ... */,
    Ordering::Equal => /* ... */,
}
```

#### Helper functions (DFS, recursion)
```rust
pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut res = vec![true];
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<bool>) -> i32 {
        // recursive logic
    }
    dfs(root, res.as_mut());
    res[0]
}
```

### Formatting

- Use 4 spaces for indentation
- Maximum line length: 100 characters (Rust default)
- Place opening brace on same line for functions and control structures
- Use trailing commas in multi-line imports and struct literals

### Comments

- Document public functions with LeetCode problem description if not obvious
- Use inline comments sparingly, only for non-obvious logic
- Remove commented-out code before submitting

### Testing

- Add `#[cfg(test)]` module with `mod tests`
- Use `#[test]` attribute for test functions
- Test functions should match the pattern: `#[test] fn test_<problem_name>()`
- Use assert macros: `assert_eq!`, `assert!`, `assert_ne!`

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
```

### Performance Considerations

- Use `Rc<RefCell<T>>` for shared ownership of tree/list nodes
- Prefer iterators over index-based loops when possible
- Use `Vec::with_capacity()` when final size is known
- Avoid unnecessary cloning; use references where appropriate
