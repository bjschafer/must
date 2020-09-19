#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod tape;

pub fn foo() {
    tape::tape::status("/dev/nst0");
}