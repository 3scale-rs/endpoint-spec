//use anyhow::{anyhow, bail, Error};
use anyhow::bail;
use std::collections::{BTreeMap, BTreeSet};
use std::iter::FromIterator;
use std::string::ToString;
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ValidationError<'a> {
    #[error("validation error")]
    Custom {
        source: anyhow::Error,
        values: BTreeMap<&'a str, String>,
    },
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SerializationError {
    #[error("no serializer set")]
    MissingSerializer,
    #[error("serialization error")]
    Custom(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SpecError<'a> {
    #[error("key {0} is not valid in this spec")]
    InvalidKey(String),
    #[error("{error:?}")]
    ValidationFailed { error: ValidationError<'a> },
}

#[derive(Clone)]
pub struct Parameters<'a> {
    map: BTreeMap<&'a str, String>,
    serialize: Option<fn(&Self) -> Result<String, SerializationError>>,
}

impl<'a> IntoIterator for Parameters<'a> {
    type Item = <BTreeMap<&'a str, String> as IntoIterator>::Item;

    type IntoIter = <BTreeMap<&'a str, String> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl<'p, 'a> IntoIterator for &'p Parameters<'a> {
    type Item = <&'p BTreeMap<&'a str, String> as IntoIterator>::Item;

    type IntoIter = <&'p BTreeMap<&'a str, String> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.map).into_iter()
    }
}

impl<'p, 'a> IntoIterator for &'p mut Parameters<'a> {
    type Item = <&'p mut BTreeMap<&'a str, String> as IntoIterator>::Item;

    type IntoIter = <&'p mut BTreeMap<&'a str, String> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.map).into_iter()
    }
}

impl<'a> Parameters<'a> {
    pub fn new(
        map: BTreeMap<&'a str, String>,
        serialize: Option<fn(&Self) -> Result<String, SerializationError>>,
    ) -> Self {
        Self { map, serialize }
    }

    pub fn set_serializer(
        &mut self,
        serialize: Option<fn(&Self) -> Result<String, SerializationError>>,
    ) {
        self.serialize = serialize;
    }

    pub fn serialize(&self) -> Result<String, SerializationError> {
        let s = self
            .serialize
            .ok_or(SerializationError::MissingSerializer)?;

        s(self)
    }

    pub fn inner(&self) -> &BTreeMap<&'a str, String> {
        &self.map
    }

    pub fn into_inner(self) -> BTreeMap<&'a str, String> {
        self.map
    }

    pub fn iter(&self) -> impl Iterator<Item = (&&'a str, &String)> {
        self.map.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&&'a str, &mut String)> {
        self.map.iter_mut()
    }

    pub fn as_vec(&self) -> Vec<(&'a str, &str)> {
        self.iter().map(|(&k, v)| (k, v.as_str())).collect()
    }

    pub fn as_vec_mut(&mut self) -> Vec<(&'a str, &mut String)> {
        self.iter_mut().map(|(&k, v)| (k, v)).collect()
    }

    pub fn into_vec(self) -> Vec<(&'a str, String)> {
        self.map.into_iter().collect()
    }

    pub fn insert(&mut self, key: &'a str, value: String) -> Option<String> {
        self.map.insert(key, value)
    }

    pub fn remove(&mut self, key: &'a str) -> Option<String> {
        self.map.remove(key)
    }
}

pub struct ParametersSpec<'a> {
    keys: BTreeSet<String>,
    validate: Option<
        fn(
            &'a Self,
            BTreeMap<&'a str, String>,
        ) -> Result<BTreeMap<&'a str, String>, ValidationError<'a>>,
    >,
    serialize: Option<fn(&Parameters) -> Result<String, SerializationError>>,
}

impl<S: ToString> FromIterator<S> for ParametersSpec<'_> {
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        Self {
            keys: BTreeSet::from_iter(iter.into_iter().map(|i| i.to_string())),
            validate: None,
            serialize: None,
        }
    }
}

impl<S: Into<BTreeSet<String>>> From<S> for ParametersSpec<'_> {
    fn from(s: S) -> Self {
        Self {
            keys: s.into(),
            validate: None,
            serialize: None,
        }
    }
}

impl<'a> ParametersSpec<'a> {
    pub fn new<I: ToString, L: Iterator<Item = I>>(iter: L) -> Self {
        Self {
            keys: iter.map(|i| i.to_string()).collect(),
            validate: None,
            serialize: None,
        }
    }

    pub fn set_validator(
        &mut self,
        f: fn(
            &'a Self,
            BTreeMap<&'a str, String>,
        ) -> Result<BTreeMap<&'a str, String>, ValidationError<'a>>,
    ) {
        self.validate = Some(f);
    }

    pub fn set_serializer(&mut self, f: fn(&Parameters) -> Result<String, SerializationError>) {
        self.serialize = f.into();
    }

    pub fn fill(
        &'a self,
        mut values: BTreeMap<&'a str, String>,
    ) -> Result<Parameters<'a>, SpecError<'a>> {
        let params = BTreeMap::<&str, String>::new();
        for (&key, value) in &values {
            if !self.keys.contains(key) {
                return Err(SpecError::InvalidKey(key.to_string()));
            }
        }

        if let Some(f) = self.validate {
            values = f(self, values).map_err(|e| SpecError::ValidationFailed { error: e })?;
        }

        Ok(Parameters::new(values, self.serialize))
    }
}

pub enum ParameterQuantifier {
    PairingSegments,
    JoiningSegments,
}

pub(super) struct PathBuilder<'a, 's> {
    segments: &'a [&'s str],
    quantifier: ParameterQuantifier,
}

// Untyped mixing of strings to build paths.
impl<'a, 's> PathBuilder<'a, 's> {
    pub const fn new(segments: &'a [&'s str], quantifier: ParameterQuantifier) -> Self {
        Self {
            segments,
            quantifier,
        }
    }

    pub fn accepted_parameters(&self) -> usize {
        match self.quantifier {
            ParameterQuantifier::PairingSegments => self.segments.len(),
            ParameterQuantifier::JoiningSegments => self.segments.len() - 1,
        }
    }

    // Similar to what itertools::Itertools::zip_longest would do
    pub fn build(&self, params: &[&str]) -> Result<String, anyhow::Error> {
        if params.len() != self.accepted_parameters() {
            bail!(
                "required {} parameters but {} were provided",
                self.accepted_parameters(),
                params.len()
            );
        }

        let mut s = self.segments.iter().zip(params.iter()).fold(
            String::new(),
            |mut acc, (segment, arg)| {
                acc.push_str(segment);
                acc.push_str(arg);
                acc
            },
        );

        // If we were just joining, the operation above would've stopped right before
        // appending the last segment, so append it now.
        if let ParameterQuantifier::JoiningSegments = self.quantifier {
            s.push_str(self.segments.last().unwrap());
        }

        Ok(s)
    }
}
