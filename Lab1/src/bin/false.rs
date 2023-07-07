fn main() {
    std::process::exit(1);
}

/*
    The code below will cause the program finish with error too, process::abort will cause stack overrun 
    which is a kind of error.

    https://doc.rust-lang.org/std/process/fn.abort.html
*/
/*
    fn main() {
        std::process::abort();
    }
*/
