use neko_securities::{Position, Cash, Equity, CASH};
use neko_securities::currency::{get_currency, USD};
use crate::order::{Order, Side};
use crate::strategy::Strategy;

pub struct BacktestRunner;

impl BacktestRunner {
    pub fn run_backtest(strategy: &mut dyn Strategy, initial_portfolio: Vec<Position>, price_path: &[f64]) -> Vec<Position> {
        let mut current_portfolio = initial_portfolio;
        for price in price_path.iter() {
            let orders = strategy.iterate(&current_portfolio, *price);
            current_portfolio = apply_orders(current_portfolio, &orders, *price);
        }
        current_portfolio
    }
}

fn apply_orders(mut portfolio: Vec<Position>, orders: &[Order], price: f64) -> Vec<Position> {
    for order in orders {
        let security_type = order.security.security_type();
        match order.side {
            Side::Buy => {
                let cost = order.quantity * price;
                // Deduct cash
                for pos in portfolio.iter_mut() {
                    if pos.security.security_type() == CASH {
                        pos.quantity -= cost;
                        break;
                    }
                }
                // Add or increase equity position
                if let Some(pos) = portfolio.iter_mut().find(|p| p.security.security_type() == security_type) {
                    pos.quantity += order.quantity;
                } else {
                    portfolio.push(Position {
                        security: Box::new(Equity { ticker: order.security.name().to_string() }),
                        quantity: order.quantity,
                        currency: get_currency(USD.to_string()),
                    });
                }
            }
            Side::Sell => {
                let proceeds = order.quantity * price;
                // Reduce equity position
                for pos in portfolio.iter_mut() {
                    if pos.security.security_type() == security_type {
                        pos.quantity -= order.quantity;
                        break;
                    }
                }
                // Add cash proceeds
                if let Some(pos) = portfolio.iter_mut().find(|p| p.security.security_type() == CASH) {
                    pos.quantity += proceeds;
                } else {
                    portfolio.push(Position {
                        security: Box::new(Cash),
                        quantity: proceeds,
                        currency: get_currency(USD.to_string()),
                    });
                }
            }
        }
    }
    portfolio
}
