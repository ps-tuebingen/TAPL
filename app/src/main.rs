mod cli;

fn main() {
    let res = cli::run();
    if let Err(err) = res {
        println!("Program exited with error:\n\t{err}")
    }
}
