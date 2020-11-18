use serde::de::{Deserialize, DeserializeOwned};

pub trait DeserializerFormat {
    fn parse_str<'de, T: Deserialize<'de>>(&self, s: &'de str) -> Result<T, anyhow::Error>;
    fn parse_slice<'de, T: Deserialize<'de>>(&self, s: &'de [u8]) -> Result<T, anyhow::Error>;
    fn parse_reader<T: DeserializeOwned, R: std::io::Read>(
        &self,
        rdr: R,
    ) -> Result<T, anyhow::Error>;
}

pub struct JsonDeserializer;

impl DeserializerFormat for JsonDeserializer {
    fn parse_str<'de, T: Deserialize<'de>>(&self, s: &'de str) -> Result<T, anyhow::Error> {
        let res = serde_json::from_str(s)?;
        Ok(res)
    }

    fn parse_slice<'de, T: Deserialize<'de>>(&self, s: &'de [u8]) -> Result<T, anyhow::Error> {
        let res = serde_json::from_slice(s)?;
        Ok(res)
    }

    fn parse_reader<T: DeserializeOwned, R: std::io::Read>(
        &self,
        rdr: R,
    ) -> Result<T, anyhow::Error> {
        let res = serde_json::from_reader(rdr)?;
        Ok(res)
    }
}
