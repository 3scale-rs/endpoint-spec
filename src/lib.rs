#![deny(clippy::warnings)]

mod body_spec;
mod endpoint_spec;
mod headers_spec;
mod path_spec;
mod serde;

#[cfg(test)]
mod tests {
    #[test]
    fn some_fn_is_42() {
        assert_eq!(42, 42);
    }
}
