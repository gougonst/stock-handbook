pub const MONGODB_CONN_STR_ENV: &str = "MONGODB_CONNECTION_STRING";
pub const DATABASE_NAME: &str = "stock_handnote";

pub const USER_COLL_NAME: &str = "users";
pub const USER_COLL_USERNAME_COL: &str = "username";
pub const USER_COLL_PASSWORD_COL: &str = "password";

pub const GET_MONGODB_CONN_STR_ENV_FAIL: &str = "Get mongodb connection string from environment variable failed";
pub const INIT_DB_ERR: &str = "Init database error";

pub const HTTP_OK: &str = "Ok";
pub const HTTP_USER_NOT_FOUND: &str = "User not found";
pub const HTTP_USER_PASSWORD_INCORRECT: &str = "User password incorrect";
pub const HTTP_USER_ALREADY_EXIST: &str = "User already exist";
pub const HTTP_INTERNAL_ERROR: &str = "Internal server error";
