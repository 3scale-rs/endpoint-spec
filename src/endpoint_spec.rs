use core::marker::PhantomData;

use http::Method;
use serde::de::{Deserialize, DeserializeOwned};

use super::serde::DeserializerFormat;

pub struct EndpointSpec<B, D, T> {
    method: Method,
    path: String,
    headers: Vec<(String, String)>,
    body: Option<B>,
    trailers: Option<Vec<(String, String)>>,
    deserializer: D,
    data_type: PhantomData<T>,
}

impl<B, D, T> EndpointSpec<B, D, T> {
    pub fn new(
        method: Method,
        path: impl ToString,
        headers: Vec<(String, String)>,
        body: Option<B>,
        trailers: Option<Vec<(String, String)>>,
        deserializer: D,
    ) -> Self {
        Self {
            method,
            path: path.to_string(),
            headers,
            body,
            trailers,
            deserializer,
            data_type: PhantomData,
        }
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &str {
        self.path.as_str()
    }

    pub fn headers(&self) -> &Vec<(String, String)> {
        self.headers.as_ref()
    }

    pub fn body(&self) -> Option<&B> {
        self.body.as_ref()
    }

    pub fn headers_as_str(&self) -> Vec<(&str, &str)> {
        self.headers()
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }
}

impl<'de, B, D: DeserializerFormat, T: Deserialize<'de>> EndpointSpec<B, D, T> {
    #[inline(always)]
    pub fn parse_str(&self, response: &'de str) -> Result<T, anyhow::Error> {
        self.deserializer.parse_str(response)
    }

    #[inline(always)]
    pub fn parse_slice(&self, response: &'de [u8]) -> Result<T, anyhow::Error> {
        self.deserializer.parse_slice(response)
    }
}

impl<'de, B, D: DeserializerFormat, T: DeserializeOwned> EndpointSpec<B, D, T> {
    #[inline(always)]
    fn parse_reader<R: std::io::Read>(&self, rdr: R) -> Result<T, anyhow::Error> {
        self.deserializer.parse_reader(rdr)
    }
}
