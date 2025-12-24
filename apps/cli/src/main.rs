use driver::Driver;

fn main() -> Result<(), String> {
    let mut driver = Driver::new();
    driver.run_cli().map_err(|err| err.to_string())
}
