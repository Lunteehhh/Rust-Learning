pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn sub(left: u64, right: u64) -> u64 {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_sub() {
        let result = sub(4, 3);
        assert_eq!(result, 1)
    }
}
