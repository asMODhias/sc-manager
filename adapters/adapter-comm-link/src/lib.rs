pub fn name() -> &'static str { "CommLink" }

#[cfg(test)]
mod tests {
    use super::name;

    #[test]
    fn stub_builds() {
        assert_eq!(name(), "CommLink");
    }
}
