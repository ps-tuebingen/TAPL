use driver::Driver;

fn main() -> Result<(), String> {
    let driver = Driver;
    driver.run_cli().map_err(|err| err.to_string())
}
