pub trait Security {
    fn name(&self) -> &str;
    fn security_type(&self) -> &str;
}
