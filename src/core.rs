pub mod buf {
    pub trait ExtendWithNull {
        fn extend_with_null(&mut self, s: &str);
    }
    
    impl ExtendWithNull for Vec<u8> {
        fn extend_with_null(&mut self, s: &str) {
            self.extend(s.as_bytes());
            self.push(0);
        }
    }
}
