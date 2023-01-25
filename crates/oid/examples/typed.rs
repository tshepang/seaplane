use seaplane_oid::{error::*, OidPrefix, TypedOid};

// `Tst` serves as our prefix and will be converted to ASCII lowercase for its prefix
#[derive(Debug)]
struct Tst;
impl OidPrefix for Tst {}

fn main() -> Result<()> {
    // We can create a new OID by generating a random UUID
    let oid: TypedOid<Tst> = TypedOid::new();
    println!("{oid}");
    // Prints something similar to: "tst-5wacbutjwbdexonddvdb2lnyxu"

    // We can go the other direction and parse a string to a TypedOid
    let oid: TypedOid<Tst> = "tst-5wacbutjwbdexonddvdb2lnyxu"
        .parse::<TypedOid<Tst>>()
        .unwrap();
    // One can retrieve the various parts of the OID if needed
    println!("Prefix: {}", oid.prefix());
    println!("Value: {}", oid.value());
    println!("UUID: {}", oid.uuid());

    // However, if we change the prefix to something that doesn't match our Tst type we get an
    // error
    let res = "frm-5wacbutjwbdexonddvdb2lnyxu".parse::<TypedOid<Tst>>();
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), Error::InvalidPrefixChar);

    Ok(())
}
