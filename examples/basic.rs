extern crate repsheet_etl;

fn main() {
    match repsheet_etl::process("samples/access.log") {
        Ok(actors) => println!("Processed {} actors", actors.keys().len()),
        Err(e) => println!("{}", e),
    };
}
