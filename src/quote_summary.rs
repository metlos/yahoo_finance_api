use serde::Deserialize;
use serde_json::Value;

use crate::YahooError;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuoteSummary {
    // this is by no means complete.
    // See https://github.com/gadicc/node-yahoo-finance2/blob/devel/src/modules/quoteSummary-iface.ts for a much more complete
    // list.
    pub asset_profile: Option<AssetProfile>,
    pub default_key_statistics: Option<DefaultKeyStatistics>,
    pub financial_data: Option<FinancialData>,
    pub quote_type: Option<QuoteType>,
    pub summary_detail: Option<SummaryDetail>,
    pub earnings: Option<Earnings>,
    pub earnings_history: Option<EarningsHistory>,
    pub earnings_trend: Option<EarningsTrend>,
    pub price: Option<Price>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum QuoteSummaryField {
    AssetProfile,
    DefaultKeyStatistics,
    FinancialData,
    QuoteType,
    SummaryDetail,
    Earnings,
    EarningsHistory,
    EarningsTrend,
    Price,
}

impl QuoteSummaryField {
    fn as_str(&self) -> &str {
        match self {
            QuoteSummaryField::AssetProfile => "assetProfile",
            QuoteSummaryField::DefaultKeyStatistics => "defaultKeyStatistics",
            QuoteSummaryField::FinancialData => "financialData",
            QuoteSummaryField::QuoteType => "quoteType",
            QuoteSummaryField::SummaryDetail => "summaryDetail",
            QuoteSummaryField::Earnings => "earnings",
            QuoteSummaryField::EarningsHistory => "earningsHistory",
            QuoteSummaryField::EarningsTrend => "earningsTrend",
            QuoteSummaryField::Price => "price",
        }
    }
}
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetProfile {
    // eh, there's just too much of this...
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DefaultKeyStatistics {
    #[serde(rename = "52WeekChange")]
    pub fifty_two_week_change: Option<f64>,
    #[serde(rename = "SandP52WeekChange")]
    pub s_and_p_52_week_change: Option<f64>,
    pub beta: Option<f64>,
    pub book_value: Option<f64>,
    pub category: Option<String>,
    pub date_short_interest: Option<usize>,
    pub earnings_quarterly_growth: Option<f64>,
    pub enterprise_to_ebitda: Option<f64>,
    pub enterprise_to_revenue: Option<f64>,
    pub enterprise_value: Option<f64>,
    pub float_shares: Option<f64>,
    pub forward_eps: Option<f64>,
    #[serde(rename = "forwardPE")]
    pub forward_pe: Option<f64>,
    pub fund_family: Option<String>,
    pub held_percent_insiders: Option<f64>,
    pub implied_shares_outstanding: Option<usize>,
    pub last_dividend_date: Option<usize>,
    pub last_dividend_value: Option<f64>,
    pub last_fiscal_year_end: Option<usize>,
    pub last_split_date: Option<usize>,
    pub last_split_factor: Option<String>,
    pub legal_type: Option<String>,
    pub max_age: Option<usize>,
    pub most_recent_quarter: Option<usize>,
    pub net_income_to_common: Option<f64>,
    pub next_fiscal_year_end: Option<usize>,
    pub peg_ratio: Option<f64>,
    pub price_hint: Option<f64>,
    pub price_to_book: Option<f64>,
    pub profit_margins: Option<f64>,
    pub shares_outstanding: Option<usize>,
    pub shares_percent_shares_out: Option<f64>,
    pub shares_short: Option<usize>,
    pub shares_short_previous_month_date: Option<usize>,
    pub shares_short_prior_month: Option<usize>,
    pub shares_percent_of_float: Option<f64>,
    pub short_ratio: Option<f64>,
    pub trailing_eps: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct FinancialData {
    pub current_price: Option<f64>,
    pub current_ratio: Option<f64>,
    pub debt_to_equity: Option<f64>,
    pub earnings_growth: Option<f64>,
    pub ebitda: Option<f64>,
    pub ebitda_margins: Option<f64>,
    pub financial_currency: Option<String>,
    pub free_cash_flow: Option<f64>,
    pub gross_margins: Option<f64>,
    pub max_age: Option<usize>,
    pub number_of_analyst_opinions: Option<usize>,
    pub operating_cash_flow: Option<f64>,
    pub operating_margins: Option<f64>,
    pub profit_margins: Option<f64>,
    pub quick_ratio: Option<f64>,
    pub recommendation_key: Option<String>,
    pub recommendation_mean: Option<f64>,
    pub return_on_assets: Option<f64>,
    pub return_on_equity: Option<f64>,
    pub revenue_growth: Option<f64>,
    pub revenue_per_share: Option<f64>,
    pub target_high_price: Option<f64>,
    pub target_low_price: Option<f64>,
    pub target_mean_price: Option<f64>,
    pub target_median_price: Option<f64>,
    pub total_cash: Option<f64>,
    pub total_cash_per_share: Option<f64>,
    pub total_debt: Option<f64>,
    pub total_revenue: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuoteType {
    pub exchange: Option<String>,
    pub first_tradwe_date_epoch_utc: Option<usize>,
    pub gmt_off_set_milliseconds: Option<isize>,
    pub long_name: Option<String>,
    pub max_age: Option<usize>,
    pub message_board_id: Option<String>,
    pub quote_type: Option<String>,
    pub short_name: Option<String>,
    pub symbol: Option<String>,
    pub time_zone_full_name: Option<String>,
    pub time_zone_short_name: Option<String>,
    pub underlying_symbol: Option<String>,
    pub uuid: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SummaryDetail {
    pub algorithm: Option<String>,
    pub ask: Option<f64>,
    pub ask_size: Option<f64>,
    pub average_daily_volume_10_day: Option<f64>,
    pub average_volume: Option<f64>,
    pub average_volume_10_days: Option<f64>,
    pub beta: Option<f64>,
    pub bid: Option<f64>,
    pub bid_size: Option<f64>,
    pub coin_market_cap_link: Option<String>,
    pub currency: Option<String>,
    pub day_high: Option<f64>,
    pub day_low: Option<f64>,
    pub dividend_rate: Option<f64>,
    pub dividend_yield: Option<f64>,
    pub ex_dividend_date: Option<usize>,
    pub fifty_day_average: Option<f64>,
    pub fifty_two_week_high: Option<f64>,
    pub fifty_two_week_low: Option<f64>,
    pub five_year_avg_dividend_yield: Option<f64>,
    #[serde(rename = "forwardPE")]
    pub forward_pe: Option<f64>,
    pub from_currency: Option<String>,
    pub last_market: Option<String>,
    pub market_cap: Option<f64>,
    pub max_age: Option<usize>,
    pub open: Option<f64>,
    pub payout_ratio: Option<f64>,
    pub previous_close: Option<f64>,
    pub price_hint: Option<f64>,
    #[serde(rename = "priceToSalesTrailing12Months")]
    pub price_to_sales_ttm: Option<f64>,
    pub regular_market_day_high: Option<f64>,
    pub regular_market_day_low: Option<f64>,
    pub regular_market_open: Option<f64>,
    pub regular_market_previous_close: Option<f64>,
    pub regular_market_volume: Option<f64>,
    pub to_currency: Option<String>,
    pub tradeable: Option<bool>,
    pub trailing_annual_dividend_rate: Option<f64>,
    pub trailing_annual_dividend_yield: Option<f64>,
    #[serde(rename = "trailingPE")]
    pub trailing_pe: Option<f64>,
    pub two_hundred_day_average: Option<f64>,
    pub volume: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Earnings {
    pub max_age: Option<usize>,
    pub earnings_chart: Option<EarningsChart>,
    pub financial_chart: Option<FinancialChart>,
    pub financial_currency: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EarningsChart {
    pub current_quarter_estimate: Option<f64>,
    pub current_quarter_estimate_date: Option<String>,
    pub current_quarter_estimate_year: Option<usize>,
    pub earnings_date: Option<Vec<usize>>,
    pub quarterly: Option<Vec<EarningsChartQuarterly>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EarningsChartQuarterly {
    pub date: Option<String>,
    pub actual: Option<f64>,
    pub estimate: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FinancialChart {
    pub yearly: Option<Vec<FinancialChartYearly>>,
    pub quarterly: Option<Vec<FinancialChartQuarterly>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FinancialChartYearly {
    pub date: Option<usize>,
    pub revenue: Option<f64>,
    pub earnings: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FinancialChartQuarterly {
    pub date: Option<String>,
    pub revenue: Option<f64>,
    pub earnings: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EarningsHistory {
    pub history: Option<Vec<EarningsHistoryEntry>>,
    pub max_age: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EarningsHistoryEntry {
    pub max_age: Option<f64>,
    pub eps_actual: Option<Formatted<f64>>,
    pub eps_estimate: Option<Formatted<f64>>,
    pub eps_difference: Option<Formatted<f64>>,
    pub surprise_percent: Option<Formatted<f64>>,
    pub quarter: Option<Formatted<usize>>,
    pub period: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EarningsTrend {
    pub trend: Option<Vec<EarningsTrendEntry>>,
    pub max_age: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EarningsTrendEntry {
    pub max_age: Option<usize>,
    pub period: Option<String>,
    pub end_date: Option<String>,
    pub growth: Option<Formatted<f64>>,
    pub earning_estimate: Option<EarningsEstimate>,
    pub revenue_estimate: Option<RevenueEstimate>,
    pub eps_trend: Option<EpsTrend>,
    pub eps_revisions: Option<EpsRevisions>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EarningsEstimate {
    pub avg: Option<Formatted<f64>>,
    pub low: Option<Formatted<f64>>,
    pub high: Option<Formatted<f64>>,
    pub year_ago_eps: Option<Formatted<f64>>,
    pub number_of_analysts: Option<Formatted<usize>>,
    pub growth: Option<Formatted<f64>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EpsRevisions {
    pub up_last_7days: Option<Formatted<f64>>,
    pub up_last_30days: Option<Formatted<f64>>,
    pub down_last_30days: Option<Formatted<f64>>,
    pub down_last_90days: Option<Formatted<f64>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EpsTrend {
    pub current: Option<Formatted<f64>>,
    #[serde(rename = "7daysAgo")]
    pub seven_days_ago: Option<Formatted<f64>>,
    #[serde(rename = "30daysAgo")]
    pub thirty_days_ago: Option<Formatted<f64>>,
    #[serde(rename = "60daysAgo")]
    pub sixty_days_ago: Option<Formatted<f64>>,
    #[serde(rename = "90daysAgo")]
    pub ninety_days_ago: Option<Formatted<f64>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RevenueEstimate {
    pub avg: Option<Formatted<f64>>,
    pub low: Option<Formatted<f64>>,
    pub high: Option<Formatted<f64>>,
    pub number_of_analysts: Option<Formatted<f64>>,
    pub year_ago_revenue: Option<Formatted<f64>>,
    pub growth: Option<Formatted<f64>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub average_daily_volume_10day: Option<f64>,
    pub average_daily_volume_3month: Option<f64>,
    pub exchange: Option<String>,
    pub exchange_name: Option<String>,
    pub exchage_data_delayed_y: Option<usize>,
    pub max_age: Option<usize>,
    pub post_market_change_percent: Option<f64>,
    pub post_market_chage: Option<f64>,
    pub post_market_time: Option<usize>,
    pub post_market_price: Option<f64>,
    pub post_market_source: Option<String>,
    pub pre_market_change_percent: Option<f64>,
    pub pre_market_chage: Option<f64>,
    pub pre_market_time: Option<usize>,
    pub pre_market_price: Option<f64>,
    pub pre_market_source: Option<String>,
    pub price_hint: Option<f64>,
    pub regular_market_change_percent: Option<f64>,
    pub regular_market_change: Option<f64>,
    pub regular_market_time: Option<usize>,
    pub regular_market_price: Option<f64>,
    pub regular_market_day_high: Option<f64>,
    pub regular_market_day_low: Option<f64>,
    pub regular_market_volume: Option<f64>,
    pub regular_market_previous_close: Option<f64>,
    pub regular_market_source: Option<String>,
    pub regular_market_open: Option<f64>,
    pub quote_source_name: Option<String>,
    pub quote_type: Option<String>,
    pub symbol: Option<String>,
    pub underlying_symbol: Option<String>,
    pub short_name: Option<String>,
    pub long_name: Option<String>,
    pub last_market: Option<String>,
    pub market_state: Option<String>,
    pub market_cap: Option<f64>,
    pub currency: Option<String>,
    pub currency_symbol: Option<String>,
    pub from_currency: Option<String>,
    pub to_currency: Option<String>,
    pub volume_24_hr: Option<f64>,
    pub volume_all_currencies: Option<f64>,
    pub circulating_supply: Option<f64>,
    pub expire_date: Option<usize>,
    pub open_interest: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Formatted<T> {
    pub fmt: Option<String>,
    pub raw: Option<T>,
}

macro_rules! QUERY {
    () => {
        "https://query2.finance.yahoo.com/v10/finance/quoteSummary/{symbol}?modules={modules}&corsDomain=finance.yahoo.com&formatted=false&symbol={symbol}"
    };
}

pub(crate) fn compose_url(symbol: &str, fields: &[QuoteSummaryField]) -> String {
    let mut modules = String::new();
    if fields.len() > 0 {
        modules.push_str(fields[0].as_str());
        for i in 1..fields.len() {
            modules.push_str(",");
            modules.push_str(fields[i].as_str());
        }
    }

    format!(QUERY!(), symbol = symbol, modules = modules)
}

pub(crate) fn from_response(json: Value) -> Result<QuoteSummary, YahooError> {
    if let Some(Value::Array(results)) = json.get("quoteSummary").and_then(|v| v.get("result")) {
        if results.len() != 1 {
            return Err(YahooError::FetchFailed(format!(
                "Expecting exactly 1 result but got {}",
                results.len()
            )));
        }

        return serde_json::from_value(results.get(0).unwrap().clone())
            .map_err(|e| YahooError::DeserializeFailed(e));
    }
    Err(YahooError::FetchFailed(
        "quoteSummary.result not found in the response JSON".into(),
    ))
}

// Example output:
//
// {
//   "quoteSummary": {
//     "error": null,
//     "result": [
//       {
//         "defaultKeyStatistics": {
//           "52WeekChange": 0.2570964,
//           "SandP52WeekChange": 0.2644137,
//           "beta": 1.244,
//           "bookValue": 4.382,
//           "category": null,
//           "dateShortInterest": 1722384000,
//           "earningsQuarterlyGrowth": 0.079,
//           "enterpriseToEbitda": 26.205,
//           "enterpriseToRevenue": 8.956,
//           "enterpriseValue": 3453287923712,
//           "floatShares": 15179810381,
//           "forwardEps": 7.48,
//           "forwardPE": 30.01738,
//           "fundFamily": null,
//           "heldPercentInsiders": 0.02703,
//           "heldPercentInstitutions": 0.60853,
//           "impliedSharesOutstanding": 15410899968,
//           "lastDividendDate": 1723420800,
//           "lastDividendValue": 0.25,
//           "lastFiscalYearEnd": 1696032000,
//           "lastSplitDate": 1598832000,
//           "lastSplitFactor": "4:1",
//           "legalType": null,
//           "maxAge": 1,
//           "mostRecentQuarter": 1719619200,
//           "netIncomeToCommon": 101956001792,
//           "nextFiscalYearEnd": 1727654400,
//           "pegRatio": 3.02,
//           "priceHint": 2,
//           "priceToBook": 51.23916,
//           "profitMargins": 0.26441,
//           "sharesOutstanding": 15204100096,
//           "sharesPercentSharesOut": 0.0077,
//           "sharesShort": 117696224,
//           "sharesShortPreviousMonthDate": 1719532800,
//           "sharesShortPriorMonth": 132235437,
//           "shortPercentOfFloat": 0.0077,
//           "shortRatio": 2.25,
//           "trailingEps": 6.58
//         },
//         "earnings": {
//           "earningsChart": {
//             "currentQuarterEstimate": 1.59,
//             "currentQuarterEstimateDate": "3Q",
//             "currentQuarterEstimateYear": 2024,
//             "earningsDate": [
//               1730372340,
//               1730721600
//             ],
//             "isEarningsDateEstimate": true,
//             "quarterly": [
//               {
//                 "actual": 1.36,
//                 "date": "3Q2023",
//                 "estimate": 1.3
//               },
//               {
//                 "actual": 2.04,
//                 "date": "4Q2023",
//                 "estimate": 1.96
//               },
//               {
//                 "actual": 1.53,
//                 "date": "1Q2024",
//                 "estimate": 1.5
//               },
//               {
//                 "actual": 1.4,
//                 "date": "2Q2024",
//                 "estimate": 1.35
//               }
//             ]
//           },
//           "financialCurrency": "USD",
//           "financialsChart": {
//             "quarterly": [
//               {
//                 "date": "3Q2023",
//                 "earnings": 22956000000,
//                 "revenue": 89498000000
//               },
//               {
//                 "date": "4Q2023",
//                 "earnings": 33916000000,
//                 "revenue": 119575000000
//               },
//               {
//                 "date": "1Q2024",
//                 "earnings": 23636000000,
//                 "revenue": 90753000000
//               },
//               {
//                 "date": "2Q2024",
//                 "earnings": 21448000000,
//                 "revenue": 85777000000
//               }
//             ],
//             "yearly": [
//               {
//                 "date": 2020,
//                 "earnings": 57411000000,
//                 "revenue": 274515000000
//               },
//               {
//                 "date": 2021,
//                 "earnings": 94680000000,
//                 "revenue": 365817000000
//               },
//               {
//                 "date": 2022,
//                 "earnings": 99803000000,
//                 "revenue": 394328000000
//               },
//               {
//                 "date": 2023,
//                 "earnings": 96995000000,
//                 "revenue": 383285000000
//               }
//             ]
//           },
//           "maxAge": 86400
//         },
//         "earningsHistory": {
//           "history": [
//             {
//               "epsActual": {
//                 "fmt": "1.36",
//                 "raw": 1.36
//               },
//               "epsDifference": {
//                 "fmt": "0.06",
//                 "raw": 0.06
//               },
//               "epsEstimate": {
//                 "fmt": "1.3",
//                 "raw": 1.3
//               },
//               "maxAge": 1,
//               "period": "-4q",
//               "quarter": {
//                 "fmt": "2023-09-30",
//                 "raw": 1696032000
//               },
//               "surprisePercent": {
//                 "fmt": "4.60%",
//                 "raw": 0.046
//               }
//             },
//             {
//               "epsActual": {
//                 "fmt": "2.04",
//                 "raw": 2.04
//               },
//               "epsDifference": {
//                 "fmt": "0.08",
//                 "raw": 0.08
//               },
//               "epsEstimate": {
//                 "fmt": "1.96",
//                 "raw": 1.96
//               },
//               "maxAge": 1,
//               "period": "-3q",
//               "quarter": {
//                 "fmt": "2023-12-31",
//                 "raw": 1703980800
//               },
//               "surprisePercent": {
//                 "fmt": "4.10%",
//                 "raw": 0.040999997
//               }
//             },
//             {
//               "epsActual": {
//                 "fmt": "1.53",
//                 "raw": 1.53
//               },
//               "epsDifference": {
//                 "fmt": "0.03",
//                 "raw": 0.03
//               },
//               "epsEstimate": {
//                 "fmt": "1.5",
//                 "raw": 1.5
//               },
//               "maxAge": 1,
//               "period": "-2q",
//               "quarter": {
//                 "fmt": "2024-03-31",
//                 "raw": 1711843200
//               },
//               "surprisePercent": {
//                 "fmt": "2.00%",
//                 "raw": 0.02
//               }
//             },
//             {
//               "epsActual": {
//                 "fmt": "1.4",
//                 "raw": 1.4
//               },
//               "epsDifference": {
//                 "fmt": "0.05",
//                 "raw": 0.05
//               },
//               "epsEstimate": {
//                 "fmt": "1.35",
//                 "raw": 1.35
//               },
//               "maxAge": 1,
//               "period": "-1q",
//               "quarter": {
//                 "fmt": "2024-06-30",
//                 "raw": 1719705600
//               },
//               "surprisePercent": {
//                 "fmt": "3.70%",
//                 "raw": 0.037
//               }
//             }
//           ],
//           "maxAge": 86400
//         },
//         "earningsTrend": {
//           "maxAge": 1,
//           "trend": [
//             {
//               "earningsEstimate": {
//                 "avg": {
//                   "fmt": "1.59",
//                   "raw": 1.59
//                 },
//                 "growth": {
//                   "fmt": "16.90%",
//                   "raw": 0.169
//                 },
//                 "high": {
//                   "fmt": "1.63",
//                   "raw": 1.63
//                 },
//                 "low": {
//                   "fmt": "1.53",
//                   "raw": 1.53
//                 },
//                 "numberOfAnalysts": {
//                   "fmt": "26",
//                   "longFmt": "26",
//                   "raw": 26
//                 },
//                 "yearAgoEps": {
//                   "fmt": "1.36",
//                   "raw": 1.36
//                 }
//               },
//               "endDate": "2024-09-30",
//               "epsRevisions": {
//                 "downLast30days": {
//                   "fmt": null,
//                   "longFmt": "0",
//                   "raw": 0
//                 },
//                 "downLast90days": {},
//                 "upLast30days": {
//                   "fmt": "16",
//                   "longFmt": "16",
//                   "raw": 16
//                 },
//                 "upLast7days": {
//                   "fmt": null,
//                   "longFmt": "0",
//                   "raw": 0
//                 }
//               },
//               "epsTrend": {
//                 "30daysAgo": {
//                   "fmt": "1.55",
//                   "raw": 1.55
//                 },
//                 "60daysAgo": {
//                   "fmt": "1.43",
//                   "raw": 1.43
//                 },
//                 "7daysAgo": {
//                   "fmt": "1.59",
//                   "raw": 1.59
//                 },
//                 "90daysAgo": {
//                   "fmt": "1.53",
//                   "raw": 1.53
//                 },
//                 "current": {
//                   "fmt": "1.59",
//                   "raw": 1.59
//                 }
//               },
//               "growth": {
//                 "fmt": "16.90%",
//                 "raw": 0.169
//               },
//               "maxAge": 1,
//               "period": "0q",
//               "revenueEstimate": {
//                 "avg": {
//                   "fmt": "94.3B",
//                   "longFmt": "94,297,700,000",
//                   "raw": 94297700000
//                 },
//                 "growth": {
//                   "fmt": "12.80%",
//                   "raw": 0.128
//                 },
//                 "high": {
//                   "fmt": "95.67B",
//                   "longFmt": "95,671,000,000",
//                   "raw": 95671000000
//                 },
//                 "low": {
//                   "fmt": "93.65B",
//                   "longFmt": "93,653,000,000",
//                   "raw": 93653000000
//                 },
//                 "numberOfAnalysts": {
//                   "fmt": "25",
//                   "longFmt": "25",
//                   "raw": 25
//                 },
//                 "yearAgoRevenue": {
//                   "fmt": "83.6B",
//                   "longFmt": "83,600,100,000",
//                   "raw": 83600100000
//                 }
//               }
//             },
//             {
//               "earningsEstimate": {
//                 "avg": {
//                   "fmt": "2.4",
//                   "raw": 2.4
//                 },
//                 "growth": {
//                   "fmt": "17.60%",
//                   "raw": 0.176
//                 },
//                 "high": {
//                   "fmt": "2.61",
//                   "raw": 2.61
//                 },
//                 "low": {
//                   "fmt": "2.21",
//                   "raw": 2.21
//                 },
//                 "numberOfAnalysts": {
//                   "fmt": "20",
//                   "longFmt": "20",
//                   "raw": 20
//                 },
//                 "yearAgoEps": {
//                   "fmt": "2.04",
//                   "raw": 2.04
//                 }
//               },
//               "endDate": "2024-12-31",
//               "epsRevisions": {
//                 "downLast30days": {
//                   "fmt": null,
//                   "longFmt": "0",
//                   "raw": 0
//                 },
//                 "downLast90days": {},
//                 "upLast30days": {
//                   "fmt": "8",
//                   "longFmt": "8",
//                   "raw": 8
//                 },
//                 "upLast7days": {
//                   "fmt": null,
//                   "longFmt": "0",
//                   "raw": 0
//                 }
//               },
//               "epsTrend": {
//                 "30daysAgo": {
//                   "fmt": "2.37",
//                   "raw": 2.37
//                 },
//                 "60daysAgo": {
//                   "fmt": "2.18",
//                   "raw": 2.18
//                 },
//                 "7daysAgo": {
//                   "fmt": "2.4",
//                   "raw": 2.4
//                 },
//                 "90daysAgo": {
//                   "fmt": "2.3",
//                   "raw": 2.3
//                 },
//                 "current": {
//                   "fmt": "2.4",
//                   "raw": 2.4
//                 }
//               },
//               "growth": {
//                 "fmt": "17.60%",
//                 "raw": 0.176
//               },
//               "maxAge": 1,
//               "period": "+1q",
//               "revenueEstimate": {
//                 "avg": {
//                   "fmt": "128.63B",
//                   "longFmt": "128,626,000,000",
//                   "raw": 128626000000
//                 },
//                 "growth": {
//                   "fmt": "15.20%",
//                   "raw": 0.152
//                 },
//                 "high": {
//                   "fmt": "136.39B",
//                   "longFmt": "136,387,000,000",
//                   "raw": 136387000000
//                 },
//                 "low": {
//                   "fmt": "123.88B",
//                   "longFmt": "123,883,000,000",
//                   "raw": 123883000000
//                 },
//                 "numberOfAnalysts": {
//                   "fmt": "19",
//                   "longFmt": "19",
//                   "raw": 19
//                 },
//                 "yearAgoRevenue": {
//                   "fmt": "111.69B",
//                   "longFmt": "111,695,000,000",
//                   "raw": 111695000000
//                 }
//               }
//             },
//             {
//               "earningsEstimate": {
//                 "avg": {
//                   "fmt": "6.7",
//                   "raw": 6.7
//                 },
//                 "growth": {
//                   "fmt": "16.90%",
//                   "raw": 0.169
//                 },
//                 "high": {
//                   "fmt": "6.75",
//                   "raw": 6.75
//                 },
//                 "low": {
//                   "fmt": "6.57",
//                   "raw": 6.57
//                 },
//                 "numberOfAnalysts": {
//                   "fmt": "40",
//                   "longFmt": "40",
//                   "raw": 40
//                 },
//                 "yearAgoEps": {
//                   "fmt": "5.73",
//                   "raw": 5.73
//                 }
//               },
//               "endDate": "2024-09-30",
//               "epsRevisions": {
//                 "downLast30days": {
//                   "fmt": null,
//                   "longFmt": "0",
//                   "raw": 0
//                 },
//                 "downLast90days": {},
//                 "upLast30days": {
//                   "fmt": "34",
//                   "longFmt": "34",
//                   "raw": 34
//                 },
//                 "upLast7days": {
//                   "fmt": null,
//                   "longFmt": "0",
//                   "raw": 0
//                 }
//               },
//               "epsTrend": {
//                 "30daysAgo": {
//                   "fmt": "6.61",
//                   "raw": 6.61
//                 },
//                 "60daysAgo": {
//                   "fmt": "6.16",
//                   "raw": 6.16
//                 },
//                 "7daysAgo": {
//                   "fmt": "6.7",
//                   "raw": 6.7
//                 },
//                 "90daysAgo": {
//                   "fmt": "6.59",
//                   "raw": 6.59
//                 },
//                 "current": {
//                   "fmt": "6.7",
//                   "raw": 6.7
//                 }
//               },
//               "growth": {
//                 "fmt": "16.90%",
//                 "raw": 0.169
//               },
//               "maxAge": 1,
//               "period": "0y",
//               "revenueEstimate": {
//                 "avg": {
//                   "fmt": "390.21B",
//                   "longFmt": "390,213,000,000",
//                   "raw": 390213000000
//                 },
//                 "growth": {
//                   "fmt": "9.00%",
//                   "raw": 0.09
//                 },
//                 "high": {
//                   "fmt": "391.78B",
//                   "longFmt": "391,776,000,000",
//                   "raw": 391776000000
//                 },
//                 "low": {
//                   "fmt": "387.12B",
//                   "longFmt": "387,118,000,000",
//                   "raw": 387118000000
//                 },
//                 "numberOfAnalysts": {
//                   "fmt": "38",
//                   "longFmt": "38",
//                   "raw": 38
//                 },
//                 "yearAgoRevenue": {
//                   "fmt": "358.03B",
//                   "longFmt": "358,027,000,000",
//                   "raw": 358027000000
//                 }
//               }
//             },
//             {
//               "earningsEstimate": {
//                 "avg": {
//                   "fmt": "7.48",
//                   "raw": 7.48
//                 },
//                 "growth": {
//                   "fmt": "11.60%",
//                   "raw": 0.116000004
//                 },
//                 "high": {
//                   "fmt": "8",
//                   "raw": 8.0
//                 },
//                 "low": {
//                   "fmt": "6.88",
//                   "raw": 6.88
//                 },
//                 "numberOfAnalysts": {
//                   "fmt": "40",
//                   "longFmt": "40",
//                   "raw": 40
//                 },
//                 "yearAgoEps": {
//                   "fmt": "6.7",
//                   "raw": 6.7
//                 }
//               },
//               "endDate": "2025-09-30",
//               "epsRevisions": {
//                 "downLast30days": {
//                   "fmt": null,
//                   "longFmt": "0",
//                   "raw": 0
//                 },
//                 "downLast90days": {},
//                 "upLast30days": {
//                   "fmt": "31",
//                   "longFmt": "31",
//                   "raw": 31
//                 },
//                 "upLast7days": {
//                   "fmt": null,
//                   "longFmt": "0",
//                   "raw": 0
//                 }
//               },
//               "epsTrend": {
//                 "30daysAgo": {
//                   "fmt": "7.32",
//                   "raw": 7.32
//                 },
//                 "60daysAgo": {
//                   "fmt": "6.8",
//                   "raw": 6.8
//                 },
//                 "7daysAgo": {
//                   "fmt": "7.48",
//                   "raw": 7.48
//                 },
//                 "90daysAgo": {
//                   "fmt": "7.23",
//                   "raw": 7.23
//                 },
//                 "current": {
//                   "fmt": "7.48",
//                   "raw": 7.48
//                 }
//               },
//               "growth": {
//                 "fmt": "11.60%",
//                 "raw": 0.116000004
//               },
//               "maxAge": 1,
//               "period": "+1y",
//               "revenueEstimate": {
//                 "avg": {
//                   "fmt": "421.63B",
//                   "longFmt": "421,632,000,000",
//                   "raw": 421632000000
//                 },
//                 "growth": {
//                   "fmt": "8.10%",
//                   "raw": 0.081
//                 },
//                 "high": {
//                   "fmt": "441.88B",
//                   "longFmt": "441,882,000,000",
//                   "raw": 441882000000
//                 },
//                 "low": {
//                   "fmt": "401.69B",
//                   "longFmt": "401,691,000,000",
//                   "raw": 401691000000
//                 },
//                 "numberOfAnalysts": {
//                   "fmt": "38",
//                   "longFmt": "38",
//                   "raw": 38
//                 },
//                 "yearAgoRevenue": {
//                   "fmt": "390.21B",
//                   "longFmt": "390,213,000,000",
//                   "raw": 390213000000
//                 }
//               }
//             },
//             {
//               "earningsEstimate": {
//                 "avg": {},
//                 "growth": {},
//                 "high": {},
//                 "low": {},
//                 "numberOfAnalysts": {},
//                 "yearAgoEps": {}
//               },
//               "endDate": null,
//               "epsRevisions": {
//                 "downLast30days": {},
//                 "downLast90days": {},
//                 "upLast30days": {},
//                 "upLast7days": {}
//               },
//               "epsTrend": {
//                 "30daysAgo": {},
//                 "60daysAgo": {},
//                 "7daysAgo": {},
//                 "90daysAgo": {},
//                 "current": {}
//               },
//               "growth": {
//                 "fmt": "11.10%",
//                 "raw": 0.111
//               },
//               "maxAge": 1,
//               "period": "+5y",
//               "revenueEstimate": {
//                 "avg": {},
//                 "growth": {},
//                 "high": {},
//                 "low": {},
//                 "numberOfAnalysts": {},
//                 "yearAgoRevenue": {}
//               }
//             },
//             {
//               "earningsEstimate": {
//                 "avg": {},
//                 "growth": {},
//                 "high": {},
//                 "low": {},
//                 "numberOfAnalysts": {},
//                 "yearAgoEps": {}
//               },
//               "endDate": null,
//               "epsRevisions": {
//                 "downLast30days": {},
//                 "downLast90days": {},
//                 "upLast30days": {},
//                 "upLast7days": {}
//               },
//               "epsTrend": {
//                 "30daysAgo": {},
//                 "60daysAgo": {},
//                 "7daysAgo": {},
//                 "90daysAgo": {},
//                 "current": {}
//               },
//               "growth": {
//                 "fmt": "20.45%",
//                 "raw": 0.20446
//               },
//               "maxAge": 1,
//               "period": "-5y",
//               "revenueEstimate": {
//                 "avg": {},
//                 "growth": {},
//                 "high": {},
//                 "low": {},
//                 "numberOfAnalysts": {},
//                 "yearAgoRevenue": {}
//               }
//             }
//           ]
//         },
//         "financialData": {
//           "currentPrice": 224.53,
//           "currentRatio": 0.953,
//           "debtToEquity": 151.862,
//           "earningsGrowth": 0.111,
//           "ebitda": 131781001216,
//           "ebitdaMargins": 0.34175,
//           "financialCurrency": "USD",
//           "freeCashflow": 86158123008,
//           "grossMargins": 0.45962003,
//           "maxAge": 86400,
//           "numberOfAnalystOpinions": 39,
//           "operatingCashflow": 113040998400,
//           "operatingMargins": 0.29556,
//           "profitMargins": 0.26441,
//           "quickRatio": 0.798,
//           "recommendationKey": "buy",
//           "recommendationMean": 2.0,
//           "returnOnAssets": 0.22612,
//           "returnOnEquity": 1.60583,
//           "revenueGrowth": 0.049,
//           "revenuePerShare": 24.957,
//           "targetHighPrice": 300.0,
//           "targetLowPrice": 183.86,
//           "targetMeanPrice": 235.58,
//           "targetMedianPrice": 240.0,
//           "totalCash": 61801000960,
//           "totalCashPerShare": 4.065,
//           "totalDebt": 101304000512,
//           "totalRevenue": 385603010560
//         },
//         "price": {
//           "averageDailyVolume10Day": 40651800,
//           "averageDailyVolume3Month": 64811409,
//           "currency": "USD",
//           "currencySymbol": "$",
//           "exchange": "NMS",
//           "exchangeDataDelayedBy": 0,
//           "exchangeName": "NasdaqGS",
//           "fromCurrency": null,
//           "lastMarket": null,
//           "longName": "Apple Inc.",
//           "marketCap": 3413776531456,
//           "marketState": "PRE",
//           "maxAge": 1,
//           "postMarketChange": 0.570206,
//           "postMarketChangePercent": 0.00253955,
//           "postMarketPrice": 225.1,
//           "postMarketSource": "FREE_REALTIME",
//           "postMarketTime": 1724371198,
//           "preMarketChange": 1.5,
//           "preMarketChangePercent": 0.0066806213,
//           "preMarketPrice": 226.03,
//           "preMarketSource": "FREE_REALTIME",
//           "preMarketTime": 1724411630,
//           "priceHint": 2,
//           "quoteSourceName": "Nasdaq Real Time Price",
//           "quoteType": "EQUITY",
//           "regularMarketChange": -1.8699951,
//           "regularMarketChangePercent": -0.008259696,
//           "regularMarketDayHigh": 228.34,
//           "regularMarketDayLow": 223.9,
//           "regularMarketOpen": 227.6,
//           "regularMarketPreviousClose": 226.4,
//           "regularMarketPrice": 224.53,
//           "regularMarketSource": "FREE_REALTIME",
//           "regularMarketTime": 1724356801,
//           "regularMarketVolume": 43434198,
//           "shortName": "Apple Inc.",
//           "symbol": "AAPL",
//           "toCurrency": null,
//           "underlyingSymbol": null
//         },
//         "quoteType": {
//           "exchange": "NMS",
//           "firstTradeDateEpochUtc": 345479400,
//           "gmtOffSetMilliseconds": -14400000,
//           "longName": "Apple Inc.",
//           "maxAge": 1,
//           "messageBoardId": "finmb_24937",
//           "quoteType": "EQUITY",
//           "shortName": "Apple Inc.",
//           "symbol": "AAPL",
//           "timeZoneFullName": "America/New_York",
//           "timeZoneShortName": "EDT",
//           "underlyingSymbol": "AAPL",
//           "uuid": "8b10e4ae-9eeb-3684-921a-9ab27e4d87aa"
//         }
//       }
//     ]
//   }
// }
