use super::security::Security;
pub const CASH: &str = "Cash";

pub struct Cash;

impl Security for Cash {
    fn name(&self) -> &str { CASH }
    fn security_type(&self) -> &str { CASH }
}
