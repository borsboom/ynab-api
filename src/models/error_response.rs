/*
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.youneedabudget.com
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "error")]
    pub error: crate::models::ErrorDetail,
}

impl ErrorResponse {
    pub fn new(error: crate::models::ErrorDetail) -> ErrorResponse {
        ErrorResponse {
            error,
        }
    }
}


