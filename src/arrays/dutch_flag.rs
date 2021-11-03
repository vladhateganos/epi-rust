use std::error::Error;
use std::boxed::Box;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct IndexOutOfBounds;

impl Display for IndexOutOfBounds {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Index out of bounds!")
    }
}

pub fn dutch_flag(arr: &mut [i32], piv_pos: u8) -> Result<&[i32], IndexOutOfBounds> {

    if piv_pos >= arr.len() as u8 {
        return Err(IndexOutOfBounds);
    }

    let mut sml = 0;
    let mut bgr = (arr.len()-1) as i32;
    let mut eq = 0 as i32;
    let piv = arr[piv_pos as usize];

    while eq < bgr {

        if eq < piv {
            let temp = arr[eq as usize];
            arr[eq as usize] = arr[sml as usize];
            arr[sml as usize] = arr[temp as usize];
            eq += 1;

        }else if eq == piv {
            eq += 1;
        }else {
            let temp = arr[eq as usize];
            arr[eq as usize] = arr[bgr as usize];
            arr[bgr as usize] = arr[temp as usize];
            bgr -= 1;
        }
    }

    Ok(arr)
}


#[cfg(test)]
mod tests {
    use super::dutch_flag;
    #[test]
    fn random_array() {
        let mut arr: [i32; 9] = [-3, 0, -1, 1, 1, 4, 6, 5, 2];
        let piv_pos = 8;
        assert_eq!(dutch_flag(&mut arr, piv_pos).unwrap(), [-3, 0, -1, 1, 1, 2, 5, 6, 4])
    }

}