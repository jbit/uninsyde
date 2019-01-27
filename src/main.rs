use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::u32;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() -> Result<(), Box<Error>> {
    let args: Vec<_> = std::env::args_os().collect();

    if args.len() < 2 {
        println!("{} - {}", PKG_NAME, PKG_VERSION);
        println!("{}", PKG_DESCRIPTION);
        println!("Usage\n\t{:?} [filename]", args[0]);
        return Err("No filename provided".into());
    }

    let path = Path::new(&args[1]);

    println!("Reading {:?}", path);

    let mut f = File::open(path)?;
    let mut bytes = vec![];

    f.read_to_end(&mut bytes)?;

    for offset in 0..bytes.len() {
        let slice = &bytes[offset..];
        // Find "$_IFLASH_" chunks
        if slice.len() < 32 {
            continue;
        }
        if !slice.starts_with(b"\0\0\0\0\0\0\0\0$_IFLASH_") {
            continue;
        }

        // Process the chunk name and sizes
        let name = String::from_utf8_lossy(&slice[17..24]).to_string();
        let file_size = u32::from_le_bytes([slice[24], slice[25], slice[26], slice[27]]) as usize;
        let data_size = u32::from_le_bytes([slice[28], slice[29], slice[30], slice[31]]) as usize;

        println!(
            "offset={:08x} name={} file_size={} data_size={} (padding={})",
            offset, name, file_size, data_size, file_size - data_size
        );

        // Write the chunk payload to disk
        let data = &slice[32..32 + data_size];
        let mut out = File::create(Path::new(&(name + ".bin")))?;
        out.write_all(data)?;
    }

    println!("Done");

    Ok(())
}
