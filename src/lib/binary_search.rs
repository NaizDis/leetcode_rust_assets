use std::{cmp::Ordering, usize};

use super::linklist::middle_node;
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len() as i32 - 1;

    while l <= r {
        let m = l + (r - l) / 2;

        if nums[m as usize] == target {
            return m as i32;
        } else if nums[m as usize] < target {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    return -1 as i32;
}
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len() as i32 - 1;

    while l <= r {
        let m = l + (r - l) / 2;

        if nums[m as usize] == target {
            return m as i32;
        } else if nums[m as usize] < target {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }
    if l == 0 && r == 0 {
        0
    } else {
        l + (r - l) / 2 + 1
    }
}
pub fn is_perfect_square(num: i32) -> bool {
    let (mut l, mut r) = (0, num);
    while r >= l {
        let m = (l + r) / 2;
        let sq = m * m;
        match sq.cmp(&num) {
            Ordering::Equal => return true,
            Ordering::Greater => r = m - 1,
            Ordering::Less => l = m + 1,
        }
    }
    false
}
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;

    let mut start = 0;
    let mut end = rows * cols - 1;

    while start <= end {
        let mid = start + (end - start) / 2;
        let val = matrix[(mid / cols) as usize][(mid % cols) as usize];

        if val == target {
            return true;
        } else if val > target {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }
    false
}
pub fn find_min(nums: Vec<i32>) -> i32 {
    let (mut l, mut r) = (0 as i32, (nums.len() - 1) as i32);
    let mut m = 0;

    while r > l {
        m = (r + l) / 2;

        if nums[m as usize] > nums[r as usize] {
            l = m + 1;
        } else {
            r = m;
        }
    }
    nums[l as usize]
}
pub fn search_rot(nums: Vec<i32>, target: i32) -> i32 {
    let (mut l, mut r) = (0 as i32, (nums.len() - 1) as i32);
    let mut m = 0;
    let n = nums.len() as i32;

    while r > l {
        m = (r + l) / 2;

        if nums[m as usize] > nums[r as usize] {
            l = m + 1;
        } else {
            r = m;
        }
    }

    let mut start: i32;
    let mut end: i32;

    if l == 0 {
        start = 0;
        end = n - 1;
    } else if target >= nums[0] && target <= nums[(l - 1) as usize] {
        start = 0;
        end = l - 1;
    } else {
        start = l;
        end = n - 1;
    }
    let mut l = start;
    let mut r = end;

    while l <= r {
        let m = l + (r - l) / 2;

        if nums[m as usize] == target {
            return m as i32;
        } else if nums[m as usize] < target {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    return -1 as i32;
}
pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut max = *piles.iter().max().unwrap();
    let mut l = 1;

    while max > l {
        let m = (max + l) / 2;
        if k_test(&piles, h, m) {
            max = m;
        } else {
            l = m + 1;
        }
    }
    l
}
pub fn k_test(pp: &Vec<i32>, hr: i32, k: i32) -> bool {
    let mut hrs = 0;
    for i in pp {
        hrs += (*i as f32 / k as f32).ceil() as i32;
    }
    hrs <= hr
}
