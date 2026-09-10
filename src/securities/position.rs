use crate::currency::Currency;
use super::security::Security;

pub struct Position {
    pub security: Box<dyn Security>,
    pub quantity: f64,
    pub currency: Currency,
}
