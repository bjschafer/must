#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod tape;
pub mod backup;

pub fn foo() {
    tape::tape::status("/dev/nst0");
    // tape::tape::fastforward("/dev/nst0", 1);
    // tape::tape::get_position("/dev/nst0");
}
