pub fn hash(value: &str) -> bcrypt::BcryptResult<String> {
    bcrypt::hash(value, 12)
}
