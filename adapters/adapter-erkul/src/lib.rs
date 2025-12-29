pub fn name() -> &'static str { "Erkul" }

#[cfg(test)]
mod tests {
    use super::name;

    #[test]
    fn stub_builds() {
        assert_eq!(name(), "Erkul");
    }
}
