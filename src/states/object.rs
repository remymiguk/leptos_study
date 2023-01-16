use serde::{de::DeserializeOwned, Serialize};

pub trait Object:
    Serialize + DeserializeOwned + Clone + PartialEq + Eq + std::fmt::Debug + 'static
{
}

impl<T: Serialize + DeserializeOwned + Clone + PartialEq + Eq + std::fmt::Debug + 'static> Object
    for T
{
}
