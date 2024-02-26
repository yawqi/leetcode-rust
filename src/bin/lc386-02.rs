struct Solution;

impl Solution {
    fn get_intersect(bl1: &Vec<i32>, tr1: &Vec<i32>, bl2: &Vec<i32>, tr2: &Vec<i32>) -> i64 {
        let (x1, y1) = (bl1[0], bl1[1]);
        let (x2, y2) = (tr1[0], tr1[1]);
        let (xx1, yy1) = (bl2[0], bl2[1]);
        let (xx2, yy2) = (tr2[0], tr2[1]);

        let mut xlen = 0;
        let mut ylen = 0;
        if xx1 >= x1 && xx1 <= x2 {
            xlen = x2.min(xx2) - xx1;
        } else if x1 >= xx1 && x1 <= xx2 {
            xlen = xx2.min(x2) - x1;
        }

        if yy1 >= y1 && yy1 <= y2 {
            ylen = y2.min(yy2) - yy1;
        } else if y1 >= yy1 && y1 <= yy2 {
            ylen = y2.min(yy2) - y1;
        }

        println!("{xlen} {ylen}");
        let len = xlen.min(ylen) as i64;
        len * len
    }
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let mut max_area = 0;
        let len = bottom_left.len();
        for i in 0..len - 1 {
            for j in i + 1..len {
                max_area = max_area.max(Self::get_intersect(
                    &bottom_left[i],
                    &top_right[i],
                    &bottom_left[j],
                    &top_right[j],
                ));
            }
        }
        max_area
    }
}

fn main() {
    let bottom_left = vec![vec![3, 2], vec![3, 1], vec![6, 1]];
    let top_right = vec![vec![8, 3], vec![4, 8], vec![9, 10]];
    let ans = Solution::largest_square_area(bottom_left, top_right);
    println!("{ans}");
}
