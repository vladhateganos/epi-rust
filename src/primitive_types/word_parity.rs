fn word_parity(number: &u64) -> u8 {

    let mut copy = number.to_owned();
    let mut parity = 0;

    while copy != 0 { // -> there are bits of 1
        parity = parity ^ 1;
        copy = copy & (copy - 1); // drop the lowest set bit.
    }

    parity
}
