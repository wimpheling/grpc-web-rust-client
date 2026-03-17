use std::collections::BTreeMap;

#[derive(Default, Clone)]
pub struct Metadata {
    inner: BTreeMap<String, String>,
}

impl Metadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.inner.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.inner.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.inner.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    pub fn to_headers(&self) -> Vec<(&str, &str)> {
        self.iter().collect()
    }
}

impl From<BTreeMap<String, String>> for Metadata {
    fn from(inner: BTreeMap<String, String>) -> Self {
        Self { inner }
    }
}

impl<K, V> FromIterator<(K, V)> for Metadata
where
    K: Into<String>,
    V: Into<String>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut metadata = Metadata::new();
        for (k, v) in iter {
            metadata.insert(k, v);
        }
        metadata
    }
}

pub type MetadataMap = Metadata;
