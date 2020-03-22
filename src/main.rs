use kv::*;
use osmpbf::{ElementReader, Element};
use std::error::Error;
use std::env;

fn main() {
    run();
}

fn run() -> Result<(), Box<dyn Error>>  {
    let args: Vec<String> = env::args().collect();
    let reader = ElementReader::from_path(&args[1]).unwrap();

    let mut cfg = Config::new("./beep");
    // Open the key/value store
    let store = Store::new(cfg)?;
    let refs = store.bucket::<Raw, Raw>(Some("refs"))?;
    let allItems = store.bucket::<Raw, Raw>(Some("allItems"))?;

    // Increment the counter by one for each way.
    let ways = reader.par_map_reduce(|item| {
        match item {
            Element::Node(_) => {
                allItems[item.id()] = item
            },
            Element::Way(_) => {
                allItems[item.id()] = item;
                item.refs().map(|ref| {
                    if !refs.get(ref) {
                        refs[ref] = allItems[ref]
                    }
                })

            },
            _ => 0,
        }
    },
    || 0_u64,      // Zero is the identity value for addition
    |a, b| a + b   // Sum the partial results
    ).unwrap();

    println!("Number of ways: {}", ways);
    Ok(())
}
