/*
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.youneedabudget.com
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    /// The type of account. Note: payPal, merchantAccount, investmentAccount, and mortgage types have been deprecated and will be removed in the future.
    #[serde(rename = "type")]
    pub _type: Type,
    /// Whether this account is on budget or not
    #[serde(rename = "on_budget")]
    pub on_budget: bool,
    /// Whether this account is closed or not
    #[serde(rename = "closed")]
    pub closed: bool,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// The current balance of the account in milliunits format
    #[serde(rename = "balance")]
    pub balance: i64,
    /// The current cleared balance of the account in milliunits format
    #[serde(rename = "cleared_balance")]
    pub cleared_balance: i64,
    /// The current uncleared balance of the account in milliunits format
    #[serde(rename = "uncleared_balance")]
    pub uncleared_balance: i64,
    /// The payee id which should be used when transferring to this account
    #[serde(rename = "transfer_payee_id")]
    pub transfer_payee_id: String,
    /// Whether or not the account has been deleted.  Deleted accounts will only be included in delta requests.
    #[serde(rename = "deleted")]
    pub deleted: bool,
}

impl Account {
    pub fn new(id: String, name: String, _type: Type, on_budget: bool, closed: bool, balance: i64, cleared_balance: i64, uncleared_balance: i64, transfer_payee_id: String, deleted: bool) -> Account {
        Account {
            id: id,
            name: name,
            _type: _type,
            on_budget: on_budget,
            closed: closed,
            note: None,
            balance: balance,
            cleared_balance: cleared_balance,
            uncleared_balance: uncleared_balance,
            transfer_payee_id: transfer_payee_id,
            deleted: deleted,
        }
    }
}

/// The type of account. Note: payPal, merchantAccount, investmentAccount, and mortgage types have been deprecated and will be removed in the future.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "checking")]
    Checking,
    #[serde(rename = "savings")]
    Savings,
    #[serde(rename = "cash")]
    Cash,
    #[serde(rename = "creditCard")]
    CreditCard,
    #[serde(rename = "lineOfCredit")]
    LineOfCredit,
    #[serde(rename = "otherAsset")]
    OtherAsset,
    #[serde(rename = "otherLiability")]
    OtherLiability,
    #[serde(rename = "payPal")]
    PayPal,
    #[serde(rename = "merchantAccount")]
    MerchantAccount,
    #[serde(rename = "investmentAccount")]
    InvestmentAccount,
    #[serde(rename = "mortgage")]
    Mortgage,
}

