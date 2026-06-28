use std::{
    cell::RefCell,
    cmp::{self, Ordering},
    collections::HashMap,
    i32::MIN,
    rc::Rc,
    usize,
};
pub fn merge_alternately(word1: String, word2: String) -> String {
    word1
        .chars()
        .zip(word2.chars())
        .flat_map(|(c1, c2)| [c1, c2])
        .chain(word1.chars().skip(word2.len()))
        .chain(word2.chars().skip(word1.len()))
        .collect()
}

pub fn roman_to_int(s: String) -> i32 {
    let mut res = 0;
    let chars: Vec<char> = s.chars().collect();
    let hash = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    for i in 0..chars.len() - 1 {
        if hash[&chars[i]] < hash[&chars[i + 1]] {
            res -= hash[&chars[i]];
        } else {
            res += hash[&chars[i]];
        }
    }
    res += hash[&chars[chars.len() - 1]];
    res
}

pub fn is_subsequence(s: String, t: String) -> bool {
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    let (mut i, mut j) = (0, 0); // Pointers for s and t

    while i < s_chars.len() && j < t_chars.len() {
        if s_chars[i] == t_chars[j] {
            i += 1; // Move s pointer if characters match
        }
        j += 1; // Always move t pointer
    }

    i == s_chars.len() // If i reached the end of s, it's a subsequence
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut bday = prices[0];
    let mut profit: i32 = 0;
    for &i in prices.iter().skip(1) {
        if i < bday {
            bday = i;
        } else {
            let pot = i - bday;
            if pot > profit {
                profit = pot;
            }
        }
    }
    profit
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut min_len: usize = usize::MAX;
    for i in &strs {
        if i.len() < min_len {
            min_len = i.len();
        }
    }
    let mut p = 0;
    'pp: while p < min_len {
        for j in &strs {
            if j.chars().nth(p) != strs[0].chars().nth(p) {
                break 'pp;
            }
        }
        p += 1;
    }
    strs[0][..p].to_string()
}
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut i = 0;
    while i < nums.len() {
        let start = nums[i];
        while i < nums.len() - 1 && nums[i] + 1 == nums[i + 1] {
            i += 1;
        }
        if nums[i] == start {
            res.push(format!("{}", start));
        } else {
            res.push(format!("{}->{}", start, nums[i]));
        }
        i += 1;
    }
    res
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut l_mult: i32 = 1;
    let mut r_mult: i32 = 1;
    let n = nums.len();
    let mut l_arr = Vec::new();
    let mut r_arr = Vec::new();

    for i in 0..n {
        let j = n - 1 - i;
        l_arr.push(l_mult);
        r_arr.push(r_mult);
        l_mult *= nums[i];
        r_mult *= nums[j];
    }
    r_arr.reverse();

    l_arr.iter().zip(&r_arr).map(|(a, b)| a * b).collect()
}

pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_by(|q, r| q[0].cmp(&r[0]));
    let mut res: Vec<Vec<i32>> = Vec::new();

    for interval in intervals {
        let prev = match res.last_mut() {
            Some(a) => a,
            None => {
                res.push(interval);
                continue;
            }
        };

        if prev[1] >= interval[0] {
            *prev = vec![prev[0], std::cmp::max(prev[1], interval[1])];
        } else {
            res.push(interval);
        }
    }
    res
}

pub fn find_closest_number(nums: Vec<i32>) -> i32 {
    use std::cmp::Ordering;
    nums.into_iter()
        .max_by(|x, y| match y.abs().cmp(&(x.abs())) {
            Ordering::Equal => x.cmp(y),
            abs_cmp => abs_cmp,
        })
        .unwrap()
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }

    let mut res = Vec::new();
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut top = 0;
    let mut bottom = rows as i32 - 1;
    let mut left = 0;
    let mut right = cols as i32 - 1;

    while top <= bottom && left <= right {
        // Traverse right
        for c in left..=right {
            res.push(matrix[top as usize][c as usize]);
        }
        top += 1;

        // Traverse down
        for r in top..=bottom {
            res.push(matrix[r as usize][right as usize]);
        }
        right -= 1;

        // Traverse left, if there are still rows to traverse
        if top <= bottom {
            for c in (left..=right).rev() {
                res.push(matrix[bottom as usize][c as usize]);
            }
            bottom -= 1;
        }

        // Traverse up, if there are still columns to traverse
        if left <= right {
            for r in (top..=bottom).rev() {
                res.push(matrix[r as usize][left as usize]);
            }
            left += 1;
        }
    }
    res
}

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let dim = matrix.len();
    for i in 0..dim {
        for j in i + 1..dim {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }
    for i in matrix {
        i.reverse();
    }
}
pub fn larg_arith_sub(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n <= 2 {
        return n as i32; // An array of 0, 1, or 2 elements is trivially an arithmetic subarray
    }

    let mut max_len = 2;
    let mut current_len = 2;
    // Iterate from the third element to check for arithmetic progression
    for i in 2..n {
        if nums[i] - nums[i - 1] == nums[i - 1] - nums[i - 2] {
            current_len += 1;
        } else {
            current_len = 2; // Reset if the progression breaks
        }
        max_len = cmp::max(max_len, current_len);
    }
    max_len
}
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max: i32 = MIN;
    let mut sum = 0;
    for i in nums {
        sum += i;
        max = max.max(sum);
        if sum < 0 {
            sum = 0;
        }
    }
    max
}
pub fn fib(n: i32) -> i32 {
    let mut arr = vec![0, 1];
    for i in 2..=n as usize {
        arr.push(arr[i - 1] + arr[i - 2]);
    }
    arr[n as usize]
}
pub fn master_magic_dcode(n: i32, x: i32, y: i32, diff: Vec<i32>) -> i32 {
    let mut res = 0;
    for i in diff {
        match (i * x).cmp(&y) {
            Ordering::Greater | Ordering::Equal => res += y,
            Ordering::Less => res += i * x,
        }
    }
    res
}
pub fn rnd_file_dcode(nums: Vec<i32>) -> (i32, i32) {
    let mut nums: Vec<i32> = nums.clone();
    nums.sort_by(|a, b| b.abs().cmp(&a.abs()));
    let (mut a, mut b) = (0, 0);
    let mut turn = true;
    for i in nums {
        match turn {
            true => {
                if i > 0 {
                    a += i;
                } else {
                    b += i;
                }
            }
            false => {
                if i > 0 {
                    b += i;
                } else {
                    a += i;
                }
            }
        }
        turn = !turn;
    }
    (a, b)
}
pub fn chosen_one_dcode(strr: String) -> char {
    let mut map = HashMap::new();
    for i in strr.chars() {
        map.entry(i).and_modify(|e| *e += 1).or_insert(1);
    }
    *map.iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(x, _v)| x)
        .unwrap()
}
pub fn paint_wall_dcode(strr: String) -> bool {
    let chrs: Vec<char> = strr.chars().collect();
    let n = strr.len();
    let (mut l, mut r) = (0, n - 1);
    let mut temp = vec!['0'; n];

    // Loop until pointers meet or cross, ensuring all characters are considered
    while l <= r {
        let mut found_one_at_l = false;
        let mut found_one_at_r = false;

        // Find the first '1' from the left, or reach r
        while l <= r && chrs[l] == '0' {
            // If we find a '0', the corresponding right side must also be a '0'
            // for symmetry, if we haven't processed it yet
            if temp[n - 1 - l] != '0' { // Check if the symmetric position was '1' already
                return false; // Asymmetry found
            }
            l += 1;
        }
        if l <= r && chrs[l] == '1' {
            found_one_at_l = true;
        }

        // Find the first '1' from the right, or reach l
        while l <= r && chrs[r] == '0' {
            // If we find a '0', the corresponding left side must also be a '0'
            // for symmetry, if we haven't processed it yet
            if temp[n - 1 - r] != '0' { // Check if the symmetric position was '1' already
                return false; // Asymmetry found
            }
            r -= 1;
        }
        if l <= r && chrs[r] == '1' {
            found_one_at_r = true;
        }

        // If '1's were found symmetrically, mark them in temp
        if found_one_at_l && found_one_at_r && l <= r {
            temp[l] = '1';
            temp[r] = '1';
            l += 1;
            r -= 1;
        } else if found_one_at_l || found_one_at_r {
            // If only one side found a '1' (or l == r and only one '1' found in middle)
            // This indicates asymmetry unless it's the very middle for an odd length string
            if l == r && found_one_at_l && !found_one_at_r { // Middle element '1'
                temp[l] = '1';
                l += 1;
                r -= 1;
            } else {
                return false; // Asymmetry (e.g., '10', '01')
            }
        }
    }
    
    // After processing, the original string must exactly match the constructed 'temp'
    // This ensures all '0's were handled correctly and only symmetric '1's were "painted".
    chrs == temp
}

pub fn death_note_dcode(l: i32, r: i32) -> bool {
    let smll = l.min(r);
    let big = l.max(r);
    (l + r) % 3 == 0 && smll >= big / 2
}

pub fn pascal_tri(rowId: u32, _row: Vec<u32>) -> Vec<u32> { // _row is ignored as we calculate the row from scratch
    if rowId == 0 {
        return vec![1];
    }

    let mut prev_row = vec![1]; // Start with row 0

    for _k in 1..=rowId { // Iterate from row 1 up to rowId
        let mut next_row = vec![1]; // Each row starts with 1
        for i in 0..prev_row.len() - 1 {
            next_row.push(prev_row[i] + prev_row[i + 1]);
        }
        next_row.push(1); // Each row ends with 1
        prev_row = next_row;
    }
    prev_row // Return the calculated rowId-th row
}

struct Node {
    key: i32,
    value: i32,
    next: Option<Nodevalue>,
    prev: Option<Nodevalue>,
}
type Nodevalue = Rc<RefCell<Node>>;

#[derive(Default)]
struct LRUCache {
    hash: HashMap<i32, Nodevalue>,
    cap: usize,
    head: Option<Nodevalue>,
    tail: Option<Nodevalue>,
}

pub fn is_trionic(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let (mut p, mut q) = (0, n - 1);
    if n <= 3 {
        return false;
    }
    loop {
        if nums[p] < nums[p + 1] && p < n - 1 {
            break;
        };
        p = p + 1;
    }
    loop {
        if nums[q] > nums[q - 1] && q > 0 {
            break;
        };
        q = q - 1;
    }
    p < q
}
