use std::io;
use std::path::PathBuf;

error_chain! {

    foreign_links {
        Io(io::Error) #[doc = "Error during IO"];
    }

    errors {
        Loader(path: PathBuf) {
            description("could not find loader")
            display("unable to find loader for path {:?}", path)
        }
    }

}
