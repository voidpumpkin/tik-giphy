pub const ENV_KEY_BACKEND_HOST: &str = "BACKEND_HOST";
pub const ENV_KEY_DATABASE_URL: &str = "DATABASE_URL";
pub const ENV_KEY_PRIVATE_AUTH_KEY: &str = "PRIVATE_AUTH_KEY";
pub const ENV_KEY_EXPIRE_JWT_BEARER_IN_SEC: &str = "EXPIRE_JWT_BEARER_IN_SEC";

pub const RUST_LOG_INFO: &str = "INFO";
pub const DEFAULT_BACKEND_HOST: &str = "127.0.0.1:8081";
pub const DEFAULT_ENV_KEY_EXPIRE_JWT_BEARER_IN_SEC: i64 = 2 * 60;

pub const ERROR_MESSAGE_USERNAME_UNIQUE_VIOLATION: &str = "Specified username already exists";
pub const ERROR_MESSAGE_EMAIL_UNIQUE_VIOLATION: &str = "Specified email already exists";
pub const ERROR_MESSAGE_USER_NOT_FOUND: &str = "User not found";
pub const ERROR_MESSAGE_INCORRECT_PASSWORD: &str = "Incorrect password";

pub const USER_ROLE_BASIC: &str = "BASIC";

pub const JWT_TOKEN_TYPE_BEARER: &str = "Bearer";

pub const TITLE: &str = "title";
pub const USERNAME: &str = "username";
pub const EMAIL: &str = "email";
pub const ACCESS_TOKEN: &str = "access_token";
pub const TOKEN_TYPE: &str = "token_type";
pub const EXPIRES_IN: &str = "expires_in";
