pub fn validate_currency(currency: &str) -> bool {
    matches!(currency, "NGN" | "GHS" | "KES" | "USD" | "EUR")
}
