#[cfg(test)]
mod tests {
    use crate::sort::{insert_sort, merge_sort, shell_sort};

    #[test]
    fn insert_sort_test() {
        let mut arr = vec![2, 5, 4, 6, 1, 3];
        insert_sort(&mut arr);

        println!("{:?}", arr)
    }

    #[test]
    fn shell_sort_test() {
        let mut arr = vec![55, 94, 87, 1, 4, 32, 11, 77, 39, 42, 64, 53, 70, 12, 9];
        shell_sort(&mut arr);

        println!("{:?}", arr)
    }

    #[test]
    fn merge_sort_test() {
        let arr = vec![55, 94, 87, 1, 4, 32, 11, 77, 39, 42, 64, 53, 70, 12, 9];
        let result = merge_sort(arr);

        println!("{:?}", result)
    }
}

pub fn shell_sort<T: Ord + Copy>(arr: &mut [T]) {
    let n = arr.len();
    if n < 2 {
        return;
    }

    let mut key = n / 2;
    while key >= 1 {
        let mut i = key;
        while i < n {
            let mut temp = arr[i];
            let mut j = i - key;
            while arr[j] < temp {
                arr[j + key] = arr[j];
                if j < key {
                    break;
                }
                j = j - key;
            }
            arr[j + key] = temp;
            i += 1;
        }
        key /= 2;
    }
}

pub fn insert_sort<T: Ord + Copy>(arr: &mut [T]) {
    let mut i = 0;
    while i < arr.len() - 1 {
        let mut j = i + 1;
        while j > 0 {
            if arr[j] > arr[j - 1] {
                break;
            }

            let temp = arr[j - 1];
            arr[j - 1] = arr[j];
            arr[j] = temp;

            j -= 1;
        }

        i += 1;
    }
}

pub fn merge_sort<T: Ord + Copy>(arr: Vec<T>) -> Vec<T> {
    let n = arr.len();
    if n < 2 {
        return arr;
    }

    let key = n / 2;
    let mut left = merge_sort(arr[0..key].to_vec());
    let mut right = merge_sort(arr[key..].to_vec());

    merge(&mut left, &mut right)
}

fn merge<T: Ord + Copy>(left: &mut Vec<T>, right: &mut Vec<T>) -> Vec<T> {
    let mut tmp = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            tmp.push(left[i]);
            i += 1;
        } else {
            tmp.push(right[j]);
            j += 1;
        }
    }

    tmp.append(&mut left[i..].to_vec());
    tmp.append(&mut right[j..].to_vec());

    tmp
}
