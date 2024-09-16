pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut sol: Vec<i32> = vec![];
    let n = nums.len();

    fn dfs(nums: &Vec<i32>, n: usize, id: usize, res: &mut Vec<Vec<i32>>, sol: &mut Vec<i32>) {
        if id == n {
            res.push(sol.clone());
            return;
        }
        //Left{Dont Pick Number[i]}
        dfs(nums, n, id + 1, res, sol);

        //Right{Pick Number[i]}
        sol.push(nums[id]);
        dfs(nums, n, id + 1, res, sol);
        sol.pop();
    }
    dfs(&nums, n, 0, &mut res, &mut sol);
    res
}
