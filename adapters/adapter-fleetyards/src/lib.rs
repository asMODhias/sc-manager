pub fn name() -> &'static str { "FleetYards" }

#[cfg(test)]
mod tests {
    use super::name;

    #[test]
    fn stub_builds() {
        assert_eq!(name(), "FleetYards");
    }
}
