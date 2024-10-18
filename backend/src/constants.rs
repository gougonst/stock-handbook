pub const MONGODB_CONN_STR_ENV: &str = "MONGODB_CONNECTION_STRING";
pub const DATABASE_NAME: &str = "stock_handnote";
pub const CORS_DOMAIN: &str = "http://192.168.1.148:8080";

pub const USER_COLL_NAME: &str = "users";
pub const USER_COLL_USERNAME_COL: &str = "username";

pub const STOCK_COLL_NAME: &str = "stocks";
pub const STOCK_COLL_BUY_PRICE_COL: &str = "buy_price";
pub const STOCK_COLL_CODE_COL: &str = "code";
pub const STOCK_COLL_CURRENT_PRICE_COL: &str = "current_price";
pub const STOCK_COLL_DATE_COL: &str = "date";
pub const STOCK_COLL_ID_COL: &str = "_id";
pub const STOCK_COLL_SHARES_COL: &str = "shares";
pub const STOCK_COLL_USERNAME_COL: &str = "username";

pub const GET_MONGODB_CONN_STR_ENV_FAIL: &str =
    "Get mongodb connection string from environment variable failed";
pub const INIT_DB_ERR: &str = "Init database error";

pub const HTTP_OK: &str = "Ok";
pub const HTTP_USER_NOT_FOUND: &str = "User not found";
pub const HTTP_USER_PASSWORD_INCORRECT: &str = "User password incorrect";
pub const HTTP_USER_ALREADY_EXIST: &str = "User already exist";
pub const HTTP_INTERNAL_ERROR: &str = "Internal server error";
