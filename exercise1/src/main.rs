fn main() {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [3, 5, 6];

    if check_sub_arr(org_arr, sub_arr) {
        println!("Array sub_arr is a subarray of array org_arr");
    } else {
        println!("Array sub_arr is not a subarray of array org_arr");
    }
}

fn check_sub_arr(org_arr: [i32; 8], sub_arr: [i32; 3]) -> bool {
    
    for i in 0..org_arr.len() {
        if org_arr[i] == sub_arr[0] {
            let mut j = i;
            let mut k = 0;
            while j < org_arr.len() && k < sub_arr.len() {
                if org_arr[j] == sub_arr[k] {
                    k += 1;
                } else {
                    break;
                }
                j += 1;
            }
            if k == sub_arr.len() {
                return true;
            }
        }
    }

    false
}