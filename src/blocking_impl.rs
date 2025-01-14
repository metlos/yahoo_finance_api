use reqwest::{Request, Url};

use self::fundamentals::{IncomeStatement, IncomeStatementFact, Period};

use super::*;

impl YahooConnector {
    /// Retrieve the quotes of the last day for the given ticker
    pub fn get_latest_quotes(&self, ticker: &str, interval: &str) -> Result<YResponse, YahooError> {
        self.get_quote_range(ticker, interval, "1mo")
    }

    /// Retrieve the quote history for the given ticker form date start to end (inclusive), if available
    pub fn get_quote_history(
        &self,
        ticker: &str,
        start: OffsetDateTime,
        end: OffsetDateTime,
    ) -> Result<YResponse, YahooError> {
        self.get_quote_history_interval(ticker, start, end, "1d")
    }

    /// Retrieve quotes for the given ticker for an arbitrary range
    pub fn get_quote_range(
        &self,
        ticker: &str,
        interval: &str,
        range: &str,
    ) -> Result<YResponse, YahooError> {
        let url: String = format!(
            YCHART_RANGE_QUERY!(),
            url = self.url,
            symbol = ticker,
            interval = interval,
            range = range
        );
        YResponse::from_json(self.send_request(&url)?)
    }

    /// Retrieve the quote history for the given ticker form date start to end (inclusive), if available; specifying the interval of the ticker.
    pub fn get_quote_history_interval(
        &self,
        ticker: &str,
        start: OffsetDateTime,
        end: OffsetDateTime,
        interval: &str,
    ) -> Result<YResponse, YahooError> {
        let url = format!(
            YCHART_PERIOD_QUERY!(),
            url = self.url,
            symbol = ticker,
            start = start.unix_timestamp(),
            end = end.unix_timestamp(),
            interval = interval,
        );
        YResponse::from_json(self.send_request(&url)?)
    }

    /// Retrieve the quote history for the given ticker for a given period and ticker interval and optionally before and after regular trading hours
    pub fn get_quote_period_interval(
        &self,
        ticker: &str,
        period: &str,
        interval: &str,
        prepost: bool,
    ) -> Result<YResponse, YahooError> {
        let url = format!(
            YCHART_PERIOD_INTERVAL_QUERY!(),
            url = self.url,
            symbol = ticker,
            period = period,
            interval = interval,
            prepost = prepost,
        );
        YResponse::from_json(self.send_request(&url)?)
    }

    /// Retrieve the list of quotes found searching a given name
    pub fn search_ticker_opt(&self, name: &str) -> Result<YSearchResultOpt, YahooError> {
        let url = format!(YTICKER_QUERY!(), url = self.search_url, name = name);
        YSearchResultOpt::from_json(self.send_request(&url)?)
    }

    /// Retrieve the list of quotes found searching a given name
    pub fn search_ticker(&self, name: &str) -> Result<YSearchResult, YahooError> {
        let result = self.search_ticker_opt(name)?;
        Ok(YSearchResult::from_opt(&result))
    }

    /// Get list for options for a given name
    pub fn search_options(&self, name: &str) -> Result<YOptionResults, YahooError> {
        let url = format!("https://finance.yahoo.com/quote/{name}/options?p={name}");
        let resp = self.client.get(url).send()?.text()?;
        Ok(YOptionResults::scrape(&resp))
    }

    pub fn get_income_statement(
        &self,
        name: &str,
        period: Period,
        until: OffsetDateTime,
        facts: &[fundamentals::IncomeStatementFact],
    ) -> Result<fundamentals::IncomeStatement, YahooError> {
        let url = fundamentals::compose_fundamentals_url(name, period.clone(), until, facts);
        let resp = self.send_request(&url)?;
        fundamentals::from_response(resp, period, facts)
    }

    pub fn get_balancesheet(
        &self,
        name: &str,
        period: fundamentals::Period,
        until: OffsetDateTime,
        facts: &[fundamentals::BalanceSheetFact],
    ) -> Result<fundamentals::BalanceSheet, YahooError> {
        let url = fundamentals::compose_fundamentals_url(name, period.clone(), until, facts);
        let resp = self.send_request(&url)?;
        fundamentals::from_response(resp, period, facts)
    }

    pub fn get_cashflow(
        &self,
        name: &str,
        period: fundamentals::Period,
        until: OffsetDateTime,
        facts: &[fundamentals::CashflowFact],
    ) -> Result<fundamentals::Cashflow, YahooError> {
        let url = fundamentals::compose_fundamentals_url(name, period.clone(), until, facts);
        let resp = self.send_request(&url)?;
        fundamentals::from_response(resp, period, facts)
    }

    pub fn get_quote_summary(
        &self,
        name: &str,
        fields: &[quote_summary::QuoteSummaryField],
    ) -> Result<quote_summary::QuoteSummary, YahooError> {
        let url = quote_summary::compose_url(name, fields);
        let resp = self.send_request(&url)?;
        quote_summary::from_response(resp)
    }

    pub fn get_options(&self, name: &str) -> Result<options::Options, YahooError> {
        let url = options::compose_options_url(name);
        let resp = self.send_request(&url)?;
        options::options_from_response(resp)
    }

    pub fn get_option_chain(
        &self,
        name: &str,
        expiration_date: OffsetDateTime,
    ) -> Result<options::OptionChain, YahooError> {
        let url = options::compose_option_chain_url(name, expiration_date);
        let resp = self.send_request(&url)?;
        options::option_chain_from_response(resp)
    }

    /// Send request to yahoo! finance server and transform response to JSON value
    fn send_request(&self, url: &str) -> Result<serde_json::Value, YahooError> {
        let mut url = Url::parse(url)
            .map_err(|e| YahooError::FetchFailed(format!("failed to parse the URL: {}", e)))?;

        self.crumb.enrich(&mut url)?;

        let (status, body) = if log::log_enabled!(log::Level::Trace) {
            let resp = self.client.get(url.clone()).send()?;
            let status = resp.status();
            let body = resp.text()?;
            log::trace!(
                "Yahoo URL {} response status {}, body: {}",
                url,
                status,
                body
            );
            (status, body)
        } else {
            let resp = self.client.get(url).send()?;
            let status = resp.status();
            let body = resp.text()?;
            (status, body)
        };

        match status {
            StatusCode::OK => Ok(serde_json::from_str(&body)?),
            status => Err(YahooError::FetchFailed(format!(
                "status {}, response: {}",
                status, body
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::quote_summary::QuoteSummaryField;

    use super::*;
    use time::macros::datetime;

    #[test]
    fn test_get_single_quote() {
        let provider = YahooConnector::new();
        let response = provider.get_latest_quotes("HNL.DE", "1d").unwrap();
        assert_eq!(&response.chart.result[0].meta.symbol, "HNL.DE");
        assert_eq!(&response.chart.result[0].meta.range, "1mo");
        assert_eq!(&response.chart.result[0].meta.data_granularity, "1d");
        let _ = response.last_quote().unwrap();
    }

    #[test]
    fn test_strange_api_responses() {
        let provider = YahooConnector::new();
        let start = datetime!(2019-07-03 0:00:00.00 UTC);
        let end = datetime!(2020-07-04 23:59:59.99 UTC);
        let resp = provider.get_quote_history("IBM", start, end).unwrap();

        assert_eq!(&resp.chart.result[0].meta.symbol, "IBM");
        assert_eq!(&resp.chart.result[0].meta.data_granularity, "1d");
        assert_eq!(
            &resp.chart.result[0].meta.first_trade_date,
            &Some(-252322200)
        );

        let _ = resp.last_quote().unwrap();
    }

    #[test]
    #[should_panic(expected = "DeserializeFailed")]
    fn test_api_responses_missing_fields() {
        let provider = YahooConnector::new();
        let response = provider.get_latest_quotes("BF.B", "1m").unwrap();

        assert_eq!(&response.chart.result[0].meta.symbol, "BF.B");
        assert_eq!(&response.chart.result[0].meta.range, "1d");
        assert_eq!(&response.chart.result[0].meta.data_granularity, "1m");
        let _ = response.last_quote().unwrap();
    }

    #[test]
    fn test_get_quote_history() {
        let provider = YahooConnector::new();

        let start = datetime!(2020-01-01 0:00:00.00 UTC);
        let end = datetime!(2020-01-31 23:59:59.99 UTC);

        let resp = provider.get_quote_history("AAPL", start, end);
        if resp.is_ok() {
            let resp = resp.unwrap();
            assert_eq!(resp.chart.result[0].timestamp.len(), 21);
            let quotes = resp.quotes().unwrap();
            assert_eq!(quotes.len(), 21);
        }
    }

    #[test]
    fn test_get_quote_range() {
        let provider = YahooConnector::new();
        let response = provider.get_quote_range("HNL.DE", "1d", "1mo").unwrap();
        assert_eq!(&response.chart.result[0].meta.symbol, "HNL.DE");
        assert_eq!(&response.chart.result[0].meta.range, "1mo");
        assert_eq!(&response.chart.result[0].meta.data_granularity, "1d");
        let _ = response.last_quote().unwrap();
    }

    #[test]
    fn test_get_metadata() {
        let provider = YahooConnector::new();
        let response = provider.get_quote_range("HNL.DE", "1d", "1mo").unwrap();
        let metadata = response.metadata().unwrap();
        assert_eq!(metadata.symbol, "HNL.DE");
    }

    #[test]
    fn test_get() {
        let provider = YahooConnector::new();

        let start = datetime!(2019-01-01 0:00:00.00 UTC);
        let end = datetime!(2020-01-31 23:59:59.99 UTC);

        let response = provider
            .get_quote_history_interval("AAPL", start, end, "1mo")
            .unwrap();
        assert_eq!(&response.chart.result[0].timestamp.len(), &13);
        assert_eq!(&response.chart.result[0].meta.data_granularity, "1mo");
        let quotes = response.quotes().unwrap();
        assert_eq!(quotes.len(), 13usize);
    }

    #[test]
    fn test_large_volume() {
        let provider = YahooConnector::new();
        let response = provider.get_quote_range("BTC-USD", "1d", "5d").unwrap();
        let quotes = response.quotes().unwrap();
        assert!(quotes.len() > 0usize);
    }

    #[test]
    fn test_search_ticker() {
        let provider = YahooConnector::new();
        let resp = provider.search_ticker("Apple").unwrap();

        assert_eq!(resp.count, 15);
        let mut apple_found = false;
        for item in resp.quotes {
            if item.exchange == "NMS" && item.symbol == "AAPL" && item.short_name == "Apple Inc." {
                apple_found = true;
                break;
            }
        }
        assert!(apple_found)
    }

    #[test]
    fn test_mutual_fund_history() {
        let provider = YahooConnector::new();

        let start = datetime!(2020-01-01 0:00:00.00 UTC);
        let end = datetime!(2020-01-31 23:59:59.99 UTC);

        let resp = provider.get_quote_history("VTSAX", start, end);
        if resp.is_ok() {
            let resp = resp.unwrap();
            assert_eq!(resp.chart.result[0].timestamp.len(), 21);
            let quotes = resp.quotes().unwrap();
            assert_eq!(quotes.len(), 21);
        }
    }

    #[test]
    fn test_mutual_fund_latest() {
        let provider = YahooConnector::new();
        let response = provider.get_latest_quotes("VTSAX", "1d").unwrap();

        assert_eq!(&response.chart.result[0].meta.symbol, "VTSAX");
        assert_eq!(&response.chart.result[0].meta.range, "1mo");
        assert_eq!(&response.chart.result[0].meta.data_granularity, "1d");
        let _ = response.last_quote().unwrap();
    }

    #[test]
    fn test_mutual_fund_range() {
        let provider = YahooConnector::new();
        let response = provider.get_quote_range("VTSAX", "1d", "1mo").unwrap();
        assert_eq!(&response.chart.result[0].meta.symbol, "VTSAX");
        assert_eq!(&response.chart.result[0].meta.range, "1mo");
        assert_eq!(&response.chart.result[0].meta.data_granularity, "1d");
    }

    #[test]
    fn test_mutual_fund_capital_gains() {
        let provider = YahooConnector::new();
        let response = provider.get_quote_range("AMAGX", "1d", "5y").unwrap();

        assert_eq!(&response.chart.result[0].meta.symbol, "AMAGX");
        assert_eq!(&response.chart.result[0].meta.range, "5y");
        assert_eq!(&response.chart.result[0].meta.data_granularity, "1d");
        let capital_gains = response.capital_gains().unwrap();
        assert!(capital_gains.len() > 0usize);
    }

    #[test]
    fn test_quote_summary() {
        let provider = YahooConnector::new();
        let response = provider
            .get_quote_summary(
                "AAPL",
                &[
                    QuoteSummaryField::DefaultKeyStatistics,
                    QuoteSummaryField::FinancialData,
                    QuoteSummaryField::QuoteType,
                    QuoteSummaryField::SummaryDetail,
                    QuoteSummaryField::Earnings,
                    QuoteSummaryField::EarningsHistory,
                    QuoteSummaryField::EarningsTrend,
                    QuoteSummaryField::Price,
                ],
            )
            .unwrap();

        assert_eq!(response.asset_profile.is_none(), true);
        assert_eq!(response.default_key_statistics.is_some(), true);
        assert_eq!(response.financial_data.is_some(), true);
        assert_eq!(response.quote_type.is_some(), true);
        assert_eq!(response.summary_detail.is_some(), true);
        assert_eq!(response.earnings.is_some(), true);
        assert_eq!(response.earnings_history.is_some(), true);
        assert_eq!(response.earnings_trend.is_some(), true);
        assert_eq!(response.price.is_some(), true);
    }
}
