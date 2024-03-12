use time::macros::datetime;

use yahoo_finance_api as yahoo;

#[cfg(not(feature = "blocking"))]
#[tokio::main]
async fn main() {
    use yahoo::fundamentals;

    let conn = yahoo::YahooConnector::new();
    let ticker = "OKE";
    let end = datetime!(2024-03-21 00:00:00.00 UTC);
    let hist = conn
        .get_income_statement(
            ticker,
            fundamentals::Period::Quarter,
            end,
            &[fundamentals::IncomeStatementFact::DilutedEPS],
        )
        .await
        .unwrap();

    println!("{}", ticker);
    println!("EPS");
    if let Some(data) = hist.get(&fundamentals::IncomeStatementFact::DilutedEPS) {
        for val in data {
            println!("{} | {:.2}", val.0, val.1);
        }
    }
}

#[cfg(feature = "blocking")]
fn main() {
    use yahoo::fundamentals;

    let conn = yahoo::YahooConnector::new();
    let ticker = "OKE";
    let end = datetime!(2024-03-21 00:00:00.00 UTC);
    let hist = conn
        .get_income_statement(
            ticker,
            fundamentals::Period::Quarter,
            end,
            &[fundamentals::IncomeStatementFact::DilutedEPS],
        )
        .unwrap();

    println!("{}", ticker);
    println!("EPS");
    if let Some(data) = hist.get(&fundamentals::IncomeStatementFact::DilutedEPS) {
        for val in data {
            println!("{} | {:.2}", val.0, val.1);
        }
    }
}
