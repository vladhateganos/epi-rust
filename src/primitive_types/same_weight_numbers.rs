fn count_bits(n: i32) -> i32 {

    let mut copy = n.to_owned();
    let mut cnt = 0;
    while copy != 0 {
        cnt += 1;
        copy = copy & (copy - 1);
    }
    cnt
}

pub fn same_weight_numbers(n: i32) -> i32 {

    let mut i = n+1;
    let mut j = n-1;
    let n_bits = count_bits(n);

    loop {
        if count_bits(i) == n_bits {
            return i;
        }
        if count_bits(j) == n_bits {
            return j;
        }

        i += 1;
        j -= 1;
    }
}

mod tests {
    use super::same_weight_numbers;
    #[test]
    fn find_upwards() {
        assert_eq!(same_weight_numbers(9), 10);
    }

    #[test]
    fn find_downwards() {
        assert_eq!(same_weight_numbers(10), 9);
    }

    #[test]
    fn one_bit() {
        assert_eq!(same_weight_numbers(32), 16);
        assert_eq!(same_weight_numbers(1), 2);
        assert_eq!(same_weight_numbers(4), 2);

    }
}