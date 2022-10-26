use serde_yaml::with::singleton_map_recursive;
pub use serde_yaml::Result;
pub use serde_yaml::Value;

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: ?Sized + serde::Serialize,
{
    let mut vec = Vec::with_capacity(128);
    let mut serializer = serde_yaml::Serializer::new(&mut vec);

    singleton_map_recursive::serialize(&value, &mut serializer)?;
    String::from_utf8(vec).map_err(serde::ser::Error::custom)
}

pub fn from_str<'de, T>(s: &'de str) -> Result<T>
where
    T: serde::Deserialize<'de>,
{
    let deserializer = serde_yaml::Deserializer::from_str(s);
    singleton_map_recursive::deserialize(deserializer)
}
