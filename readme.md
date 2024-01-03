## 1 判断一个数字是否为回文

    fn is_palindrome_number(num: i64) -> bool {
        num.to_string().chars().rev().eq(num.to_string().chars())
    }

## 2 罗马数字转换为int

    fn roman_to_int(s: String) -> i32 {
        let s_translated = s
            .replace("IV", "IIII")
            .replace("IX", "VIIII")
            .replace("XL", "XXXX")
            .replace("XC", "LXXXX")
            .replace("CD", "CCCC")
            .replace("CM", "DCCCC");

        s_translated
            .chars()
            .map(|c| match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            })
            .sum()
    }

## 3 查找最大相同字符

解决方法1:

    fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let mut prefix = strs[0].clone(); // 假设第一个字符串为公共前缀
        for s in strs.iter().skip(1) {
            while !s.starts_with(&prefix) {
                // 若当前字符串不以当前公共前缀开头，则缩短当前公共前缀
                prefix.pop();
                if prefix.is_empty() {
                    return String::new(); // 如果公共前缀为空，直接返回
                }
            }
        }

        prefix
    }

解决方法2:

    pub fn longest_common_prefix(input: Vec<String>) -> String {
        input.into_iter().reduce(|acc,cur|{
            acc.chars()
            .zip(cur.chars())
            .take_while(|(a,c)| a== c)
            .map(|(c,_)|c)
            .collect()
        }).unwrap()
    }

## 4 valid parentheses

### 思路概述

创建一个空栈，用于存储左括号。
遍历输入的字符串，对于每个字符：
如果是左括号（'('、'[' 或 '{'），将其推入栈中。
如果是右括号（')'、']' 或 '}'），检查栈是否为空。
如果栈为空，或栈顶元素与当前右括号不匹配，则返回 false。
如果栈顶元素与当前右括号匹配，则弹出栈顶元素，继续遍历下一个字符。
最后，检查栈是否为空。如果栈为空，则表示所有括号都匹配，返回 true；否则返回 false。

    fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for i in s.chars() {
            match i {
                '{' => stack.push('}'),
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '}'|')'|']' if Some(i) != stack.pop() => return false,
                _ => (),
            }
        }   
         stack.is_empty()
    }

## 链表合并

    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (Some(x), Some(y)) => {
                if x.val < y.val {
                    Some(Box::new(ListNode {
                        val: x.val,
                        next: merge_two_lists(x.next, Some(y)),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: y.val,
                        next: merge_two_lists(Some(x), y.next),
                    }))
                }
            }
        }
    }

## Search Insert Position

Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

You must write an algorithm with O(log n) runtime complexity.

Example 1:

Input: nums = [1,3,5,6], target = 5
Output: 2
Example 2:

Input: nums = [1,3,5,6], target = 2
Output: 1
Example 3:

Input: nums = [1,3,5,6], target = 7
Output: 4

### Solution:二分查找

函数通过二分查找的方式在有序数组中查找目标值的位置或应该插入的位置
二分查找的过程中，通过维护 low 和 high 指针来不断缩小搜索范围，直到找到目标值或确定插入位置。在每次迭代中，计算中间值 mid，与目标值进行比较，并根据比较结果缩小搜索范围

    fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len() as i32 - 1;

        while low <= high {
            let mid = low + (high - low) / 2;
            if nums[mid as usize] == target {
                return mid;
            } else if nums[mid as usize] < target {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        low
    }

    // equal to this line
    nums.binary_search(&target).unwrap_or_else(|x| x) as i32

## Length of Last Word

Given a string s consisting of words and spaces, return the length of the last word in the string.

A word is a maximal
substring
 consisting of non-space characters only.

Example 1:

Input: s = "Hello World"
Output: 5
Explanation: The last word is "World" with length 5.
Example 2:

Input: s = "   fly me   to   the moon  "
Output: 4
Explanation: The last word is "moon" with length 4.
Example 3:

Input: s = "luffy is still joyboy"
Output: 6
Explanation: The last word is "joyboy" with length 6.

### Solution

    fn length_of_last_word(s: String) -> i32 {
        let trimmed = s.trim_end();
        match trimmed.rfind(' ') {
            Some(idx) => (trimmed.len() - idx - 1) as i32,
            None => trimmed.len() as i32,
        }
    }

## Plus One

You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer. The digits are ordered from most significant to least significant in left-to-right order. The large integer does not contain any leading 0's.

Increment the large integer by one and return the resulting array of digits.

Example 1:

Input: digits = [1,2,3]
Output: [1,2,4]
Explanation: The array represents the integer 123.
Incrementing by one gives 123 + 1 = 124.
Thus, the result should be [1,2,4].
Example 2:

Input: digits = [4,3,2,1]
Output: [4,3,2,2]
Explanation: The array represents the integer 4321.
Incrementing by one gives 4321 + 1 = 4322.
Thus, the result should be [4,3,2,2].
Example 3:

Input: digits = [9]
Output: [1,0]
Explanation: The array represents the integer 9.
Incrementing by one gives 9 + 1 = 10.
Thus, the result should be [1,0].

### Solution

    fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result = digits.clone(); // 克隆原始数组，避免修改原数组

        // 从数组末尾开始处理加一操作
        for i in (0..result.len()).rev() {
            if result[i] < 9 {
                result[i] += 1;
                return result;
            } else {
                result[i] = 0;
            }
        }

        // 处理特殊情况，所有位都产生了进位
        let mut new_result = vec![0; result.len() + 1];
        new_result[0] = 1;
        new_result
    }

## Sqrt(x)

Given a non-negative integer x, return the square root of x rounded down to the nearest integer. The returned integer should be non-negative as well.

You must not use any built-in exponent function or operator.

For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.

Example 1:

Input: x = 4
Output: 2
Explanation: The square root of 4 is 2, so we return 2.
Example 2:

Input: x = 8
Output: 2
Explanation: The square root of 8 is 2.82842..., and since we round it down to the nearest integer, 2 is returned.

### Solution

使用二分查找或者牛顿迭代法:
使用二分查找:

    fn my_sqrt(x: i32) -> i32 {
        if x == 0 || x == 1 {
            return x;
        }

        let mut left = 1;
        let mut right = x;
        let mut result = 0;

        while left <= right {
            let mid = left + (right - left) / 2;
            let square = mid * mid;

            if square == x {
                return mid;
            } else if square < x {
                result = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        result
    }

使用牛顿迭代:

1. 初始值设为 n。
2. 迭代更新公式为 x = (x + n / x) / 2。
3. 重复步骤 2 直到收敛，即 x 和 x^2 非常接近。

如下：

    fn my_sqrt(x: i32) -> i32 {
        if x == 0 || x == 1 {
            return x;
        }

        let mut root = x as i64;
        while root > x as i64 / root {
            root = (root + x as i64 / root) / 2;
        }

        root as i32
    }

但该解决办法存在数据溢出的问题，需要进行类型转换,i32 数据下最大的开平方数据为 <code> 46340</code>

    fn my_sqrt(x: i32) -> i32 {
        if x == 0 { // non-negative => 0 is allowed
            return 0;
        }

        let mut lower = 2;
        let mut higher = 46340.min(x / 2); // max possible root sqrt(i32::MAX)

        // Boundary conditions for optimization

        if x <= 3 {
            return 1;
        }

        if x >= higher * higher {
            return higher;
        }

        // Binary search the correct perfect square

        while higher - lower > 1 {
            let mid = (higher + lower) / 2;
            let pow = mid * mid;
            match pow.cmp(&x) {
                Ordering::Less => {
                    lower = mid;
                }
                Ordering::Greater => {
                    higher = mid;
                }
                _ => {
                    return mid;
                }
            }
        }

        // In the edge case that the number is between 2 values,
        // we take the lower which is equivalent to taking the floored mean
        lower
    }

## Climbing Stairs

You are climbing a staircase. It takes n steps to reach the top.

Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

Example 1:

Input: n = 2
Output: 2
Explanation: There are two ways to climb to the top.

1. 1 step + 1 step
2. 2 steps
Example 2:

Input: n = 3
Output: 3
Explanation: There are three ways to climb to the top.

1. 1 step + 1 step + 1 step
2. 1 step + 2 steps
3. 2 steps + 1 step

### Solution

    fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }

        let mut dp = vec![0; n as usize];
        dp[0] = 1;
        dp[1] = 2;

        for i in 2..n as usize {
            dp[i] = dp[i - 1] + dp[i - 2];
        }

        dp[n as usize - 1]
    }

or 用斐波那契数列备忘数组作弊

    const LUT: &[i32] = &[
        0, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946,
        17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269, 2178309, 3524578,
        5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155, 165580141, 267914296,
        433494437, 701408733, 1134903170, 1836311903,
    ];

    impl Solution {
        pub fn climb_stairs(n: i32) -> i32 {
            return LUT[n as usize];
        }
    }
