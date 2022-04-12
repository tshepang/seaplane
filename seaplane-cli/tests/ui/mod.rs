// We have to go through this little bit of indirection because of how integration directory
// structure works.

mod semantic;
#[cfg(feature = "ui_tests")]
mod trycmd;
