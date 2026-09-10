use super::security::Security;

pub const EQUITY: &str = "Equity";

#[allow(dead_code)]
pub struct Equity {
    pub ticker: String,
}

impl Security for Equity {
    fn name(&self) -> &str { &self.ticker }
    fn security_type(&self) -> &str { EQUITY }
}
