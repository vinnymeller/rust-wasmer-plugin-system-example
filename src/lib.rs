pub use example_macro::plugin_helper;

pub fn convert_data<'a, D>(bytes: &'a [u8]) -> D
where
    D: serde::Deserialize<'a>,
{
    bincode::deserialize(bytes).expect("Failed to deserialize bytes")
}

pub fn revert_data<S>(s: S) -> Vec<u8>
where
    S: serde::Serialize,
{
    bincode::serialize(&s).expect("Failed to serialize data")
}
