fn swap_bits(number: &u32, first: &u8, second: &u8) -> u32 {

    let mask = (1 << first) | (1 << second);
    let mut result = number.to_owned();
    if (number & mask == 0) || (number & mask == mask) {
        return result;
    }

    result = number ^ mask;
    result
}

#[cfg(test)]
mod tests {
    use super::swap_bits;

    #[test]
    fn same_bits_of_one() {
        let number = 19;
        let first = 1;
        let second = 4;
        let result = swap_bits(&number, &first, &second);
        assert_eq!(result, 19);
    }

    #[test]
    fn same_bits_of_zero() {
        let number = 16;
        let first = 0;
        let second = 1;
        let result = swap_bits(&number, &first, &second);
        assert_eq!(result, 16);
    }

    #[test]
    fn different_bits() {
        let number = 17;
        let first = 0;
        let second = 1;
        let result = swap_bits(&number, &first, &second);
        assert_eq!(result, 18);
    }


}