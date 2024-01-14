pub mod message_broker;

pub fn test_add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = test_add(2, 2);
        assert_eq!(result, 4);
    }
}
