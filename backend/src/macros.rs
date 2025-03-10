#[macro_export]
macro_rules! hdbe {
    ($data:expr, $nfm:expr) => {
        match $data {
            Ok(val) => val,
            Err(e) => return crate::response::handle_db_error(e, Some($nfm)),
        }
    };
    ($data:expr) => {
        match $data {
            Ok(val) => val,
            Err(e) => return crate::response::handle_db_error(e, None::<String>),
        }
    };
}
