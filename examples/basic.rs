extern crate repsheet_etl;

fn main() {
    let _ = match repsheet_etl::process("access.log") {
        Ok(actors) => println!("Processed {} actors", actors.keys().len()),
        Err(e) => println!("{}", e),
    };
}
