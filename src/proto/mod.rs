pub use self::transactions::{CreateCounterTxBody, IncrementCounterTxBody};

include!(concat!(env!("OUT_DIR"), "/protobuf_mod.rs"));
