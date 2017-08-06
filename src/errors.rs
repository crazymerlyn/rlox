error_chain! {
    errors {
        ScanError(line: usize, t: &'static str) {
            description("Invalid program")
            display("Error at line {}: {}", line, t)
        }
    }
    foreign_links {
        IO(::std::io::Error);
    }
}
