/// This is ODBC 3 Rust interface
/// Author: Hossein Noroozpour
/// Email: Hossein.Noroozpour@GMail.com
use std;
use std::collections::HashMap;
// primary types
type SQLSMALLINT = ::std::os::raw::c_short;
type SQLHANDLE = *mut ::std::os::raw::c_void;
type SQLUSMALLINT = ::std::os::raw::c_ushort;
type SQLCHAR = ::std::os::raw::c_uchar;
type SQLINTEGER = ::std::os::raw::c_int;
type SQLPOINTER = *mut ::std::os::raw::c_void;
// secondary types
type SQLHENV = SQLHANDLE;
type SQLHDBC = SQLHANDLE;
type SQLHSTMT = SQLHANDLE;
type SQLHDESC = SQLHANDLE;
type SQLRETURN = SQLSMALLINT;
// constants
const SQL_SUCCESS: SQLRETURN = 0;
const SQL_NO_DATA_FOUND: SQLRETURN = 100;
const SQL_SUCCESS_WITH_INFO: SQLRETURN = 1;
const SQL_ERROR: SQLRETURN = -1;
const SQL_INVALID_HANDLE: SQLRETURN = -2;
const SQL_NO_DATA: SQLRETURN = 100;

const SQL_FETCH_NEXT: SQLUSMALLINT = 1;

const SQL_HANDLE_ENV: SQLSMALLINT = 1;
const SQL_HANDLE_DBC: SQLSMALLINT = 2;
const SQL_HANDLE_STMT: SQLSMALLINT = 3;
const SQL_HANDLE_DESC: SQLSMALLINT = 4;

const SQL_NULL_HANDLE: SQLHANDLE = 0 as SQLHANDLE;

const SQL_ATTR_ODBC_VERSION: SQLINTEGER = 200;

const SQL_OV_ODBC3: SQLINTEGER = 3;

const SQL_NTS: SQLSMALLINT = -3;
pub const SQL_NTSL: SQLINTEGER = -3;

const SQL_DRIVER_NOPROMPT: SQLUSMALLINT = 0;

#[cfg_attr(target_os = "linux", link(name = "odbc", kind= "dylib"))]
#[cfg_attr(target_os = "windows", link(name = "odbc32", kind= "dylib"))]
extern "C" {
    fn SQLAllocHandle(
        handle_type: SQLSMALLINT,
        input_handle: SQLHANDLE,
        output_handle: *mut SQLHANDLE) -> SQLRETURN;
    fn SQLSetEnvAttr(
        environment_handle: SQLHENV,
        attribute: SQLINTEGER,
        value: SQLPOINTER,
        string_length: SQLINTEGER) -> SQLRETURN;
    fn SQLDrivers(
        henv: SQLHENV,
        f_direction: SQLUSMALLINT,
        sz_driver_desc: *mut SQLCHAR,
        cb_driver_desc_max: SQLSMALLINT,
        pcb_driver_desc: *mut SQLSMALLINT,
        sz_driver_attributes: *mut SQLCHAR,
        cb_drvr_attr_max: SQLSMALLINT,
        pcb_drvr_attr: *mut SQLSMALLINT) -> SQLRETURN;
    pub fn SQLDriverConnect(
        hdbc: SQLHDBC,
        hwnd: SQLHANDLE,
        sz_conn_str_in: *mut SQLCHAR,
        cb_conn_str_in: SQLSMALLINT,
        sz_conn_str_out: *mut SQLCHAR,
        cb_conn_str_out_max: SQLSMALLINT,
        pcb_conn_str_out: *mut SQLSMALLINT,
        f_driver_completion: SQLUSMALLINT) -> SQLRETURN;
    pub fn SQLExecDirect(
        statement_handle: SQLHSTMT,
        statement_text: *mut SQLCHAR,
        text_length: SQLINTEGER) -> SQLRETURN;
    pub fn SQLNumResultCols(
        statement_handle: SQLHSTMT,
        column_count: *mut SQLSMALLINT) -> SQLRETURN;
}

#[derive(Debug, Clone, Copy)]
pub struct Environment {
    environment: SQLHENV,
}

// TODO: implement Drop

#[derive(Debug, Clone)]
pub struct DriverInfo {
    name: String,
    description: String,
}

pub trait Driver {

}

pub struct Database {
    connection: SQLHDBC,
    statement: SQLHSTMT,
    query: ColumnsDescriptions,
}

struct ColumnDescription {
    name: String,
}

struct ColumnsDescriptions {
    descriptions: Vec<ColumnDescription>,
    name_to_id: HashMap<String, usize>,
}

impl Environment {
    pub fn new() -> Result<Self, String> {
        let mut env: SQLHENV = std::ptr::null_mut();
        let res = unsafe { SQLAllocHandle(SQL_HANDLE_ENV, SQL_NULL_HANDLE, &mut env) };
        if res != SQL_SUCCESS {
            return Err("Error in initializing environment!".to_string());
        }
        let res = unsafe {
            SQLSetEnvAttr(
                env,
                SQL_ATTR_ODBC_VERSION,
                SQL_OV_ODBC3 as SQLPOINTER,
                0 as SQLINTEGER)
        };
        if res != SQL_SUCCESS {
            return Err("Error ODBC version 3 interface does not exist!".to_string());
        }
        Ok(
            Environment {
                environment: env
            }
        )
    }

    pub fn get_drivers_info(&mut self) -> Result<Vec<DriverInfo>, String> {
        let mut name = [0i8; 2048];
        let mut name_ret : SQLSMALLINT = 0;
        let mut desc = [0i8; 2048];
        let mut desc_ret : SQLSMALLINT = 0;
        let mut infos = Vec::new();
        unsafe {
            loop {
                let ret = SQLDrivers(
                    self.environment, SQL_FETCH_NEXT,
                    name.as_mut_ptr() as *mut SQLCHAR, name.len() as SQLSMALLINT, &mut name_ret,
                    desc.as_mut_ptr() as *mut SQLCHAR, desc.len() as SQLSMALLINT, &mut desc_ret);
                if ret == SQL_NO_DATA {
                    break;
                } else if ret != SQL_SUCCESS && ret != SQL_SUCCESS_WITH_INFO  {
                    return Err("Error in fetching driver info!".to_string());
                }
                let n = match std::ffi::CStr::from_ptr(name.as_ptr()).to_str() {
                    Ok(s) => s.to_string(),
                    Err(_) => return Err("Error in utf parsing of driver name!".to_string()),
                };
                let d = match std::ffi::CStr::from_ptr(desc.as_ptr()).to_str() {
                    Ok(s) => s.to_string(),
                    Err(_) => return Err("Error in utf parsing of driver description!".to_string()),
                };
                infos.push(
                    DriverInfo {
                        name: n,
                        description: d,
                    }
                );
            }
        }
        Ok(infos)
    }
}

impl DriverInfo {
    pub fn to_string(&self) -> String {
        format!("{} [{}]", self.name, self.description)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Database {
    pub fn new(env: &'a mut Environment, connection_string: &String) -> Result<Self, String> {
        let mut connection = unsafe {
            let mut connection: SQLHDBC = std::ptr::null_mut();
            connection
        };

        let mut statement = unsafe {
            let mut statement: SQLHSTMT = std::ptr::null_mut();
            statement
        };

        let res = unsafe {
            SQLAllocHandle(SQL_HANDLE_DBC, env.environment, &mut connection as *mut SQLHDBC)
        };
        if res != SQL_SUCCESS {
            return Err("Error failed to allocate handle of database connection!".to_string());
        }
        let connection_string = std::ffi::CString::new(connection_string.as_str())
            .expect("Unexpected behavior!");

        let res = unsafe {
            SQLDriverConnect(
                connection,
                SQL_NULL_HANDLE,
                connection_string.as_ptr() as *mut u8,
                SQL_NTS,
                SQL_NULL_HANDLE as *mut u8,
                0 as SQLSMALLINT,
                SQL_NULL_HANDLE as *mut SQLSMALLINT,
                SQL_DRIVER_NOPROMPT)
        };
        if res == SQL_ERROR {
            return Err("Error failed to connect to driver of database connection!".to_string());
        }

        let res = unsafe {
            SQLAllocHandle(SQL_HANDLE_STMT, connection, &mut statement as *mut SQLHSTMT)
        };
        if res != SQL_SUCCESS {
            return Err("Error failed to allocate handle of statement!".to_string());
        }

        return Ok(Database {
            environment: env,
            connection: connection,
            statement: statement,
            query: None,
        });

    }
}

impl Query {
    pub fn new(database: &'a mut Database<'a>, s: &String) -> Result<&Self, String> {
        let cs = std::ffi::CString::new(s.as_str()).unwrap();
        let res = unsafe {
            SQLExecDirect(database.statement, cs.as_ptr() as *mut SQLCHAR, SQL_NTSL)
        };
        // TODO: Add some warning showing maybe a good macro
        if res != SQL_SUCCESS && res != SQL_SUCCESS_WITH_INFO {
            return Err("Error in executing query!".to_string());
        }
        let mut columns : SQLSMALLINT = 0;
        let res = unsafe {
            SQLNumResultCols(database.statement, &mut columns)
        };
        if res != SQL_SUCCESS && res != SQL_SUCCESS_WITH_INFO || columns < 0 {
            return Err("Error in getting number of rows!".to_string());
        }
        database.query = Some(Query {
            database: database,
        });
        return Ok(database.query.as_ref().unwrap());
    }
}
