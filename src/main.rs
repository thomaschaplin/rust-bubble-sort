fn bubble_sort(mut vec: Vec<i32>) -> Vec<i32> {
    loop {
        let mut swapped = false;
        let mut i = 0;
        while i < vec.len() {
            if i >= vec.len() - 1 {
                break;
            }
            if vec[i] > vec[i + 1] {
                swapped = true;
                let value = vec[i];
                vec.remove(i);
                vec.insert(i + 1, value);
                break;
            }
            i += 1;
        }
        if !swapped {
            break;
        }
    }
    vec
}

fn main() {
    let unsorted_vector: Vec<i32> = vec![5, 4, 3, 2, 1];
    println!("Starting a bubble sort on {:?}", unsorted_vector);
    let sorted_vector: Vec<i32> = bubble_sort(unsorted_vector);
    println!(
        "Bubble sort completed, here's the result: {:?}",
        sorted_vector
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_unsorted_vector_is_sorted_correctly() {
        let unsorted_vector: Vec<i32> = vec![5, 4, 3, 2, 1];
        let sorted_vector: Vec<i32> = vec![1, 2, 3, 4, 5];
        assert_eq!(sorted_vector, bubble_sort(unsorted_vector));
    }
}
