fn word_parity_liniar(number: &u64) -> u8 {

    let mut copy = number.to_owned();
    let mut cnt = 0;

    while copy != 0 {
        if (copy & 1) == 1 {
            cnt += 1;
            copy >> 1;
        }
    }

    if cnt % 2 == 0 {
        return 1;
    }

    return 0;
}

fn word_parity(number: &u64) -> u8 {

    let mut copy = number.to_owned();
    let mut parity = 0;

    while copy != 0 { // -> there are bits of 1
        parity = parity ^ 1;
        copy = copy & (copy - 1); // drop the lowest set bit.
    }

    parity
}

#[cfg(test)]
mod tests {
    use crate::primitive_types::word_parity::word_parity;

    #[test]
    fn big_number() {
        let big_number: u64 = 568184680;
        let parity = word_parity(&big_number);
        assert_eq!(parity, 1);
    }

    #[test]
    fn zero_test() {
        let zero: u64 = 0;
        let parity = word_parity(&zero);
        assert_eq!(parity, 0);
    }

    #[test]
    fn even_test() {
        let even_parity_number = 20;
        let parity = word_parity(&even_parity_number);
        assert_eq!(parity, 0);
    }

    #[test]
    fn odd_test() {
        let odd_parity_number = 8;
        let parity = word_parity(&odd_parity_number);
        assert_eq!(parity, 1);
    }

}