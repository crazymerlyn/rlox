#![allow(unknown_lints)]

error_chain! {
    errors {
        ScanError(line: usize, t: String) {
            description("Invalid program")
            display("Error at line {}: {}", line, t)
        }
    }
    foreign_links {
        IO(::std::io::Error);
    }
}
