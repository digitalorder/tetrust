pub mod tlv {
    pub type Type = u8;
    pub type Length = u8;

    pub fn encode(t: Type, l: Length, v: &[u8], ostream: &mut dyn std::io::Write) -> std::io::Result<()> {
        let tl: [u8; 2] = [t, l];
        ostream.write(&tl[..])?;
        ostream.write(v)?;
        Ok(())
    }

    pub fn decode_one(istream: &mut dyn std::io::Read) -> std::io::Result<(Type, Length, Vec<u8>)> {
        let mut tl: [u8; 2] = [0, 0];
        istream.read(&mut tl[..])?;
        let t = tl[0];
        let l = tl[1] as usize;
        let mut v = vec![0; l];
        istream.read(&mut v[0..l])?;
        Ok((t as Type, l as Length, v))
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

    impl std::io::Read for TestIo {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            if self.buf.len() > 0 {
                buf.copy_from_slice(&self.buf[..buf.len()]);
                self.buf.drain(0..buf.len());
                std::io::Result::Ok(0)
            } else {
                std::io::Result::Err(std::io::Error::new(std::io::ErrorKind::WouldBlock, "reading empty"))
            }
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

    #[test]
    fn decode_one_tlv() {
        let mut io = TestIo::default();
        io.buf.resize(4, 0);
        io.buf.copy_from_slice(&[1, 2, 23, 42]);
        let result = decode_one(&mut io);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), (1, 2, vec![23, 42]));
    }
}
