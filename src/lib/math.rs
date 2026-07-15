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
pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
    n
}
