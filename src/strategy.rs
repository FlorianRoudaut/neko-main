use neko_securities::{Position, Equity, CASH, EQUITY};
use crate::order::{Order, OrderType, Side};

pub trait Strategy {
    fn iterate(&mut self, portfolio: &[Position], price: f64) -> Vec<Order>;
}

pub struct BuyAllSellNextDay {
    pub has_bought: bool,
}

fn get_available_cash(portfolio: &[Position]) -> f64 {
    portfolio.iter()
        .filter(|p| p.security.security_type() == CASH)
        .map(|p| p.quantity)
        .sum()
}

fn get_available_stock(portfolio: &[Position]) -> f64 {
    portfolio.iter()
        .filter(|p| p.security.security_type() == EQUITY)
        .map(|p| p.quantity)
        .sum()
}

impl BuyAllSellNextDay {
    fn sell_orders(&mut self, portfolio: &[Position]) -> Vec<Order> {
        let available_stock = get_available_stock(portfolio);
        self.has_bought = false;
        if available_stock > 0.0 {
            vec![Order {
                side: Side::Sell,
                quantity: available_stock,
                security: Box::new(Equity { ticker: "Stock".to_string() }),
                order_type: OrderType::Market,
            }]
        } else {
            vec![]
        }
    }

    fn buy_orders(&mut self, portfolio: &[Position], price: f64) -> Vec<Order> {
        let available_cash = get_available_cash(portfolio);
        let stock_to_buy = (available_cash / price).floor();
        self.has_bought = true;
        if stock_to_buy > 0.0 {
            vec![Order {
                side: Side::Buy,
                quantity: stock_to_buy,
                security: Box::new(Equity { ticker: "Stock".to_string() }),
                order_type: OrderType::Market,
            }]
        } else {
            vec![]
        }
    }
}

impl Strategy for BuyAllSellNextDay {
    fn iterate(&mut self, portfolio: &[Position], price: f64) -> Vec<Order> {
        if self.has_bought {
            self.sell_orders(portfolio)
        } else {
            self.buy_orders(portfolio, price)
        }
    }
}
