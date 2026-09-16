use neko_securities::Security;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
    Market,
    Limit { limit_price: f64 },
    Stop { stop_price: f64, is_triggered: bool },
}

pub struct Order {
    pub side: Side,
    pub quantity: f64,
    pub security: Box<dyn Security>,
    pub order_type: OrderType,
}
