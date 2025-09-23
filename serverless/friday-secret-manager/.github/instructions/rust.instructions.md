---
applyTo: '**/*.rs'
---
# Rust Microservices Architecture Guide - FridayAssistant

This document defines the mandatory patterns, conventions, and architectural rules for Rust microservices in the FridayAssistant ecosystem.

## Project Structure Laws

**LAW 1: Domain Module Organization**
- Every service SHALL have a single domain module named `{domain}_mod/`
- The domain module SHALL contain exactly these files: `mod.rs`, `{domain}_controller.rs`, `{domain}_logic.rs`, `{domain}_list.rs`
- External service integrations SHALL be named `{service_name}_api.rs` within the domain module

**LAW 2: Root Module Requirements** 
- Every service SHALL have these root modules: `main.rs`, `load_env.rs`, `openapi.rs`, `business_response.rs`
- No other organization patterns are permitted at the root level

**LAW 3: File Naming Convention**
- ALL Rust files SHALL use snake_case naming
- Controller files SHALL end with `_controller.rs`
- Business logic files SHALL end with `_logic.rs`
- Domain model files SHALL end with `_list.rs`
- External HTTP API clients SHALL end with `_api.rs`
- DTO files SHALL be named `{dto_name}.rs` and placed in appropriate module folders
- Each DTO SHALL have its own file for maintainability

## Naming Convention Laws

**LAW 4: Function Naming Standards**
- ALL functions SHALL use snake_case
- CRUD operations SHALL use prefixes: `get_`, `create_`, `update_`, `delete_`
- Business operations SHALL use descriptive verbs: `generate_`, `load_`, `refresh_`

**LAW 5: Struct Naming Standards**
- ALL structs SHALL use PascalCase
- Request DTOs SHALL end with `Request`
- Response DTOs SHALL end with `Response`
- HTTP API client structs SHALL end with `Api`

**LAW 6: Instrumentation Naming**
- Tracing instrument names SHALL include layer prefix: `logic_`, `data_`, `oauth_`
- Controller instruments SHALL use endpoint names without prefix

## Response Pattern Laws

**LAW 7: Universal Response Structure**
- ALL endpoints SHALL return `Response<T>` from `business_response.rs`
- The Response struct SHALL have exactly three fields: `success: bool`, `data: Option<T>`, `errors: Vec<String>`
- Successful operations SHALL return HTTP 200 with `Response::success(data)`
- Business failures SHALL return HTTP 400 with `Response::error(message)`
- System failures SHALL return HTTP 500 with `Response::error(generic_message)`
- NO other response wrapper patterns are permitted

**LAW 8: Response Creation Standards**
- Success responses SHALL use `Response::success(data)`
- Error responses SHALL use `Response::error(message)`
- Generic constructor SHALL only be used when both data and errors exist

## Error Handling Laws

**LAW 9: Proper Result-Based Error Strategy**
- Controller layer SHALL convert `Err()` results to appropriate HTTP status codes with Response wrapper
- Controller layer SHALL convert `Ok(data)` results to HTTP 200 with Response::success
- Logic layer SHALL return `Result<T, BusinessError>` where T is the success type
- Logic layer SHALL use custom error types for business validation failures
- Data layer SHALL return `Result<T, E>` where E represents the actual error type
- System errors SHALL propagate up through Result::Err until converted at controller boundary

**LAW 10: Error Logging Requirements**
- System errors SHALL be logged at ERROR level with full details
- Business failures SHALL be logged at WARN level
- Successful operations SHALL be logged at INFO level

## Environment Configuration Laws

**LAW 11: Environment Loading Pattern**
- Environment variables SHALL be loaded through `load_env_variables()` function
- Production detection SHALL use `!cfg!(debug_assertions)`
- Environment struct SHALL derive `Debug` and `Deserialize`
- Global configuration SHALL use `once_cell::sync::Lazy` pattern

**LAW 12: Environment File Requirements**
- Development environment SHALL load from `.env`
- Production environment SHALL load from `.env.prod`
- Environment loading SHALL be fail-safe with `.ok()` chaining

## Logging and Tracing Laws

**LAW 13: Log Level Standards**
- Production environments SHALL use INFO level by default
- Development environments SHALL use DEBUG level by default
- Log configuration SHALL disable timestamps and targets for containerized deployment

## Business Logic Layer Laws

**LAW 14: Input Validation Requirements**
- ALL inputs SHALL be validated before processing
- Empty or whitespace-only strings SHALL be rejected
- Validation failures SHALL return business errors, not system errors

**LAW 15: External Service Integration**
- OAuth token retrieval SHALL precede ALL external API calls
- External service failures SHALL be converted to business errors
- Network failures SHALL be propagated as system errors

## Controller Layer Laws

**LAW 16: HTTP Endpoint Standards**
- ALL endpoints SHALL have OpenAPI documentation with `#[utoipa::path]`
- ALL endpoints SHALL use appropriate HTTP method attributes
- Path parameters SHALL be extracted using `web::Path<String>`

**LAW 17: Response Status Mapping**
- Successful operations SHALL return HTTP 200
- Business logic failures SHALL return HTTP 400 or 404 as appropriate
- System errors SHALL return HTTP 500
- Response bodies SHALL ALWAYS contain the `Response<T>` structure

## Data Layer Laws

**LAW 18: External HTTP API Client Structure**
- External HTTP API clients SHALL be structs with `client` and `base_url` fields
- HTTP client SHALL be `reqwest::Client`
- Base URLs SHALL be configurable through environment variables with defaults

**LAW 19: API Response Mapping**
- External API responses SHALL have separate structs from internal domain models
- Mapping SHALL be implemented using `From` trait
- Missing optional fields SHALL use `Option<T>` types

## OpenAPI Documentation Laws

**LAW 20: API Documentation Requirements**
- ALL schemas used in endpoints SHALL be listed in OpenAPI components
- Documentation SHALL include service description and version
- Tags SHALL group related endpoints logically
- Swagger UI SHALL be accessible at `/swagger/{_:.*}` path

**LAW 21: Schema Registration**
- Response types SHALL include generic variations: `Response<Entity>`, `Response<Vec<Entity>>`
- Request DTOs SHALL be registered as separate schemas
- Domain models SHALL derive `ToSchema` from utoipa

## Application Setup Laws

**LAW 22: Main Function Requirements**
- HTTP server SHALL bind to `0.0.0.0` for containerized deployment
- Worker count SHALL be fixed at 4 for consistent performance

**LAW 23: Service Registration**
- ALL controller endpoints SHALL be registered individually with `.service()`
- Swagger configuration SHALL be registered as a service
- Root health check SHALL be available at `/` path

These laws define the non-negotiable patterns for Rust microservices. Adherence ensures consistency, maintainability, and operational excellence across all services.