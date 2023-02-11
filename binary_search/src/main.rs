fn main() {
    let arr: &[u32] = &[1, 4, 10, 22, 23, 23, 27, 44, 60, 70, 80];
    let res = binary_search(arr, 81);
    println!("{:?}", res);
}

fn binary_search(arr: &[u32], k: u32) -> isize {
    let mut start: isize = 0;
    let mut end: isize = (arr.len() - 1) as isize;
    while start <= end {
        let mid: isize = start + (end - start) / 2;
        if arr[mid as usize] == k {
            return mid as isize;
        }
        else if  arr[mid as usize] < k{
            start = mid + 1;
        }
        else {
            end = mid - 1;
            
        }
    }
    /*while arr[result as usize] != k {
        if start > end {
            result = start -1;
            break;
        }
        println!("{start}, {end}");
        let mid = start + (end - start) / 2;
        if arr[mid as usize] == k {
            result = mid;
        }
        else if  arr[mid as usize] < k{
            start = mid + 1;
        }
        else {
            end = mid - 1;
            
        }
    } */

    start -1  
}
