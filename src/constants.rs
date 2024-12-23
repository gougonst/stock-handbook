pub const PORT_ENV: &str = "PORT";
pub const MONGODB_CONN_STR_ENV: &str = "MONGODB_CONNECTION_STRING";
pub const DATABASE_NAME: &str = "stock_handnote";
pub const CORS_DOMAIN: &str = "http://192.168.1.148:8080";

pub const USER_COLL_NAME: &str = "users";
pub const USER_COLL_USERNAME_COL: &str = "username";

pub const RECORD_COLL_NAME: &str = "stock_records";
pub const RECORD_COLL_ACTION_COL: &str = "action";
pub const RECORD_COLL_TRANSACTION_PRICE_COL: &str = "transaction_price";
pub const RECORD_COLL_CODE_COL: &str = "code";
pub const RECORD_COLL_CURRENT_PRICE_COL: &str = "current_price";
pub const RECORD_COLL_DATE_COL: &str = "date";
pub const RECORD_COLL_ID_COL: &str = "_id";
pub const RECORD_COLL_SHARES_COL: &str = "shares";
pub const RECORD_COLL_USERNAME_COL: &str = "username";

pub const ACTION_ADD: &str = "Add";
pub const ACTION_DELETE: &str = "Delete";

pub const TRANSACTION_BUY: &str = "buy";
pub const TRANSACTION_SELL: &str = "sell";

pub const GET_MONGODB_CONN_STR_ENV_FAIL: &str =
    "Get mongodb connection string from environment variable failed";
pub const INIT_DB_ERR: &str = "Init database error";

pub const HTTP_OK: &str = "Ok";
pub const HTTP_USER_NOT_FOUND: &str = "User not found";
pub const HTTP_USER_PASSWORD_INCORRECT: &str = "User password incorrect";
pub const HTTP_USER_ALREADY_EXIST: &str = "User already exist";
pub const HTTP_INTERNAL_ERROR: &str = "Internal server error";
