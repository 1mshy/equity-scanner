use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub quote_summary: Option<QuoteSummary>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteSummary {
    pub error: Option<serde_json::Value>,
    pub result: Option<Vec<CompanyData>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyData {
    pub asset_profile: Option<AssetProfile>,
    pub financial_data: Option<FinancialData>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetProfile {
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub audit_risk: Option<u8>,
    pub board_risk: Option<u8>,
    pub city: Option<String>,
    pub company_officers: Option<Vec<CompanyOfficer>>,
    pub compensation_as_of_epoch_date: Option<i64>,
    pub compensation_risk: Option<u8>,
    pub country: Option<String>,
    pub governance_epoch_date: Option<i64>,
    pub industry: Option<String>,
    pub industry_disp: Option<String>,
    pub industry_key: Option<String>,
    pub long_business_summary: Option<String>,
    pub max_age: Option<u32>,
    pub overall_risk: Option<u8>,
    pub phone: Option<String>,
    pub sector: Option<String>,
    pub sector_disp: Option<String>,
    pub sector_key: Option<String>,
    pub share_holder_rights_risk: Option<u8>,
    pub state: Option<String>,
    pub website: Option<String>,
    pub zip: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyOfficer {
    pub age: Option<u8>,
    pub exercised_value: Option<MonetaryValue>,
    pub fiscal_year: Option<u16>,
    pub max_age: Option<u8>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub total_pay: Option<MonetaryValue>,
    pub unexercised_value: Option<MonetaryValue>,
    pub year_born: Option<u16>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancialData {
    pub current_price: Option<NumericValue>,
    pub current_ratio: Option<NumericValue>,
    pub debt_to_equity: Option<NumericValue>,
    pub ebitda: Option<MonetaryValue>,
    pub ebitda_margins: Option<NumericValue>,
    pub financial_currency: Option<String>,
    pub free_cashflow: Option<MonetaryValue>,
    pub gross_margins: Option<NumericValue>,
    pub gross_profits: Option<MonetaryValue>,
    pub max_age: Option<u32>,
    pub number_of_analyst_opinions: Option<NumericValue>,
    pub operating_cashflow: Option<MonetaryValue>,
    pub operating_margins: Option<NumericValue>,
    pub profit_margins: Option<NumericValue>,
    pub quick_ratio: Option<NumericValue>,
    pub recommendation_key: Option<String>,
    pub recommendation_mean: Option<NumericValue>,
    pub return_on_assets: Option<NumericValue>,
    pub return_on_equity: Option<NumericValue>,
    pub revenue_growth: Option<NumericValue>,
    pub revenue_per_share: Option<NumericValue>,
    pub target_high_price: Option<NumericValue>,
    pub target_low_price: Option<NumericValue>,
    pub target_mean_price: Option<NumericValue>,
    pub target_median_price: Option<NumericValue>,
    pub total_cash: Option<MonetaryValue>,
    pub total_cash_per_share: Option<NumericValue>,
    pub total_debt: Option<MonetaryValue>,
    pub total_revenue: Option<MonetaryValue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonetaryValue {
    pub fmt: Option<String>,
    pub long_fmt: Option<String>,
    pub raw: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumericValue {
    pub fmt: Option<String>,
    pub raw: Option<f64>,
}
