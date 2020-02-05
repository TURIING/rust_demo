/*
 * 排序算法
 */
pub mod sort {
    /*
     * 功能: 冒泡排序
     */
    pub fn bubble (array: &mut [i32;4]){
        for i in 0..array.len()-1 {
            for j in 0..array.len()-i-1 {
                if array[j] > array[j+1] {
                    let tmp = array[j];
                    array[j] = array[j+1];
                    array[j+1] = tmp;
                }
            }
        }
    }

}
#[cfg(test)]
mod tests {
    use super::sort;
    #[test]
    fn work() {
        let mut array: [i32;4] = [23, 12, 34, 21];
        sort::bubble(&mut array);
        for i in array.iter() {
            print!("{} ",i);
        }
    }
}
