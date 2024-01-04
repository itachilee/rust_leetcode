use itertools::Itertools;

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct SharedData {
    value: RefCell<i32>,
}

/// RefCell 只能同时存在一处借用，若同时存在多出借用则会panic
/// Rc 计数引用
fn main() {
    // let data = Rc::new(5);
    // let shared1 = Rc::clone(&data);
    // let shared2 = Rc::clone(&data);

    // println!("data = {}", data);
    // println!("shared1 = {}", shared1);
    // println!("shared2 = {}", shared2);

    // 创建一个包含 RefCell 的共享数据结构
    let shared_data = Rc::new(SharedData {
        value: RefCell::new(5),
    });

    // 克隆 Rc 实例，以便有多个所有者
    let shared1 = Rc::clone(&shared_data);
    let shared2 = Rc::clone(&shared_data);

    // 在不同的所有者下，通过 RefCell 修改内部数据
    {
        let mut value = shared1.value.borrow_mut();
        *value += 10;
        println!("shared1: {:?}", *value);
    }

    // 可以在其他所有者下再次修改内部数据
    {
        let mut value = shared2.value.borrow_mut();
        *value -= 5;
        println!("shared2: {:?}", *value);
    }

    // 虽然有多个所有者，但 RefCell 保证了内部数据的可变性
    println!("Original shared data: {:?}", shared_data.value.borrow());
}

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

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let node_borrow = node.borrow();
            let left_depth = max_depth(node_borrow.left.clone());
            let right_depth = max_depth(node_borrow.right.clone());

            1 + left_depth.max(right_depth)
        }
    }
}

fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let node_borrow = node.borrow();
            let left_depth = min_depth(node_borrow.left.clone());
            let right_depth = min_depth(node_borrow.right.clone());

            if left_depth == 0 || right_depth == 0 {
                left_depth + right_depth + 1
            } else {
                1 + left_depth.min(right_depth)
            }
        }
    }
}

/// input number = 5
///  [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]

fn get_row(row_index: i32) -> Vec<i32> {
    let mut triangle = Vec::new();
    let mut res = Vec::new();

    let num_rows = row_index + 1;
    for i in 0..num_rows {
        let mut row = Vec::new();

        for j in 0..=i {
            if j == 0 || j == i {
                row.push(1);
            } else {
                let prev_row: &Vec<i32> = &triangle[i as usize - 1];

                let val = prev_row[j as usize - 1] + prev_row[j as usize];
                row.push(val);
            }
        }
        // triangle.push(copy_from_slice(row));

        if row_index == i {
            res = row;
        } else {
            triangle.push(row);
        }
    }

    res
}
fn generate_pascals_triangle(num_rows: i32) -> Vec<Vec<i32>> {
    let mut triangle = Vec::new();

    for i in 0..num_rows {
        let mut row = Vec::new();

        for j in 0..=i {
            if j == 0 || j == i {
                row.push(1);
            } else {
                let prev_row: &Vec<i32> = &triangle[i as usize - 1];

                let val = prev_row[j as usize - 1] + prev_row[j as usize];
                row.push(val);
            }
        }

        triangle.push(row);
    }

    triangle
}

fn is_palindrome(s: String) -> bool {
    if s.len() <= 1 {
        return true;
    }
    let t: Vec<_> = s
        .to_lowercase()
        .chars()
        .filter(|x| x.is_ascii_alphanumeric())
        .collect();

    t.iter().rev().eq(t.iter())
}
mod tests {

    use super::*;
    #[test]

    fn test_insert() {
        let res = '0'.is_ascii_alphanumeric();
        debug_assert_eq!(true, res);
    }
}
