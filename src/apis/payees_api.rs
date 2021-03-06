/*
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.youneedabudget.com
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use reqwest;

use super::{Error, configuration};

pub struct PayeesApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl PayeesApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> PayeesApiClient {
        PayeesApiClient {
            configuration: configuration,
        }
    }
}

pub trait PayeesApi {
    fn get_payee_by_id(&self, budget_id: &str, payee_id: &str) -> Result<crate::models::PayeeResponse, Error>;
    fn get_payees(&self, budget_id: &str, last_knowledge_of_server: i64) -> Result<crate::models::PayeesResponse, Error>;
}

impl PayeesApi for PayeesApiClient {
    fn get_payee_by_id(&self, budget_id: &str, payee_id: &str) -> Result<crate::models::PayeeResponse, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/budgets/{budget_id}/payees/{payee_id}", configuration.base_path, budget_id=crate::apis::urlencode(budget_id), payee_id=crate::apis::urlencode(payee_id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_payees(&self, budget_id: &str, last_knowledge_of_server: i64) -> Result<crate::models::PayeesResponse, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/budgets/{budget_id}/payees", configuration.base_path, budget_id=crate::apis::urlencode(budget_id));
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("last_knowledge_of_server", &last_knowledge_of_server.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}
