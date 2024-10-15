pub mod buf {
    use bytes::{BufMut, BytesMut};

    pub trait PutNull {
        fn put_null(&mut self, s: &str);
    }
    
    impl PutNull for BytesMut {
        fn put_null(&mut self, s: &str) {
            self.extend(s.as_bytes());
            self.put_u8(0);
        }
    }
}

pub mod addition {
    use rand::Rng;

    pub fn random_nonce() -> String {
        let mut rng = rand::thread_rng();
        (0..24)
            .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
            .collect()
    }
}
