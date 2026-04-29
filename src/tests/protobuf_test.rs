
#[cfg(test)]
mod tests {
    use crate::proto::*;

    #[test]
    fn create_request() {
        let request = AppRequest {
            seq: 1,
            ..Default::default()
        };
        assert_eq!(request.seq, 1);
    }
}