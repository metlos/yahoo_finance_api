use crumb::Crumb;
use std::time::Duration;
use time::OffsetDateTime;
use wreq::Emulation;

use wreq::{Client, ClientBuilder, EmulationFactory};

// re-export time crate
pub use time;

mod crumb;
pub mod fundamentals;
mod options;
pub mod quote_summary;
mod quotes;
mod search_result;
mod yahoo_error;

pub use quotes::{
    AdjClose, CapitalGain, Dividend, PeriodInfo, Quote, QuoteBlock, QuoteList, Split,
    TradingPeriods, YChart, YMetaData, YQuoteBlock, YResponse,
};
pub use search_result::{
    YNewsItem, YOptionResult, YOptionResults, YQuoteItem, YQuoteItemOpt, YSearchResult,
    YSearchResultOpt,
};
pub use yahoo_error::YahooError;

const YCHART_URL: &str = "https://query1.finance.yahoo.com/v8/finance/chart";
const YSEARCH_URL: &str = "https://query2.finance.yahoo.com/v1/finance/search";
const YFUNDAMENTALS_URL: &str =
    "https://query2.finance.yahoo.com/ws/fundamentals-timeseries/v1/finance/timeseries";
const OPTIONS_URL: &str = "https://query2.finance.yahoo.com/v7/finance/options";

const DEFAULT_USER_AGENT_HEADER: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/39.0.2171.95 Safari/537.36";

// Macros instead of constants,
macro_rules! YCHART_PERIOD_QUERY {
    () => {
        "{url}/{symbol}?symbol={symbol}&period1={start}&period2={end}&interval={interval}&events=div|split|capitalGains"
    };
}
macro_rules! YCHART_RANGE_QUERY {
    () => {
        "{url}/{symbol}?symbol={symbol}&interval={interval}&range={range}&events=div|split|capitalGains"
    };
}
macro_rules! YCHART_PERIOD_INTERVAL_QUERY {
    () => {
        "{url}/{symbol}?symbol={symbol}&period={period}&interval={interval}&includePrePost={prepost}"
    };
}
macro_rules! YTICKER_QUERY {
    () => {
        "{url}?q={name}"
    };
}

/// Container for connection parameters to yahoo! finance server
pub struct YahooConnector {
    client: Client,
    crumb: Crumb,
    url: &'static str,
    search_url: &'static str,
}

#[derive(Default)]
pub struct YahooConnectorBuilder {
    client_builder: Option<ClientBuilder>,
    emulation: Option<Emulation>,
    timeout: Option<Duration>,
}

impl YahooConnector {
    /// Constructor for a new instance of the yahoo connector.
    pub fn new() -> YahooConnector {
        Self::builder().build().unwrap()
    }

    pub fn builder() -> YahooConnectorBuilder {
        YahooConnectorBuilder::default()
    }
}

impl Default for YahooConnector {
    fn default() -> Self {
        Self::new()
    }
}

impl YahooConnectorBuilder {
    pub fn build(self) -> Result<YahooConnector, YahooError> {
        let mut cb = self.client_builder.unwrap_or_else(Client::builder);

        if let Some(timeout) = self.timeout {
            cb = cb.timeout(timeout);
        }

        cb = cb.cookie_store(true);

        let emu = self
            .emulation
            .unwrap_or_else(|| wreq_util::Emulation::random().emulation());

        cb = cb.emulation(emu);

        let client = cb
            .build()
            .map_err(|e| YahooError::from_wreq_while(e, "building the client"))?;

        let crumb = Crumb::new(client.clone());

        Ok(YahooConnector {
            client,
            crumb,
            url: YCHART_URL,
            search_url: YSEARCH_URL,
        })
    }

    pub fn use_client_builder(mut self, client_builder: ClientBuilder) -> Self {
        self.client_builder = Some(client_builder);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn emulation<EF: EmulationFactory>(mut self, emulation: EF) -> Self {
        self.emulation = Some(emulation.emulation());
        self
    }
}

pub mod async_impl;

#[cfg(test)]
mod tests {
    use crate::{quote_summary::QuoteSummaryField, YahooConnector};

    #[test]
    fn test_quote_summary_live() {
        tokio_test::block_on(async {
            env_logger::builder()
                .filter_level(log::LevelFilter::Trace)
                .init();

            let c = YahooConnector::new();
            let res = c
                .get_quote_summary(
                    "AAPL",
                    &[
                        QuoteSummaryField::EarningsTrend,
                        QuoteSummaryField::Earnings,
                        QuoteSummaryField::DefaultKeyStatistics,
                    ],
                )
                .await;
            assert!(res.is_ok());
            println!("{:?}", res.unwrap());
            panic!("just to see the output");
        });
    }
}
