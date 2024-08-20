use std::{
    collections::{HashMap, HashSet},
    usize,
};
pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut map = HashMap::new();
    for i in stones.chars() {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    let mut res = 0;
    for (c, i) in map {
        if jewels.chars().any(|i| i == c) {
            res += i;
        }
    }
    res
}
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut map = HashMap::new();
    for i in nums {
        map.entry(i).and_modify(|e| *e += 1).or_insert(1);
    }
    map.values().any(|&b| b > 1)
}
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut map = HashMap::new();
    for i in magazine.chars() {
        map.entry(i).and_modify(|e| *e += 1).or_insert(1);
    }
    for i in ransom_note.chars() {
        map.entry(i).and_modify(|e| *e -= 1).or_insert(-1);
    }
    !map.values().any(|&b| b < 0)
}
pub fn is_anagram(s: String, t: String) -> bool {
    let mut s_map = [0u32; 26];
    for i in s.as_bytes() {
        s_map[(i - b'a') as usize] += 1;
    }
    for i in t.as_bytes() {
        s_map[(i - b'a') as usize] -= 1;
    }
    !s_map.iter().any(|&i| i != 0)
}
pub fn max_number_of_balloons(text: String) -> i32 {
    let mut res = 0;
    let mut s_map = [0u32; 26];
    let ball = "balloon".as_bytes();
    for i in text.as_bytes() {
        s_map[(i - b'a') as usize] += 1;
    }
    'pp: loop {
        for i in ball {
            if s_map[(i - b'a') as usize] == 0 {
                break 'pp;
            } else {
                s_map[(i - b'a') as usize] -= 1;
            }
        }
        res += 1;
    }
    res
}
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        match map.get(&(target - *v)) {
            Some(&i2) => return vec![i as i32, i2],
            None => map.insert(*v, i as i32),
        };
    }
    vec![]
}
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();
    for i in strs {
        let mut key: Vec<char> = i.chars().collect();
        key.sort();
        map.entry(key).or_insert(vec![]).push(i);
    }
    map.values().cloned().collect()
}
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let (mut m, mut c) = (0, 0);
    for i in nums {
        if c == 0 {
            m = i;
            c = 1;
        } else if i == m {
            c += 1;
        } else {
            c -= 1;
        }
    }
    m
}
pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    nums.sort();
    nums.dedup();
    let mut prev = nums[0];
    let mut max = 1;
    let mut count = 1;
    for i in nums {
        if i == prev + 1 {
            count += 1;
        } else {
            count = 1;
        }
        prev = i;
        max = std::cmp::max(max, count);
    }
    max = std::cmp::max(max, count);
    max
}
//Simpler Use take_while And Count
pub fn longest_consecutive2(nums: Vec<i32>) -> i32 {
    let num_set: HashSet<_> = nums.into_iter().collect();
    let mut ans = 0;
    for &num in &num_set {
        if !num_set.contains(&(num - 1)) {
            let count = (num..).take_while(|x| num_set.contains(x)).count();
            ans = ans.max(count);
        }
    }
    ans as i32
}
