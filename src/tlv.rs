pub mod tlv {
    pub type Type = u8;
    pub type Length = u8;

    pub fn encode(t: Type, l: Length, v: &[u8], ostream: &mut dyn std::io::Write) -> std::io::Result<()> {
        let tl: [u8; 2] = [t, l];
        ostream.write(&tl[..])?;
        ostream.write(v)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::tlv::*;

    #[derive(Default)]
    struct TestIo {
        pub buf: Vec<u8>,
    }

    impl std::io::Write for TestIo {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.buf.extend_from_slice(buf);
            std::io::Result::Ok(0)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn encode_trivial() {
        let mut io = TestIo::default();
        let result = encode(1, 2, &[23, 42], &mut io);
        let encoded: [u8; 4] = [1, 2, 23, 42];
        assert_eq!(result.is_ok(), true);
        assert_eq!(&encoded[..], &io.buf[..]);
    }
}
