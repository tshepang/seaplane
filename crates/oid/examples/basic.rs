use seaplane_oid::{error::*, Oid};
use uuid::Uuid;

fn main() -> Result<()> {
    // OIDs can be created with a given prefix alone, which generates a new
    // UUIDv7 using the current unix timestamp
    let oid = Oid::new("exm")?;
    println!("{oid}");

    // OIDs can be parsed from strings, however the "value" must be a valid
    // base32 encoded UUIDv7
    let oid: Oid = "tst-agc6amh7z527vijkv2cutplwaa".parse()?;
    println!("{oid}");

    // OIDs can also be created from the raw parts
    let oid = Oid::with_uuid(
        "exm",
        "0185e030-ffcf-75fa-a12a-ae8549bd7600"
            .parse::<Uuid>()
            .unwrap(),
    )?;

    // One can retrieve the various parts of the OID if needed
    println!("Prefix: {}", oid.prefix());
    println!("Value: {}", oid.value());
    println!("UUID: {}", oid.uuid());

    Ok(())
}
