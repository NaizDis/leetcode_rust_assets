use std::i32;

// LeetCode #682
pub fn cal_points(operations: vec<string>) -> i32 {
    let mut rec = vec![];
    for i in operations {
        match i.as_str() {
            "c" => {
                rec.pop();
            }
            "d" => {
                let rs = rec.iter().rev().next().unwrap();
                rec.push(rs * 2);
            }
            "+" => {
                let mut it = rec.iter().rev();
                let r1 = it.next().unwrap();
                let r2 = it.next().unwrap();
                rec.push(r1 + r2);
            }
            _ => {
                rec.push(i.parse().unwrap());
            }
        }
    }
    rec.iter().fold(0, |a, c| a + c)
}
// LeetCode #20
pub fn is_valid(s: string) -> bool {
    let token: vec<char> = s.chars().collect();
    let mut stack = vec![];
    for i in token {
        if stack.len() == 0 && (i == ')' || i == '}' || i == ']') {
            return false;
        }
        match i {
            ')' => {
                if stack[stack.len() - 1] == '(' {
                    stack.pop();
                } else {
                    return false;
                }
            }
            '}' => {
                if stack[stack.len() - 1] == '{' {
                    stack.pop();
                } else {
                    return false;
                }
            }
            ']' => {
                if stack[stack.len() - 1] == '[' {
                    stack.pop();
                } else {
                    return false;
                }
            }
            _ => stack.push(i),
        }
    }
    stack.is_empty()
}
// LeetCode #150
pub fn eval_rpn(tokens: vec<string>) -> i32 {
    let mut stact = vec![];
    for i in tokens {
        match i.as_str() {
            "+" | "-" | "/" | "*" => {
                let r1 = stact.pop().unwrap();
                let r2 = stact.pop().unwrap();
                let res = match i.as_str() {
                    "+" => r1 + r2,
                    "-" => r2 - r1,
                    "*" => r2 * r1,
                    "/" => r2 / r1,
                    _ => 0,
                };
                stact.push(res);
            }
            _ => stact.push(i.parse().unwrap()),
        }
    }
    stact[0]
}
// LeetCode #739
pub fn daily_temperatures(temperatures: vec<i32>) -> vec<i32> {
    // let mut res = vec![];
    //time - o(n^2) | space - o(1)
    // for i in 0..temperatures.len() {
    //     for j in i + 1..temperatures.len() {
    //         if temperatures[j] > temperatures[i] {
    //             res.push((j - i) as i32);
    //             break;
    //         }
    //     }
    //     if res.len() != i + 1 {
    //         res.push(0);
    //     }
    // }
    //time - o(n) | space - o(n)
    let n = temperatures.len();
    let mut res = vec![0; n];
    let mut stack: vec<(i32, usize)> = vec![];

    for (i, &t) in temperatures.iter().enumerate() {
        while !stack.is_empty() && stack[stack.len() - 1].0 < t {
            let (_, stk_i) = stack.pop().unwrap();
            res[stk_i] = (i - stk_i) as i32;
        }
        stack.push((t, i));
    }

    res
}
// LeetCode #155
struct minstack {
    stack: vec<i32>,
    min_stack: vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * if you need a mutable reference, change it to `&mut self` instead.
 */
impl minstack {
    fn new() -> self {
        minstack {
            stack: vec![],
            min_stack: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);

        if self.min_stack.is_empty() || self.min_stack[self.min_stack.len() - 1] > val {
            self.min_stack.push(val);
        } else {
            self.min_stack
                .push(self.min_stack[self.min_stack.len() - 1]);
        }
    }

    fn pop(&mut self) {
        self.min_stack.pop();
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        *self.stack.iter().last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.iter().last().unwrap()
    }
}
// LeetCode #456
pub fn find132pattern(nums: vec<i32>) -> bool {
    //o(n^3) - BruteForce
    // let res: bool = false;
    // let len: usize = nums.len();
    // for (ct1, i) in nums.iter().enumerate() {
    //     if ct1 < len - 2 {
    //         for (ct2, j) in nums.iter().skip(ct1 + 1).enumerate() {
    //             if j > i {
    //                 for k in nums.iter().skip(ct2 + 1) {
    //                     if k > i && k < j {
    //                         return true;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    // res
    // o(n) - Stack
    // -- stack top store J
    let mut stk: Vec<i32> = vec![];
    //Td store K
    let mut td = std::i32::MIN;
    for i in nums.iter().rev() {
        if i < &td {
            return true;
        }
        while !stk.is_empty() && stk.last().unwrap() < i {
            td = stk.pop().unwrap();
        }
        stk.push(*i);
    }
    false
}
// LeetCode #496
pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let (n, m) = (nums1.len(), nums2.len());
    let mut gret_ele = vec![-1; m];
    let mut monotonic = vec![];

    use ::std::collections::HashMap;
    let mut hash: HashMap<i32, i32> = HashMap::new();

    for i in (0..=m - 1).rev() {
        while !monotonic.is_empty() && *monotonic.last().unwrap() <= nums2[i] {
            monotonic.pop();
        }
        if !monotonic.is_empty() && *monotonic.last().unwrap() > nums2[i] {
            gret_ele[i] = *monotonic.last().unwrap();
        }
        monotonic.push(nums2[i]);
    }
    for i in 0..m {
        *hash.entry(nums2[i]).or_insert(-1) = gret_ele[i];
    }

    let mut ans = vec![-1; n];
    for i in 0..n {
        ans[i] = hash[&nums1[i]];
    }
    ans
}
// LeetCode #503
pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    // Optimal iterate over 2*n time the array
    // let n = nums.len();
    // let mut gret_ele = vec![-1; n];
    // let mut monotonic = vec![];
    //
    // for i in (0..2 * n).rev() {
    //     while !monotonic.is_empty() && monotonic[monotonic.len() - 1] <= nums[(i % n)] {
    //         monotonic.pop();
    //     }
    //     if !monotonic.is_empty() && monotonic[monotonic.len() - 1] > nums[i % n] {
    //         gret_ele[i % n] = monotonic[monotonic.len() - 1];
    //     }
    //     monotonic.push(nums[i % n]);
    // }
    // gret_ele
    //
    //
    //
    //
    //// APproach by starting from max_value_indice
    let n = nums.len();
    let mut gret_ele = vec![-1; n];
    let mut monotonic = vec![];

    let mut idx_max = 0;

    let mut lg = i32::MIN;
    for i in 0..n {
        if nums[i] >= lg {
            lg = nums[i];
            idx_max = i;
        }
    }

    monotonic.push(nums[idx_max]);

    let mut cnt = 0;
    let mut t = (idx_max + n - 1) % n;

    while cnt != n {
        while !monotonic.is_empty() && monotonic[monotonic.len() - 1] <= nums[t] {
            monotonic.pop();
        }
        if !monotonic.is_empty() && monotonic[monotonic.len() - 1] > nums[t] {
            gret_ele[t] = monotonic[monotonic.len() - 1];
        }
        monotonic.push(nums[t]);
        t = (t + n - 1) % n;
        cnt += 1;
    }
    gret_ele
}
