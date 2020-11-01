pub mod backup {
    use std::io::prelude::*;
    use std::fs::File;
    use tar::{Archive, Builder};

    pub fn create_archive(dev: &str) -> Builder<File> {
        let file = File::create(dev).unwrap();
        Builder::new(file)
    }

}
