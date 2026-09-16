
mod strategy;
mod backtest_runner;
mod order;

pub use crate::strategy::{Strategy, BuyAllSellNextDay};
pub use crate::backtest_runner::BacktestRunner;

#[cfg(test)]
mod backtesting_tests {
    use super::*;
    use neko_securities::{Position, Cash};
    use neko_securities::currency::{get_currency, USD};

    #[test]
    fn simplest_backtesting_test() {
        //Arrange
        let price_path: Vec<f64> = vec!(1.0, 2.0, 3.0, 4.0);

        let initial_cash = Position {
            security: Box::new(Cash),
            quantity: 10.0,
            currency: get_currency(USD.to_string()),
        };

        //Act
        let mut strategy = BuyAllSellNextDay{ has_bought : false };
        let final_portfolio = BacktestRunner::run_backtest(&mut strategy, vec!(initial_cash), &price_path);
        let pl = final_portfolio[0].quantity - 10.0;

        //Assert
        assert_eq!(pl, 16.0);
    }
}
