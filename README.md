# Rust API client for ynab-api

Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.youneedabudget.com

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Build package: org.openapitools.codegen.languages.RustClientCodegen

## Installation

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```
    openapi = { path = "./generated" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.youneedabudget.com/v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AccountsApi* | [**get_account_by_id**](docs/AccountsApi.md#get_account_by_id) | **get** /budgets/{budget_id}/accounts/{account_id} | Single account
*AccountsApi* | [**get_accounts**](docs/AccountsApi.md#get_accounts) | **get** /budgets/{budget_id}/accounts | Account list
*BudgetsApi* | [**get_budget_by_id**](docs/BudgetsApi.md#get_budget_by_id) | **get** /budgets/{budget_id} | Single budget
*BudgetsApi* | [**get_budget_settings_by_id**](docs/BudgetsApi.md#get_budget_settings_by_id) | **get** /budgets/{budget_id}/settings | Budget Settings
*BudgetsApi* | [**get_budgets**](docs/BudgetsApi.md#get_budgets) | **get** /budgets | List budgets
*CategoriesApi* | [**get_categories**](docs/CategoriesApi.md#get_categories) | **get** /budgets/{budget_id}/categories | List categories
*CategoriesApi* | [**get_category_by_id**](docs/CategoriesApi.md#get_category_by_id) | **get** /budgets/{budget_id}/categories/{category_id} | Single category
*CategoriesApi* | [**get_month_category_by_id**](docs/CategoriesApi.md#get_month_category_by_id) | **get** /budgets/{budget_id}/months/{month}/categories/{category_id} | Single category for a specific budget month
*CategoriesApi* | [**update_month_category**](docs/CategoriesApi.md#update_month_category) | **patch** /budgets/{budget_id}/months/{month}/categories/{category_id} | Update a category for a specific month
*DeprecatedApi* | [**bulk_create_transactions**](docs/DeprecatedApi.md#bulk_create_transactions) | **post** /budgets/{budget_id}/transactions/bulk | Bulk create transactions
*MonthsApi* | [**get_budget_month**](docs/MonthsApi.md#get_budget_month) | **get** /budgets/{budget_id}/months/{month} | Single budget month
*MonthsApi* | [**get_budget_months**](docs/MonthsApi.md#get_budget_months) | **get** /budgets/{budget_id}/months | List budget months
*PayeeLocationsApi* | [**get_payee_location_by_id**](docs/PayeeLocationsApi.md#get_payee_location_by_id) | **get** /budgets/{budget_id}/payee_locations/{payee_location_id} | Single payee location
*PayeeLocationsApi* | [**get_payee_locations**](docs/PayeeLocationsApi.md#get_payee_locations) | **get** /budgets/{budget_id}/payee_locations | List payee locations
*PayeeLocationsApi* | [**get_payee_locations_by_payee**](docs/PayeeLocationsApi.md#get_payee_locations_by_payee) | **get** /budgets/{budget_id}/payees/{payee_id}/payee_locations | List locations for a payee
*PayeesApi* | [**get_payee_by_id**](docs/PayeesApi.md#get_payee_by_id) | **get** /budgets/{budget_id}/payees/{payee_id} | Single payee
*PayeesApi* | [**get_payees**](docs/PayeesApi.md#get_payees) | **get** /budgets/{budget_id}/payees | List payees
*ScheduledTransactionsApi* | [**get_scheduled_transaction_by_id**](docs/ScheduledTransactionsApi.md#get_scheduled_transaction_by_id) | **get** /budgets/{budget_id}/scheduled_transactions/{scheduled_transaction_id} | Single scheduled transaction
*ScheduledTransactionsApi* | [**get_scheduled_transactions**](docs/ScheduledTransactionsApi.md#get_scheduled_transactions) | **get** /budgets/{budget_id}/scheduled_transactions | List scheduled transactions
*TransactionsApi* | [**create_transaction**](docs/TransactionsApi.md#create_transaction) | **post** /budgets/{budget_id}/transactions | Create a single transaction or multiple transactions
*TransactionsApi* | [**get_transaction_by_id**](docs/TransactionsApi.md#get_transaction_by_id) | **get** /budgets/{budget_id}/transactions/{transaction_id} | Single transaction
*TransactionsApi* | [**get_transactions**](docs/TransactionsApi.md#get_transactions) | **get** /budgets/{budget_id}/transactions | List transactions
*TransactionsApi* | [**get_transactions_by_account**](docs/TransactionsApi.md#get_transactions_by_account) | **get** /budgets/{budget_id}/accounts/{account_id}/transactions | List account transactions
*TransactionsApi* | [**get_transactions_by_category**](docs/TransactionsApi.md#get_transactions_by_category) | **get** /budgets/{budget_id}/categories/{category_id}/transactions | List category transactions
*TransactionsApi* | [**get_transactions_by_payee**](docs/TransactionsApi.md#get_transactions_by_payee) | **get** /budgets/{budget_id}/payees/{payee_id}/transactions | List payee transactions
*TransactionsApi* | [**update_transaction**](docs/TransactionsApi.md#update_transaction) | **put** /budgets/{budget_id}/transactions/{transaction_id} | Updates an existing transaction
*TransactionsApi* | [**update_transactions**](docs/TransactionsApi.md#update_transactions) | **patch** /budgets/{budget_id}/transactions | Update multiple transactions
*UserApi* | [**get_user**](docs/UserApi.md#get_user) | **get** /user | User info


## Documentation For Models

 - [Account](docs/Account.md)
 - [AccountResponse](docs/AccountResponse.md)
 - [AccountResponseData](docs/AccountResponseData.md)
 - [AccountsResponse](docs/AccountsResponse.md)
 - [AccountsResponseData](docs/AccountsResponseData.md)
 - [BudgetDetail](docs/BudgetDetail.md)
 - [BudgetDetailAllOf](docs/BudgetDetailAllOf.md)
 - [BudgetDetailResponse](docs/BudgetDetailResponse.md)
 - [BudgetDetailResponseData](docs/BudgetDetailResponseData.md)
 - [BudgetSettings](docs/BudgetSettings.md)
 - [BudgetSettingsResponse](docs/BudgetSettingsResponse.md)
 - [BudgetSettingsResponseData](docs/BudgetSettingsResponseData.md)
 - [BudgetSummary](docs/BudgetSummary.md)
 - [BudgetSummaryResponse](docs/BudgetSummaryResponse.md)
 - [BudgetSummaryResponseData](docs/BudgetSummaryResponseData.md)
 - [BulkResponse](docs/BulkResponse.md)
 - [BulkResponseData](docs/BulkResponseData.md)
 - [BulkResponseDataBulk](docs/BulkResponseDataBulk.md)
 - [BulkTransactions](docs/BulkTransactions.md)
 - [CategoriesResponse](docs/CategoriesResponse.md)
 - [CategoriesResponseData](docs/CategoriesResponseData.md)
 - [Category](docs/Category.md)
 - [CategoryGroup](docs/CategoryGroup.md)
 - [CategoryGroupWithCategories](docs/CategoryGroupWithCategories.md)
 - [CategoryGroupWithCategoriesAllOf](docs/CategoryGroupWithCategoriesAllOf.md)
 - [CategoryResponse](docs/CategoryResponse.md)
 - [CategoryResponseData](docs/CategoryResponseData.md)
 - [CurrencyFormat](docs/CurrencyFormat.md)
 - [DateFormat](docs/DateFormat.md)
 - [ErrorDetail](docs/ErrorDetail.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [HybridTransaction](docs/HybridTransaction.md)
 - [HybridTransactionAllOf](docs/HybridTransactionAllOf.md)
 - [HybridTransactionsResponse](docs/HybridTransactionsResponse.md)
 - [HybridTransactionsResponseData](docs/HybridTransactionsResponseData.md)
 - [MonthDetail](docs/MonthDetail.md)
 - [MonthDetailAllOf](docs/MonthDetailAllOf.md)
 - [MonthDetailResponse](docs/MonthDetailResponse.md)
 - [MonthDetailResponseData](docs/MonthDetailResponseData.md)
 - [MonthSummariesResponse](docs/MonthSummariesResponse.md)
 - [MonthSummariesResponseData](docs/MonthSummariesResponseData.md)
 - [MonthSummary](docs/MonthSummary.md)
 - [Payee](docs/Payee.md)
 - [PayeeLocation](docs/PayeeLocation.md)
 - [PayeeLocationResponse](docs/PayeeLocationResponse.md)
 - [PayeeLocationResponseData](docs/PayeeLocationResponseData.md)
 - [PayeeLocationsResponse](docs/PayeeLocationsResponse.md)
 - [PayeeLocationsResponseData](docs/PayeeLocationsResponseData.md)
 - [PayeeResponse](docs/PayeeResponse.md)
 - [PayeeResponseData](docs/PayeeResponseData.md)
 - [PayeesResponse](docs/PayeesResponse.md)
 - [PayeesResponseData](docs/PayeesResponseData.md)
 - [SaveCategoryResponse](docs/SaveCategoryResponse.md)
 - [SaveCategoryResponseData](docs/SaveCategoryResponseData.md)
 - [SaveMonthCategory](docs/SaveMonthCategory.md)
 - [SaveMonthCategoryWrapper](docs/SaveMonthCategoryWrapper.md)
 - [SaveTransaction](docs/SaveTransaction.md)
 - [SaveTransactionWrapper](docs/SaveTransactionWrapper.md)
 - [SaveTransactionsResponse](docs/SaveTransactionsResponse.md)
 - [SaveTransactionsResponseData](docs/SaveTransactionsResponseData.md)
 - [SaveTransactionsWrapper](docs/SaveTransactionsWrapper.md)
 - [ScheduledSubTransaction](docs/ScheduledSubTransaction.md)
 - [ScheduledTransactionDetail](docs/ScheduledTransactionDetail.md)
 - [ScheduledTransactionDetailAllOf](docs/ScheduledTransactionDetailAllOf.md)
 - [ScheduledTransactionResponse](docs/ScheduledTransactionResponse.md)
 - [ScheduledTransactionResponseData](docs/ScheduledTransactionResponseData.md)
 - [ScheduledTransactionSummary](docs/ScheduledTransactionSummary.md)
 - [ScheduledTransactionsResponse](docs/ScheduledTransactionsResponse.md)
 - [ScheduledTransactionsResponseData](docs/ScheduledTransactionsResponseData.md)
 - [SubTransaction](docs/SubTransaction.md)
 - [TransactionDetail](docs/TransactionDetail.md)
 - [TransactionDetailAllOf](docs/TransactionDetailAllOf.md)
 - [TransactionResponse](docs/TransactionResponse.md)
 - [TransactionResponseData](docs/TransactionResponseData.md)
 - [TransactionSummary](docs/TransactionSummary.md)
 - [TransactionsResponse](docs/TransactionsResponse.md)
 - [TransactionsResponseData](docs/TransactionsResponseData.md)
 - [UpdateTransaction](docs/UpdateTransaction.md)
 - [UpdateTransactionWrapper](docs/UpdateTransactionWrapper.md)
 - [UpdateTransactionsResponse](docs/UpdateTransactionsResponse.md)
 - [UpdateTransactionsWrapper](docs/UpdateTransactionsWrapper.md)
 - [User](docs/User.md)
 - [UserResponse](docs/UserResponse.md)
 - [UserResponseData](docs/UserResponseData.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



Jesse Luehrs <doy@tozt.net>

Based on the OpenAPI spec found at https://github.com/ynab/ynab-sdk-js/blob/master/spec-v1-swagger.json by You Need A Budget, LLC