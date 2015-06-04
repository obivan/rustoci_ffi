//! Call bindings to Oracle OCI

extern crate libc;

pub use libc::{c_void, c_ushort, c_ulong, c_uchar, c_char, c_uint, c_int};
use std::ffi::CStr;
use std::error;
use std::fmt;
use std::ptr;

/// Opaque pointer to OCIEnv
#[repr(C)]
pub struct OCIEnv;

/// Opaque pointer to OCIError
#[repr(C)]
pub struct OCIError;

/// Opaque pointer to OCISvcCtx
#[repr(C)]
pub struct OCISvcCtx;

/// Opaque pointer to OCIServer
#[repr(C)]
pub struct OCIServer;

/// Opaque pointer to OCISession
#[repr(C)]
pub struct OCISession;

/// Opaque pointer to OCIStmt
#[repr(C)]
pub struct OCIStmt;

/// Opaque pointer to OCISnapshot
#[repr(C)]
struct OCISnapshot;

/// OCI Mode type.
/// Used in [`oci_env_nls_create`](fn.oci_env_nls_create.html),
#[allow(dead_code)]
pub enum OCIMode {
    /// `OCI_DEFAULT`. The default value, which is non-UTF-16 encoding.
    Default = 0x00000000,

    /// `OCI_THREADED`. Uses threaded environment.
    /// Internal data structures not exposed to the user are protected from concurrent
    /// accesses by multiple threads.
    Threaded = 0x00000001,

    /// `OCI_OBJECT`. Uses object features.
    Object = 0x00000002,

    /// `OCI_EVENTS`. Uses publish-subscribe notifications.
    Events = 0x00000004,

    /// `OCI_NO_UCB`. Suppresses the calling of the dynamic callback routine `OCIEnvCallback()`.
    /// The default behavior is to allow calling of `OCIEnvCallback()` when the environment
    /// is created.
    NoUcb = 0x00000040,

    /// `OCI_NO_MUTEX`. No mutual exclusion (mutex) locking occurs in this mode.
    /// All OCI calls done on the environment handle, or on handles derived from the environment
    /// handle, must be serialized.
    /// `Threaded` must also be specified when `OCI_NO_MUTEX` is specified.
    NoMutex = 0x00000080,

    /// `OCI_SUPPRESS_NLS_VALIDATION`. Suppresses NLS character validation;
    /// NLS character validation suppression is on by default beginning with
    /// Oracle Database 11g Release 1 (11.1). Use `EnableNLSValidation` to
    /// enable NLS character validation.
    SuppressNLSValidation = 0x00100000,

    /// `OCI_NCHAR_LITERAL_REPLACE_ON`. Turns on N' substitution.
    NcharLiteralReplaceOn = 0x00400000,

    /// `OCI_NCHAR_LITERAL_REPLACE_OFF`. Turns off N' substitution.
    /// If neither this mode nor `NcharLiteralReplaceOn` is used, the substitution is
    /// determined by the environment variable `ORA_NCHAR_LITERAL_REPLACE`, which can be set
    /// to `TRUE` or `FALSE`. When it is set to `TRUE`, the replacement is turned on; otherwise
    /// it is turned off, which is the default setting in OCI.
    NcharLiteralReplaceOff = 0x00800000,

    /// `OCI_ENABLE_NLS_VALIDATION`. Enables NLS character validation.
    EnableNLSValidation = 0x01000000,
}

/// Represent Oracle error.
#[derive(Debug)]
pub struct OracleError {
    /// Oracle error code.
    code:     isize,
    /// Message.
    message:  String,
    /// Function where the error occurred.
    location: String,
}

impl fmt::Display for OracleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!{f, "\n\n  Error code: {}\n  Error message: {}\n  Where: {}\n\n",
               self.code, self.message, self.location}
    }
}

impl error::Error for OracleError {
    fn description(&self) -> &str {
        "Oracle error"
    }
}

/// Type of handle
#[allow(dead_code)]
pub enum OCIHandleType {
    /// `OCI_HTYPE_ENV`
    Environment = 1,

    /// `OCI_HTYPE_ERROR`
    Error       = 2,

    /// `OCI_HTYPE_SVCCTX`
    Service     = 3,

    /// `OCI_HTYPE_STMT`
    Statement   = 4,

    /// `OCI_HTYPE_BIND`
    Bind        = 5,

    /// `OCI_HTYPE_DEFINE`
    Define      = 6,

    /// `OCI_HTYPE_DESCRIBE`
    Describe    = 7,

    /// `OCI_HTYPE_SERVER`
    Server      = 8,

    /// `OCI_HTYPE_SESSION`
    Session     = 9,

    /// `OCI_HTYPE_TRANS`
    Transaction = 10,
}

/// Type of credentials
#[allow(dead_code)]
pub enum OCICredentialsType {
    /// `OCI_CRED_RDBMS`
    Rdbms    = 1,

    /// `OCI_CRED_EXT`
    External = 2,
}

/// Type of authentication mode
#[allow(dead_code)]
pub enum OCIAuthMode {
    /// `OCI_DEFAULT`
    Default    = 0x00000000,

    /// `OCI_MIGRATE`
    Migrate    = 0x00000001,

    /// `OCI_SYSDBA`
    Sysdba     = 0x00000002,

    /// `OCI_SYSOPER`
    Sysoper    = 0x00000004,

    /// `OCI_PRELIM_AUTH`
    PrelimAuth = 0x00000008,

    /// `OCI_STMT_CACHE`
    StmtCache  = 0x00000040,
}

/// Type of syntax
enum OCISyntax {
    /// `OCI_NTV_SYNTAX`
    NtvSyntax = 1,
}

/// Type of OCIStmtPrepare2 mode
enum OCIStmtPrepare2Mode {
    /// `OCI_DEFAULT`
    Default = 0x00000000,
}

/// Type if OCI Attribute
pub enum OCIAttribute {
    /// `OCI_ATTR_SERVER`
    /// 
    /// Mode: READ/WRITE
    /// 
    /// When read, returns the pointer to the server context attribute of the service context.
    /// When changed, sets the server context attribute of the service context.
    /// Attribute Data Type: OCIServer ** / OCIServer *
    Server = 6,

    /// `OCI_ATTR_SESSION`
    /// 
    /// Mode: READ/WRITE
    /// 
    /// When read, returns the pointer to the authentication context attribute of
    /// the service context.
    /// When changed, sets the authentication context attribute of the service context.
    /// Attribute Data Type: OCISession **/ OCISession *
    Session = 7,

    /// `OCI_ATTR_USERNAME`
    /// 
    /// Mode: READ/WRITE
    /// 
    /// Specifies a user name to use for authentication.
    /// Attribute Data Type: oratext **/oratext * [oratext = c_uchar]
    Username = 22,

    /// `OCI_ATTR_PASSWORD`
    /// 
    /// Mode: WRITE
    /// 
    /// Specifies a password to use for authentication.
    /// Attribute Data Type: oratext * [oratext = c_uchar]
    Password = 23,
}

/// Type of descriptor
#[allow(dead_code)]
enum OCIDescriptorType {
    /// `OCI_DTYPE_PARAM`
    Parameter = 53,
}

/// Type of describe attribute
#[allow(dead_code)]
pub enum OCIDescribeAttribute {
    /// `OCI_ATTR_DATA_SIZE`: maximum size of the data
    DataSize = 1,

    /// `OCI_ATTR_DATA_TYPE`: the SQL type of the column/argument
    DataType = 2,

    /// `OCI_ATTR_DISP_SIZE`: the display size
    DisplaySize = 3,

    /// `OCI_ATTR_NAME`: the name of the column/argument
    Name = 4,

    /// `OCI_ATTR_PRECISION`: precision if number type
    Precision = 5,

    /// `OCI_ATTR_SCALE`: scale if number type
    Scale = 6,

    /// `OCI_ATTR_IS_NULL`: is it null?
    IsNull = 7,

    /// `OCI_ATTR_CHAR_USED`: char length semantics
    CharUsed = 285,

    /// `OCI_ATTR_CHAR_SIZE`: char length
    CharLength = 286,
}

/// Oracle datatype
#[allow(dead_code)]
enum OCIDataType {
    /// `SQLT_CHR`: (ORANET TYPE) character string
    Char = 1,

    /// `SQLT_DATE`: ANSI Date
    Date = 184,

    /// `SQLT_TIMESTAMP`: `TIMESTAMP`
    Timestamp = 187,

    /// `SQLT_TIMESTAMP_TZ`: `TIMESTAMP WITH TIME ZONE`
    TimestampWithTz  = 188,

    /// `SQLT_TIMESTAMP_LTZ`: `TIMESTAMP WITH LOCAL TZ`
    TimestampWithLocalTz = 232,

    /// `SQLT_INTERVAL_YM`: `INTERVAL YEAR TO MONTH`
    IntervalYearToMonth = 189,

    /// `SQLT_INTERVAL_DS`: `INTERVAL DAY TO SECOND`
    IntervalDayToSecond = 190,

    /// `SQLT_CLOB`: character lob
    Clob = 112,

    /// `SQLT_BLOB`: binary lob
    Blob = 113,

    /// `SQLT_INT`: (ORANET TYPE) integer
    Int = 3,

    /// `SQLT_UIN`: unsigned integer
    Uint = 68,

    /// `SQLT_FLT`: (ORANET TYPE) Floating point number
    Float = 4,

    /// `SQLT_PDN`: (ORANET TYPE) Packed Decimal Numeric
    PackedDecimalNumber = 7,

    /// `SQLT_BIN`: binary data (DTYBIN)
    Binary = 23,

    /// `SQLT_NUM`: (ORANET TYPE) oracle numeric
    Numeric = 2,

    /// `SQLT_NTY`: named object type
    NamedObject = 108,

    /// `SQLT_REF`: ref type
    Ref = 110,

    /// `SQLT_VST`: OCIString type
    OCIString = 155,

    /// `SQLT_VNU`: NUM with preceding length byte
    NumericWithLength = 6,
}

#[link(name = "clntsh")]
extern "C" {
    fn OCIEnvNlsCreate(envp: *mut *mut OCIEnv, mode: c_uint, ctxp: *mut c_void,
        malocfp: Option<extern "C" fn (ctxp: *mut c_void, size: c_ulong) -> *mut c_void>,
        ralocfp: Option<extern "C" fn (ctxp: c_void, memptr: c_void, newsize: c_ulong) -> *mut c_void>,
        mfreefp: Option<extern "C" fn (ctxp: *mut c_void, memptr: *mut c_void)>,
        xtramem_sz: c_ulong, usrmempp: *mut *mut c_void, charset: c_ushort,
        ncharset: c_ushort) -> c_int;

    fn OCIHandleAlloc(parenth: *const c_void, hndlpp: *mut *mut c_void, _type: c_uint,
                      xtramem_sz: c_ulong, usrmempp: *mut *mut c_void) -> c_int;

    fn OCIServerAttach(srvhp: *mut OCIServer, errhp: *mut OCIError, dblink: *const c_uchar,
                       dblink_len: c_int, mode: c_uint) -> c_int;

    fn OCIErrorGet(hndlp: *mut c_void, recordno: c_uint, sqlstate: *mut c_uchar,
                   errcodep: *mut c_int, bufp: *mut c_uchar, bufsiz: c_uint, _type: c_uint) -> c_int;

    fn OCIAttrSet(trgthndlp: *mut c_void, trghndltyp: c_uint, attributep: *mut c_void, size: c_uint,
                  attrtype: c_uint, errhp: *mut OCIError) -> c_int;

    fn OCISessionBegin(svchp: *mut OCISvcCtx, errhp: *mut OCIError, usrhp: *mut OCISession,
                       credt: c_uint, mode: c_uint) -> c_int;

    fn OCISessionEnd(svchp: *mut OCISvcCtx, errhp: *mut OCIError,
                     usrhp: *mut OCISession, mode: c_uint) -> c_int;

    fn OCIServerDetach(srvhp: *mut OCIServer, errhp: *mut OCIError, mode: c_uint) -> c_int;

    fn OCIHandleFree(hndlp: *mut c_void, _type: c_uint) -> c_int;

    fn OCIStmtPrepare2(svchp: *mut OCISvcCtx, stmtp: *mut *mut OCIStmt, errhp: *mut OCIError,
                       stmt: *const c_uchar, stmt_len: c_uint, key: *const c_uchar, key_len: c_uint,
                       language: c_uint, mode: c_uint) -> c_int;

    fn OCIStmtExecute(svchp: *mut OCISvcCtx, stmtp: *mut OCIStmt, errhp: *mut OCIError,
                      iters: c_uint, rowoff: c_uint, snap_in: *const OCISnapshot,
                      snap_out: *mut OCISnapshot, mode: c_uint) -> c_int;

    fn OCIStmtRelease(stmtp: *mut OCIStmt, errhp: *mut OCIError, key: *const c_uchar,
                      key_len: c_uint, mode: c_uint) -> c_int;

    fn OCIParamGet(hndlp: *const c_void, htype: c_uint, errhp: *mut OCIError,
                   parmdpp: *mut *mut c_void, pos: c_uint) -> c_int;

    fn OCIAttrGet(trgthndlp: *const c_void, trghndltyp: c_uint, attributep: *mut c_void,
                  sizep: *mut c_uint, attrtype: c_uint, errhp: *mut OCIError) -> c_int;
}

/// Binds [`OCIEnvNlsCreate()`](http://docs.oracle.com/cd/E11882_01/appdev.112/e10646/oci16rel001.htm#LNOCI17114).
pub fn oci_env_nls_create(mode: OCIMode) -> Result<*mut OCIEnv, OracleError> {
    let mut handle = ptr::null_mut();
    let res = unsafe {
        OCIEnvNlsCreate(
            &mut handle,     // envp
            mode as c_uint,  // mode
            ptr::null_mut(), // ctxp
            None,            // malocfp
            None,            // ralocfp
            None,            // mfreefp
            0,               // xtramem_sz
            ptr::null_mut(), // usrmempp
            0,               // charset
            0                // ncharset
        )
    };
    match check_error(res, None, "ffi::oci_env_nls_create") {
        None      => Ok(handle),
        Some(err) => Err(err),
    }
}

/// Binds [`OCIHandleAlloc()`](http://docs.oracle.com/cd/E11882_01/appdev.112/e10646/oci16rel002.htm#LNOCI17134).
pub fn oci_handle_alloc(envh: *mut OCIEnv,
                        htype: OCIHandleType) -> Result<*mut c_void, OracleError> {
    let mut handle = ptr::null_mut();
    let res = unsafe {
        OCIHandleAlloc(
            envh as *const _, // parenth
            &mut handle,      // hndlpp
            htype as c_uint,  // type
            0,                // xtramem_sz
            ptr::null_mut()   // usrmempp
        )
    };
    match check_error(res, None, "ffi::oci_handle_alloc") {
        None => Ok(handle),
        Some(err) => Err(err),
    }
}

/// Binds [`OCIServerAttach()`](http://docs.oracle.com/cd/E11882_01/appdev.112/e10646/oci16rel001.htm#LNOCI17119).
pub fn oci_server_attach(server_handle: *mut OCIServer,
                         error_handle: *mut OCIError,
                         db: String,
                         mode: OCIMode) -> Result<(), OracleError> {
    let res = unsafe {
        OCIServerAttach(
            server_handle,                 // srvhp
            error_handle,                  // errhp
            db.as_ptr() as *const c_uchar, // dblink
            db.len() as c_int,             // dblink_len
            mode as c_uint                 // mode
        )
    };
    match check_error(res, Some(error_handle), "ffi::oci_server_attach") {
        None => Ok(()),
        Some(err) => Err(err),
    }
}

/// Binds [`OCIErrorGet()`](http://docs.oracle.com/cd/E11882_01/appdev.112/e10646/oci17msc007.htm#LNOCI17287).
pub fn oci_error_get(error_handle: *mut OCIError, location: &str) -> OracleError {
    let errc: *mut isize = &mut 0;
    let buf = String::with_capacity(3072);
    unsafe {
        OCIErrorGet(
            error_handle as *mut c_void,                                  // hndlp
            1,                                                            // recordno
            ptr::null_mut(),                                              // sqlstate
            errc as *mut c_int,                                           // errcodep
            buf.as_ptr() as *mut c_uchar,                                 // bufp
            buf.capacity() as c_uint,                                     // bufsiz
            OCIHandleType::Error as c_uint                                // type
        )
    };
    OracleError {code: unsafe { *errc }, message: buf, location: location.to_string()}
}

/// Binds [`OCIAttrSet()`](http://docs.oracle.com/cd/E11882_01/appdev.112/e10646/oci16rel002.htm#LNOCI17131).
pub fn oci_attr_set(handle: *mut c_void,
                    htype: OCIHandleType,
                    value: *mut c_void,
                    attr_type: OCIAttribute,
                    error_handle: *mut OCIError) -> Result<(), OracleError> {
    let size: c_uint = match attr_type {
        OCIAttribute::Username | OCIAttribute::Password => unsafe {
            CStr::from_ptr(value as *const c_char).to_bytes().len() as c_uint
        },
        _ => 0,
    };
    let res = unsafe {
        OCIAttrSet(
            handle,              // trgthndlp
            htype as c_uint,     // trghndltyp
            value,               // attributep
            size,                // size
            attr_type as c_uint, // attrtype
            error_handle         // errhp
        )
    };
    match check_error(res, Some(error_handle), "ffi::oci_attr_set") {
        None => Ok(()),
        Some(err) => Err(err),
    }
}

/// Binds [`OCISessionBegin()`](http://docs.oracle.com/cd/E11882_01/appdev.112/e10646/oci16rel001.htm#LNOCI17121).
pub fn oci_session_begin(service_handle: *mut OCISvcCtx,
                         error_handle: *mut OCIError,
                         session_handle: *mut OCISession,
                         credentials_type: OCICredentialsType,
                         mode: OCIAuthMode) -> Result<(), OracleError> {
    let res = unsafe {
        OCISessionBegin(
            service_handle,             // svchp
            error_handle,               // errhp
            session_handle,             // usrhp
            credentials_type as c_uint, // credt
            mode as c_uint              // mode
        )
    };
    match check_error(res, Some(error_handle), "ffi::oci_session_begin") {
        None => Ok(()),
        Some(err) => Err(err),
    }
}

/// Binds [`OCISessionEnd()`](http://docs.oracle.com/cd/E11882_01/appdev.112/e10646/oci16rel001.htm#LNOCI17122).
pub fn oci_session_end(service_handle: *mut OCISvcCtx,
                       error_handle: *mut OCIError,
                       session_handle: *mut OCISession) -> Result<(), OracleError> {
    let res = unsafe {
        OCISessionEnd(
            service_handle,                // svchp
            error_handle,                  // errhp
            session_handle,                // usrhp
            OCIAuthMode::Default as c_uint // mode
        )
    };
    match check_error(res, Some(error_handle), "ffi::oci_session_end") {
        None => Ok(()),
        Some(err) => Err(err),
    }
}

/// Binds [`OCIServerDetach()`](http://docs.oracle.com/cd/E11882_01/appdev.112/e10646/oci16rel001.htm#LNOCI17120).
pub fn oci_server_detach(server_handle: *mut OCIServer,
                         error_handle: *mut OCIError) -> Result<(), OracleError> {
    let res = unsafe {
        OCIServerDetach(server_handle, error_handle, OCIMode::Default as c_uint)
    };
    match check_error(res, Some(error_handle), "ffi::oci_server_detach") {
        None => Ok(()),
        Some(err) => Err(err),
    }
}

/// Binds [`OCIHandleFree()`](http://docs.oracle.com/cd/E11882_01/appdev.112/e10646/oci16rel002.htm#LNOCI17135).
pub fn oci_handle_free(handle: *mut c_void, htype: OCIHandleType) -> Result<(), OracleError> {
    let res = unsafe {
        OCIHandleFree(handle, htype as c_uint)
    };
    match check_error(res, None, "ffi::oci_handle_free") {
        None => Ok(()),
        Some(err) => Err(err),
    }
}

/// Binds [`OCIStmtPrepare2()`](http://docs.oracle.com/cd/E11882_01/appdev.112/e10646/oci17msc001.htm#LNOCI17168).
pub fn oci_stmt_prepare2(service_handle: *mut OCISvcCtx,
                         error_handle: *mut OCIError,
                         stmt_text: &String,
                         stmt_hash: &String) -> Result<*mut OCIStmt, OracleError> {
    let mut stmt_handle = ptr::null_mut();
    let res = unsafe {
        OCIStmtPrepare2(
            service_handle,                        // svchp
            &mut stmt_handle,                      // stmtp
            error_handle,                          // errhp
            stmt_text.as_ptr(),                    // stmttext
            stmt_text.len() as c_uint,             // stmt_len
            stmt_hash.as_ptr(),                    // key
            stmt_hash.len() as c_uint,             // key_len
            OCISyntax::NtvSyntax as c_uint,        // language
            OCIStmtPrepare2Mode::Default as c_uint // mode
        )
    };
    match check_error(res, Some(error_handle), "ffi::oci_stmt_prepare2") {
        None => Ok(stmt_handle),
        Some(err) => Err(err),
    }
}

/// Binds [`OCIStmtExecute()`](http://docs.oracle.com/cd/E11882_01/appdev.112/e10646/oci17msc001.htm#LNOCI17163).
pub fn oci_stmt_execute(service_handle: *mut OCISvcCtx,
                        stmt_handle: *mut OCIStmt,
                        error_handle: *mut OCIError) -> Result<(), OracleError> {
    let res = unsafe {
        OCIStmtExecute(
            service_handle,            // svchp
            stmt_handle,               // stmtp
            error_handle,              // errhp
            0 as c_uint,               // iters
            0 as c_uint,               // rowoff
            ptr::null(),               // snap_in
            ptr::null_mut(),           // snap_out
            OCIMode::Default as c_uint // mode
        )
    };
    match check_error(res, Some(error_handle), "ffi::oci_stmt_execute") {
        None => Ok(()),
        Some(err) => Err(err),
    }
}

/// Binds [`OCIStmtRelease()`](http://docs.oracle.com/cd/E11882_01/appdev.112/e10646/oci17msc001.htm#LNOCI17169).
pub fn oci_stmt_release(stmt_handle: *mut OCIStmt,
                        error_handle: *mut OCIError,
                        stmt_hash: &String) -> Result<(), OracleError> {
    let res = unsafe {
        OCIStmtRelease(
            stmt_handle,               // stmtp
            error_handle,              // errhp
            stmt_hash.as_ptr(),        // key
            stmt_hash.len() as c_uint, // keylen
            OCIMode::Default as c_uint // mode
        )
    };
    match check_error(res, Some(error_handle), "ffi::oci_stmt_release") {
        None => Ok(()),
        Some(err) => Err(err),
    }
}

/// Binds [`OCIParamGet()`](http://docs.oracle.com/cd/E11882_01/appdev.112/e10646/oci16rel002.htm#LNOCI17136).
pub fn oci_param_get(stmt_handle: *mut OCIStmt,
                     error_handle: *mut OCIError,
                     position: usize) -> Result<*mut c_void, OracleError> {
    let mut parameter_descriptor = ptr::null_mut();
    let res = unsafe {
        OCIParamGet(
            stmt_handle as *const _,            // hndlp
            OCIHandleType::Statement as c_uint, // htype
            error_handle,                       // errhp
            &mut parameter_descriptor,          // parmdpp
            position as c_uint                  // pos
        )
    };
    match check_error(res, Some(error_handle), "ffi::oci_param_get") {
        None => Ok(parameter_descriptor),
        Some(err) => Err(err),
    }
}

/// Binds [`OCIAttrGet()`](http://docs.oracle.com/cd/E11882_01/appdev.112/e10646/oci16rel002.htm#LNOCI17130).
pub fn oci_attr_get(attr_handle: *mut c_void,
                    error_handle: *mut OCIError,
                    attr_type: OCIDescribeAttribute) -> Result<(*mut c_void, isize), OracleError> {
    let attribute = ptr::null_mut();
    let mut attribute_size = 0;
    let res = unsafe {
        OCIAttrGet(
            attr_handle as *const _,                // trgthndlp
            OCIDescriptorType::Parameter as c_uint, // trghndltyp
            attribute,                              // attributep
            &mut attribute_size,                    // sizep
            attr_type as c_uint,                    // attrtype
            error_handle                            // errhp
        )
    };
    match check_error(res, Some(error_handle), "ffi::oci_attr_get") {
        None => Ok((attribute, attribute_size as isize)),
        Some(err) => Err(err),
    }
}

/// Convert oracle error codes to [`OracleError`](struct.OracleError.html).
pub fn check_error(code: c_int,
                   error_handle: Option<*mut OCIError>,
                   location: &str) -> Option<OracleError> {
    let by_handle = match error_handle {
        Some(handle) => Some(oci_error_get(handle, location)),
        None         => None,
    };
    match code {
        0     => None,
        100   => Some(OracleError {
            code: code as isize, message: "No data".to_string(), location: location.to_string()
        }),
        -2    => Some(OracleError {
            code: code as isize, message: "Invalid handle".to_string(), location: location.to_string()
        }),
        99    => Some(OracleError {
            code: code as isize, message: "Need data".to_string(), location: location.to_string()
        }),
        -3123 => Some(OracleError {
            code: code as isize, message: "Still executing".to_string(),
            location: location.to_string()
        }),
        -1    => Some(by_handle.unwrap_or(OracleError {
            code: code as isize, message: "Error with no details".to_string(),
            location: location.to_string()
        })),
        1     => Some(by_handle.unwrap_or(OracleError {
            code: code as isize, message: "Success with info".to_string(),
            location: location.to_string()
        })),
        _     => panic!("Unknown return code"),
    }
}
