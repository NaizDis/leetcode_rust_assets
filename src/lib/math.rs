// LeetCode #1291
pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    let mut ans = vec![];
    for i in 1..=9 {
        let mut num = i;
        for j in i + 1..=9 {
            num = num * 10 + j;
            if num >= low && num <= high {
                ans.push(num);
            }
        }
    }

    ans.sort_unstable();
    ans
}
// LeetCode#
pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
    n
}
// LeetCode #1979
pub fn find_gcd(nums: Vec<i32>) -> i32 {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
    let (mut m, mut n) = (i32::MIN, i32::MAX);
    for i in nums {
        if i > m {
            m = i;
        }
        if i < n {
            n = i;
        }
    }
    gcd(m, n)
}
