fn main() {
    let array = vec![1, 2, 3, 4, 5];
    println!("Original array: {:?}", array);
    let pairs = create_pairs(array);
    println!("Array pairs: {:?}", pairs);

    let array = vec![1, 2, 3, 4, 5, 6];
    println!("Original array: {:?}", array);
    let pairs = create_pairs(array);
    println!("Array pairs: {:?}", pairs);
}

fn create_pairs(array: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let len = array.len();

    for i in 0..(len / 2) {
        result.push(vec![array[i], array[len - 1 - i]]);
    }

    if len % 2 == 1 {
        let middle_index = len / 2;
        result.push(vec![array[middle_index], array[middle_index]]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_length_array() {
        let array = vec![1, 2, 3, 4, 5];
        let pairs = create_pairs(array);
        assert_eq!(pairs, vec![vec![1, 5], vec![2, 4], vec![3, 3]]);
    }

    #[test]
    fn test_even_length_array() {
        let array = vec![1, 2, 3, 4];
        let pairs = create_pairs(array);
        assert_eq!(pairs, vec![vec![1, 4], vec![2, 3]]);
    }

    #[test]
    fn test_single_element_array() {
        let array = vec![1];
        let pairs = create_pairs(array);
        assert_eq!(pairs, vec![vec![1, 1]]);
    }

    #[test]
    fn test_empty_array() {
        let array = Vec::new();
        let pairs = create_pairs(array);
        assert_eq!(pairs, Vec::<Vec<i32>>::new());
    }
}
