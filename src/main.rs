// 给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数 。
// 算法的时间复杂度应该为 O(log (m+n))
fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut nums1 = nums1;
    let mut nums2 = nums2;
    nums1.append(&mut nums2);
    nums1.sort();
    let len = nums1.len();
    if len % 2 == 0 {
        (nums1[len / 2 - 1] + nums1[len / 2]) as f64 / 2.0
    } else {
        nums1[len / 2] as f64
    }
}

fn main() {
    println!("Hello, world!");
}



