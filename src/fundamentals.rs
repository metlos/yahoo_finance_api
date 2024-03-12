use std::collections::{BTreeMap, HashMap};

use serde_json::Value;
use time::macros::offset;

use super::*;

macro_rules! QUERY {
    () => {
        "{url}/{symbol}?symbol={symbol}&period1={start}&period2={end}&type={typ}"
    };
}

pub(crate) trait AsStr {
    fn as_str(&self) -> &str;
}

pub(crate) fn compose_fundamentals_url<T: AsStr>(
    symbol: &str,
    period: Period,
    until: time::OffsetDateTime,
    facts: &[T],
) -> String {
    // yahoo only returns 5 latest records at most 4 years back, so we can safely just
    // hardcode the start time to some time more in the past than that.
    let start = time::Date::from_ordinal_date(2010, 1)
        .map(|d| d.midnight().assume_offset(offset!(UTC)))
        .unwrap();

    let typ = facts
        .iter()
        .map(|f| format!("{}{}", period.to_yquery_identifier(), f.as_str()))
        .reduce(|mut acc, e| {
            acc.push(',');
            acc.push_str(&e);
            acc
        })
        .unwrap_or("".into());

    let url = format!(
        QUERY!(),
        url = YFUNDAMENTALS_URL,
        symbol = symbol,
        start = start.unix_timestamp(),
        end = until.unix_timestamp(),
        typ = typ,
    );
    url
}

#[derive(Debug, Clone)]
pub enum Period {
    Year,
    Quarter,
}

impl Period {
    fn to_yquery_identifier(&self) -> &'static str {
        match self {
            Period::Year => "annual",
            Period::Quarter => "quarterly",
        }
    }
}

type FundamentalsData<K> = HashMap<K, BTreeMap<time::Date, f64>>;
pub type IncomeStatement = FundamentalsData<IncomeStatementFact>;
pub type BalanceSheet = FundamentalsData<BalanceSheetFact>;
pub type Cashflow = FundamentalsData<CashflowFact>;

pub fn from_response<K>(
    resp: Value,
    requested_period: Period,
    requested_facts: &[K],
) -> Result<FundamentalsData<K>, YahooError>
where
    K: std::fmt::Debug + Clone + Eq + std::hash::Hash,
{
    if let Some(Value::Array(results)) = resp.get("timeseries").and_then(|v| v.get("result")) {
        let mut ret = HashMap::default();
        for res in results {
            let dates = extract_timestamps_from_response(res)?;
            for rf in requested_facts {
                let key = format!("{}{:?}", requested_period.to_yquery_identifier(), rf);
                if let Some(Value::Array(values)) = res.get(key) {
                    let data = ret.entry(rf.clone()).or_insert_with(BTreeMap::new);
                    for (idx, val) in values.iter().enumerate() {
                        if let Some(val) = val.get("reportedValue").and_then(|v| v.get("raw")) {
                            if let Some(val) = val.as_f64() {
                                data.insert(dates[idx], val);
                            }
                        }
                    }
                }
            }
        }
        return Ok(ret);
    }
    Err(YahooError::DataInconsistency)
}

fn extract_timestamps_from_response(result_entry: &Value) -> Result<Vec<time::Date>, YahooError> {
    let mut dates = vec![];
    if let Some(Value::Array(timestamps)) = result_entry.get("timestamp") {
        for ts in timestamps {
            match ts {
                Value::Number(ts) => match ts.as_i64() {
                    Some(ts) => match time::OffsetDateTime::from_unix_timestamp(ts) {
                        Ok(ts) => dates.push(ts.date()),
                        Err(_) => return Err(YahooError::DataInconsistency),
                    },
                    None => return Err(YahooError::DataInconsistency),
                },
                _ => return Err(YahooError::DataInconsistency),
            };
        }
    } else {
        return Err(YahooError::DataInconsistency);
    }
    Ok(dates)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IncomeStatementFact {
    TaxEffectOfUnusualItems,
    TaxRateForCalcs,
    NormalizedEBITDA,
    NormalizedDilutedEPS,
    NormalizedBasicEPS,
    TotalUnusualItems,
    TotalUnusualItemsExcludingGoodwill,
    NetIncomeFromContinuingOperationNetMinorityInterest,
    ReconciledDepreciation,
    ReconciledCostOfRevenue,
    #[allow(clippy::upper_case_acronyms)]
    EBITDA,
    #[allow(clippy::upper_case_acronyms)]
    EBIT,
    NetInterestIncome,
    InterestExpense,
    InterestIncome,
    ContinuingAndDiscontinuedDilutedEPS,
    ContinuingAndDiscontinuedBasicEPS,
    NormalizedIncome,
    NetIncomeFromContinuingAndDiscontinuedOperation,
    TotalExpenses,
    RentExpenseSupplemental,
    ReportedNormalizedDilutedEPS,
    ReportedNormalizedBasicEPS,
    TotalOperatingIncomeAsReported,
    DividendPerShare,
    DilutedAverageShares,
    BasicAverageShares,
    DilutedEPS,
    DilutedEPSOtherGainsLosses,
    TaxLossCarryforwardDilutedEPS,
    DilutedAccountingChange,
    DilutedExtraordinary,
    DilutedDiscontinuousOperations,
    DilutedContinuousOperations,
    BasicEPS,
    BasicEPSOtherGainsLosses,
    TaxLossCarryforwardBasicEPS,
    BasicAccountingChange,
    BasicExtraordinary,
    BasicDiscontinuousOperations,
    BasicContinuousOperations,
    DilutedNIAvailtoComStockholders,
    AverageDilutionEarnings,
    NetIncomeCommonStockholders,
    OtherunderPreferredStockDividend,
    PreferredStockDividends,
    NetIncome,
    MinorityInterests,
    NetIncomeIncludingNoncontrollingInterests,
    NetIncomeFromTaxLossCarryforward,
    NetIncomeExtraordinary,
    NetIncomeDiscontinuousOperations,
    NetIncomeContinuousOperations,
    EarningsFromEquityInterestNetOfTax,
    TaxProvision,
    PretaxIncome,
    OtherIncomeExpense,
    OtherNonOperatingIncomeExpenses,
    SpecialIncomeCharges,
    GainOnSaleOfPPE,
    GainOnSaleOfBusiness,
    OtherSpecialCharges,
    WriteOff,
    ImpairmentOfCapitalAssets,
    RestructuringAndMergernAcquisition,
    SecuritiesAmortization,
    EarningsFromEquityInterest,
    GainOnSaleOfSecurity,
    NetNonOperatingInterestIncomeExpense,
    TotalOtherFinanceCost,
    InterestExpenseNonOperating,
    InterestIncomeNonOperating,
    OperatingIncome,
    OperatingExpense,
    OtherOperatingExpenses,
    OtherTaxes,
    ProvisionForDoubtfulAccounts,
    DepreciationAmortizationDepletionIncomeStatement,
    DepletionIncomeStatement,
    DepreciationAndAmortizationInIncomeStatement,
    Amortization,
    AmortizationOfIntangiblesIncomeStatement,
    DepreciationIncomeStatement,
    ResearchAndDevelopment,
    SellingGeneralAndAdministration,
    SellingAndMarketingExpense,
    GeneralAndAdministrativeExpense,
    OtherGandA,
    InsuranceAndClaims,
    RentAndLandingFees,
    SalariesAndWages,
    GrossProfit,
    CostOfRevenue,
    TotalRevenue,
    ExciseTaxes,
    OperatingRevenue,
}

impl IncomeStatementFact {
    pub fn all() -> &'static [Self] {
        static ALL: &[IncomeStatementFact] = &[
            IncomeStatementFact::TaxEffectOfUnusualItems,
            IncomeStatementFact::TaxRateForCalcs,
            IncomeStatementFact::NormalizedEBITDA,
            IncomeStatementFact::NormalizedDilutedEPS,
            IncomeStatementFact::NormalizedBasicEPS,
            IncomeStatementFact::TotalUnusualItems,
            IncomeStatementFact::TotalUnusualItemsExcludingGoodwill,
            IncomeStatementFact::NetIncomeFromContinuingOperationNetMinorityInterest,
            IncomeStatementFact::ReconciledDepreciation,
            IncomeStatementFact::ReconciledCostOfRevenue,
            IncomeStatementFact::EBITDA,
            IncomeStatementFact::EBIT,
            IncomeStatementFact::NetInterestIncome,
            IncomeStatementFact::InterestExpense,
            IncomeStatementFact::InterestIncome,
            IncomeStatementFact::ContinuingAndDiscontinuedDilutedEPS,
            IncomeStatementFact::ContinuingAndDiscontinuedBasicEPS,
            IncomeStatementFact::NormalizedIncome,
            IncomeStatementFact::NetIncomeFromContinuingAndDiscontinuedOperation,
            IncomeStatementFact::TotalExpenses,
            IncomeStatementFact::RentExpenseSupplemental,
            IncomeStatementFact::ReportedNormalizedDilutedEPS,
            IncomeStatementFact::ReportedNormalizedBasicEPS,
            IncomeStatementFact::TotalOperatingIncomeAsReported,
            IncomeStatementFact::DividendPerShare,
            IncomeStatementFact::DilutedAverageShares,
            IncomeStatementFact::BasicAverageShares,
            IncomeStatementFact::DilutedEPS,
            IncomeStatementFact::DilutedEPSOtherGainsLosses,
            IncomeStatementFact::TaxLossCarryforwardDilutedEPS,
            IncomeStatementFact::DilutedAccountingChange,
            IncomeStatementFact::DilutedExtraordinary,
            IncomeStatementFact::DilutedDiscontinuousOperations,
            IncomeStatementFact::DilutedContinuousOperations,
            IncomeStatementFact::BasicEPS,
            IncomeStatementFact::BasicEPSOtherGainsLosses,
            IncomeStatementFact::TaxLossCarryforwardBasicEPS,
            IncomeStatementFact::BasicAccountingChange,
            IncomeStatementFact::BasicExtraordinary,
            IncomeStatementFact::BasicDiscontinuousOperations,
            IncomeStatementFact::BasicContinuousOperations,
            IncomeStatementFact::DilutedNIAvailtoComStockholders,
            IncomeStatementFact::AverageDilutionEarnings,
            IncomeStatementFact::NetIncomeCommonStockholders,
            IncomeStatementFact::OtherunderPreferredStockDividend,
            IncomeStatementFact::PreferredStockDividends,
            IncomeStatementFact::NetIncome,
            IncomeStatementFact::MinorityInterests,
            IncomeStatementFact::NetIncomeIncludingNoncontrollingInterests,
            IncomeStatementFact::NetIncomeFromTaxLossCarryforward,
            IncomeStatementFact::NetIncomeExtraordinary,
            IncomeStatementFact::NetIncomeDiscontinuousOperations,
            IncomeStatementFact::NetIncomeContinuousOperations,
            IncomeStatementFact::EarningsFromEquityInterestNetOfTax,
            IncomeStatementFact::TaxProvision,
            IncomeStatementFact::PretaxIncome,
            IncomeStatementFact::OtherIncomeExpense,
            IncomeStatementFact::OtherNonOperatingIncomeExpenses,
            IncomeStatementFact::SpecialIncomeCharges,
            IncomeStatementFact::GainOnSaleOfPPE,
            IncomeStatementFact::GainOnSaleOfBusiness,
            IncomeStatementFact::OtherSpecialCharges,
            IncomeStatementFact::WriteOff,
            IncomeStatementFact::ImpairmentOfCapitalAssets,
            IncomeStatementFact::RestructuringAndMergernAcquisition,
            IncomeStatementFact::SecuritiesAmortization,
            IncomeStatementFact::EarningsFromEquityInterest,
            IncomeStatementFact::GainOnSaleOfSecurity,
            IncomeStatementFact::NetNonOperatingInterestIncomeExpense,
            IncomeStatementFact::TotalOtherFinanceCost,
            IncomeStatementFact::InterestExpenseNonOperating,
            IncomeStatementFact::InterestIncomeNonOperating,
            IncomeStatementFact::OperatingIncome,
            IncomeStatementFact::OperatingExpense,
            IncomeStatementFact::OtherOperatingExpenses,
            IncomeStatementFact::OtherTaxes,
            IncomeStatementFact::ProvisionForDoubtfulAccounts,
            IncomeStatementFact::DepreciationAmortizationDepletionIncomeStatement,
            IncomeStatementFact::DepletionIncomeStatement,
            IncomeStatementFact::DepreciationAndAmortizationInIncomeStatement,
            IncomeStatementFact::Amortization,
            IncomeStatementFact::AmortizationOfIntangiblesIncomeStatement,
            IncomeStatementFact::DepreciationIncomeStatement,
            IncomeStatementFact::ResearchAndDevelopment,
            IncomeStatementFact::SellingGeneralAndAdministration,
            IncomeStatementFact::SellingAndMarketingExpense,
            IncomeStatementFact::GeneralAndAdministrativeExpense,
            IncomeStatementFact::OtherGandA,
            IncomeStatementFact::InsuranceAndClaims,
            IncomeStatementFact::RentAndLandingFees,
            IncomeStatementFact::SalariesAndWages,
            IncomeStatementFact::GrossProfit,
            IncomeStatementFact::CostOfRevenue,
            IncomeStatementFact::TotalRevenue,
            IncomeStatementFact::ExciseTaxes,
            IncomeStatementFact::OperatingRevenue,
        ];
        ALL
    }
}

impl AsStr for IncomeStatementFact {
    fn as_str(&self) -> &str {
        match self {
            Self::TaxEffectOfUnusualItems => "TaxEffectOfUnusualItems",
            Self::TaxRateForCalcs => "TaxRateForCalcs",
            Self::NormalizedEBITDA => "NormalizedEBITDA",
            Self::NormalizedDilutedEPS => "NormalizedDilutedEPS",
            Self::NormalizedBasicEPS => "NormalizedBasicEPS",
            Self::TotalUnusualItems => "TotalUnusualItems",
            Self::TotalUnusualItemsExcludingGoodwill => "TotalUnusualItemsExcludingGoodwill",
            Self::NetIncomeFromContinuingOperationNetMinorityInterest => {
                "NetIncomeFromContinuingOperationNetMinorityInterest"
            }
            Self::ReconciledDepreciation => "ReconciledDepreciation",
            Self::ReconciledCostOfRevenue => "ReconciledCostOfRevenue",
            Self::EBITDA => "EBITDA",
            Self::EBIT => "EBIT",
            Self::NetInterestIncome => "NetInterestIncome",
            Self::InterestExpense => "InterestExpense",
            Self::InterestIncome => "InterestIncome",
            Self::ContinuingAndDiscontinuedDilutedEPS => "ContinuingAndDiscontinuedDilutedEPS",
            Self::ContinuingAndDiscontinuedBasicEPS => "ContinuingAndDiscontinuedBasicEPS",
            Self::NormalizedIncome => "NormalizedIncome",
            Self::NetIncomeFromContinuingAndDiscontinuedOperation => {
                "NetIncomeFromContinuingAndDiscontinuedOperation"
            }
            Self::TotalExpenses => "TotalExpenses",
            Self::RentExpenseSupplemental => "RentExpenseSupplemental",
            Self::ReportedNormalizedDilutedEPS => "ReportedNormalizedDilutedEPS",
            Self::ReportedNormalizedBasicEPS => "ReportedNormalizedBasicEPS",
            Self::TotalOperatingIncomeAsReported => "TotalOperatingIncomeAsReported",
            Self::DividendPerShare => "DividendPerShare",
            Self::DilutedAverageShares => "DilutedAverageShares",
            Self::BasicAverageShares => "BasicAverageShares",
            Self::DilutedEPS => "DilutedEPS",
            Self::DilutedEPSOtherGainsLosses => "DilutedEPSOtherGainsLosses",
            Self::TaxLossCarryforwardDilutedEPS => "TaxLossCarryforwardDilutedEPS",
            Self::DilutedAccountingChange => "DilutedAccountingChange",
            Self::DilutedExtraordinary => "DilutedExtraordinary",
            Self::DilutedDiscontinuousOperations => "DilutedDiscontinuousOperations",
            Self::DilutedContinuousOperations => "DilutedContinuousOperations",
            Self::BasicEPS => "BasicEPS",
            Self::BasicEPSOtherGainsLosses => "BasicEPSOtherGainsLosses",
            Self::TaxLossCarryforwardBasicEPS => "TaxLossCarryforwardBasicEPS",
            Self::BasicAccountingChange => "BasicAccountingChange",
            Self::BasicExtraordinary => "BasicExtraordinary",
            Self::BasicDiscontinuousOperations => "BasicDiscontinuousOperations",
            Self::BasicContinuousOperations => "BasicContinuousOperations",
            Self::DilutedNIAvailtoComStockholders => "DilutedNIAvailtoComStockholders",
            Self::AverageDilutionEarnings => "AverageDilutionEarnings",
            Self::NetIncomeCommonStockholders => "NetIncomeCommonStockholders",
            Self::OtherunderPreferredStockDividend => "OtherunderPreferredStockDividend",
            Self::PreferredStockDividends => "PreferredStockDividends",
            Self::NetIncome => "NetIncome",
            Self::MinorityInterests => "MinorityInterests",
            Self::NetIncomeIncludingNoncontrollingInterests => {
                "NetIncomeIncludingNoncontrollingInterests"
            }
            Self::NetIncomeFromTaxLossCarryforward => "NetIncomeFromTaxLossCarryforward",
            Self::NetIncomeExtraordinary => "NetIncomeExtraordinary",
            Self::NetIncomeDiscontinuousOperations => "NetIncomeDiscontinuousOperations",
            Self::NetIncomeContinuousOperations => "NetIncomeContinuousOperations",
            Self::EarningsFromEquityInterestNetOfTax => "EarningsFromEquityInterestNetOfTax",
            Self::TaxProvision => "TaxProvision",
            Self::PretaxIncome => "PretaxIncome",
            Self::OtherIncomeExpense => "OtherIncomeExpense",
            Self::OtherNonOperatingIncomeExpenses => "OtherNonOperatingIncomeExpenses",
            Self::SpecialIncomeCharges => "SpecialIncomeCharges",
            Self::GainOnSaleOfPPE => "GainOnSaleOfPPE",
            Self::GainOnSaleOfBusiness => "GainOnSaleOfBusiness",
            Self::OtherSpecialCharges => "OtherSpecialCharges",
            Self::WriteOff => "WriteOff",
            Self::ImpairmentOfCapitalAssets => "ImpairmentOfCapitalAssets",
            Self::RestructuringAndMergernAcquisition => "RestructuringAndMergernAcquisition",
            Self::SecuritiesAmortization => "SecuritiesAmortization",
            Self::EarningsFromEquityInterest => "EarningsFromEquityInterest",
            Self::GainOnSaleOfSecurity => "GainOnSaleOfSecurity",
            Self::NetNonOperatingInterestIncomeExpense => "NetNonOperatingInterestIncomeExpense",
            Self::TotalOtherFinanceCost => "TotalOtherFinanceCost",
            Self::InterestExpenseNonOperating => "InterestExpenseNonOperating",
            Self::InterestIncomeNonOperating => "InterestIncomeNonOperating",
            Self::OperatingIncome => "OperatingIncome",
            Self::OperatingExpense => "OperatingExpense",
            Self::OtherOperatingExpenses => "OtherOperatingExpenses",
            Self::OtherTaxes => "OtherTaxes",
            Self::ProvisionForDoubtfulAccounts => "ProvisionForDoubtfulAccounts",
            Self::DepreciationAmortizationDepletionIncomeStatement => {
                "DepreciationAmortizationDepletionIncomeStatement"
            }
            Self::DepletionIncomeStatement => "DepletionIncomeStatement",
            Self::DepreciationAndAmortizationInIncomeStatement => {
                "DepreciationAndAmortizationInIncomeStatement"
            }
            Self::Amortization => "Amortization",
            Self::AmortizationOfIntangiblesIncomeStatement => {
                "AmortizationOfIntangiblesIncomeStatement"
            }
            Self::DepreciationIncomeStatement => "DepreciationIncomeStatement",
            Self::ResearchAndDevelopment => "ResearchAndDevelopment",
            Self::SellingGeneralAndAdministration => "SellingGeneralAndAdministration",
            Self::SellingAndMarketingExpense => "SellingAndMarketingExpense",
            Self::GeneralAndAdministrativeExpense => "GeneralAndAdministrativeExpense",
            Self::OtherGandA => "OtherGandA",
            Self::InsuranceAndClaims => "InsuranceAndClaims",
            Self::RentAndLandingFees => "RentAndLandingFees",
            Self::SalariesAndWages => "SalariesAndWages",
            Self::GrossProfit => "GrossProfit",
            Self::CostOfRevenue => "CostOfRevenue",
            Self::TotalRevenue => "TotalRevenue",
            Self::ExciseTaxes => "ExciseTaxes",
            Self::OperatingRevenue => "OperatingRevenue",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BalanceSheetFact {
    TreasurySharesNumber,
    PreferredSharesNumber,
    OrdinarySharesNumber,
    ShareIssued,
    NetDebt,
    TotalDebt,
    TangibleBookValue,
    InvestedCapital,
    WorkingCapital,
    NetTangibleAssets,
    CapitalLeaseObligations,
    CommonStockEquity,
    PreferredStockEquity,
    TotalCapitalization,
    TotalEquityGrossMinorityInterest,
    MinorityInterest,
    StockholdersEquity,
    OtherEquityInterest,
    GainsLossesNotAffectingRetainedEarnings,
    OtherEquityAdjustments,
    FixedAssetsRevaluationReserve,
    ForeignCurrencyTranslationAdjustments,
    MinimumPensionLiabilities,
    UnrealizedGainLoss,
    TreasuryStock,
    RetainedEarnings,
    AdditionalPaidInCapital,
    CapitalStock,
    OtherCapitalStock,
    CommonStock,
    PreferredStock,
    TotalPartnershipCapital,
    GeneralPartnershipCapital,
    LimitedPartnershipCapital,
    TotalLiabilitiesNetMinorityInterest,
    TotalNonCurrentLiabilitiesNetMinorityInterest,
    OtherNonCurrentLiabilities,
    LiabilitiesHeldforSaleNonCurrent,
    RestrictedCommonStock,
    PreferredSecuritiesOutsideStockEquity,
    DerivativeProductLiabilities,
    EmployeeBenefits,
    NonCurrentPensionAndOtherPostretirementBenefitPlans,
    NonCurrentAccruedExpenses,
    DuetoRelatedPartiesNonCurrent,
    TradeandOtherPayablesNonCurrent,
    NonCurrentDeferredLiabilities,
    NonCurrentDeferredRevenue,
    NonCurrentDeferredTaxesLiabilities,
    LongTermDebtAndCapitalLeaseObligation,
    LongTermCapitalLeaseObligation,
    LongTermDebt,
    LongTermProvisions,
    CurrentLiabilities,
    OtherCurrentLiabilities,
    CurrentDeferredLiabilities,
    CurrentDeferredRevenue,
    CurrentDeferredTaxesLiabilities,
    CurrentDebtAndCapitalLeaseObligation,
    CurrentCapitalLeaseObligation,
    CurrentDebt,
    OtherCurrentBorrowings,
    LineOfCredit,
    CommercialPaper,
    CurrentNotesPayable,
    PensionandOtherPostRetirementBenefitPlansCurrent,
    CurrentProvisions,
    PayablesAndAccruedExpenses,
    CurrentAccruedExpenses,
    InterestPayable,
    Payables,
    OtherPayable,
    DuetoRelatedPartiesCurrent,
    DividendsPayable,
    TotalTaxPayable,
    IncomeTaxPayable,
    AccountsPayable,
    TotalAssets,
    TotalNonCurrentAssets,
    OtherNonCurrentAssets,
    DefinedPensionBenefit,
    NonCurrentPrepaidAssets,
    NonCurrentDeferredAssets,
    NonCurrentDeferredTaxesAssets,
    DuefromRelatedPartiesNonCurrent,
    NonCurrentNoteReceivables,
    NonCurrentAccountsReceivable,
    FinancialAssets,
    InvestmentsAndAdvances,
    OtherInvestments,
    InvestmentinFinancialAssets,
    HeldToMaturitySecurities,
    AvailableForSaleSecurities,
    FinancialAssetsDesignatedasFairValueThroughProfitorLossTotal,
    TradingSecurities,
    LongTermEquityInvestment,
    InvestmentsinJointVenturesatCost,
    InvestmentsInOtherVenturesUnderEquityMethod,
    InvestmentsinAssociatesatCost,
    InvestmentsinSubsidiariesatCost,
    InvestmentProperties,
    GoodwillAndOtherIntangibleAssets,
    OtherIntangibleAssets,
    Goodwill,
    NetPPE,
    AccumulatedDepreciation,
    GrossPPE,
    Leases,
    ConstructionInProgress,
    OtherProperties,
    MachineryFurnitureEquipment,
    BuildingsAndImprovements,
    LandAndImprovements,
    Properties,
    CurrentAssets,
    OtherCurrentAssets,
    HedgingAssetsCurrent,
    AssetsHeldForSaleCurrent,
    CurrentDeferredAssets,
    CurrentDeferredTaxesAssets,
    RestrictedCash,
    PrepaidAssets,
    Inventory,
    InventoriesAdjustmentsAllowances,
    OtherInventories,
    FinishedGoods,
    WorkInProcess,
    RawMaterials,
    Receivables,
    ReceivablesAdjustmentsAllowances,
    OtherReceivables,
    DuefromRelatedPartiesCurrent,
    TaxesReceivable,
    AccruedInterestReceivable,
    NotesReceivable,
    LoansReceivable,
    AccountsReceivable,
    AllowanceForDoubtfulAccountsReceivable,
    GrossAccountsReceivable,
    CashCashEquivalentsAndShortTermInvestments,
    OtherShortTermInvestments,
    CashAndCashEquivalents,
    CashEquivalents,
    CashFinancial,
}

impl BalanceSheetFact {
    pub fn all() -> &'static [Self] {
        static ALL: &[BalanceSheetFact] = &[
            BalanceSheetFact::TreasurySharesNumber,
            BalanceSheetFact::PreferredSharesNumber,
            BalanceSheetFact::OrdinarySharesNumber,
            BalanceSheetFact::ShareIssued,
            BalanceSheetFact::NetDebt,
            BalanceSheetFact::TotalDebt,
            BalanceSheetFact::TangibleBookValue,
            BalanceSheetFact::InvestedCapital,
            BalanceSheetFact::WorkingCapital,
            BalanceSheetFact::NetTangibleAssets,
            BalanceSheetFact::CapitalLeaseObligations,
            BalanceSheetFact::CommonStockEquity,
            BalanceSheetFact::PreferredStockEquity,
            BalanceSheetFact::TotalCapitalization,
            BalanceSheetFact::TotalEquityGrossMinorityInterest,
            BalanceSheetFact::MinorityInterest,
            BalanceSheetFact::StockholdersEquity,
            BalanceSheetFact::OtherEquityInterest,
            BalanceSheetFact::GainsLossesNotAffectingRetainedEarnings,
            BalanceSheetFact::OtherEquityAdjustments,
            BalanceSheetFact::FixedAssetsRevaluationReserve,
            BalanceSheetFact::ForeignCurrencyTranslationAdjustments,
            BalanceSheetFact::MinimumPensionLiabilities,
            BalanceSheetFact::UnrealizedGainLoss,
            BalanceSheetFact::TreasuryStock,
            BalanceSheetFact::RetainedEarnings,
            BalanceSheetFact::AdditionalPaidInCapital,
            BalanceSheetFact::CapitalStock,
            BalanceSheetFact::OtherCapitalStock,
            BalanceSheetFact::CommonStock,
            BalanceSheetFact::PreferredStock,
            BalanceSheetFact::TotalPartnershipCapital,
            BalanceSheetFact::GeneralPartnershipCapital,
            BalanceSheetFact::LimitedPartnershipCapital,
            BalanceSheetFact::TotalLiabilitiesNetMinorityInterest,
            BalanceSheetFact::TotalNonCurrentLiabilitiesNetMinorityInterest,
            BalanceSheetFact::OtherNonCurrentLiabilities,
            BalanceSheetFact::LiabilitiesHeldforSaleNonCurrent,
            BalanceSheetFact::RestrictedCommonStock,
            BalanceSheetFact::PreferredSecuritiesOutsideStockEquity,
            BalanceSheetFact::DerivativeProductLiabilities,
            BalanceSheetFact::EmployeeBenefits,
            BalanceSheetFact::NonCurrentPensionAndOtherPostretirementBenefitPlans,
            BalanceSheetFact::NonCurrentAccruedExpenses,
            BalanceSheetFact::DuetoRelatedPartiesNonCurrent,
            BalanceSheetFact::TradeandOtherPayablesNonCurrent,
            BalanceSheetFact::NonCurrentDeferredLiabilities,
            BalanceSheetFact::NonCurrentDeferredRevenue,
            BalanceSheetFact::NonCurrentDeferredTaxesLiabilities,
            BalanceSheetFact::LongTermDebtAndCapitalLeaseObligation,
            BalanceSheetFact::LongTermCapitalLeaseObligation,
            BalanceSheetFact::LongTermDebt,
            BalanceSheetFact::LongTermProvisions,
            BalanceSheetFact::CurrentLiabilities,
            BalanceSheetFact::OtherCurrentLiabilities,
            BalanceSheetFact::CurrentDeferredLiabilities,
            BalanceSheetFact::CurrentDeferredRevenue,
            BalanceSheetFact::CurrentDeferredTaxesLiabilities,
            BalanceSheetFact::CurrentDebtAndCapitalLeaseObligation,
            BalanceSheetFact::CurrentCapitalLeaseObligation,
            BalanceSheetFact::CurrentDebt,
            BalanceSheetFact::OtherCurrentBorrowings,
            BalanceSheetFact::LineOfCredit,
            BalanceSheetFact::CommercialPaper,
            BalanceSheetFact::CurrentNotesPayable,
            BalanceSheetFact::PensionandOtherPostRetirementBenefitPlansCurrent,
            BalanceSheetFact::CurrentProvisions,
            BalanceSheetFact::PayablesAndAccruedExpenses,
            BalanceSheetFact::CurrentAccruedExpenses,
            BalanceSheetFact::InterestPayable,
            BalanceSheetFact::Payables,
            BalanceSheetFact::OtherPayable,
            BalanceSheetFact::DuetoRelatedPartiesCurrent,
            BalanceSheetFact::DividendsPayable,
            BalanceSheetFact::TotalTaxPayable,
            BalanceSheetFact::IncomeTaxPayable,
            BalanceSheetFact::AccountsPayable,
            BalanceSheetFact::TotalAssets,
            BalanceSheetFact::TotalNonCurrentAssets,
            BalanceSheetFact::OtherNonCurrentAssets,
            BalanceSheetFact::DefinedPensionBenefit,
            BalanceSheetFact::NonCurrentPrepaidAssets,
            BalanceSheetFact::NonCurrentDeferredAssets,
            BalanceSheetFact::NonCurrentDeferredTaxesAssets,
            BalanceSheetFact::DuefromRelatedPartiesNonCurrent,
            BalanceSheetFact::NonCurrentNoteReceivables,
            BalanceSheetFact::NonCurrentAccountsReceivable,
            BalanceSheetFact::FinancialAssets,
            BalanceSheetFact::InvestmentsAndAdvances,
            BalanceSheetFact::OtherInvestments,
            BalanceSheetFact::InvestmentinFinancialAssets,
            BalanceSheetFact::HeldToMaturitySecurities,
            BalanceSheetFact::AvailableForSaleSecurities,
            BalanceSheetFact::FinancialAssetsDesignatedasFairValueThroughProfitorLossTotal,
            BalanceSheetFact::TradingSecurities,
            BalanceSheetFact::LongTermEquityInvestment,
            BalanceSheetFact::InvestmentsinJointVenturesatCost,
            BalanceSheetFact::InvestmentsInOtherVenturesUnderEquityMethod,
            BalanceSheetFact::InvestmentsinAssociatesatCost,
            BalanceSheetFact::InvestmentsinSubsidiariesatCost,
            BalanceSheetFact::InvestmentProperties,
            BalanceSheetFact::GoodwillAndOtherIntangibleAssets,
            BalanceSheetFact::OtherIntangibleAssets,
            BalanceSheetFact::Goodwill,
            BalanceSheetFact::NetPPE,
            BalanceSheetFact::AccumulatedDepreciation,
            BalanceSheetFact::GrossPPE,
            BalanceSheetFact::Leases,
            BalanceSheetFact::ConstructionInProgress,
            BalanceSheetFact::OtherProperties,
            BalanceSheetFact::MachineryFurnitureEquipment,
            BalanceSheetFact::BuildingsAndImprovements,
            BalanceSheetFact::LandAndImprovements,
            BalanceSheetFact::Properties,
            BalanceSheetFact::CurrentAssets,
            BalanceSheetFact::OtherCurrentAssets,
            BalanceSheetFact::HedgingAssetsCurrent,
            BalanceSheetFact::AssetsHeldForSaleCurrent,
            BalanceSheetFact::CurrentDeferredAssets,
            BalanceSheetFact::CurrentDeferredTaxesAssets,
            BalanceSheetFact::RestrictedCash,
            BalanceSheetFact::PrepaidAssets,
            BalanceSheetFact::Inventory,
            BalanceSheetFact::InventoriesAdjustmentsAllowances,
            BalanceSheetFact::OtherInventories,
            BalanceSheetFact::FinishedGoods,
            BalanceSheetFact::WorkInProcess,
            BalanceSheetFact::RawMaterials,
            BalanceSheetFact::Receivables,
            BalanceSheetFact::ReceivablesAdjustmentsAllowances,
            BalanceSheetFact::OtherReceivables,
            BalanceSheetFact::DuefromRelatedPartiesCurrent,
            BalanceSheetFact::TaxesReceivable,
            BalanceSheetFact::AccruedInterestReceivable,
            BalanceSheetFact::NotesReceivable,
            BalanceSheetFact::LoansReceivable,
            BalanceSheetFact::AccountsReceivable,
            BalanceSheetFact::AllowanceForDoubtfulAccountsReceivable,
            BalanceSheetFact::GrossAccountsReceivable,
            BalanceSheetFact::CashCashEquivalentsAndShortTermInvestments,
            BalanceSheetFact::OtherShortTermInvestments,
            BalanceSheetFact::CashAndCashEquivalents,
            BalanceSheetFact::CashEquivalents,
            BalanceSheetFact::CashFinancial,
        ];
        ALL
    }
}

impl AsStr for BalanceSheetFact {
    fn as_str(&self) -> &str {
        match self {
            Self::TreasurySharesNumber => "TreasurySharesNumber",
            Self::PreferredSharesNumber => "PreferredSharesNumber",
            Self::OrdinarySharesNumber => "OrdinarySharesNumber",
            Self::ShareIssued => "ShareIssued",
            Self::NetDebt => "NetDebt",
            Self::TotalDebt => "TotalDebt",
            Self::TangibleBookValue => "TangibleBookValue",
            Self::InvestedCapital => "InvestedCapital",
            Self::WorkingCapital => "WorkingCapital",
            Self::NetTangibleAssets => "NetTangibleAssets",
            Self::CapitalLeaseObligations => "CapitalLeaseObligations",
            Self::CommonStockEquity => "CommonStockEquity",
            Self::PreferredStockEquity => "PreferredStockEquity",
            Self::TotalCapitalization => "TotalCapitalization",
            Self::TotalEquityGrossMinorityInterest => "TotalEquityGrossMinorityInterest",
            Self::MinorityInterest => "MinorityInterest",
            Self::StockholdersEquity => "StockholdersEquity",
            Self::OtherEquityInterest => "OtherEquityInterest",
            Self::GainsLossesNotAffectingRetainedEarnings => {
                "GainsLossesNotAffectingRetainedEarnings"
            }
            Self::OtherEquityAdjustments => "OtherEquityAdjustments",
            Self::FixedAssetsRevaluationReserve => "FixedAssetsRevaluationReserve",
            Self::ForeignCurrencyTranslationAdjustments => "ForeignCurrencyTranslationAdjustments",
            Self::MinimumPensionLiabilities => "MinimumPensionLiabilities",
            Self::UnrealizedGainLoss => "UnrealizedGainLoss",
            Self::TreasuryStock => "TreasuryStock",
            Self::RetainedEarnings => "RetainedEarnings",
            Self::AdditionalPaidInCapital => "AdditionalPaidInCapital",
            Self::CapitalStock => "CapitalStock",
            Self::OtherCapitalStock => "OtherCapitalStock",
            Self::CommonStock => "CommonStock",
            Self::PreferredStock => "PreferredStock",
            Self::TotalPartnershipCapital => "TotalPartnershipCapital",
            Self::GeneralPartnershipCapital => "GeneralPartnershipCapital",
            Self::LimitedPartnershipCapital => "LimitedPartnershipCapital",
            Self::TotalLiabilitiesNetMinorityInterest => "TotalLiabilitiesNetMinorityInterest",
            Self::TotalNonCurrentLiabilitiesNetMinorityInterest => {
                "TotalNonCurrentLiabilitiesNetMinorityInterest"
            }
            Self::OtherNonCurrentLiabilities => "OtherNonCurrentLiabilities",
            Self::LiabilitiesHeldforSaleNonCurrent => "LiabilitiesHeldforSaleNonCurrent",
            Self::RestrictedCommonStock => "RestrictedCommonStock",
            Self::PreferredSecuritiesOutsideStockEquity => "PreferredSecuritiesOutsideStockEquity",
            Self::DerivativeProductLiabilities => "DerivativeProductLiabilities",
            Self::EmployeeBenefits => "EmployeeBenefits",
            Self::NonCurrentPensionAndOtherPostretirementBenefitPlans => {
                "NonCurrentPensionAndOtherPostretirementBenefitPlans"
            }
            Self::NonCurrentAccruedExpenses => "NonCurrentAccruedExpenses",
            Self::DuetoRelatedPartiesNonCurrent => "DuetoRelatedPartiesNonCurrent",
            Self::TradeandOtherPayablesNonCurrent => "TradeandOtherPayablesNonCurrent",
            Self::NonCurrentDeferredLiabilities => "NonCurrentDeferredLiabilities",
            Self::NonCurrentDeferredRevenue => "NonCurrentDeferredRevenue",
            Self::NonCurrentDeferredTaxesLiabilities => "NonCurrentDeferredTaxesLiabilities",
            Self::LongTermDebtAndCapitalLeaseObligation => "LongTermDebtAndCapitalLeaseObligation",
            Self::LongTermCapitalLeaseObligation => "LongTermCapitalLeaseObligation",
            Self::LongTermDebt => "LongTermDebt",
            Self::LongTermProvisions => "LongTermProvisions",
            Self::CurrentLiabilities => "CurrentLiabilities",
            Self::OtherCurrentLiabilities => "OtherCurrentLiabilities",
            Self::CurrentDeferredLiabilities => "CurrentDeferredLiabilities",
            Self::CurrentDeferredRevenue => "CurrentDeferredRevenue",
            Self::CurrentDeferredTaxesLiabilities => "CurrentDeferredTaxesLiabilities",
            Self::CurrentDebtAndCapitalLeaseObligation => "CurrentDebtAndCapitalLeaseObligation",
            Self::CurrentCapitalLeaseObligation => "CurrentCapitalLeaseObligation",
            Self::CurrentDebt => "CurrentDebt",
            Self::OtherCurrentBorrowings => "OtherCurrentBorrowings",
            Self::LineOfCredit => "LineOfCredit",
            Self::CommercialPaper => "CommercialPaper",
            Self::CurrentNotesPayable => "CurrentNotesPayable",
            Self::PensionandOtherPostRetirementBenefitPlansCurrent => {
                "PensionandOtherPostRetirementBenefitPlansCurrent"
            }
            Self::CurrentProvisions => "CurrentProvisions",
            Self::PayablesAndAccruedExpenses => "PayablesAndAccruedExpenses",
            Self::CurrentAccruedExpenses => "CurrentAccruedExpenses",
            Self::InterestPayable => "InterestPayable",
            Self::Payables => "Payables",
            Self::OtherPayable => "OtherPayable",
            Self::DuetoRelatedPartiesCurrent => "DuetoRelatedPartiesCurrent",
            Self::DividendsPayable => "DividendsPayable",
            Self::TotalTaxPayable => "TotalTaxPayable",
            Self::IncomeTaxPayable => "IncomeTaxPayable",
            Self::AccountsPayable => "AccountsPayable",
            Self::TotalAssets => "TotalAssets",
            Self::TotalNonCurrentAssets => "TotalNonCurrentAssets",
            Self::OtherNonCurrentAssets => "OtherNonCurrentAssets",
            Self::DefinedPensionBenefit => "DefinedPensionBenefit",
            Self::NonCurrentPrepaidAssets => "NonCurrentPrepaidAssets",
            Self::NonCurrentDeferredAssets => "NonCurrentDeferredAssets",
            Self::NonCurrentDeferredTaxesAssets => "NonCurrentDeferredTaxesAssets",
            Self::DuefromRelatedPartiesNonCurrent => "DuefromRelatedPartiesNonCurrent",
            Self::NonCurrentNoteReceivables => "NonCurrentNoteReceivables",
            Self::NonCurrentAccountsReceivable => "NonCurrentAccountsReceivable",
            Self::FinancialAssets => "FinancialAssets",
            Self::InvestmentsAndAdvances => "InvestmentsAndAdvances",
            Self::OtherInvestments => "OtherInvestments",
            Self::InvestmentinFinancialAssets => "InvestmentinFinancialAssets",
            Self::HeldToMaturitySecurities => "HeldToMaturitySecurities",
            Self::AvailableForSaleSecurities => "AvailableForSaleSecurities",
            Self::FinancialAssetsDesignatedasFairValueThroughProfitorLossTotal => {
                "FinancialAssetsDesignatedasFairValueThroughProfitorLossTotal"
            }
            Self::TradingSecurities => "TradingSecurities",
            Self::LongTermEquityInvestment => "LongTermEquityInvestment",
            Self::InvestmentsinJointVenturesatCost => "InvestmentsinJointVenturesatCost",
            Self::InvestmentsInOtherVenturesUnderEquityMethod => {
                "InvestmentsInOtherVenturesUnderEquityMethod"
            }
            Self::InvestmentsinAssociatesatCost => "InvestmentsinAssociatesatCost",
            Self::InvestmentsinSubsidiariesatCost => "InvestmentsinSubsidiariesatCost",
            Self::InvestmentProperties => "InvestmentProperties",
            Self::GoodwillAndOtherIntangibleAssets => "GoodwillAndOtherIntangibleAssets",
            Self::OtherIntangibleAssets => "OtherIntangibleAssets",
            Self::Goodwill => "Goodwill",
            Self::NetPPE => "NetPPE",
            Self::AccumulatedDepreciation => "AccumulatedDepreciation",
            Self::GrossPPE => "GrossPPE",
            Self::Leases => "Leases",
            Self::ConstructionInProgress => "ConstructionInProgress",
            Self::OtherProperties => "OtherProperties",
            Self::MachineryFurnitureEquipment => "MachineryFurnitureEquipment",
            Self::BuildingsAndImprovements => "BuildingsAndImprovements",
            Self::LandAndImprovements => "LandAndImprovements",
            Self::Properties => "Properties",
            Self::CurrentAssets => "CurrentAssets",
            Self::OtherCurrentAssets => "OtherCurrentAssets",
            Self::HedgingAssetsCurrent => "HedgingAssetsCurrent",
            Self::AssetsHeldForSaleCurrent => "AssetsHeldForSaleCurrent",
            Self::CurrentDeferredAssets => "CurrentDeferredAssets",
            Self::CurrentDeferredTaxesAssets => "CurrentDeferredTaxesAssets",
            Self::RestrictedCash => "RestrictedCash",
            Self::PrepaidAssets => "PrepaidAssets",
            Self::Inventory => "Inventory",
            Self::InventoriesAdjustmentsAllowances => "InventoriesAdjustmentsAllowances",
            Self::OtherInventories => "OtherInventories",
            Self::FinishedGoods => "FinishedGoods",
            Self::WorkInProcess => "WorkInProcess",
            Self::RawMaterials => "RawMaterials",
            Self::Receivables => "Receivables",
            Self::ReceivablesAdjustmentsAllowances => "ReceivablesAdjustmentsAllowances",
            Self::OtherReceivables => "OtherReceivables",
            Self::DuefromRelatedPartiesCurrent => "DuefromRelatedPartiesCurrent",
            Self::TaxesReceivable => "TaxesReceivable",
            Self::AccruedInterestReceivable => "AccruedInterestReceivable",
            Self::NotesReceivable => "NotesReceivable",
            Self::LoansReceivable => "LoansReceivable",
            Self::AccountsReceivable => "AccountsReceivable",
            Self::AllowanceForDoubtfulAccountsReceivable => {
                "AllowanceForDoubtfulAccountsReceivable"
            }
            Self::GrossAccountsReceivable => "GrossAccountsReceivable",
            Self::CashCashEquivalentsAndShortTermInvestments => {
                "CashCashEquivalentsAndShortTermInvestments"
            }
            Self::OtherShortTermInvestments => "OtherShortTermInvestments",
            Self::CashAndCashEquivalents => "CashAndCashEquivalents",
            Self::CashEquivalents => "CashEquivalents",
            Self::CashFinancial => "CashFinancial",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CashflowFact {
    ForeignSales,
    DomesticSales,
    AdjustedGeographySegmentData,
    FreeCashFlow,
    RepurchaseOfCapitalStock,
    RepaymentOfDebt,
    IssuanceOfDebt,
    IssuanceOfCapitalStock,
    CapitalExpenditure,
    InterestPaidSupplementalData,
    IncomeTaxPaidSupplementalData,
    EndCashPosition,
    OtherCashAdjustmentOutsideChangeinCash,
    BeginningCashPosition,
    EffectOfExchangeRateChanges,
    ChangesInCash,
    OtherCashAdjustmentInsideChangeinCash,
    CashFlowFromDiscontinuedOperation,
    FinancingCashFlow,
    CashFromDiscontinuedFinancingActivities,
    CashFlowFromContinuingFinancingActivities,
    NetOtherFinancingCharges,
    InterestPaidCFF,
    ProceedsFromStockOptionExercised,
    CashDividendsPaid,
    PreferredStockDividendPaid,
    CommonStockDividendPaid,
    NetPreferredStockIssuance,
    PreferredStockPayments,
    PreferredStockIssuance,
    NetCommonStockIssuance,
    CommonStockPayments,
    CommonStockIssuance,
    NetIssuancePaymentsOfDebt,
    NetShortTermDebtIssuance,
    ShortTermDebtPayments,
    ShortTermDebtIssuance,
    NetLongTermDebtIssuance,
    LongTermDebtPayments,
    LongTermDebtIssuance,
    InvestingCashFlow,
    CashFromDiscontinuedInvestingActivities,
    CashFlowFromContinuingInvestingActivities,
    NetOtherInvestingChanges,
    InterestReceivedCFI,
    DividendsReceivedCFI,
    NetInvestmentPurchaseAndSale,
    SaleOfInvestment,
    PurchaseOfInvestment,
    NetInvestmentPropertiesPurchaseAndSale,
    SaleOfInvestmentProperties,
    PurchaseOfInvestmentProperties,
    NetBusinessPurchaseAndSale,
    SaleOfBusiness,
    PurchaseOfBusiness,
    NetIntangiblesPurchaseAndSale,
    SaleOfIntangibles,
    PurchaseOfIntangibles,
    NetPPEPurchaseAndSale,
    SaleOfPPE,
    PurchaseOfPPE,
    CapitalExpenditureReported,
    OperatingCashFlow,
    CashFromDiscontinuedOperatingActivities,
    CashFlowFromContinuingOperatingActivities,
    TaxesRefundPaid,
    InterestReceivedCFO,
    InterestPaidCFO,
    DividendReceivedCFO,
    DividendPaidCFO,
    ChangeInWorkingCapital,
    ChangeInOtherWorkingCapital,
    ChangeInOtherCurrentLiabilities,
    ChangeInOtherCurrentAssets,
    ChangeInPayablesAndAccruedExpense,
    ChangeInAccruedExpense,
    ChangeInInterestPayable,
    ChangeInPayable,
    ChangeInDividendPayable,
    ChangeInAccountPayable,
    ChangeInTaxPayable,
    ChangeInIncomeTaxPayable,
    ChangeInPrepaidAssets,
    ChangeInInventory,
    ChangeInReceivables,
    ChangesInAccountReceivables,
    OtherNonCashItems,
    ExcessTaxBenefitFromStockBasedCompensation,
    StockBasedCompensation,
    UnrealizedGainLossOnInvestmentSecurities,
    ProvisionandWriteOffofAssets,
    AssetImpairmentCharge,
    AmortizationOfSecurities,
    DeferredTax,
    DeferredIncomeTax,
    DepreciationAmortizationDepletion,
    Depletion,
    DepreciationAndAmortization,
    AmortizationCashFlow,
    AmortizationOfIntangibles,
    Depreciation,
    OperatingGainsLosses,
    PensionAndEmployeeBenefitExpense,
    EarningsLossesFromEquityInvestments,
    GainLossOnInvestmentSecurities,
    NetForeignCurrencyExchangeGainLoss,
    GainLossOnSaleOfPPE,
    GainLossOnSaleOfBusiness,
    NetIncomeFromContinuingOperations,
    CashFlowsfromusedinOperatingActivitiesDirect,
    TaxesRefundPaidDirect,
    InterestReceivedDirect,
    InterestPaidDirect,
    DividendsReceivedDirect,
    DividendsPaidDirect,
    ClassesofCashPayments,
    OtherCashPaymentsfromOperatingActivities,
    PaymentsonBehalfofEmployees,
    PaymentstoSuppliersforGoodsandServices,
    ClassesofCashReceiptsfromOperatingActivities,
    OtherCashReceiptsfromOperatingActivities,
    ReceiptsfromGovernmentGrants,
    ReceiptsfromCustomers,
}

impl CashflowFact {
    pub fn all() -> &'static [Self] {
        static ALL: &[CashflowFact] = &[
            CashflowFact::ForeignSales,
            CashflowFact::DomesticSales,
            CashflowFact::AdjustedGeographySegmentData,
            CashflowFact::FreeCashFlow,
            CashflowFact::RepurchaseOfCapitalStock,
            CashflowFact::RepaymentOfDebt,
            CashflowFact::IssuanceOfDebt,
            CashflowFact::IssuanceOfCapitalStock,
            CashflowFact::CapitalExpenditure,
            CashflowFact::InterestPaidSupplementalData,
            CashflowFact::IncomeTaxPaidSupplementalData,
            CashflowFact::EndCashPosition,
            CashflowFact::OtherCashAdjustmentOutsideChangeinCash,
            CashflowFact::BeginningCashPosition,
            CashflowFact::EffectOfExchangeRateChanges,
            CashflowFact::ChangesInCash,
            CashflowFact::OtherCashAdjustmentInsideChangeinCash,
            CashflowFact::CashFlowFromDiscontinuedOperation,
            CashflowFact::FinancingCashFlow,
            CashflowFact::CashFromDiscontinuedFinancingActivities,
            CashflowFact::CashFlowFromContinuingFinancingActivities,
            CashflowFact::NetOtherFinancingCharges,
            CashflowFact::InterestPaidCFF,
            CashflowFact::ProceedsFromStockOptionExercised,
            CashflowFact::CashDividendsPaid,
            CashflowFact::PreferredStockDividendPaid,
            CashflowFact::CommonStockDividendPaid,
            CashflowFact::NetPreferredStockIssuance,
            CashflowFact::PreferredStockPayments,
            CashflowFact::PreferredStockIssuance,
            CashflowFact::NetCommonStockIssuance,
            CashflowFact::CommonStockPayments,
            CashflowFact::CommonStockIssuance,
            CashflowFact::NetIssuancePaymentsOfDebt,
            CashflowFact::NetShortTermDebtIssuance,
            CashflowFact::ShortTermDebtPayments,
            CashflowFact::ShortTermDebtIssuance,
            CashflowFact::NetLongTermDebtIssuance,
            CashflowFact::LongTermDebtPayments,
            CashflowFact::LongTermDebtIssuance,
            CashflowFact::InvestingCashFlow,
            CashflowFact::CashFromDiscontinuedInvestingActivities,
            CashflowFact::CashFlowFromContinuingInvestingActivities,
            CashflowFact::NetOtherInvestingChanges,
            CashflowFact::InterestReceivedCFI,
            CashflowFact::DividendsReceivedCFI,
            CashflowFact::NetInvestmentPurchaseAndSale,
            CashflowFact::SaleOfInvestment,
            CashflowFact::PurchaseOfInvestment,
            CashflowFact::NetInvestmentPropertiesPurchaseAndSale,
            CashflowFact::SaleOfInvestmentProperties,
            CashflowFact::PurchaseOfInvestmentProperties,
            CashflowFact::NetBusinessPurchaseAndSale,
            CashflowFact::SaleOfBusiness,
            CashflowFact::PurchaseOfBusiness,
            CashflowFact::NetIntangiblesPurchaseAndSale,
            CashflowFact::SaleOfIntangibles,
            CashflowFact::PurchaseOfIntangibles,
            CashflowFact::NetPPEPurchaseAndSale,
            CashflowFact::SaleOfPPE,
            CashflowFact::PurchaseOfPPE,
            CashflowFact::CapitalExpenditureReported,
            CashflowFact::OperatingCashFlow,
            CashflowFact::CashFromDiscontinuedOperatingActivities,
            CashflowFact::CashFlowFromContinuingOperatingActivities,
            CashflowFact::TaxesRefundPaid,
            CashflowFact::InterestReceivedCFO,
            CashflowFact::InterestPaidCFO,
            CashflowFact::DividendReceivedCFO,
            CashflowFact::DividendPaidCFO,
            CashflowFact::ChangeInWorkingCapital,
            CashflowFact::ChangeInOtherWorkingCapital,
            CashflowFact::ChangeInOtherCurrentLiabilities,
            CashflowFact::ChangeInOtherCurrentAssets,
            CashflowFact::ChangeInPayablesAndAccruedExpense,
            CashflowFact::ChangeInAccruedExpense,
            CashflowFact::ChangeInInterestPayable,
            CashflowFact::ChangeInPayable,
            CashflowFact::ChangeInDividendPayable,
            CashflowFact::ChangeInAccountPayable,
            CashflowFact::ChangeInTaxPayable,
            CashflowFact::ChangeInIncomeTaxPayable,
            CashflowFact::ChangeInPrepaidAssets,
            CashflowFact::ChangeInInventory,
            CashflowFact::ChangeInReceivables,
            CashflowFact::ChangesInAccountReceivables,
            CashflowFact::OtherNonCashItems,
            CashflowFact::ExcessTaxBenefitFromStockBasedCompensation,
            CashflowFact::StockBasedCompensation,
            CashflowFact::UnrealizedGainLossOnInvestmentSecurities,
            CashflowFact::ProvisionandWriteOffofAssets,
            CashflowFact::AssetImpairmentCharge,
            CashflowFact::AmortizationOfSecurities,
            CashflowFact::DeferredTax,
            CashflowFact::DeferredIncomeTax,
            CashflowFact::DepreciationAmortizationDepletion,
            CashflowFact::Depletion,
            CashflowFact::DepreciationAndAmortization,
            CashflowFact::AmortizationCashFlow,
            CashflowFact::AmortizationOfIntangibles,
            CashflowFact::Depreciation,
            CashflowFact::OperatingGainsLosses,
            CashflowFact::PensionAndEmployeeBenefitExpense,
            CashflowFact::EarningsLossesFromEquityInvestments,
            CashflowFact::GainLossOnInvestmentSecurities,
            CashflowFact::NetForeignCurrencyExchangeGainLoss,
            CashflowFact::GainLossOnSaleOfPPE,
            CashflowFact::GainLossOnSaleOfBusiness,
            CashflowFact::NetIncomeFromContinuingOperations,
            CashflowFact::CashFlowsfromusedinOperatingActivitiesDirect,
            CashflowFact::TaxesRefundPaidDirect,
            CashflowFact::InterestReceivedDirect,
            CashflowFact::InterestPaidDirect,
            CashflowFact::DividendsReceivedDirect,
            CashflowFact::DividendsPaidDirect,
            CashflowFact::ClassesofCashPayments,
            CashflowFact::OtherCashPaymentsfromOperatingActivities,
            CashflowFact::PaymentsonBehalfofEmployees,
            CashflowFact::PaymentstoSuppliersforGoodsandServices,
            CashflowFact::ClassesofCashReceiptsfromOperatingActivities,
            CashflowFact::OtherCashReceiptsfromOperatingActivities,
            CashflowFact::ReceiptsfromGovernmentGrants,
            CashflowFact::ReceiptsfromCustomers,
        ];
        ALL
    }
}

impl AsStr for CashflowFact {
    fn as_str(&self) -> &str {
        match self {
            Self::ForeignSales => "ForeignSales",
            Self::DomesticSales => "DomesticSales",
            Self::AdjustedGeographySegmentData => "AdjustedGeographySegmentData",
            Self::FreeCashFlow => "FreeCashFlow",
            Self::RepurchaseOfCapitalStock => "RepurchaseOfCapitalStock",
            Self::RepaymentOfDebt => "RepaymentOfDebt",
            Self::IssuanceOfDebt => "IssuanceOfDebt",
            Self::IssuanceOfCapitalStock => "IssuanceOfCapitalStock",
            Self::CapitalExpenditure => "CapitalExpenditure",
            Self::InterestPaidSupplementalData => "InterestPaidSupplementalData",
            Self::IncomeTaxPaidSupplementalData => "IncomeTaxPaidSupplementalData",
            Self::EndCashPosition => "EndCashPosition",
            Self::OtherCashAdjustmentOutsideChangeinCash => {
                "OtherCashAdjustmentOutsideChangeinCash"
            }
            Self::BeginningCashPosition => "BeginningCashPosition",
            Self::EffectOfExchangeRateChanges => "EffectOfExchangeRateChanges",
            Self::ChangesInCash => "ChangesInCash",
            Self::OtherCashAdjustmentInsideChangeinCash => "OtherCashAdjustmentInsideChangeinCash",
            Self::CashFlowFromDiscontinuedOperation => "CashFlowFromDiscontinuedOperation",
            Self::FinancingCashFlow => "FinancingCashFlow",
            Self::CashFromDiscontinuedFinancingActivities => {
                "CashFromDiscontinuedFinancingActivities"
            }
            Self::CashFlowFromContinuingFinancingActivities => {
                "CashFlowFromContinuingFinancingActivities"
            }
            Self::NetOtherFinancingCharges => "NetOtherFinancingCharges",
            Self::InterestPaidCFF => "InterestPaidCFF",
            Self::ProceedsFromStockOptionExercised => "ProceedsFromStockOptionExercised",
            Self::CashDividendsPaid => "CashDividendsPaid",
            Self::PreferredStockDividendPaid => "PreferredStockDividendPaid",
            Self::CommonStockDividendPaid => "CommonStockDividendPaid",
            Self::NetPreferredStockIssuance => "NetPreferredStockIssuance",
            Self::PreferredStockPayments => "PreferredStockPayments",
            Self::PreferredStockIssuance => "PreferredStockIssuance",
            Self::NetCommonStockIssuance => "NetCommonStockIssuance",
            Self::CommonStockPayments => "CommonStockPayments",
            Self::CommonStockIssuance => "CommonStockIssuance",
            Self::NetIssuancePaymentsOfDebt => "NetIssuancePaymentsOfDebt",
            Self::NetShortTermDebtIssuance => "NetShortTermDebtIssuance",
            Self::ShortTermDebtPayments => "ShortTermDebtPayments",
            Self::ShortTermDebtIssuance => "ShortTermDebtIssuance",
            Self::NetLongTermDebtIssuance => "NetLongTermDebtIssuance",
            Self::LongTermDebtPayments => "LongTermDebtPayments",
            Self::LongTermDebtIssuance => "LongTermDebtIssuance",
            Self::InvestingCashFlow => "InvestingCashFlow",
            Self::CashFromDiscontinuedInvestingActivities => {
                "CashFromDiscontinuedInvestingActivities"
            }
            Self::CashFlowFromContinuingInvestingActivities => {
                "CashFlowFromContinuingInvestingActivities"
            }
            Self::NetOtherInvestingChanges => "NetOtherInvestingChanges",
            Self::InterestReceivedCFI => "InterestReceivedCFI",
            Self::DividendsReceivedCFI => "DividendsReceivedCFI",
            Self::NetInvestmentPurchaseAndSale => "NetInvestmentPurchaseAndSale",
            Self::SaleOfInvestment => "SaleOfInvestment",
            Self::PurchaseOfInvestment => "PurchaseOfInvestment",
            Self::NetInvestmentPropertiesPurchaseAndSale => {
                "NetInvestmentPropertiesPurchaseAndSale"
            }
            Self::SaleOfInvestmentProperties => "SaleOfInvestmentProperties",
            Self::PurchaseOfInvestmentProperties => "PurchaseOfInvestmentProperties",
            Self::NetBusinessPurchaseAndSale => "NetBusinessPurchaseAndSale",
            Self::SaleOfBusiness => "SaleOfBusiness",
            Self::PurchaseOfBusiness => "PurchaseOfBusiness",
            Self::NetIntangiblesPurchaseAndSale => "NetIntangiblesPurchaseAndSale",
            Self::SaleOfIntangibles => "SaleOfIntangibles",
            Self::PurchaseOfIntangibles => "PurchaseOfIntangibles",
            Self::NetPPEPurchaseAndSale => "NetPPEPurchaseAndSale",
            Self::SaleOfPPE => "SaleOfPPE",
            Self::PurchaseOfPPE => "PurchaseOfPPE",
            Self::CapitalExpenditureReported => "CapitalExpenditureReported",
            Self::OperatingCashFlow => "OperatingCashFlow",
            Self::CashFromDiscontinuedOperatingActivities => {
                "CashFromDiscontinuedOperatingActivities"
            }
            Self::CashFlowFromContinuingOperatingActivities => {
                "CashFlowFromContinuingOperatingActivities"
            }
            Self::TaxesRefundPaid => "TaxesRefundPaid",
            Self::InterestReceivedCFO => "InterestReceivedCFO",
            Self::InterestPaidCFO => "InterestPaidCFO",
            Self::DividendReceivedCFO => "DividendReceivedCFO",
            Self::DividendPaidCFO => "DividendPaidCFO",
            Self::ChangeInWorkingCapital => "ChangeInWorkingCapital",
            Self::ChangeInOtherWorkingCapital => "ChangeInOtherWorkingCapital",
            Self::ChangeInOtherCurrentLiabilities => "ChangeInOtherCurrentLiabilities",
            Self::ChangeInOtherCurrentAssets => "ChangeInOtherCurrentAssets",
            Self::ChangeInPayablesAndAccruedExpense => "ChangeInPayablesAndAccruedExpense",
            Self::ChangeInAccruedExpense => "ChangeInAccruedExpense",
            Self::ChangeInInterestPayable => "ChangeInInterestPayable",
            Self::ChangeInPayable => "ChangeInPayable",
            Self::ChangeInDividendPayable => "ChangeInDividendPayable",
            Self::ChangeInAccountPayable => "ChangeInAccountPayable",
            Self::ChangeInTaxPayable => "ChangeInTaxPayable",
            Self::ChangeInIncomeTaxPayable => "ChangeInIncomeTaxPayable",
            Self::ChangeInPrepaidAssets => "ChangeInPrepaidAssets",
            Self::ChangeInInventory => "ChangeInInventory",
            Self::ChangeInReceivables => "ChangeInReceivables",
            Self::ChangesInAccountReceivables => "ChangesInAccountReceivables",
            Self::OtherNonCashItems => "OtherNonCashItems",
            Self::ExcessTaxBenefitFromStockBasedCompensation => {
                "ExcessTaxBenefitFromStockBasedCompensation"
            }
            Self::StockBasedCompensation => "StockBasedCompensation",
            Self::UnrealizedGainLossOnInvestmentSecurities => {
                "UnrealizedGainLossOnInvestmentSecurities"
            }
            Self::ProvisionandWriteOffofAssets => "ProvisionandWriteOffofAssets",
            Self::AssetImpairmentCharge => "AssetImpairmentCharge",
            Self::AmortizationOfSecurities => "AmortizationOfSecurities",
            Self::DeferredTax => "DeferredTax",
            Self::DeferredIncomeTax => "DeferredIncomeTax",
            Self::DepreciationAmortizationDepletion => "DepreciationAmortizationDepletion",
            Self::Depletion => "Depletion",
            Self::DepreciationAndAmortization => "DepreciationAndAmortization",
            Self::AmortizationCashFlow => "AmortizationCashFlow",
            Self::AmortizationOfIntangibles => "AmortizationOfIntangibles",
            Self::Depreciation => "Depreciation",
            Self::OperatingGainsLosses => "OperatingGainsLosses",
            Self::PensionAndEmployeeBenefitExpense => "PensionAndEmployeeBenefitExpense",
            Self::EarningsLossesFromEquityInvestments => "EarningsLossesFromEquityInvestments",
            Self::GainLossOnInvestmentSecurities => "GainLossOnInvestmentSecurities",
            Self::NetForeignCurrencyExchangeGainLoss => "NetForeignCurrencyExchangeGainLoss",
            Self::GainLossOnSaleOfPPE => "GainLossOnSaleOfPPE",
            Self::GainLossOnSaleOfBusiness => "GainLossOnSaleOfBusiness",
            Self::NetIncomeFromContinuingOperations => "NetIncomeFromContinuingOperations",
            Self::CashFlowsfromusedinOperatingActivitiesDirect => {
                "CashFlowsfromusedinOperatingActivitiesDirect"
            }
            Self::TaxesRefundPaidDirect => "TaxesRefundPaidDirect",
            Self::InterestReceivedDirect => "InterestReceivedDirect",
            Self::InterestPaidDirect => "InterestPaidDirect",
            Self::DividendsReceivedDirect => "DividendsReceivedDirect",
            Self::DividendsPaidDirect => "DividendsPaidDirect",
            Self::ClassesofCashPayments => "ClassesofCashPayments",
            Self::OtherCashPaymentsfromOperatingActivities => {
                "OtherCashPaymentsfromOperatingActivities"
            }
            Self::PaymentsonBehalfofEmployees => "PaymentsonBehalfofEmployees",
            Self::PaymentstoSuppliersforGoodsandServices => {
                "PaymentstoSuppliersforGoodsandServices"
            }
            Self::ClassesofCashReceiptsfromOperatingActivities => {
                "ClassesofCashReceiptsfromOperatingActivities"
            }
            Self::OtherCashReceiptsfromOperatingActivities => {
                "OtherCashReceiptsfromOperatingActivities"
            }
            Self::ReceiptsfromGovernmentGrants => "ReceiptsfromGovernmentGrants",
            Self::ReceiptsfromCustomers => "ReceiptsfromCustomers",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INCOME_STATEMENT: &str = r#"
    {
  "timeseries": {
    "result": [
      {
        "meta": {
          "symbol": [
            "AAPL"
          ],
          "type": [
            "quarterlyDilutedEPS"
          ]
        },
        "timestamp": [
          1672444800,
          1680220800,
          1688083200,
          1696032000,
          1703980800
        ],
        "quarterlyDilutedEPS": [
          {
            "dataId": 29009,
            "asOfDate": "2022-12-31",
            "periodType": "3M",
            "currencyCode": "USD",
            "reportedValue": {
              "raw": 1.88,
              "fmt": "1.88"
            }
          },
          {
            "dataId": 29009,
            "asOfDate": "2023-03-31",
            "periodType": "3M",
            "currencyCode": "USD",
            "reportedValue": {
              "raw": 1.52,
              "fmt": "1.52"
            }
          },
          {
            "dataId": 29009,
            "asOfDate": "2023-06-30",
            "periodType": "3M",
            "currencyCode": "USD",
            "reportedValue": {
              "raw": 1.26,
              "fmt": "1.26"
            }
          },
          {
            "dataId": 29009,
            "asOfDate": "2023-09-30",
            "periodType": "3M",
            "currencyCode": "USD",
            "reportedValue": {
              "raw": 1.46,
              "fmt": "1.46"
            }
          },
          {
            "dataId": 29009,
            "asOfDate": "2023-12-31",
            "periodType": "3M",
            "currencyCode": "USD",
            "reportedValue": {
              "raw": 2.18,
              "fmt": "2.18"
            }
          }
        ]
      },
      {
        "meta": {
          "symbol": [
            "AAPL"
          ],
          "type": [
            "quarterlyTaxEffectOfUnusualItems"
          ]
        },
        "timestamp": [
          1672444800,
          1680220800,
          1688083200,
          1696032000,
          1703980800
        ],
        "quarterlyTaxEffectOfUnusualItems": [
          {
            "dataId": 20419,
            "asOfDate": "2022-12-31",
            "periodType": "3M",
            "currencyCode": "USD",
            "reportedValue": {
              "raw": 0.0,
              "fmt": "0.00"
            }
          },
          {
            "dataId": 20419,
            "asOfDate": "2023-03-31",
            "periodType": "3M",
            "currencyCode": "USD",
            "reportedValue": {
              "raw": 0.0,
              "fmt": "0.00"
            }
          },
          {
            "dataId": 20419,
            "asOfDate": "2023-06-30",
            "periodType": "3M",
            "currencyCode": "USD",
            "reportedValue": {
              "raw": 0.0,
              "fmt": "0.00"
            }
          },
          {
            "dataId": 20419,
            "asOfDate": "2023-09-30",
            "periodType": "3M",
            "currencyCode": "USD",
            "reportedValue": {
              "raw": 0.0,
              "fmt": "0.00"
            }
          },
          {
            "dataId": 20419,
            "asOfDate": "2023-12-31",
            "periodType": "3M",
            "currencyCode": "USD",
            "reportedValue": {
              "raw": 0.0,
              "fmt": "0.00"
            }
          }
        ]
      }
    ],
    "error": null
  }
}"#;

    #[test]
    fn test_income_statement_parsing() {
        let json: Value = serde_json::from_str(INCOME_STATEMENT).unwrap();
        let income_statement = from_response(
            json,
            Period::Quarter,
            &[
                IncomeStatementFact::DilutedEPS,
                IncomeStatementFact::TaxEffectOfUnusualItems,
            ],
        )
        .unwrap();

        assert_eq!(income_statement.len(), 2);

        let diluted_eps = income_statement
            .get(&IncomeStatementFact::DilutedEPS)
            .unwrap();
        let tax_effect_of_unusual_items = income_statement
            .get(&IncomeStatementFact::TaxEffectOfUnusualItems)
            .unwrap();

        assert_eq!(diluted_eps.len(), 5);
        assert_eq!(tax_effect_of_unusual_items.len(), 5);

        assert_eq!(
            diluted_eps
                .get(&time::Date::from_calendar_date(2022, time::Month::December, 31).unwrap()),
            Some(&1.88f64)
        );
    }

    #[test]
    fn compose_income_statement_url() {
        let url = super::compose_fundamentals_url(
            "IBM",
            Period::Year,
            OffsetDateTime::from_unix_timestamp(1710248726).unwrap(),
            &[
                IncomeStatementFact::EBITDA,
                IncomeStatementFact::NetIncome,
                IncomeStatementFact::BasicEPS,
            ],
        );
        assert_eq!(url, "https://query2.finance.yahoo.com/ws/fundamentals-timeseries/v1/finance/timeseries/IBM?symbol=IBM&period1=1262304000&period2=1710248726&type=annualEBITDA,annualNetIncome,annualBasicEPS");
    }
}
