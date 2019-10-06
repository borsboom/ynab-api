/*
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.youneedabudget.com
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DateFormat : The date format setting for the budget.  In some cases the format will not be available and will be specified as null.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateFormat {
    #[serde(rename = "format")]
    pub format: String,
}

impl DateFormat {
    /// The date format setting for the budget.  In some cases the format will not be available and will be specified as null.
    pub fn new(format: String) -> DateFormat {
        DateFormat {
            format,
        }
    }
}


