use core::marker::PhantomData;

use http::Method;
use serde::de::{Deserialize, DeserializeOwned};

use super::body_spec::BodySpec;
use super::headers_spec::{HeadersSpec, TrailersSpec};
use super::path_spec::PathSpec;
use super::serde::DeserializerFormat;

pub struct EndpointSpec<'a, B, D, T> {
    method: Method,
    path_spec: PathSpec<'a>,
    headers_spec: HeadersSpec,
    body_spec: BodySpec<B>,
    trailers_spec: TrailersSpec,
    deserializer: D,
    data_type: PhantomData<T>,
}

impl<'a, B, D, T> EndpointSpec<'a, B, D, T> {
    pub fn new(
        method: Method,
        path_spec: PathSpec<'a>,
        headers_spec: HeadersSpec,
        body_spec: BodySpec<B>,
        trailers_spec: TrailersSpec,
        deserializer: D,
    ) -> Self {
        Self {
            method,
            path_spec,
            headers_spec,
            body_spec,
            trailers_spec,
            deserializer,
            data_type: PhantomData,
        }
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path_spec(&self) -> &PathSpec<'a> {
        &self.path_spec
    }

    pub fn headers_spec(&self) -> &HeadersSpec {
        &self.headers_spec
    }

    pub fn body_spec(&self) -> &BodySpec<B> {
        &self.body_spec
    }

    //pub fn headers_as_str(&self) -> Vec<(&str, &str)> {
    //    self.headers_spec()
    //        .iter()
    //        .map(|(k, v)| (k.as_str(), v.as_str()))
    //        .collect()
    //}
}

impl<'de, B, D: DeserializerFormat, T: Deserialize<'de>> EndpointSpec<'_, B, D, T> {
    #[inline(always)]
    pub fn parse_str(&self, response: &'de str) -> Result<T, anyhow::Error> {
        self.deserializer.parse_str(response)
    }

    #[inline(always)]
    pub fn parse_slice(&self, response: &'de [u8]) -> Result<T, anyhow::Error> {
        self.deserializer.parse_slice(response)
    }
}

impl<B, D: DeserializerFormat, T: DeserializeOwned> EndpointSpec<'_, B, D, T> {
    #[inline(always)]
    fn parse_reader<R: std::io::Read>(&self, rdr: R) -> Result<T, anyhow::Error> {
        self.deserializer.parse_reader(rdr)
    }
}
