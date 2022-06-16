//! TDengine error codes.
//! THIS IS AUTO GENERATED FROM TDENGINE <taoserror.h>, MAKE SURE YOU KNOW WHAT YOU ARE CHANING.

use std::fmt;

use num_enum::{FromPrimitive, IntoPrimitive};

/// TDengine error code.
#[derive(Debug, Clone, Copy, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
#[derive(serde::Deserialize)]
pub enum TaosCode {
    /// Success, 0
    Success = 0x0000,
    /// RPC_ACTION_IN_PROGRESS: Action in progress"
    RpcActionInProgress = 0x0001,
    /// RPC_AUTH_REQUIRED: Authentication required"
    RpcAuthRequired = 0x0002,
    /// RPC_AUTH_FAILURE: Authentication failure"
    RpcAuthFailure = 0x0003,
    /// RPC_REDIRECT: Redirect"
    RpcRedirect = 0x0004,
    /// RPC_NOT_READY: System not ready"    // peer is not ready to process data
    RpcNotReady = 0x0005,
    /// RPC_ALREADY_PROCESSED: Message already processed"
    RpcAlreadyProcessed = 0x0006,
    /// RPC_LAST_SESSION_NOT_FINISHED: Last session not finished"
    RpcLastSessionNotFinished = 0x0007,
    /// RPC_MISMATCHED_LINK_ID: Mismatched meter id"
    RpcMismatchedLinkId = 0x0008,
    /// RPC_TOO_SLOW: Processing of request timed out"
    RpcTooSlow = 0x0009,
    /// RPC_MAX_SESSIONS: Number of sessions reached limit"    // too many sessions
    RpcMaxSessions = 0x000A,
    /// RPC_NETWORK_UNAVAIL: Unable to establish connection"
    RpcNetworkUnavail = 0x000B,
    /// RPC_APP_ERROR: Unexpected generic error in RPC"
    RpcAppError = 0x000C,
    /// RPC_UNEXPECTED_RESPONSE: Unexpected response"
    RpcUnexpectedResponse = 0x000D,
    /// RPC_INVALID_VALUE: Invalid value"
    RpcInvalidValue = 0x000E,
    /// RPC_INVALID_TRAN_ID: Invalid transaction id"
    RpcInvalidTranId = 0x000F,
    /// RPC_INVALID_SESSION_ID: Invalid session id"
    RpcInvalidSessionId = 0x0010,
    /// RPC_INVALID_MSG_TYPE: Invalid message type"
    RpcInvalidMsgType = 0x0011,
    /// RPC_INVALID_RESPONSE_TYPE: Invalid response type"
    RpcInvalidResponseType = 0x0012,
    /// RPC_INVALID_TIME_STAMP: Client and server's time is not synchronized"
    RpcInvalidTimeStamp = 0x0013,
    /// APP_NOT_READY: Database not ready"
    AppNotReady = 0x0014,
    /// RPC_FQDN_ERROR: Unable to resolve FQDN"
    RpcFqdnError = 0x0015,
    /// RPC_INVALID_VERSION: Invalid app version"
    RpcInvalidVersion = 0x0016,
    /// COM_OPS_NOT_SUPPORT: Operation not supported"
    ComOpsNotSupport = 0x0100,
    /// COM_MEMORY_CORRUPTED: Memory corrupted"
    ComMemoryCorrupted = 0x0101,
    /// COM_OUT_OF_MEMORY: Out of memory"
    ComOutOfMemory = 0x0102,
    /// COM_INVALID_CFG_MSG: Invalid config message"
    ComInvalidCfgMsg = 0x0103,
    /// COM_FILE_CORRUPTED: Data file corrupted"
    ComFileCorrupted = 0x0104,
    /// REF_NO_MEMORY: Ref out of memory"
    RefNoMemory = 0x0105,
    /// REF_FULL: too many Ref Objs"
    RefFull = 0x0106,
    /// REF_ID_REMOVED: Ref ID is removed"
    RefIdRemoved = 0x0107,
    /// REF_INVALID_ID: Invalid Ref ID"
    RefInvalidId = 0x0108,
    /// REF_ALREADY_EXIST: Ref is already there"
    RefAlreadyExist = 0x0109,
    /// REF_NOT_EXIST: Ref is not there"
    RefNotExist = 0x010A,
    /// TSC_INVALID_OPERATION: Invalid Operation")
    TscInvalidOperation = 0x0200,
    /// TSC_INVALID_QHANDLE: Invalid qhandle")
    TscInvalidQhandle = 0x0201,
    /// TSC_INVALID_TIME_STAMP: Invalid combination of client/service time")
    TscInvalidTimeStamp = 0x0202,
    /// TSC_INVALID_VALUE: Invalid value in client")
    TscInvalidValue = 0x0203,
    /// TSC_INVALID_VERSION: Invalid client version")
    TscInvalidVersion = 0x0204,
    /// TSC_INVALID_IE: Invalid client ie")
    TscInvalidIe = 0x0205,
    /// TSC_INVALID_FQDN: Invalid host name")
    TscInvalidFqdn = 0x0206,
    /// TSC_INVALID_USER_LENGTH: Invalid user name")
    TscInvalidUserLength = 0x0207,
    /// TSC_INVALID_PASS_LENGTH: Invalid password")
    TscInvalidPassLength = 0x0208,
    /// TSC_INVALID_DB_LENGTH: Database name too long")
    TscInvalidDbLength = 0x0209,
    /// TSC_INVALID_TABLE_ID_LENGTH: Table name too long")
    TscInvalidTableIdLength = 0x020A,
    /// TSC_INVALID_CONNECTION: Invalid connection")
    TscInvalidConnection = 0x020B,
    /// TSC_OUT_OF_MEMORY: System out of memory")
    TscOutOfMemory = 0x020C,
    /// TSC_NO_DISKSPACE: System out of disk space")
    TscNoDiskspace = 0x020D,
    /// TSC_QUERY_CACHE_ERASED: Query cache erased")
    TscQueryCacheErased = 0x020E,
    /// TSC_QUERY_CANCELLED: Query terminated")
    TscQueryCancelled = 0x020F,
    /// TSC_SORTED_RES_TOO_MANY: Result set too large to be sorted")      // too many result for ordered super table projection query
    TscSortedResTooMany = 0x0210,
    /// TSC_APP_ERROR: Application error")
    TscAppError = 0x0211,
    /// TSC_ACTION_IN_PROGRESS: Action in progress")
    TscActionInProgress = 0x0212,
    /// TSC_DISCONNECTED: Disconnected from service")
    TscDisconnected = 0x0213,
    /// TSC_NO_WRITE_AUTH: No write permission")
    TscNoWriteAuth = 0x0214,
    /// TSC_CONN_KILLED: Connection killed")
    TscConnKilled = 0x0215,
    /// TSC_SQL_SYNTAX_ERROR: Syntax error in SQL")
    TscSqlSyntaxError = 0x0216,
    /// TSC_DB_NOT_SELECTED: Database not specified or available")
    TscDbNotSelected = 0x0217,
    /// TSC_INVALID_TABLE_NAME: Table does not exist")
    TscInvalidTableName = 0x0218,
    /// TSC_EXCEED_SQL_LIMIT: SQL statement too long check maxSQLLength config")
    TscExceedSqlLimit = 0x0219,
    /// TSC_FILE_EMPTY: File is empty")
    TscFileEmpty = 0x021A,
    /// TSC_LINE_SYNTAX_ERROR: Syntax error in Line")
    TscLineSyntaxError = 0x021B,
    /// TSC_NO_META_CACHED: No table meta cached")
    TscNoMetaCached = 0x021C,
    /// TSC_DUP_COL_NAMES: duplicated column names")
    TscDupColNames = 0x021D,
    /// TSC_INVALID_TAG_LENGTH: Invalid tag length")
    TscInvalidTagLength = 0x021E,
    /// TSC_INVALID_COLUMN_LENGTH: Invalid column length")
    TscInvalidColumnLength = 0x021F,
    /// TSC_DUP_TAG_NAMES: duplicated tag names")
    TscDupTagNames = 0x0220,
    /// TSC_INVALID_JSON: Invalid JSON format")
    TscInvalidJson = 0x0221,
    /// TSC_INVALID_JSON_TYPE: Invalid JSON data type")
    TscInvalidJsonType = 0x0222,
    /// TSC_INVALID_JSON_CONFIG: Invalid JSON configuration")
    TscInvalidJsonConfig = 0x0223,
    /// TSC_VALUE_OUT_OF_RANGE: Value out of range")
    TscValueOutOfRange = 0x0224,
    /// TSC_INVALID_PROTOCOL_TYPE: Invalid line protocol type")
    TscInvalidProtocolType = 0x0225,
    /// TSC_INVALID_PRECISION_TYPE: Invalid timestamp precision type")
    TscInvalidPrecisionType = 0x0226,
    /// TSC_RES_TOO_MANY: Result set too large to be output")
    TscResTooMany = 0x0227,
    /// TSC_INVALID_SCHEMA_VERSION: invalid table schema version")
    TscInvalidSchemaVersion = 0x0228,
    /// MND_MSG_NOT_PROCESSED: Message not processed"
    MndMsgNotProcessed = 0x0300,
    /// MND_ACTION_IN_PROGRESS: Message is progressing"
    MndActionInProgress = 0x0301,
    /// MND_ACTION_NEED_REPROCESSED: Messag need to be reprocessed"
    MndActionNeedReprocessed = 0x0302,
    /// MND_NO_RIGHTS: Insufficient privilege for operation"
    MndNoRights = 0x0303,
    /// MND_APP_ERROR: Unexpected generic error in mnode"
    MndAppError = 0x0304,
    /// MND_INVALID_CONNECTION: Invalid message connection"
    MndInvalidConnection = 0x0305,
    /// MND_INVALID_MSG_VERSION: Incompatible protocol version"
    MndInvalidMsgVersion = 0x0306,
    /// MND_INVALID_MSG_LEN: Invalid message length"
    MndInvalidMsgLen = 0x0307,
    /// MND_INVALID_MSG_TYPE: Invalid message type"
    MndInvalidMsgType = 0x0308,
    /// MND_TOO_MANY_SHELL_CONNS: Too many connections"
    MndTooManyShellConns = 0x0309,
    /// MND_OUT_OF_MEMORY: Out of memory in mnode"
    MndOutOfMemory = 0x030A,
    /// MND_INVALID_SHOWOBJ: Data expired"
    MndInvalidShowobj = 0x030B,
    /// MND_INVALID_QUERY_ID: Invalid query id"
    MndInvalidQueryId = 0x030C,
    /// MND_INVALID_STREAM_ID: Invalid stream id"
    MndInvalidStreamId = 0x030D,
    /// MND_INVALID_CONN_ID: Invalid connection id"
    MndInvalidConnId = 0x030E,
    /// MND_MNODE_IS_RUNNING: mnode is already running"
    MndMnodeIsRunning = 0x0310,
    /// MND_FAILED_TO_CONFIG_SYNC: failed to config sync"
    MndFailedToConfigSync = 0x0311,
    /// MND_FAILED_TO_START_SYNC: failed to start sync"
    MndFailedToStartSync = 0x0312,
    /// MND_FAILED_TO_CREATE_DIR: failed to create mnode dir"
    MndFailedToCreateDir = 0x0313,
    /// MND_FAILED_TO_INIT_STEP: failed to init components"
    MndFailedToInitStep = 0x0314,
    /// MND_SDB_OBJ_ALREADY_THERE: Object already there"
    MndSdbObjAlreadyThere = 0x0320,
    /// MND_SDB_ERROR: Unexpected generic error in sdb"
    MndSdbError = 0x0321,
    /// MND_SDB_INVALID_TABLE_TYPE: Invalid table type"
    MndSdbInvalidTableType = 0x0322,
    /// MND_SDB_OBJ_NOT_THERE: Object not there"
    MndSdbObjNotThere = 0x0323,
    /// MND_SDB_INVAID_META_ROW: Invalid meta row"
    MndSdbInvaidMetaRow = 0x0324,
    /// MND_SDB_INVAID_KEY_TYPE: Invalid key type"
    MndSdbInvaidKeyType = 0x0325,
    /// MND_DNODE_ALREADY_EXIST: DNode already exists"
    MndDnodeAlreadyExist = 0x0330,
    /// MND_DNODE_NOT_EXIST: DNode does not exist"
    MndDnodeNotExist = 0x0331,
    /// MND_VGROUP_NOT_EXIST: VGroup does not exist"
    MndVgroupNotExist = 0x0332,
    /// MND_NO_REMOVE_MASTER: Master DNode cannot be removed"
    MndNoRemoveMaster = 0x0333,
    /// MND_NO_ENOUGH_DNODES: Out of DNodes"
    MndNoEnoughDnodes = 0x0334,
    /// MND_CLUSTER_CFG_INCONSISTENT: Cluster cfg inconsistent"
    MndClusterCfgInconsistent = 0x0335,
    /// MND_INVALID_DNODE_CFG_OPTION: Invalid dnode cfg option"
    MndInvalidDnodeCfgOption = 0x0336,
    /// MND_BALANCE_ENABLED: Balance already enabled"
    MndBalanceEnabled = 0x0337,
    /// MND_VGROUP_NOT_IN_DNODE: Vgroup not in dnode"
    MndVgroupNotInDnode = 0x0338,
    /// MND_VGROUP_ALREADY_IN_DNODE: Vgroup already in dnode"
    MndVgroupAlreadyInDnode = 0x0339,
    /// MND_DNODE_NOT_FREE: Dnode not avaliable"
    MndDnodeNotFree = 0x033A,
    /// MND_INVALID_CLUSTER_ID: Cluster id not match"
    MndInvalidClusterId = 0x033B,
    /// MND_NOT_READY: Cluster not ready"
    MndNotReady = 0x033C,
    /// MND_DNODE_ID_NOT_CONFIGURED: Dnode Id not configured"
    MndDnodeIdNotConfigured = 0x033D,
    /// MND_DNODE_EP_NOT_CONFIGURED: Dnode Ep not configured"
    MndDnodeEpNotConfigured = 0x033E,
    /// MND_ACCT_ALREADY_EXIST: Account already exists"
    MndAcctAlreadyExist = 0x0340,
    /// MND_INVALID_ACCT: Invalid account"
    MndInvalidAcct = 0x0341,
    /// MND_INVALID_ACCT_OPTION: Invalid account options"
    MndInvalidAcctOption = 0x0342,
    /// MND_ACCT_EXPIRED: Account authorization has expired"
    MndAcctExpired = 0x0343,
    /// MND_USER_ALREADY_EXIST: User already exists"
    MndUserAlreadyExist = 0x0350,
    /// MND_INVALID_USER: Invalid user"
    MndInvalidUser = 0x0351,
    /// MND_INVALID_USER_FORMAT: Invalid user format"
    MndInvalidUserFormat = 0x0352,
    /// MND_INVALID_PASS_FORMAT: Invalid password format"
    MndInvalidPassFormat = 0x0353,
    /// MND_NO_USER_FROM_CONN: Can not get user from conn"
    MndNoUserFromConn = 0x0354,
    /// MND_TOO_MANY_USERS: Too many users"
    MndTooManyUsers = 0x0355,
    /// MND_TABLE_ALREADY_EXIST: Table already exists"
    MndTableAlreadyExist = 0x0360,
    /// MND_INVALID_TABLE_ID: Table name too long"
    MndInvalidTableId = 0x0361,
    /// MND_INVALID_TABLE_NAME: Table does not exist"
    MndInvalidTableName = 0x0362,
    /// MND_INVALID_TABLE_TYPE: Invalid table type in tsdb"
    MndInvalidTableType = 0x0363,
    /// MND_TOO_MANY_TAGS: Too many tags"
    MndTooManyTags = 0x0364,
    /// MND_TOO_MANY_COLUMNS: Too many columns"
    MndTooManyColumns = 0x0365,
    /// MND_TOO_MANY_TIMESERIES: Too many time series"
    MndTooManyTimeseries = 0x0366,
    /// MND_NOT_SUPER_TABLE: Not super table"           // operation only available for super table
    MndNotSuperTable = 0x0367,
    /// MND_COL_NAME_TOO_LONG: Tag name too long"
    MndColNameTooLong = 0x0368,
    /// MND_TAG_ALREAY_EXIST: Tag already exists"
    MndTagAlreayExist = 0x0369,
    /// MND_TAG_NOT_EXIST: Tag does not exist"
    MndTagNotExist = 0x036A,
    /// MND_FIELD_ALREAY_EXIST: Field already exists"
    MndFieldAlreayExist = 0x036B,
    /// MND_FIELD_NOT_EXIST: Field does not exist"
    MndFieldNotExist = 0x036C,
    /// MND_INVALID_STABLE_NAME: Super table does not exist"
    MndInvalidStableName = 0x036D,
    /// MND_INVALID_CREATE_TABLE_MSG: Invalid create table message"
    MndInvalidCreateTableMsg = 0x036E,
    /// MND_EXCEED_MAX_ROW_BYTES: Exceed max row bytes"
    MndExceedMaxRowBytes = 0x036F,
    /// MND_INVALID_FUNC_NAME: Invalid func name"
    MndInvalidFuncName = 0x0370,
    /// MND_INVALID_FUNC_LEN: Invalid func length"
    MndInvalidFuncLen = 0x0371,
    /// MND_INVALID_FUNC_CODE: Invalid func code"
    MndInvalidFuncCode = 0x0372,
    /// MND_FUNC_ALREADY_EXIST: Func already exists"
    MndFuncAlreadyExist = 0x0373,
    /// MND_INVALID_FUNC: Invalid func"
    MndInvalidFunc = 0x0374,
    /// MND_INVALID_FUNC_BUFSIZE: Invalid func bufSize"
    MndInvalidFuncBufsize = 0x0375,
    /// MND_INVALID_TAG_LENGTH: invalid tag length"
    MndInvalidTagLength = 0x0376,
    /// MND_INVALID_COLUMN_LENGTH: invalid column length"
    MndInvalidColumnLength = 0x0377,
    /// MND_DB_NOT_SELECTED: Database not specified or available"
    MndDbNotSelected = 0x0380,
    /// MND_DB_ALREADY_EXIST: Database already exists"
    MndDbAlreadyExist = 0x0381,
    /// MND_INVALID_DB_OPTION: Invalid database options"
    MndInvalidDbOption = 0x0382,
    /// MND_INVALID_DB: Invalid database name"
    MndInvalidDb = 0x0383,
    /// MND_MONITOR_DB_FORBIDDEN: Cannot delete monitor database"
    MndMonitorDbForbidden = 0x0384,
    /// MND_TOO_MANY_DATABASES: Too many databases for account"
    MndTooManyDatabases = 0x0385,
    /// MND_DB_IN_DROPPING: Database not available"
    MndDbInDropping = 0x0386,
    /// MND_VGROUP_NOT_READY: Database unsynced"
    MndVgroupNotReady = 0x0387,
    /// MND_INVALID_DB_OPTION_DAYS: Invalid database option: days out of range"
    MndInvalidDbOptionDays = 0x0390,
    /// MND_INVALID_DB_OPTION_KEEP: Invalid database option: keep >= keep1 >= keep0 >= days"
    MndInvalidDbOptionKeep = 0x0391,
    /// MND_INVALID_TOPIC: Invalid topic name)
    MndInvalidTopic = 0x0392,
    /// MND_INVALID_TOPIC_OPTION: Invalid topic option)
    MndInvalidTopicOption = 0x0393,
    /// MND_INVALID_TOPIC_PARTITONS: Invalid topic partitons num, valid range: [1, 1000])
    MndInvalidTopicPartitons = 0x0394,
    /// MND_TOPIC_ALREADY_EXIST: Topic already exists)
    MndTopicAlreadyExist = 0x0395,
    /// DND_MSG_NOT_PROCESSED: Message not processed"
    DndMsgNotProcessed = 0x0400,
    /// DND_OUT_OF_MEMORY: Dnode out of memory"
    DndOutOfMemory = 0x0401,
    /// DND_NO_WRITE_ACCESS: No permission for disk files in dnode"
    DndNoWriteAccess = 0x0402,
    /// DND_INVALID_MSG_LEN: Invalid message length"
    DndInvalidMsgLen = 0x0403,
    /// DND_ACTION_IN_PROGRESS: Action in progress"
    DndActionInProgress = 0x0404,
    /// DND_TOO_MANY_VNODES: Too many vnode directories"
    DndTooManyVnodes = 0x0405,
    /// DND_EXITING: Dnode is exiting"
    DndExiting = 0x0406,
    /// DND_VNODE_OPEN_FAILED: Vnode open failed"
    DndVnodeOpenFailed = 0x0407,
    /// VND_ACTION_IN_PROGRESS: Action in progress"
    VndActionInProgress = 0x0500,
    /// VND_MSG_NOT_PROCESSED: Message not processed"
    VndMsgNotProcessed = 0x0501,
    /// VND_ACTION_NEED_REPROCESSED: Action need to be reprocessed"
    VndActionNeedReprocessed = 0x0502,
    /// VND_INVALID_VGROUP_ID: Invalid Vgroup ID"
    VndInvalidVgroupId = 0x0503,
    /// VND_INIT_FAILED: Vnode initialization failed"
    VndInitFailed = 0x0504,
    /// VND_NO_DISKSPACE: System out of disk space"
    VndNoDiskspace = 0x0505,
    /// VND_NO_DISK_PERMISSIONS: No write permission for disk files"
    VndNoDiskPermissions = 0x0506,
    /// VND_NO_SUCH_FILE_OR_DIR: Missing data file"
    VndNoSuchFileOrDir = 0x0507,
    /// VND_OUT_OF_MEMORY: Out of memory"
    VndOutOfMemory = 0x0508,
    /// VND_APP_ERROR: Unexpected generic error in vnode"
    VndAppError = 0x0509,
    /// VND_INVALID_VRESION_FILE: Invalid version file"
    VndInvalidVresionFile = 0x050A,
    /// VND_IS_FULL: Database memory is full for commit failed"
    VndIsFull = 0x050B,
    /// VND_IS_FLOWCTRL: Database memory is full for waiting commit"
    VndIsFlowctrl = 0x050C,
    /// VND_IS_DROPPING: Database is dropping"
    VndIsDropping = 0x050D,
    /// VND_IS_BALANCING: Database is balancing"
    VndIsBalancing = 0x050E,
    /// VND_IS_CLOSING: Database is closing"
    VndIsClosing = 0x0510,
    /// VND_NOT_SYNCED: Database suspended"
    VndNotSynced = 0x0511,
    /// VND_NO_WRITE_AUTH: Database write operation denied"
    VndNoWriteAuth = 0x0512,
    /// VND_IS_SYNCING: Database is syncing"
    VndIsSyncing = 0x0513,
    /// VND_INVALID_TSDB_STATE: Invalid tsdb state"
    VndInvalidTsdbState = 0x0514,
    /// WAIT_THREAD_TOO_MANY: Wait threads too many"
    WaitThreadTooMany = 0x0515,
    /// TDB_INVALID_TABLE_ID: Invalid table ID")
    TdbInvalidTableId = 0x0600,
    /// TDB_INVALID_TABLE_TYPE: Invalid table type")
    TdbInvalidTableType = 0x0601,
    /// TDB_IVD_TB_SCHEMA_VERSION: Invalid table schema version")
    TdbIvdTbSchemaVersion = 0x0602,
    /// TDB_TABLE_ALREADY_EXIST: Table already exists")
    TdbTableAlreadyExist = 0x0603,
    /// TDB_INVALID_CONFIG: Invalid configuration")
    TdbInvalidConfig = 0x0604,
    /// TDB_INIT_FAILED: Tsdb init failed")
    TdbInitFailed = 0x0605,
    /// TDB_NO_DISKSPACE: No diskspace for tsdb")
    TdbNoDiskspace = 0x0606,
    /// TDB_NO_DISK_PERMISSIONS: No permission for disk files")
    TdbNoDiskPermissions = 0x0607,
    /// TDB_FILE_CORRUPTED: Data file(s) corrupted")
    TdbFileCorrupted = 0x0608,
    /// TDB_OUT_OF_MEMORY: Out of memory")
    TdbOutOfMemory = 0x0609,
    /// TDB_TAG_VER_OUT_OF_DATE: Tag too old")
    TdbTagVerOutOfDate = 0x060A,
    /// TDB_TIMESTAMP_OUT_OF_RANGE: Timestamp data out of range")
    TdbTimestampOutOfRange = 0x060B,
    /// TDB_SUBMIT_MSG_MSSED_UP: Submit message is messed up")
    TdbSubmitMsgMssedUp = 0x060C,
    /// TDB_INVALID_ACTION: Invalid operation")
    TdbInvalidAction = 0x060D,
    /// TDB_INVALID_CREATE_TB_MSG: Invalid creation of table")
    TdbInvalidCreateTbMsg = 0x060E,
    /// TDB_NO_TABLE_DATA_IN_MEM: No table data in memory skiplist")
    TdbNoTableDataInMem = 0x060F,
    /// TDB_FILE_ALREADY_EXISTS: File already exists")
    TdbFileAlreadyExists = 0x0610,
    /// TDB_TABLE_RECONFIGURE: Need to reconfigure table")
    TdbTableReconfigure = 0x0611,
    /// TDB_IVD_CREATE_TABLE_INFO: Invalid information to create table")
    TdbIvdCreateTableInfo = 0x0612,
    /// TDB_NO_AVAIL_DISK: No available disk")
    TdbNoAvailDisk = 0x0613,
    /// TDB_MESSED_MSG: TSDB messed message")
    TdbMessedMsg = 0x0614,
    /// TDB_IVLD_TAG_VAL: TSDB invalid tag value")
    TdbIvldTagVal = 0x0615,
    /// TDB_NO_CACHE_LAST_ROW: TSDB no cache last row data")
    TdbNoCacheLastRow = 0x0616,
    /// TDB_INCOMPLETE_DFILESET: TSDB incomplete DFileSet")
    TdbIncompleteDfileset = 0x0617,
    /// QRY_INVALID_QHANDLE: Invalid handle")
    QryInvalidQhandle = 0x0700,
    /// QRY_INVALID_MSG: Invalid message")    // failed to validate the sql expression msg by vnode
    QryInvalidMsg = 0x0701,
    /// QRY_NO_DISKSPACE: No diskspace for query")
    QryNoDiskspace = 0x0702,
    /// QRY_OUT_OF_MEMORY: System out of memory")
    QryOutOfMemory = 0x0703,
    /// QRY_APP_ERROR: Unexpected generic error in query")
    QryAppError = 0x0704,
    /// QRY_DUP_JOIN_KEY: Duplicated join key")
    QryDupJoinKey = 0x0705,
    /// QRY_EXCEED_TAGS_LIMIT: Tag condition too many")
    QryExceedTagsLimit = 0x0706,
    /// QRY_NOT_READY: Query not ready")
    QryNotReady = 0x0707,
    /// QRY_HAS_RSP: Query should response")
    QryHasRsp = 0x0708,
    /// QRY_IN_EXEC: Multiple retrieval of this query")
    QryInExec = 0x0709,
    /// QRY_TOO_MANY_TIMEWINDOW: Too many time window in query")
    QryTooManyTimewindow = 0x070A,
    /// QRY_NOT_ENOUGH_BUFFER: Query buffer limit has reached")
    QryNotEnoughBuffer = 0x070B,
    /// QRY_INCONSISTAN: File inconsistency in replica")
    QryInconsistan = 0x070C,
    /// QRY_SYS_ERROR: System error")
    QrySysError = 0x070D,
    /// QRY_INVALID_TIME_CONDITION: invalid time condition")
    QryInvalidTimeCondition = 0x070E,
    /// QRY_INVALID_SCHEMA_VERSION: invalid schema version")
    QryInvalidSchemaVersion = 0x0710,
    /// QRY_RESULT_TOO_LARGE: result num is too large")
    QryResultTooLarge = 0x0711,
    /// GRANT_EXPIRED: License expired"
    GrantExpired = 0x0800,
    /// GRANT_DNODE_LIMITED: DNode creation limited by licence"
    GrantDnodeLimited = 0x0801,
    /// GRANT_ACCT_LIMITED: Account creation limited by license"
    GrantAcctLimited = 0x0802,
    /// GRANT_TIMESERIES_LIMITED: Table creation limited by license"
    GrantTimeseriesLimited = 0x0803,
    /// GRANT_DB_LIMITED: DB creation limited by license"
    GrantDbLimited = 0x0804,
    /// GRANT_USER_LIMITED: User creation limited by license"
    GrantUserLimited = 0x0805,
    /// GRANT_CONN_LIMITED: Conn creation limited by license"
    GrantConnLimited = 0x0806,
    /// GRANT_STREAM_LIMITED: Stream creation limited by license"
    GrantStreamLimited = 0x0807,
    /// GRANT_SPEED_LIMITED: Write speed limited by license"
    GrantSpeedLimited = 0x0808,
    /// GRANT_STORAGE_LIMITED: Storage capacity limited by license"
    GrantStorageLimited = 0x0809,
    /// GRANT_QUERYTIME_LIMITED: Query time limited by license"
    GrantQuerytimeLimited = 0x080A,
    /// GRANT_CPU_LIMITED: CPU cores limited by license"
    GrantCpuLimited = 0x080B,
    /// SYN_INVALID_CONFIG: Invalid Sync Configuration"
    SynInvalidConfig = 0x0900,
    /// SYN_NOT_ENABLED: Sync module not enabled"
    SynNotEnabled = 0x0901,
    /// SYN_INVALID_VERSION: Invalid Sync version"
    SynInvalidVersion = 0x0902,
    /// SYN_CONFIRM_EXPIRED: Sync confirm expired"
    SynConfirmExpired = 0x0903,
    /// SYN_TOO_MANY_FWDINFO: Too many sync fwd infos"
    SynTooManyFwdinfo = 0x0904,
    /// SYN_MISMATCHED_PROTOCOL: Mismatched protocol"
    SynMismatchedProtocol = 0x0905,
    /// SYN_MISMATCHED_CLUSTERID: Mismatched clusterId"
    SynMismatchedClusterid = 0x0906,
    /// SYN_MISMATCHED_SIGNATURE: Mismatched signature"
    SynMismatchedSignature = 0x0907,
    /// SYN_INVALID_CHECKSUM: Invalid msg checksum"
    SynInvalidChecksum = 0x0908,
    /// SYN_INVALID_MSGLEN: Invalid msg length"
    SynInvalidMsglen = 0x0909,
    /// SYN_INVALID_MSGTYPE: Invalid msg type"
    SynInvalidMsgtype = 0x090A,
    /// WAL_APP_ERROR: Unexpected generic error in wal"
    WalAppError = 0x1000,
    /// WAL_FILE_CORRUPTED: WAL file is corrupted"
    WalFileCorrupted = 0x1001,
    /// WAL_SIZE_LIMIT: WAL size exceeds limit"
    WalSizeLimit = 0x1002,
    /// HTTP_SERVER_OFFLINE: http server is not online"
    HttpServerOffline = 0x1100,
    /// HTTP_UNSUPPORT_URL: url is not support"
    HttpUnsupportUrl = 0x1101,
    /// HTTP_INVALID_URL: nvalid url format"
    HttpInvalidUrl = 0x1102,
    /// HTTP_NO_ENOUGH_MEMORY: no enough memory"
    HttpNoEnoughMemory = 0x1103,
    /// HTTP_REQUSET_TOO_BIG: request size is too big"
    HttpRequsetTooBig = 0x1104,
    /// HTTP_NO_AUTH_INFO: no auth info input"
    HttpNoAuthInfo = 0x1105,
    /// HTTP_NO_MSG_INPUT: request is empty"
    HttpNoMsgInput = 0x1106,
    /// HTTP_NO_SQL_INPUT: no sql input"
    HttpNoSqlInput = 0x1107,
    /// HTTP_NO_EXEC_USEDB: no need to execute use db cmd"
    HttpNoExecUsedb = 0x1108,
    /// HTTP_SESSION_FULL: session list was full"
    HttpSessionFull = 0x1109,
    /// HTTP_GEN_TAOSD_TOKEN_ERR: generate taosd token error"
    HttpGenTaosdTokenErr = 0x110A,
    /// HTTP_INVALID_MULTI_REQUEST: size of multi request is 0"
    HttpInvalidMultiRequest = 0x110B,
    /// HTTP_CREATE_GZIP_FAILED: failed to create gzip"
    HttpCreateGzipFailed = 0x110C,
    /// HTTP_FINISH_GZIP_FAILED: failed to finish gzip"
    HttpFinishGzipFailed = 0x110D,
    /// HTTP_LOGIN_FAILED: failed to login"
    HttpLoginFailed = 0x110E,
    /// HTTP_INVALID_VERSION: invalid http version"
    HttpInvalidVersion = 0x1120,
    /// HTTP_INVALID_CONTENT_LENGTH: invalid content length"
    HttpInvalidContentLength = 0x1121,
    /// HTTP_INVALID_AUTH_TYPE: invalid type of Authorization"
    HttpInvalidAuthType = 0x1122,
    /// HTTP_INVALID_AUTH_FORMAT: invalid format of Authorization"
    HttpInvalidAuthFormat = 0x1123,
    /// HTTP_INVALID_BASIC_AUTH: invalid basic Authorization"
    HttpInvalidBasicAuth = 0x1124,
    /// HTTP_INVALID_TAOSD_AUTH: invalid taosd Authorization"
    HttpInvalidTaosdAuth = 0x1125,
    /// HTTP_PARSE_METHOD_FAILED: failed to parse method"
    HttpParseMethodFailed = 0x1126,
    /// HTTP_PARSE_TARGET_FAILED: failed to parse target"
    HttpParseTargetFailed = 0x1127,
    /// HTTP_PARSE_VERSION_FAILED: failed to parse http version"
    HttpParseVersionFailed = 0x1128,
    /// HTTP_PARSE_SP_FAILED: failed to parse sp"
    HttpParseSpFailed = 0x1129,
    /// HTTP_PARSE_STATUS_FAILED: failed to parse status"
    HttpParseStatusFailed = 0x112A,
    /// HTTP_PARSE_PHRASE_FAILED: failed to parse phrase"
    HttpParsePhraseFailed = 0x112B,
    /// HTTP_PARSE_CRLF_FAILED: failed to parse crlf"
    HttpParseCrlfFailed = 0x112C,
    /// HTTP_PARSE_HEADER_FAILED: failed to parse header"
    HttpParseHeaderFailed = 0x112D,
    /// HTTP_PARSE_HEADER_KEY_FAILED: failed to parse header key"
    HttpParseHeaderKeyFailed = 0x112E,
    /// HTTP_PARSE_HEADER_VAL_FAILED: failed to parse header val"
    HttpParseHeaderValFailed = 0x112F,
    /// HTTP_PARSE_CHUNK_SIZE_FAILED: failed to parse chunk size"
    HttpParseChunkSizeFailed = 0x1130,
    /// HTTP_PARSE_CHUNK_FAILED: failed to parse chunk"
    HttpParseChunkFailed = 0x1131,
    /// HTTP_PARSE_END_FAILED: failed to parse end section"
    HttpParseEndFailed = 0x1132,
    /// HTTP_PARSE_INVALID_STATE: invalid parse state"
    HttpParseInvalidState = 0x1134,
    /// HTTP_PARSE_ERROR_STATE: failed to parse error section"
    HttpParseErrorState = 0x1135,
    /// HTTP_GC_QUERY_NULL: query size is 0"
    HttpGcQueryNull = 0x1150,
    /// HTTP_GC_QUERY_SIZE: query size can not more than 100"
    HttpGcQuerySize = 0x1151,
    /// HTTP_GC_REQ_PARSE_ERROR: parse grafana json error"
    HttpGcReqParseError = 0x1152,
    /// HTTP_TG_DB_NOT_INPUT: database name can not be null"
    HttpTgDbNotInput = 0x1160,
    /// HTTP_TG_DB_TOO_LONG: database name too long"
    HttpTgDbTooLong = 0x1161,
    /// HTTP_TG_INVALID_JSON: invalid telegraf json fromat"
    HttpTgInvalidJson = 0x1162,
    /// HTTP_TG_METRICS_NULL: metrics size is 0"
    HttpTgMetricsNull = 0x1163,
    /// HTTP_TG_METRICS_SIZE: metrics size can not more than 1K"
    HttpTgMetricsSize = 0x1164,
    /// HTTP_TG_METRIC_NULL: metric name not find"
    HttpTgMetricNull = 0x1165,
    /// HTTP_TG_METRIC_TYPE: metric name type should be string"
    HttpTgMetricType = 0x1166,
    /// HTTP_TG_METRIC_NAME_NULL: metric name length is 0"
    HttpTgMetricNameNull = 0x1167,
    /// HTTP_TG_METRIC_NAME_LONG: metric name length too long"
    HttpTgMetricNameLong = 0x1168,
    /// HTTP_TG_TIMESTAMP_NULL: timestamp not find"
    HttpTgTimestampNull = 0x1169,
    /// HTTP_TG_TIMESTAMP_TYPE: timestamp type should be integer"
    HttpTgTimestampType = 0x116A,
    /// HTTP_TG_TIMESTAMP_VAL_NULL: timestamp value smaller than 0"
    HttpTgTimestampValNull = 0x116B,
    /// HTTP_TG_TAGS_NULL: tags not find"
    HttpTgTagsNull = 0x116C,
    /// HTTP_TG_TAGS_SIZE_0: tags size is 0"
    HttpTgTagsSize0 = 0x116D,
    /// HTTP_TG_TAGS_SIZE_LONG: tags size too long"
    HttpTgTagsSizeLong = 0x116E,
    /// HTTP_TG_TAG_NULL: tag is null"
    HttpTgTagNull = 0x116F,
    /// HTTP_TG_TAG_NAME_NULL: tag name is null"
    HttpTgTagNameNull = 0x1170,
    /// HTTP_TG_TAG_NAME_SIZE: tag name length too long"
    HttpTgTagNameSize = 0x1171,
    /// HTTP_TG_TAG_VALUE_TYPE: tag value type should be number or string"
    HttpTgTagValueType = 0x1172,
    /// HTTP_TG_TAG_VALUE_NULL: tag value is null"
    HttpTgTagValueNull = 0x1173,
    /// HTTP_TG_TABLE_NULL: table is null"
    HttpTgTableNull = 0x1174,
    /// HTTP_TG_TABLE_SIZE: table name length too long"
    HttpTgTableSize = 0x1175,
    /// HTTP_TG_FIELDS_NULL: fields not find"
    HttpTgFieldsNull = 0x1176,
    /// HTTP_TG_FIELDS_SIZE_0: fields size is 0"
    HttpTgFieldsSize0 = 0x1177,
    /// HTTP_TG_FIELDS_SIZE_LONG: fields size too long"
    HttpTgFieldsSizeLong = 0x1178,
    /// HTTP_TG_FIELD_NULL: field is null"
    HttpTgFieldNull = 0x1179,
    /// HTTP_TG_FIELD_NAME_NULL: field name is null"
    HttpTgFieldNameNull = 0x117A,
    /// HTTP_TG_FIELD_NAME_SIZE: field name length too long"
    HttpTgFieldNameSize = 0x117B,
    /// HTTP_TG_FIELD_VALUE_TYPE: field value type should be number or string"
    HttpTgFieldValueType = 0x117C,
    /// HTTP_TG_FIELD_VALUE_NULL: field value is null"
    HttpTgFieldValueNull = 0x117D,
    /// HTTP_TG_HOST_NOT_STRING: host type should be string"
    HttpTgHostNotString = 0x117E,
    /// HTTP_TG_STABLE_NOT_EXIST: stable not exist"
    HttpTgStableNotExist = 0x117F,
    /// HTTP_OP_DB_NOT_INPUT: database name can not be null"
    HttpOpDbNotInput = 0x1190,
    /// HTTP_OP_DB_TOO_LONG: database name too long"
    HttpOpDbTooLong = 0x1191,
    /// HTTP_OP_INVALID_JSON: invalid opentsdb json fromat"
    HttpOpInvalidJson = 0x1192,
    /// HTTP_OP_METRICS_NULL: metrics size is 0"
    HttpOpMetricsNull = 0x1193,
    /// HTTP_OP_METRICS_SIZE: metrics size can not more than 10K"
    HttpOpMetricsSize = 0x1194,
    /// HTTP_OP_METRIC_NULL: metric name not find"
    HttpOpMetricNull = 0x1195,
    /// HTTP_OP_METRIC_TYPE: metric name type should be string"
    HttpOpMetricType = 0x1196,
    /// HTTP_OP_METRIC_NAME_NULL: metric name length is 0"
    HttpOpMetricNameNull = 0x1197,
    /// HTTP_OP_METRIC_NAME_LONG: metric name length can not more than 22"
    HttpOpMetricNameLong = 0x1198,
    /// HTTP_OP_TIMESTAMP_NULL: timestamp not find"
    HttpOpTimestampNull = 0x1199,
    /// HTTP_OP_TIMESTAMP_TYPE: timestamp type should be integer"
    HttpOpTimestampType = 0x119A,
    /// HTTP_OP_TIMESTAMP_VAL_NULL: timestamp value smaller than 0"
    HttpOpTimestampValNull = 0x119B,
    /// HTTP_OP_TAGS_NULL: tags not find"
    HttpOpTagsNull = 0x119C,
    /// HTTP_OP_TAGS_SIZE_0: tags size is 0"
    HttpOpTagsSize0 = 0x119D,
    /// HTTP_OP_TAGS_SIZE_LONG: tags size too long"
    HttpOpTagsSizeLong = 0x119E,
    /// HTTP_OP_TAG_NULL: tag is null"
    HttpOpTagNull = 0x119F,
    /// HTTP_OP_TAG_NAME_NULL: tag name is null"
    HttpOpTagNameNull = 0x11A0,
    /// HTTP_OP_TAG_NAME_SIZE: tag name length too long"
    HttpOpTagNameSize = 0x11A1,
    /// HTTP_OP_TAG_VALUE_TYPE: tag value type should be boolean number or string"
    HttpOpTagValueType = 0x11A2,
    /// HTTP_OP_TAG_VALUE_NULL: tag value is null"
    HttpOpTagValueNull = 0x11A3,
    /// HTTP_OP_TAG_VALUE_TOO_LONG: tag value can not more than 64"
    HttpOpTagValueTooLong = 0x11A4,
    /// HTTP_OP_VALUE_NULL: value not find"
    HttpOpValueNull = 0x11A5,
    /// HTTP_OP_VALUE_TYPE: value type should be boolean number or string"
    HttpOpValueType = 0x11A6,
    /// HTTP_REQUEST_JSON_ERROR: http request json error"
    HttpRequestJsonError = 0x1F00,
    /// ODBC_OOM: out of memory"
    OdbcOom = 0x2100,
    /// ODBC_CONV_CHAR_NOT_NUM: convertion not a valid literal input"
    OdbcConvCharNotNum = 0x2101,
    /// ODBC_CONV_UNDEF: convertion undefined"
    OdbcConvUndef = 0x2102,
    /// ODBC_CONV_TRUNC_FRAC: convertion fractional truncated"
    OdbcConvTruncFrac = 0x2103,
    /// ODBC_CONV_TRUNC: convertion truncated"
    OdbcConvTrunc = 0x2104,
    /// ODBC_CONV_NOT_SUPPORT: convertion not supported"
    OdbcConvNotSupport = 0x2105,
    /// ODBC_CONV_OOR: convertion numeric value out of range"
    OdbcConvOor = 0x2106,
    /// ODBC_OUT_OF_RANGE: out of range"
    OdbcOutOfRange = 0x2107,
    /// ODBC_NOT_SUPPORT: not supported yet"
    OdbcNotSupport = 0x2108,
    /// ODBC_INVALID_HANDLE: invalid handle"
    OdbcInvalidHandle = 0x2109,
    /// ODBC_NO_RESULT: no result set"
    OdbcNoResult = 0x210a,
    /// ODBC_NO_FIELDS: no fields returned"
    OdbcNoFields = 0x210b,
    /// ODBC_INVALID_CURSOR: invalid cursor"
    OdbcInvalidCursor = 0x210c,
    /// ODBC_STATEMENT_NOT_READY: statement not ready"
    OdbcStatementNotReady = 0x210d,
    /// ODBC_CONNECTION_BUSY: connection still busy"
    OdbcConnectionBusy = 0x210e,
    /// ODBC_BAD_CONNSTR: bad connection string"
    OdbcBadConnstr = 0x210f,
    /// ODBC_BAD_ARG: bad argument"
    OdbcBadArg = 0x2110,
    /// ODBC_CONV_NOT_VALID_TS: not a valid timestamp"
    OdbcConvNotValidTs = 0x2111,
    /// ODBC_CONV_SRC_TOO_LARGE: src too large"
    OdbcConvSrcTooLarge = 0x2112,
    /// ODBC_CONV_SRC_BAD_SEQ: src bad sequence"
    OdbcConvSrcBadSeq = 0x2113,
    /// ODBC_CONV_SRC_INCOMPLETE: src incomplete"
    OdbcConvSrcIncomplete = 0x2114,
    /// ODBC_CONV_SRC_GENERAL: src general"
    OdbcConvSrcGeneral = 0x2115,
    /// FS_OUT_OF_MEMORY: tfs out of memory"
    FsOutOfMemory = 0x2200,
    /// FS_INVLD_CFG: tfs invalid mount config"
    FsInvldCfg = 0x2201,
    /// FS_TOO_MANY_MOUNT: tfs too many mount"
    FsTooManyMount = 0x2202,
    /// FS_DUP_PRIMARY: tfs duplicate primary mount"
    FsDupPrimary = 0x2203,
    /// FS_NO_PRIMARY_DISK: tfs no primary mount"
    FsNoPrimaryDisk = 0x2204,
    /// FS_NO_MOUNT_AT_TIER: tfs no mount at tier"
    FsNoMountAtTier = 0x2205,
    /// FS_FILE_ALREADY_EXISTS: tfs file already exists"
    FsFileAlreadyExists = 0x2206,
    /// FS_INVLD_LEVEL: tfs invalid level"
    FsInvldLevel = 0x2207,
    /// FS_NO_VALID_DISK: tfs no valid disk"
    FsNoValidDisk = 0x2208,
    /// MON_CONNECTION_INVALID: monitor invalid monitor db connection"
    MonConnectionInvalid = 0x2300,

    #[num_enum(default)]
    Unknown = 0xffff,
}

use TaosCode::*;

impl fmt::Display for TaosCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl TaosCode {
    pub fn success(&self) -> bool {
        matches!(self, Success)
    }
    /// RPC_ACTION_IN_PROGRESS: Action in progress"
    pub fn rpc_action_in_progress(&self) -> bool {
        matches!(self, RpcActionInProgress)
    }
    /// RPC_AUTH_REQUIRED: Authentication required"
    pub fn rpc_auth_required(&self) -> bool {
        matches!(self, RpcAuthRequired)
    }
    /// RPC_AUTH_FAILURE: Authentication failure"
    pub fn rpc_auth_failure(&self) -> bool {
        matches!(self, RpcAuthFailure)
    }
    /// RPC_REDIRECT: Redirect"
    pub fn rpc_redirect(&self) -> bool {
        matches!(self, RpcRedirect)
    }
    /// RPC_NOT_READY: System not ready"    // peer is not ready to process data
    pub fn rpc_not_ready(&self) -> bool {
        matches!(self, RpcNotReady)
    }
    /// RPC_ALREADY_PROCESSED: Message already processed"
    pub fn rpc_already_processed(&self) -> bool {
        matches!(self, RpcAlreadyProcessed)
    }
    /// RPC_LAST_SESSION_NOT_FINISHED: Last session not finished"
    pub fn rpc_last_session_not_finished(&self) -> bool {
        matches!(self, RpcLastSessionNotFinished)
    }
    /// RPC_MISMATCHED_LINK_ID: Mismatched meter id"
    pub fn rpc_mismatched_link_id(&self) -> bool {
        matches!(self, RpcMismatchedLinkId)
    }
    /// RPC_TOO_SLOW: Processing of request timed out"
    pub fn rpc_too_slow(&self) -> bool {
        matches!(self, RpcTooSlow)
    }
    /// RPC_MAX_SESSIONS: Number of sessions reached limit"    // too many sessions
    pub fn rpc_max_sessions(&self) -> bool {
        matches!(self, RpcMaxSessions)
    }
    /// RPC_NETWORK_UNAVAIL: Unable to establish connection"
    pub fn rpc_network_unavail(&self) -> bool {
        matches!(self, RpcNetworkUnavail)
    }
    /// RPC_APP_ERROR: Unexpected generic error in RPC"
    pub fn rpc_app_error(&self) -> bool {
        matches!(self, RpcAppError)
    }
    /// RPC_UNEXPECTED_RESPONSE: Unexpected response"
    pub fn rpc_unexpected_response(&self) -> bool {
        matches!(self, RpcUnexpectedResponse)
    }
    /// RPC_INVALID_VALUE: Invalid value"
    pub fn rpc_invalid_value(&self) -> bool {
        matches!(self, RpcInvalidValue)
    }
    /// RPC_INVALID_TRAN_ID: Invalid transaction id"
    pub fn rpc_invalid_tran_id(&self) -> bool {
        matches!(self, RpcInvalidTranId)
    }
    /// RPC_INVALID_SESSION_ID: Invalid session id"
    pub fn rpc_invalid_session_id(&self) -> bool {
        matches!(self, RpcInvalidSessionId)
    }
    /// RPC_INVALID_MSG_TYPE: Invalid message type"
    pub fn rpc_invalid_msg_type(&self) -> bool {
        matches!(self, RpcInvalidMsgType)
    }
    /// RPC_INVALID_RESPONSE_TYPE: Invalid response type"
    pub fn rpc_invalid_response_type(&self) -> bool {
        matches!(self, RpcInvalidResponseType)
    }
    /// RPC_INVALID_TIME_STAMP: Client and server's time is not synchronized"
    pub fn rpc_invalid_time_stamp(&self) -> bool {
        matches!(self, RpcInvalidTimeStamp)
    }
    /// APP_NOT_READY: Database not ready"
    pub fn app_not_ready(&self) -> bool {
        matches!(self, AppNotReady)
    }
    /// RPC_FQDN_ERROR: Unable to resolve FQDN"
    pub fn rpc_fqdn_error(&self) -> bool {
        matches!(self, RpcFqdnError)
    }
    /// RPC_INVALID_VERSION: Invalid app version"
    pub fn rpc_invalid_version(&self) -> bool {
        matches!(self, RpcInvalidVersion)
    }
    /// COM_OPS_NOT_SUPPORT: Operation not supported"
    pub fn com_ops_not_support(&self) -> bool {
        matches!(self, ComOpsNotSupport)
    }
    /// COM_MEMORY_CORRUPTED: Memory corrupted"
    pub fn com_memory_corrupted(&self) -> bool {
        matches!(self, ComMemoryCorrupted)
    }
    /// COM_OUT_OF_MEMORY: Out of memory"
    pub fn com_out_of_memory(&self) -> bool {
        matches!(self, ComOutOfMemory)
    }
    /// COM_INVALID_CFG_MSG: Invalid config message"
    pub fn com_invalid_cfg_msg(&self) -> bool {
        matches!(self, ComInvalidCfgMsg)
    }
    /// COM_FILE_CORRUPTED: Data file corrupted"
    pub fn com_file_corrupted(&self) -> bool {
        matches!(self, ComFileCorrupted)
    }
    /// REF_NO_MEMORY: Ref out of memory"
    pub fn ref_no_memory(&self) -> bool {
        matches!(self, RefNoMemory)
    }
    /// REF_FULL: too many Ref Objs"
    pub fn ref_full(&self) -> bool {
        matches!(self, RefFull)
    }
    /// REF_ID_REMOVED: Ref ID is removed"
    pub fn ref_id_removed(&self) -> bool {
        matches!(self, RefIdRemoved)
    }
    /// REF_INVALID_ID: Invalid Ref ID"
    pub fn ref_invalid_id(&self) -> bool {
        matches!(self, RefInvalidId)
    }
    /// REF_ALREADY_EXIST: Ref is already there"
    pub fn ref_already_exist(&self) -> bool {
        matches!(self, RefAlreadyExist)
    }
    /// REF_NOT_EXIST: Ref is not there"
    pub fn ref_not_exist(&self) -> bool {
        matches!(self, RefNotExist)
    }
    /// TSC_INVALID_OPERATION: Invalid Operation")
    pub fn tsc_invalid_operation(&self) -> bool {
        matches!(self, TscInvalidOperation)
    }
    /// TSC_INVALID_QHANDLE: Invalid qhandle")
    pub fn tsc_invalid_qhandle(&self) -> bool {
        matches!(self, TscInvalidQhandle)
    }
    /// TSC_INVALID_TIME_STAMP: Invalid combination of client/service time")
    pub fn tsc_invalid_time_stamp(&self) -> bool {
        matches!(self, TscInvalidTimeStamp)
    }
    /// TSC_INVALID_VALUE: Invalid value in client")
    pub fn tsc_invalid_value(&self) -> bool {
        matches!(self, TscInvalidValue)
    }
    /// TSC_INVALID_VERSION: Invalid client version")
    pub fn tsc_invalid_version(&self) -> bool {
        matches!(self, TscInvalidVersion)
    }
    /// TSC_INVALID_IE: Invalid client ie")
    pub fn tsc_invalid_ie(&self) -> bool {
        matches!(self, TscInvalidIe)
    }
    /// TSC_INVALID_FQDN: Invalid host name")
    pub fn tsc_invalid_fqdn(&self) -> bool {
        matches!(self, TscInvalidFqdn)
    }
    /// TSC_INVALID_USER_LENGTH: Invalid user name")
    pub fn tsc_invalid_user_length(&self) -> bool {
        matches!(self, TscInvalidUserLength)
    }
    /// TSC_INVALID_PASS_LENGTH: Invalid password")
    pub fn tsc_invalid_pass_length(&self) -> bool {
        matches!(self, TscInvalidPassLength)
    }
    /// TSC_INVALID_DB_LENGTH: Database name too long")
    pub fn tsc_invalid_db_length(&self) -> bool {
        matches!(self, TscInvalidDbLength)
    }
    /// TSC_INVALID_TABLE_ID_LENGTH: Table name too long")
    pub fn tsc_invalid_table_id_length(&self) -> bool {
        matches!(self, TscInvalidTableIdLength)
    }
    /// TSC_INVALID_CONNECTION: Invalid connection")
    pub fn tsc_invalid_connection(&self) -> bool {
        matches!(self, TscInvalidConnection)
    }
    /// TSC_OUT_OF_MEMORY: System out of memory")
    pub fn tsc_out_of_memory(&self) -> bool {
        matches!(self, TscOutOfMemory)
    }
    /// TSC_NO_DISKSPACE: System out of disk space")
    pub fn tsc_no_diskspace(&self) -> bool {
        matches!(self, TscNoDiskspace)
    }
    /// TSC_QUERY_CACHE_ERASED: Query cache erased")
    pub fn tsc_query_cache_erased(&self) -> bool {
        matches!(self, TscQueryCacheErased)
    }
    /// TSC_QUERY_CANCELLED: Query terminated")
    pub fn tsc_query_cancelled(&self) -> bool {
        matches!(self, TscQueryCancelled)
    }
    /// TSC_SORTED_RES_TOO_MANY: Result set too large to be sorted")      // too many result for ordered super table projection query
    pub fn tsc_sorted_res_too_many(&self) -> bool {
        matches!(self, TscSortedResTooMany)
    }
    /// TSC_APP_ERROR: Application error")
    pub fn tsc_app_error(&self) -> bool {
        matches!(self, TscAppError)
    }
    /// TSC_ACTION_IN_PROGRESS: Action in progress")
    pub fn tsc_action_in_progress(&self) -> bool {
        matches!(self, TscActionInProgress)
    }
    /// TSC_DISCONNECTED: Disconnected from service")
    pub fn tsc_disconnected(&self) -> bool {
        matches!(self, TscDisconnected)
    }
    /// TSC_NO_WRITE_AUTH: No write permission")
    pub fn tsc_no_write_auth(&self) -> bool {
        matches!(self, TscNoWriteAuth)
    }
    /// TSC_CONN_KILLED: Connection killed")
    pub fn tsc_conn_killed(&self) -> bool {
        matches!(self, TscConnKilled)
    }
    /// TSC_SQL_SYNTAX_ERROR: Syntax error in SQL")
    pub fn tsc_sql_syntax_error(&self) -> bool {
        matches!(self, TscSqlSyntaxError)
    }
    /// TSC_DB_NOT_SELECTED: Database not specified or available")
    pub fn tsc_db_not_selected(&self) -> bool {
        matches!(self, TscDbNotSelected)
    }
    /// TSC_INVALID_TABLE_NAME: Table does not exist")
    pub fn tsc_invalid_table_name(&self) -> bool {
        matches!(self, TscInvalidTableName)
    }
    /// TSC_EXCEED_SQL_LIMIT: SQL statement too long check maxSQLLength config")
    pub fn tsc_exceed_sql_limit(&self) -> bool {
        matches!(self, TscExceedSqlLimit)
    }
    /// TSC_FILE_EMPTY: File is empty")
    pub fn tsc_file_empty(&self) -> bool {
        matches!(self, TscFileEmpty)
    }
    /// TSC_LINE_SYNTAX_ERROR: Syntax error in Line")
    pub fn tsc_line_syntax_error(&self) -> bool {
        matches!(self, TscLineSyntaxError)
    }
    /// TSC_NO_META_CACHED: No table meta cached")
    pub fn tsc_no_meta_cached(&self) -> bool {
        matches!(self, TscNoMetaCached)
    }
    /// TSC_DUP_COL_NAMES: duplicated column names")
    pub fn tsc_dup_col_names(&self) -> bool {
        matches!(self, TscDupColNames)
    }
    /// TSC_INVALID_TAG_LENGTH: Invalid tag length")
    pub fn tsc_invalid_tag_length(&self) -> bool {
        matches!(self, TscInvalidTagLength)
    }
    /// TSC_INVALID_COLUMN_LENGTH: Invalid column length")
    pub fn tsc_invalid_column_length(&self) -> bool {
        matches!(self, TscInvalidColumnLength)
    }
    /// TSC_DUP_TAG_NAMES: duplicated tag names")
    pub fn tsc_dup_tag_names(&self) -> bool {
        matches!(self, TscDupTagNames)
    }
    /// TSC_INVALID_JSON: Invalid JSON format")
    pub fn tsc_invalid_json(&self) -> bool {
        matches!(self, TscInvalidJson)
    }
    /// TSC_INVALID_JSON_TYPE: Invalid JSON data type")
    pub fn tsc_invalid_json_type(&self) -> bool {
        matches!(self, TscInvalidJsonType)
    }
    /// TSC_INVALID_JSON_CONFIG: Invalid JSON configuration")
    pub fn tsc_invalid_json_config(&self) -> bool {
        matches!(self, TscInvalidJsonConfig)
    }
    /// TSC_VALUE_OUT_OF_RANGE: Value out of range")
    pub fn tsc_value_out_of_range(&self) -> bool {
        matches!(self, TscValueOutOfRange)
    }
    /// TSC_INVALID_PROTOCOL_TYPE: Invalid line protocol type")
    pub fn tsc_invalid_protocol_type(&self) -> bool {
        matches!(self, TscInvalidProtocolType)
    }
    /// TSC_INVALID_PRECISION_TYPE: Invalid timestamp precision type")
    pub fn tsc_invalid_precision_type(&self) -> bool {
        matches!(self, TscInvalidPrecisionType)
    }
    /// TSC_RES_TOO_MANY: Result set too large to be output")
    pub fn tsc_res_too_many(&self) -> bool {
        matches!(self, TscResTooMany)
    }
    /// TSC_INVALID_SCHEMA_VERSION: invalid table schema version")
    pub fn tsc_invalid_schema_version(&self) -> bool {
        matches!(self, TscInvalidSchemaVersion)
    }
    /// MND_MSG_NOT_PROCESSED: Message not processed"
    pub fn mnd_msg_not_processed(&self) -> bool {
        matches!(self, MndMsgNotProcessed)
    }
    /// MND_ACTION_IN_PROGRESS: Message is progressing"
    pub fn mnd_action_in_progress(&self) -> bool {
        matches!(self, MndActionInProgress)
    }
    /// MND_ACTION_NEED_REPROCESSED: Messag need to be reprocessed"
    pub fn mnd_action_need_reprocessed(&self) -> bool {
        matches!(self, MndActionNeedReprocessed)
    }
    /// MND_NO_RIGHTS: Insufficient privilege for operation"
    pub fn mnd_no_rights(&self) -> bool {
        matches!(self, MndNoRights)
    }
    /// MND_APP_ERROR: Unexpected generic error in mnode"
    pub fn mnd_app_error(&self) -> bool {
        matches!(self, MndAppError)
    }
    /// MND_INVALID_CONNECTION: Invalid message connection"
    pub fn mnd_invalid_connection(&self) -> bool {
        matches!(self, MndInvalidConnection)
    }
    /// MND_INVALID_MSG_VERSION: Incompatible protocol version"
    pub fn mnd_invalid_msg_version(&self) -> bool {
        matches!(self, MndInvalidMsgVersion)
    }
    /// MND_INVALID_MSG_LEN: Invalid message length"
    pub fn mnd_invalid_msg_len(&self) -> bool {
        matches!(self, MndInvalidMsgLen)
    }
    /// MND_INVALID_MSG_TYPE: Invalid message type"
    pub fn mnd_invalid_msg_type(&self) -> bool {
        matches!(self, MndInvalidMsgType)
    }
    /// MND_TOO_MANY_SHELL_CONNS: Too many connections"
    pub fn mnd_too_many_shell_conns(&self) -> bool {
        matches!(self, MndTooManyShellConns)
    }
    /// MND_OUT_OF_MEMORY: Out of memory in mnode"
    pub fn mnd_out_of_memory(&self) -> bool {
        matches!(self, MndOutOfMemory)
    }
    /// MND_INVALID_SHOWOBJ: Data expired"
    pub fn mnd_invalid_showobj(&self) -> bool {
        matches!(self, MndInvalidShowobj)
    }
    /// MND_INVALID_QUERY_ID: Invalid query id"
    pub fn mnd_invalid_query_id(&self) -> bool {
        matches!(self, MndInvalidQueryId)
    }
    /// MND_INVALID_STREAM_ID: Invalid stream id"
    pub fn mnd_invalid_stream_id(&self) -> bool {
        matches!(self, MndInvalidStreamId)
    }
    /// MND_INVALID_CONN_ID: Invalid connection id"
    pub fn mnd_invalid_conn_id(&self) -> bool {
        matches!(self, MndInvalidConnId)
    }
    /// MND_MNODE_IS_RUNNING: mnode is already running"
    pub fn mnd_mnode_is_running(&self) -> bool {
        matches!(self, MndMnodeIsRunning)
    }
    /// MND_FAILED_TO_CONFIG_SYNC: failed to config sync"
    pub fn mnd_failed_to_config_sync(&self) -> bool {
        matches!(self, MndFailedToConfigSync)
    }
    /// MND_FAILED_TO_START_SYNC: failed to start sync"
    pub fn mnd_failed_to_start_sync(&self) -> bool {
        matches!(self, MndFailedToStartSync)
    }
    /// MND_FAILED_TO_CREATE_DIR: failed to create mnode dir"
    pub fn mnd_failed_to_create_dir(&self) -> bool {
        matches!(self, MndFailedToCreateDir)
    }
    /// MND_FAILED_TO_INIT_STEP: failed to init components"
    pub fn mnd_failed_to_init_step(&self) -> bool {
        matches!(self, MndFailedToInitStep)
    }
    /// MND_SDB_OBJ_ALREADY_THERE: Object already there"
    pub fn mnd_sdb_obj_already_there(&self) -> bool {
        matches!(self, MndSdbObjAlreadyThere)
    }
    /// MND_SDB_ERROR: Unexpected generic error in sdb"
    pub fn mnd_sdb_error(&self) -> bool {
        matches!(self, MndSdbError)
    }
    /// MND_SDB_INVALID_TABLE_TYPE: Invalid table type"
    pub fn mnd_sdb_invalid_table_type(&self) -> bool {
        matches!(self, MndSdbInvalidTableType)
    }
    /// MND_SDB_OBJ_NOT_THERE: Object not there"
    pub fn mnd_sdb_obj_not_there(&self) -> bool {
        matches!(self, MndSdbObjNotThere)
    }
    /// MND_SDB_INVAID_META_ROW: Invalid meta row"
    pub fn mnd_sdb_invaid_meta_row(&self) -> bool {
        matches!(self, MndSdbInvaidMetaRow)
    }
    /// MND_SDB_INVAID_KEY_TYPE: Invalid key type"
    pub fn mnd_sdb_invaid_key_type(&self) -> bool {
        matches!(self, MndSdbInvaidKeyType)
    }
    /// MND_DNODE_ALREADY_EXIST: DNode already exists"
    pub fn mnd_dnode_already_exist(&self) -> bool {
        matches!(self, MndDnodeAlreadyExist)
    }
    /// MND_DNODE_NOT_EXIST: DNode does not exist"
    pub fn mnd_dnode_not_exist(&self) -> bool {
        matches!(self, MndDnodeNotExist)
    }
    /// MND_VGROUP_NOT_EXIST: VGroup does not exist"
    pub fn mnd_vgroup_not_exist(&self) -> bool {
        matches!(self, MndVgroupNotExist)
    }
    /// MND_NO_REMOVE_MASTER: Master DNode cannot be removed"
    pub fn mnd_no_remove_master(&self) -> bool {
        matches!(self, MndNoRemoveMaster)
    }
    /// MND_NO_ENOUGH_DNODES: Out of DNodes"
    pub fn mnd_no_enough_dnodes(&self) -> bool {
        matches!(self, MndNoEnoughDnodes)
    }
    /// MND_CLUSTER_CFG_INCONSISTENT: Cluster cfg inconsistent"
    pub fn mnd_cluster_cfg_inconsistent(&self) -> bool {
        matches!(self, MndClusterCfgInconsistent)
    }
    /// MND_INVALID_DNODE_CFG_OPTION: Invalid dnode cfg option"
    pub fn mnd_invalid_dnode_cfg_option(&self) -> bool {
        matches!(self, MndInvalidDnodeCfgOption)
    }
    /// MND_BALANCE_ENABLED: Balance already enabled"
    pub fn mnd_balance_enabled(&self) -> bool {
        matches!(self, MndBalanceEnabled)
    }
    /// MND_VGROUP_NOT_IN_DNODE: Vgroup not in dnode"
    pub fn mnd_vgroup_not_in_dnode(&self) -> bool {
        matches!(self, MndVgroupNotInDnode)
    }
    /// MND_VGROUP_ALREADY_IN_DNODE: Vgroup already in dnode"
    pub fn mnd_vgroup_already_in_dnode(&self) -> bool {
        matches!(self, MndVgroupAlreadyInDnode)
    }
    /// MND_DNODE_NOT_FREE: Dnode not avaliable"
    pub fn mnd_dnode_not_free(&self) -> bool {
        matches!(self, MndDnodeNotFree)
    }
    /// MND_INVALID_CLUSTER_ID: Cluster id not match"
    pub fn mnd_invalid_cluster_id(&self) -> bool {
        matches!(self, MndInvalidClusterId)
    }
    /// MND_NOT_READY: Cluster not ready"
    pub fn mnd_not_ready(&self) -> bool {
        matches!(self, MndNotReady)
    }
    /// MND_DNODE_ID_NOT_CONFIGURED: Dnode Id not configured"
    pub fn mnd_dnode_id_not_configured(&self) -> bool {
        matches!(self, MndDnodeIdNotConfigured)
    }
    /// MND_DNODE_EP_NOT_CONFIGURED: Dnode Ep not configured"
    pub fn mnd_dnode_ep_not_configured(&self) -> bool {
        matches!(self, MndDnodeEpNotConfigured)
    }
    /// MND_ACCT_ALREADY_EXIST: Account already exists"
    pub fn mnd_acct_already_exist(&self) -> bool {
        matches!(self, MndAcctAlreadyExist)
    }
    /// MND_INVALID_ACCT: Invalid account"
    pub fn mnd_invalid_acct(&self) -> bool {
        matches!(self, MndInvalidAcct)
    }
    /// MND_INVALID_ACCT_OPTION: Invalid account options"
    pub fn mnd_invalid_acct_option(&self) -> bool {
        matches!(self, MndInvalidAcctOption)
    }
    /// MND_ACCT_EXPIRED: Account authorization has expired"
    pub fn mnd_acct_expired(&self) -> bool {
        matches!(self, MndAcctExpired)
    }
    /// MND_USER_ALREADY_EXIST: User already exists"
    pub fn mnd_user_already_exist(&self) -> bool {
        matches!(self, MndUserAlreadyExist)
    }
    /// MND_INVALID_USER: Invalid user"
    pub fn mnd_invalid_user(&self) -> bool {
        matches!(self, MndInvalidUser)
    }
    /// MND_INVALID_USER_FORMAT: Invalid user format"
    pub fn mnd_invalid_user_format(&self) -> bool {
        matches!(self, MndInvalidUserFormat)
    }
    /// MND_INVALID_PASS_FORMAT: Invalid password format"
    pub fn mnd_invalid_pass_format(&self) -> bool {
        matches!(self, MndInvalidPassFormat)
    }
    /// MND_NO_USER_FROM_CONN: Can not get user from conn"
    pub fn mnd_no_user_from_conn(&self) -> bool {
        matches!(self, MndNoUserFromConn)
    }
    /// MND_TOO_MANY_USERS: Too many users"
    pub fn mnd_too_many_users(&self) -> bool {
        matches!(self, MndTooManyUsers)
    }
    /// MND_TABLE_ALREADY_EXIST: Table already exists"
    pub fn mnd_table_already_exist(&self) -> bool {
        matches!(self, MndTableAlreadyExist)
    }
    /// MND_INVALID_TABLE_ID: Table name too long"
    pub fn mnd_invalid_table_id(&self) -> bool {
        matches!(self, MndInvalidTableId)
    }
    /// MND_INVALID_TABLE_NAME: Table does not exist"
    pub fn mnd_invalid_table_name(&self) -> bool {
        matches!(self, MndInvalidTableName)
    }
    /// MND_INVALID_TABLE_TYPE: Invalid table type in tsdb"
    pub fn mnd_invalid_table_type(&self) -> bool {
        matches!(self, MndInvalidTableType)
    }
    /// MND_TOO_MANY_TAGS: Too many tags"
    pub fn mnd_too_many_tags(&self) -> bool {
        matches!(self, MndTooManyTags)
    }
    /// MND_TOO_MANY_COLUMNS: Too many columns"
    pub fn mnd_too_many_columns(&self) -> bool {
        matches!(self, MndTooManyColumns)
    }
    /// MND_TOO_MANY_TIMESERIES: Too many time series"
    pub fn mnd_too_many_timeseries(&self) -> bool {
        matches!(self, MndTooManyTimeseries)
    }
    /// MND_NOT_SUPER_TABLE: Not super table"           // operation only available for super table
    pub fn mnd_not_super_table(&self) -> bool {
        matches!(self, MndNotSuperTable)
    }
    /// MND_COL_NAME_TOO_LONG: Tag name too long"
    pub fn mnd_col_name_too_long(&self) -> bool {
        matches!(self, MndColNameTooLong)
    }
    /// MND_TAG_ALREAY_EXIST: Tag already exists"
    pub fn mnd_tag_alreay_exist(&self) -> bool {
        matches!(self, MndTagAlreayExist)
    }
    /// MND_TAG_NOT_EXIST: Tag does not exist"
    pub fn mnd_tag_not_exist(&self) -> bool {
        matches!(self, MndTagNotExist)
    }
    /// MND_FIELD_ALREAY_EXIST: Field already exists"
    pub fn mnd_field_alreay_exist(&self) -> bool {
        matches!(self, MndFieldAlreayExist)
    }
    /// MND_FIELD_NOT_EXIST: Field does not exist"
    pub fn mnd_field_not_exist(&self) -> bool {
        matches!(self, MndFieldNotExist)
    }
    /// MND_INVALID_STABLE_NAME: Super table does not exist"
    pub fn mnd_invalid_stable_name(&self) -> bool {
        matches!(self, MndInvalidStableName)
    }
    /// MND_INVALID_CREATE_TABLE_MSG: Invalid create table message"
    pub fn mnd_invalid_create_table_msg(&self) -> bool {
        matches!(self, MndInvalidCreateTableMsg)
    }
    /// MND_EXCEED_MAX_ROW_BYTES: Exceed max row bytes"
    pub fn mnd_exceed_max_row_bytes(&self) -> bool {
        matches!(self, MndExceedMaxRowBytes)
    }
    /// MND_INVALID_FUNC_NAME: Invalid func name"
    pub fn mnd_invalid_func_name(&self) -> bool {
        matches!(self, MndInvalidFuncName)
    }
    /// MND_INVALID_FUNC_LEN: Invalid func length"
    pub fn mnd_invalid_func_len(&self) -> bool {
        matches!(self, MndInvalidFuncLen)
    }
    /// MND_INVALID_FUNC_CODE: Invalid func code"
    pub fn mnd_invalid_func_code(&self) -> bool {
        matches!(self, MndInvalidFuncCode)
    }
    /// MND_FUNC_ALREADY_EXIST: Func already exists"
    pub fn mnd_func_already_exist(&self) -> bool {
        matches!(self, MndFuncAlreadyExist)
    }
    /// MND_INVALID_FUNC: Invalid func"
    pub fn mnd_invalid_func(&self) -> bool {
        matches!(self, MndInvalidFunc)
    }
    /// MND_INVALID_FUNC_BUFSIZE: Invalid func bufSize"
    pub fn mnd_invalid_func_bufsize(&self) -> bool {
        matches!(self, MndInvalidFuncBufsize)
    }
    /// MND_INVALID_TAG_LENGTH: invalid tag length"
    pub fn mnd_invalid_tag_length(&self) -> bool {
        matches!(self, MndInvalidTagLength)
    }
    /// MND_INVALID_COLUMN_LENGTH: invalid column length"
    pub fn mnd_invalid_column_length(&self) -> bool {
        matches!(self, MndInvalidColumnLength)
    }
    /// MND_DB_NOT_SELECTED: Database not specified or available"
    pub fn mnd_db_not_selected(&self) -> bool {
        matches!(self, MndDbNotSelected)
    }
    /// MND_DB_ALREADY_EXIST: Database already exists"
    pub fn mnd_db_already_exist(&self) -> bool {
        matches!(self, MndDbAlreadyExist)
    }
    /// MND_INVALID_DB_OPTION: Invalid database options"
    pub fn mnd_invalid_db_option(&self) -> bool {
        matches!(self, MndInvalidDbOption)
    }
    /// MND_INVALID_DB: Invalid database name"
    pub fn mnd_invalid_db(&self) -> bool {
        matches!(self, MndInvalidDb)
    }
    /// MND_MONITOR_DB_FORBIDDEN: Cannot delete monitor database"
    pub fn mnd_monitor_db_forbidden(&self) -> bool {
        matches!(self, MndMonitorDbForbidden)
    }
    /// MND_TOO_MANY_DATABASES: Too many databases for account"
    pub fn mnd_too_many_databases(&self) -> bool {
        matches!(self, MndTooManyDatabases)
    }
    /// MND_DB_IN_DROPPING: Database not available"
    pub fn mnd_db_in_dropping(&self) -> bool {
        matches!(self, MndDbInDropping)
    }
    /// MND_VGROUP_NOT_READY: Database unsynced"
    pub fn mnd_vgroup_not_ready(&self) -> bool {
        matches!(self, MndVgroupNotReady)
    }
    /// MND_INVALID_DB_OPTION_DAYS: Invalid database option: days out of range"
    pub fn mnd_invalid_db_option_days(&self) -> bool {
        matches!(self, MndInvalidDbOptionDays)
    }
    /// MND_INVALID_DB_OPTION_KEEP: Invalid database option: keep >= keep1 >= keep0 >= days"
    pub fn mnd_invalid_db_option_keep(&self) -> bool {
        matches!(self, MndInvalidDbOptionKeep)
    }
    /// MND_INVALID_TOPIC: Invalid topic name)
    pub fn mnd_invalid_topic(&self) -> bool {
        matches!(self, MndInvalidTopic)
    }
    /// MND_INVALID_TOPIC_OPTION: Invalid topic option)
    pub fn mnd_invalid_topic_option(&self) -> bool {
        matches!(self, MndInvalidTopicOption)
    }
    /// MND_INVALID_TOPIC_PARTITONS: Invalid topic partitons num, valid range: [1, 1000])
    pub fn mnd_invalid_topic_partitons(&self) -> bool {
        matches!(self, MndInvalidTopicPartitons)
    }
    /// MND_TOPIC_ALREADY_EXIST: Topic already exists)
    pub fn mnd_topic_already_exist(&self) -> bool {
        matches!(self, MndTopicAlreadyExist)
    }
    /// DND_MSG_NOT_PROCESSED: Message not processed"
    pub fn dnd_msg_not_processed(&self) -> bool {
        matches!(self, DndMsgNotProcessed)
    }
    /// DND_OUT_OF_MEMORY: Dnode out of memory"
    pub fn dnd_out_of_memory(&self) -> bool {
        matches!(self, DndOutOfMemory)
    }
    /// DND_NO_WRITE_ACCESS: No permission for disk files in dnode"
    pub fn dnd_no_write_access(&self) -> bool {
        matches!(self, DndNoWriteAccess)
    }
    /// DND_INVALID_MSG_LEN: Invalid message length"
    pub fn dnd_invalid_msg_len(&self) -> bool {
        matches!(self, DndInvalidMsgLen)
    }
    /// DND_ACTION_IN_PROGRESS: Action in progress"
    pub fn dnd_action_in_progress(&self) -> bool {
        matches!(self, DndActionInProgress)
    }
    /// DND_TOO_MANY_VNODES: Too many vnode directories"
    pub fn dnd_too_many_vnodes(&self) -> bool {
        matches!(self, DndTooManyVnodes)
    }
    /// DND_EXITING: Dnode is exiting"
    pub fn dnd_exiting(&self) -> bool {
        matches!(self, DndExiting)
    }
    /// DND_VNODE_OPEN_FAILED: Vnode open failed"
    pub fn dnd_vnode_open_failed(&self) -> bool {
        matches!(self, DndVnodeOpenFailed)
    }
    /// VND_ACTION_IN_PROGRESS: Action in progress"
    pub fn vnd_action_in_progress(&self) -> bool {
        matches!(self, VndActionInProgress)
    }
    /// VND_MSG_NOT_PROCESSED: Message not processed"
    pub fn vnd_msg_not_processed(&self) -> bool {
        matches!(self, VndMsgNotProcessed)
    }
    /// VND_ACTION_NEED_REPROCESSED: Action need to be reprocessed"
    pub fn vnd_action_need_reprocessed(&self) -> bool {
        matches!(self, VndActionNeedReprocessed)
    }
    /// VND_INVALID_VGROUP_ID: Invalid Vgroup ID"
    pub fn vnd_invalid_vgroup_id(&self) -> bool {
        matches!(self, VndInvalidVgroupId)
    }
    /// VND_INIT_FAILED: Vnode initialization failed"
    pub fn vnd_init_failed(&self) -> bool {
        matches!(self, VndInitFailed)
    }
    /// VND_NO_DISKSPACE: System out of disk space"
    pub fn vnd_no_diskspace(&self) -> bool {
        matches!(self, VndNoDiskspace)
    }
    /// VND_NO_DISK_PERMISSIONS: No write permission for disk files"
    pub fn vnd_no_disk_permissions(&self) -> bool {
        matches!(self, VndNoDiskPermissions)
    }
    /// VND_NO_SUCH_FILE_OR_DIR: Missing data file"
    pub fn vnd_no_such_file_or_dir(&self) -> bool {
        matches!(self, VndNoSuchFileOrDir)
    }
    /// VND_OUT_OF_MEMORY: Out of memory"
    pub fn vnd_out_of_memory(&self) -> bool {
        matches!(self, VndOutOfMemory)
    }
    /// VND_APP_ERROR: Unexpected generic error in vnode"
    pub fn vnd_app_error(&self) -> bool {
        matches!(self, VndAppError)
    }
    /// VND_INVALID_VRESION_FILE: Invalid version file"
    pub fn vnd_invalid_vresion_file(&self) -> bool {
        matches!(self, VndInvalidVresionFile)
    }
    /// VND_IS_FULL: Database memory is full for commit failed"
    pub fn vnd_is_full(&self) -> bool {
        matches!(self, VndIsFull)
    }
    /// VND_IS_FLOWCTRL: Database memory is full for waiting commit"
    pub fn vnd_is_flowctrl(&self) -> bool {
        matches!(self, VndIsFlowctrl)
    }
    /// VND_IS_DROPPING: Database is dropping"
    pub fn vnd_is_dropping(&self) -> bool {
        matches!(self, VndIsDropping)
    }
    /// VND_IS_BALANCING: Database is balancing"
    pub fn vnd_is_balancing(&self) -> bool {
        matches!(self, VndIsBalancing)
    }
    /// VND_IS_CLOSING: Database is closing"
    pub fn vnd_is_closing(&self) -> bool {
        matches!(self, VndIsClosing)
    }
    /// VND_NOT_SYNCED: Database suspended"
    pub fn vnd_not_synced(&self) -> bool {
        matches!(self, VndNotSynced)
    }
    /// VND_NO_WRITE_AUTH: Database write operation denied"
    pub fn vnd_no_write_auth(&self) -> bool {
        matches!(self, VndNoWriteAuth)
    }
    /// VND_IS_SYNCING: Database is syncing"
    pub fn vnd_is_syncing(&self) -> bool {
        matches!(self, VndIsSyncing)
    }
    /// VND_INVALID_TSDB_STATE: Invalid tsdb state"
    pub fn vnd_invalid_tsdb_state(&self) -> bool {
        matches!(self, VndInvalidTsdbState)
    }
    /// WAIT_THREAD_TOO_MANY: Wait threads too many"
    pub fn wait_thread_too_many(&self) -> bool {
        matches!(self, WaitThreadTooMany)
    }
    /// TDB_INVALID_TABLE_ID: Invalid table ID")
    pub fn tdb_invalid_table_id(&self) -> bool {
        matches!(self, TdbInvalidTableId)
    }
    /// TDB_INVALID_TABLE_TYPE: Invalid table type")
    pub fn tdb_invalid_table_type(&self) -> bool {
        matches!(self, TdbInvalidTableType)
    }
    /// TDB_IVD_TB_SCHEMA_VERSION: Invalid table schema version")
    pub fn tdb_ivd_tb_schema_version(&self) -> bool {
        matches!(self, TdbIvdTbSchemaVersion)
    }
    /// TDB_TABLE_ALREADY_EXIST: Table already exists")
    pub fn tdb_table_already_exist(&self) -> bool {
        matches!(self, TdbTableAlreadyExist)
    }
    /// TDB_INVALID_CONFIG: Invalid configuration")
    pub fn tdb_invalid_config(&self) -> bool {
        matches!(self, TdbInvalidConfig)
    }
    /// TDB_INIT_FAILED: Tsdb init failed")
    pub fn tdb_init_failed(&self) -> bool {
        matches!(self, TdbInitFailed)
    }
    /// TDB_NO_DISKSPACE: No diskspace for tsdb")
    pub fn tdb_no_diskspace(&self) -> bool {
        matches!(self, TdbNoDiskspace)
    }
    /// TDB_NO_DISK_PERMISSIONS: No permission for disk files")
    pub fn tdb_no_disk_permissions(&self) -> bool {
        matches!(self, TdbNoDiskPermissions)
    }
    /// TDB_FILE_CORRUPTED: Data file(s) corrupted")
    pub fn tdb_file_corrupted(&self) -> bool {
        matches!(self, TdbFileCorrupted)
    }
    /// TDB_OUT_OF_MEMORY: Out of memory")
    pub fn tdb_out_of_memory(&self) -> bool {
        matches!(self, TdbOutOfMemory)
    }
    /// TDB_TAG_VER_OUT_OF_DATE: Tag too old")
    pub fn tdb_tag_ver_out_of_date(&self) -> bool {
        matches!(self, TdbTagVerOutOfDate)
    }
    /// TDB_TIMESTAMP_OUT_OF_RANGE: Timestamp data out of range")
    pub fn tdb_timestamp_out_of_range(&self) -> bool {
        matches!(self, TdbTimestampOutOfRange)
    }
    /// TDB_SUBMIT_MSG_MSSED_UP: Submit message is messed up")
    pub fn tdb_submit_msg_mssed_up(&self) -> bool {
        matches!(self, TdbSubmitMsgMssedUp)
    }
    /// TDB_INVALID_ACTION: Invalid operation")
    pub fn tdb_invalid_action(&self) -> bool {
        matches!(self, TdbInvalidAction)
    }
    /// TDB_INVALID_CREATE_TB_MSG: Invalid creation of table")
    pub fn tdb_invalid_create_tb_msg(&self) -> bool {
        matches!(self, TdbInvalidCreateTbMsg)
    }
    /// TDB_NO_TABLE_DATA_IN_MEM: No table data in memory skiplist")
    pub fn tdb_no_table_data_in_mem(&self) -> bool {
        matches!(self, TdbNoTableDataInMem)
    }
    /// TDB_FILE_ALREADY_EXISTS: File already exists")
    pub fn tdb_file_already_exists(&self) -> bool {
        matches!(self, TdbFileAlreadyExists)
    }
    /// TDB_TABLE_RECONFIGURE: Need to reconfigure table")
    pub fn tdb_table_reconfigure(&self) -> bool {
        matches!(self, TdbTableReconfigure)
    }
    /// TDB_IVD_CREATE_TABLE_INFO: Invalid information to create table")
    pub fn tdb_ivd_create_table_info(&self) -> bool {
        matches!(self, TdbIvdCreateTableInfo)
    }
    /// TDB_NO_AVAIL_DISK: No available disk")
    pub fn tdb_no_avail_disk(&self) -> bool {
        matches!(self, TdbNoAvailDisk)
    }
    /// TDB_MESSED_MSG: TSDB messed message")
    pub fn tdb_messed_msg(&self) -> bool {
        matches!(self, TdbMessedMsg)
    }
    /// TDB_IVLD_TAG_VAL: TSDB invalid tag value")
    pub fn tdb_ivld_tag_val(&self) -> bool {
        matches!(self, TdbIvldTagVal)
    }
    /// TDB_NO_CACHE_LAST_ROW: TSDB no cache last row data")
    pub fn tdb_no_cache_last_row(&self) -> bool {
        matches!(self, TdbNoCacheLastRow)
    }
    /// TDB_INCOMPLETE_DFILESET: TSDB incomplete DFileSet")
    pub fn tdb_incomplete_dfileset(&self) -> bool {
        matches!(self, TdbIncompleteDfileset)
    }
    /// QRY_INVALID_QHANDLE: Invalid handle")
    pub fn qry_invalid_qhandle(&self) -> bool {
        matches!(self, QryInvalidQhandle)
    }
    /// QRY_INVALID_MSG: Invalid message")    // failed to validate the sql expression msg by vnode
    pub fn qry_invalid_msg(&self) -> bool {
        matches!(self, QryInvalidMsg)
    }
    /// QRY_NO_DISKSPACE: No diskspace for query")
    pub fn qry_no_diskspace(&self) -> bool {
        matches!(self, QryNoDiskspace)
    }
    /// QRY_OUT_OF_MEMORY: System out of memory")
    pub fn qry_out_of_memory(&self) -> bool {
        matches!(self, QryOutOfMemory)
    }
    /// QRY_APP_ERROR: Unexpected generic error in query")
    pub fn qry_app_error(&self) -> bool {
        matches!(self, QryAppError)
    }
    /// QRY_DUP_JOIN_KEY: Duplicated join key")
    pub fn qry_dup_join_key(&self) -> bool {
        matches!(self, QryDupJoinKey)
    }
    /// QRY_EXCEED_TAGS_LIMIT: Tag condition too many")
    pub fn qry_exceed_tags_limit(&self) -> bool {
        matches!(self, QryExceedTagsLimit)
    }
    /// QRY_NOT_READY: Query not ready")
    pub fn qry_not_ready(&self) -> bool {
        matches!(self, QryNotReady)
    }
    /// QRY_HAS_RSP: Query should response")
    pub fn qry_has_rsp(&self) -> bool {
        matches!(self, QryHasRsp)
    }
    /// QRY_IN_EXEC: Multiple retrieval of this query")
    pub fn qry_in_exec(&self) -> bool {
        matches!(self, QryInExec)
    }
    /// QRY_TOO_MANY_TIMEWINDOW: Too many time window in query")
    pub fn qry_too_many_timewindow(&self) -> bool {
        matches!(self, QryTooManyTimewindow)
    }
    /// QRY_NOT_ENOUGH_BUFFER: Query buffer limit has reached")
    pub fn qry_not_enough_buffer(&self) -> bool {
        matches!(self, QryNotEnoughBuffer)
    }
    /// QRY_INCONSISTAN: File inconsistency in replica")
    pub fn qry_inconsistan(&self) -> bool {
        matches!(self, QryInconsistan)
    }
    /// QRY_SYS_ERROR: System error")
    pub fn qry_sys_error(&self) -> bool {
        matches!(self, QrySysError)
    }
    /// QRY_INVALID_TIME_CONDITION: invalid time condition")
    pub fn qry_invalid_time_condition(&self) -> bool {
        matches!(self, QryInvalidTimeCondition)
    }
    /// QRY_INVALID_SCHEMA_VERSION: invalid schema version")
    pub fn qry_invalid_schema_version(&self) -> bool {
        matches!(self, QryInvalidSchemaVersion)
    }
    /// QRY_RESULT_TOO_LARGE: result num is too large")
    pub fn qry_result_too_large(&self) -> bool {
        matches!(self, QryResultTooLarge)
    }
    /// GRANT_EXPIRED: License expired"
    pub fn grant_expired(&self) -> bool {
        matches!(self, GrantExpired)
    }
    /// GRANT_DNODE_LIMITED: DNode creation limited by licence"
    pub fn grant_dnode_limited(&self) -> bool {
        matches!(self, GrantDnodeLimited)
    }
    /// GRANT_ACCT_LIMITED: Account creation limited by license"
    pub fn grant_acct_limited(&self) -> bool {
        matches!(self, GrantAcctLimited)
    }
    /// GRANT_TIMESERIES_LIMITED: Table creation limited by license"
    pub fn grant_timeseries_limited(&self) -> bool {
        matches!(self, GrantTimeseriesLimited)
    }
    /// GRANT_DB_LIMITED: DB creation limited by license"
    pub fn grant_db_limited(&self) -> bool {
        matches!(self, GrantDbLimited)
    }
    /// GRANT_USER_LIMITED: User creation limited by license"
    pub fn grant_user_limited(&self) -> bool {
        matches!(self, GrantUserLimited)
    }
    /// GRANT_CONN_LIMITED: Conn creation limited by license"
    pub fn grant_conn_limited(&self) -> bool {
        matches!(self, GrantConnLimited)
    }
    /// GRANT_STREAM_LIMITED: Stream creation limited by license"
    pub fn grant_stream_limited(&self) -> bool {
        matches!(self, GrantStreamLimited)
    }
    /// GRANT_SPEED_LIMITED: Write speed limited by license"
    pub fn grant_speed_limited(&self) -> bool {
        matches!(self, GrantSpeedLimited)
    }
    /// GRANT_STORAGE_LIMITED: Storage capacity limited by license"
    pub fn grant_storage_limited(&self) -> bool {
        matches!(self, GrantStorageLimited)
    }
    /// GRANT_QUERYTIME_LIMITED: Query time limited by license"
    pub fn grant_querytime_limited(&self) -> bool {
        matches!(self, GrantQuerytimeLimited)
    }
    /// GRANT_CPU_LIMITED: CPU cores limited by license"
    pub fn grant_cpu_limited(&self) -> bool {
        matches!(self, GrantCpuLimited)
    }
    /// SYN_INVALID_CONFIG: Invalid Sync Configuration"
    pub fn syn_invalid_config(&self) -> bool {
        matches!(self, SynInvalidConfig)
    }
    /// SYN_NOT_ENABLED: Sync module not enabled"
    pub fn syn_not_enabled(&self) -> bool {
        matches!(self, SynNotEnabled)
    }
    /// SYN_INVALID_VERSION: Invalid Sync version"
    pub fn syn_invalid_version(&self) -> bool {
        matches!(self, SynInvalidVersion)
    }
    /// SYN_CONFIRM_EXPIRED: Sync confirm expired"
    pub fn syn_confirm_expired(&self) -> bool {
        matches!(self, SynConfirmExpired)
    }
    /// SYN_TOO_MANY_FWDINFO: Too many sync fwd infos"
    pub fn syn_too_many_fwdinfo(&self) -> bool {
        matches!(self, SynTooManyFwdinfo)
    }
    /// SYN_MISMATCHED_PROTOCOL: Mismatched protocol"
    pub fn syn_mismatched_protocol(&self) -> bool {
        matches!(self, SynMismatchedProtocol)
    }
    /// SYN_MISMATCHED_CLUSTERID: Mismatched clusterId"
    pub fn syn_mismatched_clusterid(&self) -> bool {
        matches!(self, SynMismatchedClusterid)
    }
    /// SYN_MISMATCHED_SIGNATURE: Mismatched signature"
    pub fn syn_mismatched_signature(&self) -> bool {
        matches!(self, SynMismatchedSignature)
    }
    /// SYN_INVALID_CHECKSUM: Invalid msg checksum"
    pub fn syn_invalid_checksum(&self) -> bool {
        matches!(self, SynInvalidChecksum)
    }
    /// SYN_INVALID_MSGLEN: Invalid msg length"
    pub fn syn_invalid_msglen(&self) -> bool {
        matches!(self, SynInvalidMsglen)
    }
    /// SYN_INVALID_MSGTYPE: Invalid msg type"
    pub fn syn_invalid_msgtype(&self) -> bool {
        matches!(self, SynInvalidMsgtype)
    }
    /// WAL_APP_ERROR: Unexpected generic error in wal"
    pub fn wal_app_error(&self) -> bool {
        matches!(self, WalAppError)
    }
    /// WAL_FILE_CORRUPTED: WAL file is corrupted"
    pub fn wal_file_corrupted(&self) -> bool {
        matches!(self, WalFileCorrupted)
    }
    /// WAL_SIZE_LIMIT: WAL size exceeds limit"
    pub fn wal_size_limit(&self) -> bool {
        matches!(self, WalSizeLimit)
    }
    /// HTTP_SERVER_OFFLINE: http server is not online"
    pub fn http_server_offline(&self) -> bool {
        matches!(self, HttpServerOffline)
    }
    /// HTTP_UNSUPPORT_URL: url is not support"
    pub fn http_unsupport_url(&self) -> bool {
        matches!(self, HttpUnsupportUrl)
    }
    /// HTTP_INVALID_URL: nvalid url format"
    pub fn http_invalid_url(&self) -> bool {
        matches!(self, HttpInvalidUrl)
    }
    /// HTTP_NO_ENOUGH_MEMORY: no enough memory"
    pub fn http_no_enough_memory(&self) -> bool {
        matches!(self, HttpNoEnoughMemory)
    }
    /// HTTP_REQUSET_TOO_BIG: request size is too big"
    pub fn http_requset_too_big(&self) -> bool {
        matches!(self, HttpRequsetTooBig)
    }
    /// HTTP_NO_AUTH_INFO: no auth info input"
    pub fn http_no_auth_info(&self) -> bool {
        matches!(self, HttpNoAuthInfo)
    }
    /// HTTP_NO_MSG_INPUT: request is empty"
    pub fn http_no_msg_input(&self) -> bool {
        matches!(self, HttpNoMsgInput)
    }
    /// HTTP_NO_SQL_INPUT: no sql input"
    pub fn http_no_sql_input(&self) -> bool {
        matches!(self, HttpNoSqlInput)
    }
    /// HTTP_NO_EXEC_USEDB: no need to execute use db cmd"
    pub fn http_no_exec_usedb(&self) -> bool {
        matches!(self, HttpNoExecUsedb)
    }
    /// HTTP_SESSION_FULL: session list was full"
    pub fn http_session_full(&self) -> bool {
        matches!(self, HttpSessionFull)
    }
    /// HTTP_GEN_TAOSD_TOKEN_ERR: generate taosd token error"
    pub fn http_gen_taosd_token_err(&self) -> bool {
        matches!(self, HttpGenTaosdTokenErr)
    }
    /// HTTP_INVALID_MULTI_REQUEST: size of multi request is 0"
    pub fn http_invalid_multi_request(&self) -> bool {
        matches!(self, HttpInvalidMultiRequest)
    }
    /// HTTP_CREATE_GZIP_FAILED: failed to create gzip"
    pub fn http_create_gzip_failed(&self) -> bool {
        matches!(self, HttpCreateGzipFailed)
    }
    /// HTTP_FINISH_GZIP_FAILED: failed to finish gzip"
    pub fn http_finish_gzip_failed(&self) -> bool {
        matches!(self, HttpFinishGzipFailed)
    }
    /// HTTP_LOGIN_FAILED: failed to login"
    pub fn http_login_failed(&self) -> bool {
        matches!(self, HttpLoginFailed)
    }
    /// HTTP_INVALID_VERSION: invalid http version"
    pub fn http_invalid_version(&self) -> bool {
        matches!(self, HttpInvalidVersion)
    }
    /// HTTP_INVALID_CONTENT_LENGTH: invalid content length"
    pub fn http_invalid_content_length(&self) -> bool {
        matches!(self, HttpInvalidContentLength)
    }
    /// HTTP_INVALID_AUTH_TYPE: invalid type of Authorization"
    pub fn http_invalid_auth_type(&self) -> bool {
        matches!(self, HttpInvalidAuthType)
    }
    /// HTTP_INVALID_AUTH_FORMAT: invalid format of Authorization"
    pub fn http_invalid_auth_format(&self) -> bool {
        matches!(self, HttpInvalidAuthFormat)
    }
    /// HTTP_INVALID_BASIC_AUTH: invalid basic Authorization"
    pub fn http_invalid_basic_auth(&self) -> bool {
        matches!(self, HttpInvalidBasicAuth)
    }
    /// HTTP_INVALID_TAOSD_AUTH: invalid taosd Authorization"
    pub fn http_invalid_taosd_auth(&self) -> bool {
        matches!(self, HttpInvalidTaosdAuth)
    }
    /// HTTP_PARSE_METHOD_FAILED: failed to parse method"
    pub fn http_parse_method_failed(&self) -> bool {
        matches!(self, HttpParseMethodFailed)
    }
    /// HTTP_PARSE_TARGET_FAILED: failed to parse target"
    pub fn http_parse_target_failed(&self) -> bool {
        matches!(self, HttpParseTargetFailed)
    }
    /// HTTP_PARSE_VERSION_FAILED: failed to parse http version"
    pub fn http_parse_version_failed(&self) -> bool {
        matches!(self, HttpParseVersionFailed)
    }
    /// HTTP_PARSE_SP_FAILED: failed to parse sp"
    pub fn http_parse_sp_failed(&self) -> bool {
        matches!(self, HttpParseSpFailed)
    }
    /// HTTP_PARSE_STATUS_FAILED: failed to parse status"
    pub fn http_parse_status_failed(&self) -> bool {
        matches!(self, HttpParseStatusFailed)
    }
    /// HTTP_PARSE_PHRASE_FAILED: failed to parse phrase"
    pub fn http_parse_phrase_failed(&self) -> bool {
        matches!(self, HttpParsePhraseFailed)
    }
    /// HTTP_PARSE_CRLF_FAILED: failed to parse crlf"
    pub fn http_parse_crlf_failed(&self) -> bool {
        matches!(self, HttpParseCrlfFailed)
    }
    /// HTTP_PARSE_HEADER_FAILED: failed to parse header"
    pub fn http_parse_header_failed(&self) -> bool {
        matches!(self, HttpParseHeaderFailed)
    }
    /// HTTP_PARSE_HEADER_KEY_FAILED: failed to parse header key"
    pub fn http_parse_header_key_failed(&self) -> bool {
        matches!(self, HttpParseHeaderKeyFailed)
    }
    /// HTTP_PARSE_HEADER_VAL_FAILED: failed to parse header val"
    pub fn http_parse_header_val_failed(&self) -> bool {
        matches!(self, HttpParseHeaderValFailed)
    }
    /// HTTP_PARSE_CHUNK_SIZE_FAILED: failed to parse chunk size"
    pub fn http_parse_chunk_size_failed(&self) -> bool {
        matches!(self, HttpParseChunkSizeFailed)
    }
    /// HTTP_PARSE_CHUNK_FAILED: failed to parse chunk"
    pub fn http_parse_chunk_failed(&self) -> bool {
        matches!(self, HttpParseChunkFailed)
    }
    /// HTTP_PARSE_END_FAILED: failed to parse end section"
    pub fn http_parse_end_failed(&self) -> bool {
        matches!(self, HttpParseEndFailed)
    }
    /// HTTP_PARSE_INVALID_STATE: invalid parse state"
    pub fn http_parse_invalid_state(&self) -> bool {
        matches!(self, HttpParseInvalidState)
    }
    /// HTTP_PARSE_ERROR_STATE: failed to parse error section"
    pub fn http_parse_error_state(&self) -> bool {
        matches!(self, HttpParseErrorState)
    }
    /// HTTP_GC_QUERY_NULL: query size is 0"
    pub fn http_gc_query_null(&self) -> bool {
        matches!(self, HttpGcQueryNull)
    }
    /// HTTP_GC_QUERY_SIZE: query size can not more than 100"
    pub fn http_gc_query_size(&self) -> bool {
        matches!(self, HttpGcQuerySize)
    }
    /// HTTP_GC_REQ_PARSE_ERROR: parse grafana json error"
    pub fn http_gc_req_parse_error(&self) -> bool {
        matches!(self, HttpGcReqParseError)
    }
    /// HTTP_TG_DB_NOT_INPUT: database name can not be null"
    pub fn http_tg_db_not_input(&self) -> bool {
        matches!(self, HttpTgDbNotInput)
    }
    /// HTTP_TG_DB_TOO_LONG: database name too long"
    pub fn http_tg_db_too_long(&self) -> bool {
        matches!(self, HttpTgDbTooLong)
    }
    /// HTTP_TG_INVALID_JSON: invalid telegraf json fromat"
    pub fn http_tg_invalid_json(&self) -> bool {
        matches!(self, HttpTgInvalidJson)
    }
    /// HTTP_TG_METRICS_NULL: metrics size is 0"
    pub fn http_tg_metrics_null(&self) -> bool {
        matches!(self, HttpTgMetricsNull)
    }
    /// HTTP_TG_METRICS_SIZE: metrics size can not more than 1K"
    pub fn http_tg_metrics_size(&self) -> bool {
        matches!(self, HttpTgMetricsSize)
    }
    /// HTTP_TG_METRIC_NULL: metric name not find"
    pub fn http_tg_metric_null(&self) -> bool {
        matches!(self, HttpTgMetricNull)
    }
    /// HTTP_TG_METRIC_TYPE: metric name type should be string"
    pub fn http_tg_metric_type(&self) -> bool {
        matches!(self, HttpTgMetricType)
    }
    /// HTTP_TG_METRIC_NAME_NULL: metric name length is 0"
    pub fn http_tg_metric_name_null(&self) -> bool {
        matches!(self, HttpTgMetricNameNull)
    }
    /// HTTP_TG_METRIC_NAME_LONG: metric name length too long"
    pub fn http_tg_metric_name_long(&self) -> bool {
        matches!(self, HttpTgMetricNameLong)
    }
    /// HTTP_TG_TIMESTAMP_NULL: timestamp not find"
    pub fn http_tg_timestamp_null(&self) -> bool {
        matches!(self, HttpTgTimestampNull)
    }
    /// HTTP_TG_TIMESTAMP_TYPE: timestamp type should be integer"
    pub fn http_tg_timestamp_type(&self) -> bool {
        matches!(self, HttpTgTimestampType)
    }
    /// HTTP_TG_TIMESTAMP_VAL_NULL: timestamp value smaller than 0"
    pub fn http_tg_timestamp_val_null(&self) -> bool {
        matches!(self, HttpTgTimestampValNull)
    }
    /// HTTP_TG_TAGS_NULL: tags not find"
    pub fn http_tg_tags_null(&self) -> bool {
        matches!(self, HttpTgTagsNull)
    }
    /// HTTP_TG_TAGS_SIZE_0: tags size is 0"
    pub fn http_tg_tags_size_0(&self) -> bool {
        matches!(self, HttpTgTagsSize0)
    }
    /// HTTP_TG_TAGS_SIZE_LONG: tags size too long"
    pub fn http_tg_tags_size_long(&self) -> bool {
        matches!(self, HttpTgTagsSizeLong)
    }
    /// HTTP_TG_TAG_NULL: tag is null"
    pub fn http_tg_tag_null(&self) -> bool {
        matches!(self, HttpTgTagNull)
    }
    /// HTTP_TG_TAG_NAME_NULL: tag name is null"
    pub fn http_tg_tag_name_null(&self) -> bool {
        matches!(self, HttpTgTagNameNull)
    }
    /// HTTP_TG_TAG_NAME_SIZE: tag name length too long"
    pub fn http_tg_tag_name_size(&self) -> bool {
        matches!(self, HttpTgTagNameSize)
    }
    /// HTTP_TG_TAG_VALUE_TYPE: tag value type should be number or string"
    pub fn http_tg_tag_value_type(&self) -> bool {
        matches!(self, HttpTgTagValueType)
    }
    /// HTTP_TG_TAG_VALUE_NULL: tag value is null"
    pub fn http_tg_tag_value_null(&self) -> bool {
        matches!(self, HttpTgTagValueNull)
    }
    /// HTTP_TG_TABLE_NULL: table is null"
    pub fn http_tg_table_null(&self) -> bool {
        matches!(self, HttpTgTableNull)
    }
    /// HTTP_TG_TABLE_SIZE: table name length too long"
    pub fn http_tg_table_size(&self) -> bool {
        matches!(self, HttpTgTableSize)
    }
    /// HTTP_TG_FIELDS_NULL: fields not find"
    pub fn http_tg_fields_null(&self) -> bool {
        matches!(self, HttpTgFieldsNull)
    }
    /// HTTP_TG_FIELDS_SIZE_0: fields size is 0"
    pub fn http_tg_fields_size_0(&self) -> bool {
        matches!(self, HttpTgFieldsSize0)
    }
    /// HTTP_TG_FIELDS_SIZE_LONG: fields size too long"
    pub fn http_tg_fields_size_long(&self) -> bool {
        matches!(self, HttpTgFieldsSizeLong)
    }
    /// HTTP_TG_FIELD_NULL: field is null"
    pub fn http_tg_field_null(&self) -> bool {
        matches!(self, HttpTgFieldNull)
    }
    /// HTTP_TG_FIELD_NAME_NULL: field name is null"
    pub fn http_tg_field_name_null(&self) -> bool {
        matches!(self, HttpTgFieldNameNull)
    }
    /// HTTP_TG_FIELD_NAME_SIZE: field name length too long"
    pub fn http_tg_field_name_size(&self) -> bool {
        matches!(self, HttpTgFieldNameSize)
    }
    /// HTTP_TG_FIELD_VALUE_TYPE: field value type should be number or string"
    pub fn http_tg_field_value_type(&self) -> bool {
        matches!(self, HttpTgFieldValueType)
    }
    /// HTTP_TG_FIELD_VALUE_NULL: field value is null"
    pub fn http_tg_field_value_null(&self) -> bool {
        matches!(self, HttpTgFieldValueNull)
    }
    /// HTTP_TG_HOST_NOT_STRING: host type should be string"
    pub fn http_tg_host_not_string(&self) -> bool {
        matches!(self, HttpTgHostNotString)
    }
    /// HTTP_TG_STABLE_NOT_EXIST: stable not exist"
    pub fn http_tg_stable_not_exist(&self) -> bool {
        matches!(self, HttpTgStableNotExist)
    }
    /// HTTP_OP_DB_NOT_INPUT: database name can not be null"
    pub fn http_op_db_not_input(&self) -> bool {
        matches!(self, HttpOpDbNotInput)
    }
    /// HTTP_OP_DB_TOO_LONG: database name too long"
    pub fn http_op_db_too_long(&self) -> bool {
        matches!(self, HttpOpDbTooLong)
    }
    /// HTTP_OP_INVALID_JSON: invalid opentsdb json fromat"
    pub fn http_op_invalid_json(&self) -> bool {
        matches!(self, HttpOpInvalidJson)
    }
    /// HTTP_OP_METRICS_NULL: metrics size is 0"
    pub fn http_op_metrics_null(&self) -> bool {
        matches!(self, HttpOpMetricsNull)
    }
    /// HTTP_OP_METRICS_SIZE: metrics size can not more than 10K"
    pub fn http_op_metrics_size(&self) -> bool {
        matches!(self, HttpOpMetricsSize)
    }
    /// HTTP_OP_METRIC_NULL: metric name not find"
    pub fn http_op_metric_null(&self) -> bool {
        matches!(self, HttpOpMetricNull)
    }
    /// HTTP_OP_METRIC_TYPE: metric name type should be string"
    pub fn http_op_metric_type(&self) -> bool {
        matches!(self, HttpOpMetricType)
    }
    /// HTTP_OP_METRIC_NAME_NULL: metric name length is 0"
    pub fn http_op_metric_name_null(&self) -> bool {
        matches!(self, HttpOpMetricNameNull)
    }
    /// HTTP_OP_METRIC_NAME_LONG: metric name length can not more than 22"
    pub fn http_op_metric_name_long(&self) -> bool {
        matches!(self, HttpOpMetricNameLong)
    }
    /// HTTP_OP_TIMESTAMP_NULL: timestamp not find"
    pub fn http_op_timestamp_null(&self) -> bool {
        matches!(self, HttpOpTimestampNull)
    }
    /// HTTP_OP_TIMESTAMP_TYPE: timestamp type should be integer"
    pub fn http_op_timestamp_type(&self) -> bool {
        matches!(self, HttpOpTimestampType)
    }
    /// HTTP_OP_TIMESTAMP_VAL_NULL: timestamp value smaller than 0"
    pub fn http_op_timestamp_val_null(&self) -> bool {
        matches!(self, HttpOpTimestampValNull)
    }
    /// HTTP_OP_TAGS_NULL: tags not find"
    pub fn http_op_tags_null(&self) -> bool {
        matches!(self, HttpOpTagsNull)
    }
    /// HTTP_OP_TAGS_SIZE_0: tags size is 0"
    pub fn http_op_tags_size_0(&self) -> bool {
        matches!(self, HttpOpTagsSize0)
    }
    /// HTTP_OP_TAGS_SIZE_LONG: tags size too long"
    pub fn http_op_tags_size_long(&self) -> bool {
        matches!(self, HttpOpTagsSizeLong)
    }
    /// HTTP_OP_TAG_NULL: tag is null"
    pub fn http_op_tag_null(&self) -> bool {
        matches!(self, HttpOpTagNull)
    }
    /// HTTP_OP_TAG_NAME_NULL: tag name is null"
    pub fn http_op_tag_name_null(&self) -> bool {
        matches!(self, HttpOpTagNameNull)
    }
    /// HTTP_OP_TAG_NAME_SIZE: tag name length too long"
    pub fn http_op_tag_name_size(&self) -> bool {
        matches!(self, HttpOpTagNameSize)
    }
    /// HTTP_OP_TAG_VALUE_TYPE: tag value type should be boolean number or string"
    pub fn http_op_tag_value_type(&self) -> bool {
        matches!(self, HttpOpTagValueType)
    }
    /// HTTP_OP_TAG_VALUE_NULL: tag value is null"
    pub fn http_op_tag_value_null(&self) -> bool {
        matches!(self, HttpOpTagValueNull)
    }
    /// HTTP_OP_TAG_VALUE_TOO_LONG: tag value can not more than 64"
    pub fn http_op_tag_value_too_long(&self) -> bool {
        matches!(self, HttpOpTagValueTooLong)
    }
    /// HTTP_OP_VALUE_NULL: value not find"
    pub fn http_op_value_null(&self) -> bool {
        matches!(self, HttpOpValueNull)
    }
    /// HTTP_OP_VALUE_TYPE: value type should be boolean number or string"
    pub fn http_op_value_type(&self) -> bool {
        matches!(self, HttpOpValueType)
    }
    /// HTTP_REQUEST_JSON_ERROR: http request json error"
    pub fn http_request_json_error(&self) -> bool {
        matches!(self, HttpRequestJsonError)
    }
    /// ODBC_OOM: out of memory"
    pub fn odbc_oom(&self) -> bool {
        matches!(self, OdbcOom)
    }
    /// ODBC_CONV_CHAR_NOT_NUM: convertion not a valid literal input"
    pub fn odbc_conv_char_not_num(&self) -> bool {
        matches!(self, OdbcConvCharNotNum)
    }
    /// ODBC_CONV_UNDEF: convertion undefined"
    pub fn odbc_conv_undef(&self) -> bool {
        matches!(self, OdbcConvUndef)
    }
    /// ODBC_CONV_TRUNC_FRAC: convertion fractional truncated"
    pub fn odbc_conv_trunc_frac(&self) -> bool {
        matches!(self, OdbcConvTruncFrac)
    }
    /// ODBC_CONV_TRUNC: convertion truncated"
    pub fn odbc_conv_trunc(&self) -> bool {
        matches!(self, OdbcConvTrunc)
    }
    /// ODBC_CONV_NOT_SUPPORT: convertion not supported"
    pub fn odbc_conv_not_support(&self) -> bool {
        matches!(self, OdbcConvNotSupport)
    }
    /// ODBC_CONV_OOR: convertion numeric value out of range"
    pub fn odbc_conv_oor(&self) -> bool {
        matches!(self, OdbcConvOor)
    }
    /// ODBC_OUT_OF_RANGE: out of range"
    pub fn odbc_out_of_range(&self) -> bool {
        matches!(self, OdbcOutOfRange)
    }
    /// ODBC_NOT_SUPPORT: not supported yet"
    pub fn odbc_not_support(&self) -> bool {
        matches!(self, OdbcNotSupport)
    }
    /// ODBC_INVALID_HANDLE: invalid handle"
    pub fn odbc_invalid_handle(&self) -> bool {
        matches!(self, OdbcInvalidHandle)
    }
    /// ODBC_NO_RESULT: no result set"
    pub fn odbc_no_result(&self) -> bool {
        matches!(self, OdbcNoResult)
    }
    /// ODBC_NO_FIELDS: no fields returned"
    pub fn odbc_no_fields(&self) -> bool {
        matches!(self, OdbcNoFields)
    }
    /// ODBC_INVALID_CURSOR: invalid cursor"
    pub fn odbc_invalid_cursor(&self) -> bool {
        matches!(self, OdbcInvalidCursor)
    }
    /// ODBC_STATEMENT_NOT_READY: statement not ready"
    pub fn odbc_statement_not_ready(&self) -> bool {
        matches!(self, OdbcStatementNotReady)
    }
    /// ODBC_CONNECTION_BUSY: connection still busy"
    pub fn odbc_connection_busy(&self) -> bool {
        matches!(self, OdbcConnectionBusy)
    }
    /// ODBC_BAD_CONNSTR: bad connection string"
    pub fn odbc_bad_connstr(&self) -> bool {
        matches!(self, OdbcBadConnstr)
    }
    /// ODBC_BAD_ARG: bad argument"
    pub fn odbc_bad_arg(&self) -> bool {
        matches!(self, OdbcBadArg)
    }
    /// ODBC_CONV_NOT_VALID_TS: not a valid timestamp"
    pub fn odbc_conv_not_valid_ts(&self) -> bool {
        matches!(self, OdbcConvNotValidTs)
    }
    /// ODBC_CONV_SRC_TOO_LARGE: src too large"
    pub fn odbc_conv_src_too_large(&self) -> bool {
        matches!(self, OdbcConvSrcTooLarge)
    }
    /// ODBC_CONV_SRC_BAD_SEQ: src bad sequence"
    pub fn odbc_conv_src_bad_seq(&self) -> bool {
        matches!(self, OdbcConvSrcBadSeq)
    }
    /// ODBC_CONV_SRC_INCOMPLETE: src incomplete"
    pub fn odbc_conv_src_incomplete(&self) -> bool {
        matches!(self, OdbcConvSrcIncomplete)
    }
    /// ODBC_CONV_SRC_GENERAL: src general"
    pub fn odbc_conv_src_general(&self) -> bool {
        matches!(self, OdbcConvSrcGeneral)
    }
    /// FS_OUT_OF_MEMORY: tfs out of memory"
    pub fn fs_out_of_memory(&self) -> bool {
        matches!(self, FsOutOfMemory)
    }
    /// FS_INVLD_CFG: tfs invalid mount config"
    pub fn fs_invld_cfg(&self) -> bool {
        matches!(self, FsInvldCfg)
    }
    /// FS_TOO_MANY_MOUNT: tfs too many mount"
    pub fn fs_too_many_mount(&self) -> bool {
        matches!(self, FsTooManyMount)
    }
    /// FS_DUP_PRIMARY: tfs duplicate primary mount"
    pub fn fs_dup_primary(&self) -> bool {
        matches!(self, FsDupPrimary)
    }
    /// FS_NO_PRIMARY_DISK: tfs no primary mount"
    pub fn fs_no_primary_disk(&self) -> bool {
        matches!(self, FsNoPrimaryDisk)
    }
    /// FS_NO_MOUNT_AT_TIER: tfs no mount at tier"
    pub fn fs_no_mount_at_tier(&self) -> bool {
        matches!(self, FsNoMountAtTier)
    }
    /// FS_FILE_ALREADY_EXISTS: tfs file already exists"
    pub fn fs_file_already_exists(&self) -> bool {
        matches!(self, FsFileAlreadyExists)
    }
    /// FS_INVLD_LEVEL: tfs invalid level"
    pub fn fs_invld_level(&self) -> bool {
        matches!(self, FsInvldLevel)
    }
    /// FS_NO_VALID_DISK: tfs no valid disk"
    pub fn fs_no_valid_disk(&self) -> bool {
        matches!(self, FsNoValidDisk)
    }
    /// MON_CONNECTION_INVALID: monitor invalid monitor db connection"
    pub fn mon_connection_invalid(&self) -> bool {
        matches!(self, MonConnectionInvalid)
    }
}

#[test]
fn test_code_from() {
    let c = dbg!(TaosCode::from(0));
    assert_eq!(c, Success);
    assert!(c.success());
    let c = TaosCode::from(-1);
    assert_eq!(c, Unknown);
}
#[test]
fn test_codes() {
    let code = TaosCode::from(0);
    assert!(code.success());
    assert_eq!(code, TaosCode::Success);


    let c = TaosCode::from(0x0001);
    assert!(c.rpc_action_in_progress());
    assert_eq!(c, RpcActionInProgress);

    let c = TaosCode::from(0x0002);
    assert!(c.rpc_auth_required());
    assert_eq!(c, RpcAuthRequired);

    let c = TaosCode::from(0x0003);
    assert!(c.rpc_auth_failure());
    assert_eq!(c, RpcAuthFailure);

    let c = TaosCode::from(0x0004);
    assert!(c.rpc_redirect());
    assert_eq!(c, RpcRedirect);

    let c = TaosCode::from(0x0005);
    assert!(c.rpc_not_ready());
    assert_eq!(c, RpcNotReady);

    let c = TaosCode::from(0x0006);
    assert!(c.rpc_already_processed());
    assert_eq!(c, RpcAlreadyProcessed);

    let c = TaosCode::from(0x0007);
    assert!(c.rpc_last_session_not_finished());
    assert_eq!(c, RpcLastSessionNotFinished);

    let c = TaosCode::from(0x0008);
    assert!(c.rpc_mismatched_link_id());
    assert_eq!(c, RpcMismatchedLinkId);

    let c = TaosCode::from(0x0009);
    assert!(c.rpc_too_slow());
    assert_eq!(c, RpcTooSlow);

    let c = TaosCode::from(0x000A);
    assert!(c.rpc_max_sessions());
    assert_eq!(c, RpcMaxSessions);

    let c = TaosCode::from(0x000B);
    assert!(c.rpc_network_unavail());
    assert_eq!(c, RpcNetworkUnavail);

    let c = TaosCode::from(0x000C);
    assert!(c.rpc_app_error());
    assert_eq!(c, RpcAppError);

    let c = TaosCode::from(0x000D);
    assert!(c.rpc_unexpected_response());
    assert_eq!(c, RpcUnexpectedResponse);

    let c = TaosCode::from(0x000E);
    assert!(c.rpc_invalid_value());
    assert_eq!(c, RpcInvalidValue);

    let c = TaosCode::from(0x000F);
    assert!(c.rpc_invalid_tran_id());
    assert_eq!(c, RpcInvalidTranId);

    let c = TaosCode::from(0x0010);
    assert!(c.rpc_invalid_session_id());
    assert_eq!(c, RpcInvalidSessionId);

    let c = TaosCode::from(0x0011);
    assert!(c.rpc_invalid_msg_type());
    assert_eq!(c, RpcInvalidMsgType);

    let c = TaosCode::from(0x0012);
    assert!(c.rpc_invalid_response_type());
    assert_eq!(c, RpcInvalidResponseType);

    let c = TaosCode::from(0x0013);
    assert!(c.rpc_invalid_time_stamp());
    assert_eq!(c, RpcInvalidTimeStamp);

    let c = TaosCode::from(0x0014);
    assert!(c.app_not_ready());
    assert_eq!(c, AppNotReady);

    let c = TaosCode::from(0x0015);
    assert!(c.rpc_fqdn_error());
    assert_eq!(c, RpcFqdnError);

    let c = TaosCode::from(0x0016);
    assert!(c.rpc_invalid_version());
    assert_eq!(c, RpcInvalidVersion);

    let c = TaosCode::from(0x0100);
    assert!(c.com_ops_not_support());
    assert_eq!(c, ComOpsNotSupport);

    let c = TaosCode::from(0x0101);
    assert!(c.com_memory_corrupted());
    assert_eq!(c, ComMemoryCorrupted);

    let c = TaosCode::from(0x0102);
    assert!(c.com_out_of_memory());
    assert_eq!(c, ComOutOfMemory);

    let c = TaosCode::from(0x0103);
    assert!(c.com_invalid_cfg_msg());
    assert_eq!(c, ComInvalidCfgMsg);

    let c = TaosCode::from(0x0104);
    assert!(c.com_file_corrupted());
    assert_eq!(c, ComFileCorrupted);

    let c = TaosCode::from(0x0105);
    assert!(c.ref_no_memory());
    assert_eq!(c, RefNoMemory);

    let c = TaosCode::from(0x0106);
    assert!(c.ref_full());
    assert_eq!(c, RefFull);

    let c = TaosCode::from(0x0107);
    assert!(c.ref_id_removed());
    assert_eq!(c, RefIdRemoved);

    let c = TaosCode::from(0x0108);
    assert!(c.ref_invalid_id());
    assert_eq!(c, RefInvalidId);

    let c = TaosCode::from(0x0109);
    assert!(c.ref_already_exist());
    assert_eq!(c, RefAlreadyExist);

    let c = TaosCode::from(0x010A);
    assert!(c.ref_not_exist());
    assert_eq!(c, RefNotExist);

    let c = TaosCode::from(0x0200);
    assert!(c.tsc_invalid_operation());
    assert_eq!(c, TscInvalidOperation);

    let c = TaosCode::from(0x0201);
    assert!(c.tsc_invalid_qhandle());
    assert_eq!(c, TscInvalidQhandle);

    let c = TaosCode::from(0x0202);
    assert!(c.tsc_invalid_time_stamp());
    assert_eq!(c, TscInvalidTimeStamp);

    let c = TaosCode::from(0x0203);
    assert!(c.tsc_invalid_value());
    assert_eq!(c, TscInvalidValue);

    let c = TaosCode::from(0x0204);
    assert!(c.tsc_invalid_version());
    assert_eq!(c, TscInvalidVersion);

    let c = TaosCode::from(0x0205);
    assert!(c.tsc_invalid_ie());
    assert_eq!(c, TscInvalidIe);

    let c = TaosCode::from(0x0206);
    assert!(c.tsc_invalid_fqdn());
    assert_eq!(c, TscInvalidFqdn);

    let c = TaosCode::from(0x0207);
    assert!(c.tsc_invalid_user_length());
    assert_eq!(c, TscInvalidUserLength);

    let c = TaosCode::from(0x0208);
    assert!(c.tsc_invalid_pass_length());
    assert_eq!(c, TscInvalidPassLength);

    let c = TaosCode::from(0x0209);
    assert!(c.tsc_invalid_db_length());
    assert_eq!(c, TscInvalidDbLength);

    let c = TaosCode::from(0x020A);
    assert!(c.tsc_invalid_table_id_length());
    assert_eq!(c, TscInvalidTableIdLength);

    let c = TaosCode::from(0x020B);
    assert!(c.tsc_invalid_connection());
    assert_eq!(c, TscInvalidConnection);

    let c = TaosCode::from(0x020C);
    assert!(c.tsc_out_of_memory());
    assert_eq!(c, TscOutOfMemory);

    let c = TaosCode::from(0x020D);
    assert!(c.tsc_no_diskspace());
    assert_eq!(c, TscNoDiskspace);

    let c = TaosCode::from(0x020E);
    assert!(c.tsc_query_cache_erased());
    assert_eq!(c, TscQueryCacheErased);

    let c = TaosCode::from(0x020F);
    assert!(c.tsc_query_cancelled());
    assert_eq!(c, TscQueryCancelled);

    let c = TaosCode::from(0x0210);
    assert!(c.tsc_sorted_res_too_many());
    assert_eq!(c, TscSortedResTooMany);

    let c = TaosCode::from(0x0211);
    assert!(c.tsc_app_error());
    assert_eq!(c, TscAppError);

    let c = TaosCode::from(0x0212);
    assert!(c.tsc_action_in_progress());
    assert_eq!(c, TscActionInProgress);

    let c = TaosCode::from(0x0213);
    assert!(c.tsc_disconnected());
    assert_eq!(c, TscDisconnected);

    let c = TaosCode::from(0x0214);
    assert!(c.tsc_no_write_auth());
    assert_eq!(c, TscNoWriteAuth);

    let c = TaosCode::from(0x0215);
    assert!(c.tsc_conn_killed());
    assert_eq!(c, TscConnKilled);

    let c = TaosCode::from(0x0216);
    assert!(c.tsc_sql_syntax_error());
    assert_eq!(c, TscSqlSyntaxError);

    let c = TaosCode::from(0x0217);
    assert!(c.tsc_db_not_selected());
    assert_eq!(c, TscDbNotSelected);

    let c = TaosCode::from(0x0218);
    assert!(c.tsc_invalid_table_name());
    assert_eq!(c, TscInvalidTableName);

    let c = TaosCode::from(0x0219);
    assert!(c.tsc_exceed_sql_limit());
    assert_eq!(c, TscExceedSqlLimit);

    let c = TaosCode::from(0x021A);
    assert!(c.tsc_file_empty());
    assert_eq!(c, TscFileEmpty);

    let c = TaosCode::from(0x021B);
    assert!(c.tsc_line_syntax_error());
    assert_eq!(c, TscLineSyntaxError);

    let c = TaosCode::from(0x021C);
    assert!(c.tsc_no_meta_cached());
    assert_eq!(c, TscNoMetaCached);

    let c = TaosCode::from(0x021D);
    assert!(c.tsc_dup_col_names());
    assert_eq!(c, TscDupColNames);

    let c = TaosCode::from(0x021E);
    assert!(c.tsc_invalid_tag_length());
    assert_eq!(c, TscInvalidTagLength);

    let c = TaosCode::from(0x021F);
    assert!(c.tsc_invalid_column_length());
    assert_eq!(c, TscInvalidColumnLength);

    let c = TaosCode::from(0x0220);
    assert!(c.tsc_dup_tag_names());
    assert_eq!(c, TscDupTagNames);

    let c = TaosCode::from(0x0221);
    assert!(c.tsc_invalid_json());
    assert_eq!(c, TscInvalidJson);

    let c = TaosCode::from(0x0222);
    assert!(c.tsc_invalid_json_type());
    assert_eq!(c, TscInvalidJsonType);

    let c = TaosCode::from(0x0223);
    assert!(c.tsc_invalid_json_config());
    assert_eq!(c, TscInvalidJsonConfig);

    let c = TaosCode::from(0x0224);
    assert!(c.tsc_value_out_of_range());
    assert_eq!(c, TscValueOutOfRange);

    let c = TaosCode::from(0x0225);
    assert!(c.tsc_invalid_protocol_type());
    assert_eq!(c, TscInvalidProtocolType);

    let c = TaosCode::from(0x0226);
    assert!(c.tsc_invalid_precision_type());
    assert_eq!(c, TscInvalidPrecisionType);

    let c = TaosCode::from(0x0227);
    assert!(c.tsc_res_too_many());
    assert_eq!(c, TscResTooMany);

    let c = TaosCode::from(0x0228);
    assert!(c.tsc_invalid_schema_version());
    assert_eq!(c, TscInvalidSchemaVersion);

    let c = TaosCode::from(0x0300);
    assert!(c.mnd_msg_not_processed());
    assert_eq!(c, MndMsgNotProcessed);

    let c = TaosCode::from(0x0301);
    assert!(c.mnd_action_in_progress());
    assert_eq!(c, MndActionInProgress);

    let c = TaosCode::from(0x0302);
    assert!(c.mnd_action_need_reprocessed());
    assert_eq!(c, MndActionNeedReprocessed);

    let c = TaosCode::from(0x0303);
    assert!(c.mnd_no_rights());
    assert_eq!(c, MndNoRights);

    let c = TaosCode::from(0x0304);
    assert!(c.mnd_app_error());
    assert_eq!(c, MndAppError);

    let c = TaosCode::from(0x0305);
    assert!(c.mnd_invalid_connection());
    assert_eq!(c, MndInvalidConnection);

    let c = TaosCode::from(0x0306);
    assert!(c.mnd_invalid_msg_version());
    assert_eq!(c, MndInvalidMsgVersion);

    let c = TaosCode::from(0x0307);
    assert!(c.mnd_invalid_msg_len());
    assert_eq!(c, MndInvalidMsgLen);

    let c = TaosCode::from(0x0308);
    assert!(c.mnd_invalid_msg_type());
    assert_eq!(c, MndInvalidMsgType);

    let c = TaosCode::from(0x0309);
    assert!(c.mnd_too_many_shell_conns());
    assert_eq!(c, MndTooManyShellConns);

    let c = TaosCode::from(0x030A);
    assert!(c.mnd_out_of_memory());
    assert_eq!(c, MndOutOfMemory);

    let c = TaosCode::from(0x030B);
    assert!(c.mnd_invalid_showobj());
    assert_eq!(c, MndInvalidShowobj);

    let c = TaosCode::from(0x030C);
    assert!(c.mnd_invalid_query_id());
    assert_eq!(c, MndInvalidQueryId);

    let c = TaosCode::from(0x030D);
    assert!(c.mnd_invalid_stream_id());
    assert_eq!(c, MndInvalidStreamId);

    let c = TaosCode::from(0x030E);
    assert!(c.mnd_invalid_conn_id());
    assert_eq!(c, MndInvalidConnId);

    let c = TaosCode::from(0x0310);
    assert!(c.mnd_mnode_is_running());
    assert_eq!(c, MndMnodeIsRunning);

    let c = TaosCode::from(0x0311);
    assert!(c.mnd_failed_to_config_sync());
    assert_eq!(c, MndFailedToConfigSync);

    let c = TaosCode::from(0x0312);
    assert!(c.mnd_failed_to_start_sync());
    assert_eq!(c, MndFailedToStartSync);

    let c = TaosCode::from(0x0313);
    assert!(c.mnd_failed_to_create_dir());
    assert_eq!(c, MndFailedToCreateDir);

    let c = TaosCode::from(0x0314);
    assert!(c.mnd_failed_to_init_step());
    assert_eq!(c, MndFailedToInitStep);

    let c = TaosCode::from(0x0320);
    assert!(c.mnd_sdb_obj_already_there());
    assert_eq!(c, MndSdbObjAlreadyThere);

    let c = TaosCode::from(0x0321);
    assert!(c.mnd_sdb_error());
    assert_eq!(c, MndSdbError);

    let c = TaosCode::from(0x0322);
    assert!(c.mnd_sdb_invalid_table_type());
    assert_eq!(c, MndSdbInvalidTableType);

    let c = TaosCode::from(0x0323);
    assert!(c.mnd_sdb_obj_not_there());
    assert_eq!(c, MndSdbObjNotThere);

    let c = TaosCode::from(0x0324);
    assert!(c.mnd_sdb_invaid_meta_row());
    assert_eq!(c, MndSdbInvaidMetaRow);

    let c = TaosCode::from(0x0325);
    assert!(c.mnd_sdb_invaid_key_type());
    assert_eq!(c, MndSdbInvaidKeyType);

    let c = TaosCode::from(0x0330);
    assert!(c.mnd_dnode_already_exist());
    assert_eq!(c, MndDnodeAlreadyExist);

    let c = TaosCode::from(0x0331);
    assert!(c.mnd_dnode_not_exist());
    assert_eq!(c, MndDnodeNotExist);

    let c = TaosCode::from(0x0332);
    assert!(c.mnd_vgroup_not_exist());
    assert_eq!(c, MndVgroupNotExist);

    let c = TaosCode::from(0x0333);
    assert!(c.mnd_no_remove_master());
    assert_eq!(c, MndNoRemoveMaster);

    let c = TaosCode::from(0x0334);
    assert!(c.mnd_no_enough_dnodes());
    assert_eq!(c, MndNoEnoughDnodes);

    let c = TaosCode::from(0x0335);
    assert!(c.mnd_cluster_cfg_inconsistent());
    assert_eq!(c, MndClusterCfgInconsistent);

    let c = TaosCode::from(0x0336);
    assert!(c.mnd_invalid_dnode_cfg_option());
    assert_eq!(c, MndInvalidDnodeCfgOption);

    let c = TaosCode::from(0x0337);
    assert!(c.mnd_balance_enabled());
    assert_eq!(c, MndBalanceEnabled);

    let c = TaosCode::from(0x0338);
    assert!(c.mnd_vgroup_not_in_dnode());
    assert_eq!(c, MndVgroupNotInDnode);

    let c = TaosCode::from(0x0339);
    assert!(c.mnd_vgroup_already_in_dnode());
    assert_eq!(c, MndVgroupAlreadyInDnode);

    let c = TaosCode::from(0x033A);
    assert!(c.mnd_dnode_not_free());
    assert_eq!(c, MndDnodeNotFree);

    let c = TaosCode::from(0x033B);
    assert!(c.mnd_invalid_cluster_id());
    assert_eq!(c, MndInvalidClusterId);

    let c = TaosCode::from(0x033C);
    assert!(c.mnd_not_ready());
    assert_eq!(c, MndNotReady);

    let c = TaosCode::from(0x033D);
    assert!(c.mnd_dnode_id_not_configured());
    assert_eq!(c, MndDnodeIdNotConfigured);

    let c = TaosCode::from(0x033E);
    assert!(c.mnd_dnode_ep_not_configured());
    assert_eq!(c, MndDnodeEpNotConfigured);

    let c = TaosCode::from(0x0340);
    assert!(c.mnd_acct_already_exist());
    assert_eq!(c, MndAcctAlreadyExist);

    let c = TaosCode::from(0x0341);
    assert!(c.mnd_invalid_acct());
    assert_eq!(c, MndInvalidAcct);

    let c = TaosCode::from(0x0342);
    assert!(c.mnd_invalid_acct_option());
    assert_eq!(c, MndInvalidAcctOption);

    let c = TaosCode::from(0x0343);
    assert!(c.mnd_acct_expired());
    assert_eq!(c, MndAcctExpired);

    let c = TaosCode::from(0x0350);
    assert!(c.mnd_user_already_exist());
    assert_eq!(c, MndUserAlreadyExist);

    let c = TaosCode::from(0x0351);
    assert!(c.mnd_invalid_user());
    assert_eq!(c, MndInvalidUser);

    let c = TaosCode::from(0x0352);
    assert!(c.mnd_invalid_user_format());
    assert_eq!(c, MndInvalidUserFormat);

    let c = TaosCode::from(0x0353);
    assert!(c.mnd_invalid_pass_format());
    assert_eq!(c, MndInvalidPassFormat);

    let c = TaosCode::from(0x0354);
    assert!(c.mnd_no_user_from_conn());
    assert_eq!(c, MndNoUserFromConn);

    let c = TaosCode::from(0x0355);
    assert!(c.mnd_too_many_users());
    assert_eq!(c, MndTooManyUsers);

    let c = TaosCode::from(0x0360);
    assert!(c.mnd_table_already_exist());
    assert_eq!(c, MndTableAlreadyExist);

    let c = TaosCode::from(0x0361);
    assert!(c.mnd_invalid_table_id());
    assert_eq!(c, MndInvalidTableId);

    let c = TaosCode::from(0x0362);
    assert!(c.mnd_invalid_table_name());
    assert_eq!(c, MndInvalidTableName);

    let c = TaosCode::from(0x0363);
    assert!(c.mnd_invalid_table_type());
    assert_eq!(c, MndInvalidTableType);

    let c = TaosCode::from(0x0364);
    assert!(c.mnd_too_many_tags());
    assert_eq!(c, MndTooManyTags);

    let c = TaosCode::from(0x0365);
    assert!(c.mnd_too_many_columns());
    assert_eq!(c, MndTooManyColumns);

    let c = TaosCode::from(0x0366);
    assert!(c.mnd_too_many_timeseries());
    assert_eq!(c, MndTooManyTimeseries);

    let c = TaosCode::from(0x0367);
    assert!(c.mnd_not_super_table());
    assert_eq!(c, MndNotSuperTable);

    let c = TaosCode::from(0x0368);
    assert!(c.mnd_col_name_too_long());
    assert_eq!(c, MndColNameTooLong);

    let c = TaosCode::from(0x0369);
    assert!(c.mnd_tag_alreay_exist());
    assert_eq!(c, MndTagAlreayExist);

    let c = TaosCode::from(0x036A);
    assert!(c.mnd_tag_not_exist());
    assert_eq!(c, MndTagNotExist);

    let c = TaosCode::from(0x036B);
    assert!(c.mnd_field_alreay_exist());
    assert_eq!(c, MndFieldAlreayExist);

    let c = TaosCode::from(0x036C);
    assert!(c.mnd_field_not_exist());
    assert_eq!(c, MndFieldNotExist);

    let c = TaosCode::from(0x036D);
    assert!(c.mnd_invalid_stable_name());
    assert_eq!(c, MndInvalidStableName);

    let c = TaosCode::from(0x036E);
    assert!(c.mnd_invalid_create_table_msg());
    assert_eq!(c, MndInvalidCreateTableMsg);

    let c = TaosCode::from(0x036F);
    assert!(c.mnd_exceed_max_row_bytes());
    assert_eq!(c, MndExceedMaxRowBytes);

    let c = TaosCode::from(0x0370);
    assert!(c.mnd_invalid_func_name());
    assert_eq!(c, MndInvalidFuncName);

    let c = TaosCode::from(0x0371);
    assert!(c.mnd_invalid_func_len());
    assert_eq!(c, MndInvalidFuncLen);

    let c = TaosCode::from(0x0372);
    assert!(c.mnd_invalid_func_code());
    assert_eq!(c, MndInvalidFuncCode);

    let c = TaosCode::from(0x0373);
    assert!(c.mnd_func_already_exist());
    assert_eq!(c, MndFuncAlreadyExist);

    let c = TaosCode::from(0x0374);
    assert!(c.mnd_invalid_func());
    assert_eq!(c, MndInvalidFunc);

    let c = TaosCode::from(0x0375);
    assert!(c.mnd_invalid_func_bufsize());
    assert_eq!(c, MndInvalidFuncBufsize);

    let c = TaosCode::from(0x0376);
    assert!(c.mnd_invalid_tag_length());
    assert_eq!(c, MndInvalidTagLength);

    let c = TaosCode::from(0x0377);
    assert!(c.mnd_invalid_column_length());
    assert_eq!(c, MndInvalidColumnLength);

    let c = TaosCode::from(0x0380);
    assert!(c.mnd_db_not_selected());
    assert_eq!(c, MndDbNotSelected);

    let c = TaosCode::from(0x0381);
    assert!(c.mnd_db_already_exist());
    assert_eq!(c, MndDbAlreadyExist);

    let c = TaosCode::from(0x0382);
    assert!(c.mnd_invalid_db_option());
    assert_eq!(c, MndInvalidDbOption);

    let c = TaosCode::from(0x0383);
    assert!(c.mnd_invalid_db());
    assert_eq!(c, MndInvalidDb);

    let c = TaosCode::from(0x0384);
    assert!(c.mnd_monitor_db_forbidden());
    assert_eq!(c, MndMonitorDbForbidden);

    let c = TaosCode::from(0x0385);
    assert!(c.mnd_too_many_databases());
    assert_eq!(c, MndTooManyDatabases);

    let c = TaosCode::from(0x0386);
    assert!(c.mnd_db_in_dropping());
    assert_eq!(c, MndDbInDropping);

    let c = TaosCode::from(0x0387);
    assert!(c.mnd_vgroup_not_ready());
    assert_eq!(c, MndVgroupNotReady);

    let c = TaosCode::from(0x0390);
    assert!(c.mnd_invalid_db_option_days());
    assert_eq!(c, MndInvalidDbOptionDays);

    let c = TaosCode::from(0x0391);
    assert!(c.mnd_invalid_db_option_keep());
    assert_eq!(c, MndInvalidDbOptionKeep);

    let c = TaosCode::from(0x0392);
    assert!(c.mnd_invalid_topic());
    assert_eq!(c, MndInvalidTopic);

    let c = TaosCode::from(0x0393);
    assert!(c.mnd_invalid_topic_option());
    assert_eq!(c, MndInvalidTopicOption);

    let c = TaosCode::from(0x0394);
    assert!(c.mnd_invalid_topic_partitons());
    assert_eq!(c, MndInvalidTopicPartitons);

    let c = TaosCode::from(0x0395);
    assert!(c.mnd_topic_already_exist());
    assert_eq!(c, MndTopicAlreadyExist);

    let c = TaosCode::from(0x0400);
    assert!(c.dnd_msg_not_processed());
    assert_eq!(c, DndMsgNotProcessed);

    let c = TaosCode::from(0x0401);
    assert!(c.dnd_out_of_memory());
    assert_eq!(c, DndOutOfMemory);

    let c = TaosCode::from(0x0402);
    assert!(c.dnd_no_write_access());
    assert_eq!(c, DndNoWriteAccess);

    let c = TaosCode::from(0x0403);
    assert!(c.dnd_invalid_msg_len());
    assert_eq!(c, DndInvalidMsgLen);

    let c = TaosCode::from(0x0404);
    assert!(c.dnd_action_in_progress());
    assert_eq!(c, DndActionInProgress);

    let c = TaosCode::from(0x0405);
    assert!(c.dnd_too_many_vnodes());
    assert_eq!(c, DndTooManyVnodes);

    let c = TaosCode::from(0x0406);
    assert!(c.dnd_exiting());
    assert_eq!(c, DndExiting);

    let c = TaosCode::from(0x0407);
    assert!(c.dnd_vnode_open_failed());
    assert_eq!(c, DndVnodeOpenFailed);

    let c = TaosCode::from(0x0500);
    assert!(c.vnd_action_in_progress());
    assert_eq!(c, VndActionInProgress);

    let c = TaosCode::from(0x0501);
    assert!(c.vnd_msg_not_processed());
    assert_eq!(c, VndMsgNotProcessed);

    let c = TaosCode::from(0x0502);
    assert!(c.vnd_action_need_reprocessed());
    assert_eq!(c, VndActionNeedReprocessed);

    let c = TaosCode::from(0x0503);
    assert!(c.vnd_invalid_vgroup_id());
    assert_eq!(c, VndInvalidVgroupId);

    let c = TaosCode::from(0x0504);
    assert!(c.vnd_init_failed());
    assert_eq!(c, VndInitFailed);

    let c = TaosCode::from(0x0505);
    assert!(c.vnd_no_diskspace());
    assert_eq!(c, VndNoDiskspace);

    let c = TaosCode::from(0x0506);
    assert!(c.vnd_no_disk_permissions());
    assert_eq!(c, VndNoDiskPermissions);

    let c = TaosCode::from(0x0507);
    assert!(c.vnd_no_such_file_or_dir());
    assert_eq!(c, VndNoSuchFileOrDir);

    let c = TaosCode::from(0x0508);
    assert!(c.vnd_out_of_memory());
    assert_eq!(c, VndOutOfMemory);

    let c = TaosCode::from(0x0509);
    assert!(c.vnd_app_error());
    assert_eq!(c, VndAppError);

    let c = TaosCode::from(0x050A);
    assert!(c.vnd_invalid_vresion_file());
    assert_eq!(c, VndInvalidVresionFile);

    let c = TaosCode::from(0x050B);
    assert!(c.vnd_is_full());
    assert_eq!(c, VndIsFull);

    let c = TaosCode::from(0x050C);
    assert!(c.vnd_is_flowctrl());
    assert_eq!(c, VndIsFlowctrl);

    let c = TaosCode::from(0x050D);
    assert!(c.vnd_is_dropping());
    assert_eq!(c, VndIsDropping);

    let c = TaosCode::from(0x050E);
    assert!(c.vnd_is_balancing());
    assert_eq!(c, VndIsBalancing);

    let c = TaosCode::from(0x0510);
    assert!(c.vnd_is_closing());
    assert_eq!(c, VndIsClosing);

    let c = TaosCode::from(0x0511);
    assert!(c.vnd_not_synced());
    assert_eq!(c, VndNotSynced);

    let c = TaosCode::from(0x0512);
    assert!(c.vnd_no_write_auth());
    assert_eq!(c, VndNoWriteAuth);

    let c = TaosCode::from(0x0513);
    assert!(c.vnd_is_syncing());
    assert_eq!(c, VndIsSyncing);

    let c = TaosCode::from(0x0514);
    assert!(c.vnd_invalid_tsdb_state());
    assert_eq!(c, VndInvalidTsdbState);

    let c = TaosCode::from(0x0515);
    assert!(c.wait_thread_too_many());
    assert_eq!(c, WaitThreadTooMany);

    let c = TaosCode::from(0x0600);
    assert!(c.tdb_invalid_table_id());
    assert_eq!(c, TdbInvalidTableId);

    let c = TaosCode::from(0x0601);
    assert!(c.tdb_invalid_table_type());
    assert_eq!(c, TdbInvalidTableType);

    let c = TaosCode::from(0x0602);
    assert!(c.tdb_ivd_tb_schema_version());
    assert_eq!(c, TdbIvdTbSchemaVersion);

    let c = TaosCode::from(0x0603);
    assert!(c.tdb_table_already_exist());
    assert_eq!(c, TdbTableAlreadyExist);

    let c = TaosCode::from(0x0604);
    assert!(c.tdb_invalid_config());
    assert_eq!(c, TdbInvalidConfig);

    let c = TaosCode::from(0x0605);
    assert!(c.tdb_init_failed());
    assert_eq!(c, TdbInitFailed);

    let c = TaosCode::from(0x0606);
    assert!(c.tdb_no_diskspace());
    assert_eq!(c, TdbNoDiskspace);

    let c = TaosCode::from(0x0607);
    assert!(c.tdb_no_disk_permissions());
    assert_eq!(c, TdbNoDiskPermissions);

    let c = TaosCode::from(0x0608);
    assert!(c.tdb_file_corrupted());
    assert_eq!(c, TdbFileCorrupted);

    let c = TaosCode::from(0x0609);
    assert!(c.tdb_out_of_memory());
    assert_eq!(c, TdbOutOfMemory);

    let c = TaosCode::from(0x060A);
    assert!(c.tdb_tag_ver_out_of_date());
    assert_eq!(c, TdbTagVerOutOfDate);

    let c = TaosCode::from(0x060B);
    assert!(c.tdb_timestamp_out_of_range());
    assert_eq!(c, TdbTimestampOutOfRange);

    let c = TaosCode::from(0x060C);
    assert!(c.tdb_submit_msg_mssed_up());
    assert_eq!(c, TdbSubmitMsgMssedUp);

    let c = TaosCode::from(0x060D);
    assert!(c.tdb_invalid_action());
    assert_eq!(c, TdbInvalidAction);

    let c = TaosCode::from(0x060E);
    assert!(c.tdb_invalid_create_tb_msg());
    assert_eq!(c, TdbInvalidCreateTbMsg);

    let c = TaosCode::from(0x060F);
    assert!(c.tdb_no_table_data_in_mem());
    assert_eq!(c, TdbNoTableDataInMem);

    let c = TaosCode::from(0x0610);
    assert!(c.tdb_file_already_exists());
    assert_eq!(c, TdbFileAlreadyExists);

    let c = TaosCode::from(0x0611);
    assert!(c.tdb_table_reconfigure());
    assert_eq!(c, TdbTableReconfigure);

    let c = TaosCode::from(0x0612);
    assert!(c.tdb_ivd_create_table_info());
    assert_eq!(c, TdbIvdCreateTableInfo);

    let c = TaosCode::from(0x0613);
    assert!(c.tdb_no_avail_disk());
    assert_eq!(c, TdbNoAvailDisk);

    let c = TaosCode::from(0x0614);
    assert!(c.tdb_messed_msg());
    assert_eq!(c, TdbMessedMsg);

    let c = TaosCode::from(0x0615);
    assert!(c.tdb_ivld_tag_val());
    assert_eq!(c, TdbIvldTagVal);

    let c = TaosCode::from(0x0616);
    assert!(c.tdb_no_cache_last_row());
    assert_eq!(c, TdbNoCacheLastRow);

    let c = TaosCode::from(0x0617);
    assert!(c.tdb_incomplete_dfileset());
    assert_eq!(c, TdbIncompleteDfileset);

    let c = TaosCode::from(0x0700);
    assert!(c.qry_invalid_qhandle());
    assert_eq!(c, QryInvalidQhandle);

    let c = TaosCode::from(0x0701);
    assert!(c.qry_invalid_msg());
    assert_eq!(c, QryInvalidMsg);

    let c = TaosCode::from(0x0702);
    assert!(c.qry_no_diskspace());
    assert_eq!(c, QryNoDiskspace);

    let c = TaosCode::from(0x0703);
    assert!(c.qry_out_of_memory());
    assert_eq!(c, QryOutOfMemory);

    let c = TaosCode::from(0x0704);
    assert!(c.qry_app_error());
    assert_eq!(c, QryAppError);

    let c = TaosCode::from(0x0705);
    assert!(c.qry_dup_join_key());
    assert_eq!(c, QryDupJoinKey);

    let c = TaosCode::from(0x0706);
    assert!(c.qry_exceed_tags_limit());
    assert_eq!(c, QryExceedTagsLimit);

    let c = TaosCode::from(0x0707);
    assert!(c.qry_not_ready());
    assert_eq!(c, QryNotReady);

    let c = TaosCode::from(0x0708);
    assert!(c.qry_has_rsp());
    assert_eq!(c, QryHasRsp);

    let c = TaosCode::from(0x0709);
    assert!(c.qry_in_exec());
    assert_eq!(c, QryInExec);

    let c = TaosCode::from(0x070A);
    assert!(c.qry_too_many_timewindow());
    assert_eq!(c, QryTooManyTimewindow);

    let c = TaosCode::from(0x070B);
    assert!(c.qry_not_enough_buffer());
    assert_eq!(c, QryNotEnoughBuffer);

    let c = TaosCode::from(0x070C);
    assert!(c.qry_inconsistan());
    assert_eq!(c, QryInconsistan);

    let c = TaosCode::from(0x070D);
    assert!(c.qry_sys_error());
    assert_eq!(c, QrySysError);

    let c = TaosCode::from(0x070E);
    assert!(c.qry_invalid_time_condition());
    assert_eq!(c, QryInvalidTimeCondition);

    let c = TaosCode::from(0x0710);
    assert!(c.qry_invalid_schema_version());
    assert_eq!(c, QryInvalidSchemaVersion);

    let c = TaosCode::from(0x0711);
    assert!(c.qry_result_too_large());
    assert_eq!(c, QryResultTooLarge);

    let c = TaosCode::from(0x0800);
    assert!(c.grant_expired());
    assert_eq!(c, GrantExpired);

    let c = TaosCode::from(0x0801);
    assert!(c.grant_dnode_limited());
    assert_eq!(c, GrantDnodeLimited);

    let c = TaosCode::from(0x0802);
    assert!(c.grant_acct_limited());
    assert_eq!(c, GrantAcctLimited);

    let c = TaosCode::from(0x0803);
    assert!(c.grant_timeseries_limited());
    assert_eq!(c, GrantTimeseriesLimited);

    let c = TaosCode::from(0x0804);
    assert!(c.grant_db_limited());
    assert_eq!(c, GrantDbLimited);

    let c = TaosCode::from(0x0805);
    assert!(c.grant_user_limited());
    assert_eq!(c, GrantUserLimited);

    let c = TaosCode::from(0x0806);
    assert!(c.grant_conn_limited());
    assert_eq!(c, GrantConnLimited);

    let c = TaosCode::from(0x0807);
    assert!(c.grant_stream_limited());
    assert_eq!(c, GrantStreamLimited);

    let c = TaosCode::from(0x0808);
    assert!(c.grant_speed_limited());
    assert_eq!(c, GrantSpeedLimited);

    let c = TaosCode::from(0x0809);
    assert!(c.grant_storage_limited());
    assert_eq!(c, GrantStorageLimited);

    let c = TaosCode::from(0x080A);
    assert!(c.grant_querytime_limited());
    assert_eq!(c, GrantQuerytimeLimited);

    let c = TaosCode::from(0x080B);
    assert!(c.grant_cpu_limited());
    assert_eq!(c, GrantCpuLimited);

    let c = TaosCode::from(0x0900);
    assert!(c.syn_invalid_config());
    assert_eq!(c, SynInvalidConfig);

    let c = TaosCode::from(0x0901);
    assert!(c.syn_not_enabled());
    assert_eq!(c, SynNotEnabled);

    let c = TaosCode::from(0x0902);
    assert!(c.syn_invalid_version());
    assert_eq!(c, SynInvalidVersion);

    let c = TaosCode::from(0x0903);
    assert!(c.syn_confirm_expired());
    assert_eq!(c, SynConfirmExpired);

    let c = TaosCode::from(0x0904);
    assert!(c.syn_too_many_fwdinfo());
    assert_eq!(c, SynTooManyFwdinfo);

    let c = TaosCode::from(0x0905);
    assert!(c.syn_mismatched_protocol());
    assert_eq!(c, SynMismatchedProtocol);

    let c = TaosCode::from(0x0906);
    assert!(c.syn_mismatched_clusterid());
    assert_eq!(c, SynMismatchedClusterid);

    let c = TaosCode::from(0x0907);
    assert!(c.syn_mismatched_signature());
    assert_eq!(c, SynMismatchedSignature);

    let c = TaosCode::from(0x0908);
    assert!(c.syn_invalid_checksum());
    assert_eq!(c, SynInvalidChecksum);

    let c = TaosCode::from(0x0909);
    assert!(c.syn_invalid_msglen());
    assert_eq!(c, SynInvalidMsglen);

    let c = TaosCode::from(0x090A);
    assert!(c.syn_invalid_msgtype());
    assert_eq!(c, SynInvalidMsgtype);

    let c = TaosCode::from(0x1000);
    assert!(c.wal_app_error());
    assert_eq!(c, WalAppError);

    let c = TaosCode::from(0x1001);
    assert!(c.wal_file_corrupted());
    assert_eq!(c, WalFileCorrupted);

    let c = TaosCode::from(0x1002);
    assert!(c.wal_size_limit());
    assert_eq!(c, WalSizeLimit);

    let c = TaosCode::from(0x1100);
    assert!(c.http_server_offline());
    assert_eq!(c, HttpServerOffline);

    let c = TaosCode::from(0x1101);
    assert!(c.http_unsupport_url());
    assert_eq!(c, HttpUnsupportUrl);

    let c = TaosCode::from(0x1102);
    assert!(c.http_invalid_url());
    assert_eq!(c, HttpInvalidUrl);

    let c = TaosCode::from(0x1103);
    assert!(c.http_no_enough_memory());
    assert_eq!(c, HttpNoEnoughMemory);

    let c = TaosCode::from(0x1104);
    assert!(c.http_requset_too_big());
    assert_eq!(c, HttpRequsetTooBig);

    let c = TaosCode::from(0x1105);
    assert!(c.http_no_auth_info());
    assert_eq!(c, HttpNoAuthInfo);

    let c = TaosCode::from(0x1106);
    assert!(c.http_no_msg_input());
    assert_eq!(c, HttpNoMsgInput);

    let c = TaosCode::from(0x1107);
    assert!(c.http_no_sql_input());
    assert_eq!(c, HttpNoSqlInput);

    let c = TaosCode::from(0x1108);
    assert!(c.http_no_exec_usedb());
    assert_eq!(c, HttpNoExecUsedb);

    let c = TaosCode::from(0x1109);
    assert!(c.http_session_full());
    assert_eq!(c, HttpSessionFull);

    let c = TaosCode::from(0x110A);
    assert!(c.http_gen_taosd_token_err());
    assert_eq!(c, HttpGenTaosdTokenErr);

    let c = TaosCode::from(0x110B);
    assert!(c.http_invalid_multi_request());
    assert_eq!(c, HttpInvalidMultiRequest);

    let c = TaosCode::from(0x110C);
    assert!(c.http_create_gzip_failed());
    assert_eq!(c, HttpCreateGzipFailed);

    let c = TaosCode::from(0x110D);
    assert!(c.http_finish_gzip_failed());
    assert_eq!(c, HttpFinishGzipFailed);

    let c = TaosCode::from(0x110E);
    assert!(c.http_login_failed());
    assert_eq!(c, HttpLoginFailed);

    let c = TaosCode::from(0x1120);
    assert!(c.http_invalid_version());
    assert_eq!(c, HttpInvalidVersion);

    let c = TaosCode::from(0x1121);
    assert!(c.http_invalid_content_length());
    assert_eq!(c, HttpInvalidContentLength);

    let c = TaosCode::from(0x1122);
    assert!(c.http_invalid_auth_type());
    assert_eq!(c, HttpInvalidAuthType);

    let c = TaosCode::from(0x1123);
    assert!(c.http_invalid_auth_format());
    assert_eq!(c, HttpInvalidAuthFormat);

    let c = TaosCode::from(0x1124);
    assert!(c.http_invalid_basic_auth());
    assert_eq!(c, HttpInvalidBasicAuth);

    let c = TaosCode::from(0x1125);
    assert!(c.http_invalid_taosd_auth());
    assert_eq!(c, HttpInvalidTaosdAuth);

    let c = TaosCode::from(0x1126);
    assert!(c.http_parse_method_failed());
    assert_eq!(c, HttpParseMethodFailed);

    let c = TaosCode::from(0x1127);
    assert!(c.http_parse_target_failed());
    assert_eq!(c, HttpParseTargetFailed);

    let c = TaosCode::from(0x1128);
    assert!(c.http_parse_version_failed());
    assert_eq!(c, HttpParseVersionFailed);

    let c = TaosCode::from(0x1129);
    assert!(c.http_parse_sp_failed());
    assert_eq!(c, HttpParseSpFailed);

    let c = TaosCode::from(0x112A);
    assert!(c.http_parse_status_failed());
    assert_eq!(c, HttpParseStatusFailed);

    let c = TaosCode::from(0x112B);
    assert!(c.http_parse_phrase_failed());
    assert_eq!(c, HttpParsePhraseFailed);

    let c = TaosCode::from(0x112C);
    assert!(c.http_parse_crlf_failed());
    assert_eq!(c, HttpParseCrlfFailed);

    let c = TaosCode::from(0x112D);
    assert!(c.http_parse_header_failed());
    assert_eq!(c, HttpParseHeaderFailed);

    let c = TaosCode::from(0x112E);
    assert!(c.http_parse_header_key_failed());
    assert_eq!(c, HttpParseHeaderKeyFailed);

    let c = TaosCode::from(0x112F);
    assert!(c.http_parse_header_val_failed());
    assert_eq!(c, HttpParseHeaderValFailed);

    let c = TaosCode::from(0x1130);
    assert!(c.http_parse_chunk_size_failed());
    assert_eq!(c, HttpParseChunkSizeFailed);

    let c = TaosCode::from(0x1131);
    assert!(c.http_parse_chunk_failed());
    assert_eq!(c, HttpParseChunkFailed);

    let c = TaosCode::from(0x1132);
    assert!(c.http_parse_end_failed());
    assert_eq!(c, HttpParseEndFailed);

    let c = TaosCode::from(0x1134);
    assert!(c.http_parse_invalid_state());
    assert_eq!(c, HttpParseInvalidState);

    let c = TaosCode::from(0x1135);
    assert!(c.http_parse_error_state());
    assert_eq!(c, HttpParseErrorState);

    let c = TaosCode::from(0x1150);
    assert!(c.http_gc_query_null());
    assert_eq!(c, HttpGcQueryNull);

    let c = TaosCode::from(0x1151);
    assert!(c.http_gc_query_size());
    assert_eq!(c, HttpGcQuerySize);

    let c = TaosCode::from(0x1152);
    assert!(c.http_gc_req_parse_error());
    assert_eq!(c, HttpGcReqParseError);

    let c = TaosCode::from(0x1160);
    assert!(c.http_tg_db_not_input());
    assert_eq!(c, HttpTgDbNotInput);

    let c = TaosCode::from(0x1161);
    assert!(c.http_tg_db_too_long());
    assert_eq!(c, HttpTgDbTooLong);

    let c = TaosCode::from(0x1162);
    assert!(c.http_tg_invalid_json());
    assert_eq!(c, HttpTgInvalidJson);

    let c = TaosCode::from(0x1163);
    assert!(c.http_tg_metrics_null());
    assert_eq!(c, HttpTgMetricsNull);

    let c = TaosCode::from(0x1164);
    assert!(c.http_tg_metrics_size());
    assert_eq!(c, HttpTgMetricsSize);

    let c = TaosCode::from(0x1165);
    assert!(c.http_tg_metric_null());
    assert_eq!(c, HttpTgMetricNull);

    let c = TaosCode::from(0x1166);
    assert!(c.http_tg_metric_type());
    assert_eq!(c, HttpTgMetricType);

    let c = TaosCode::from(0x1167);
    assert!(c.http_tg_metric_name_null());
    assert_eq!(c, HttpTgMetricNameNull);

    let c = TaosCode::from(0x1168);
    assert!(c.http_tg_metric_name_long());
    assert_eq!(c, HttpTgMetricNameLong);

    let c = TaosCode::from(0x1169);
    assert!(c.http_tg_timestamp_null());
    assert_eq!(c, HttpTgTimestampNull);

    let c = TaosCode::from(0x116A);
    assert!(c.http_tg_timestamp_type());
    assert_eq!(c, HttpTgTimestampType);

    let c = TaosCode::from(0x116B);
    assert!(c.http_tg_timestamp_val_null());
    assert_eq!(c, HttpTgTimestampValNull);

    let c = TaosCode::from(0x116C);
    assert!(c.http_tg_tags_null());
    assert_eq!(c, HttpTgTagsNull);

    let c = TaosCode::from(0x116D);
    assert!(c.http_tg_tags_size_0());
    assert_eq!(c, HttpTgTagsSize0);

    let c = TaosCode::from(0x116E);
    assert!(c.http_tg_tags_size_long());
    assert_eq!(c, HttpTgTagsSizeLong);

    let c = TaosCode::from(0x116F);
    assert!(c.http_tg_tag_null());
    assert_eq!(c, HttpTgTagNull);

    let c = TaosCode::from(0x1170);
    assert!(c.http_tg_tag_name_null());
    assert_eq!(c, HttpTgTagNameNull);

    let c = TaosCode::from(0x1171);
    assert!(c.http_tg_tag_name_size());
    assert_eq!(c, HttpTgTagNameSize);

    let c = TaosCode::from(0x1172);
    assert!(c.http_tg_tag_value_type());
    assert_eq!(c, HttpTgTagValueType);

    let c = TaosCode::from(0x1173);
    assert!(c.http_tg_tag_value_null());
    assert_eq!(c, HttpTgTagValueNull);

    let c = TaosCode::from(0x1174);
    assert!(c.http_tg_table_null());
    assert_eq!(c, HttpTgTableNull);

    let c = TaosCode::from(0x1175);
    assert!(c.http_tg_table_size());
    assert_eq!(c, HttpTgTableSize);

    let c = TaosCode::from(0x1176);
    assert!(c.http_tg_fields_null());
    assert_eq!(c, HttpTgFieldsNull);

    let c = TaosCode::from(0x1177);
    assert!(c.http_tg_fields_size_0());
    assert_eq!(c, HttpTgFieldsSize0);

    let c = TaosCode::from(0x1178);
    assert!(c.http_tg_fields_size_long());
    assert_eq!(c, HttpTgFieldsSizeLong);

    let c = TaosCode::from(0x1179);
    assert!(c.http_tg_field_null());
    assert_eq!(c, HttpTgFieldNull);

    let c = TaosCode::from(0x117A);
    assert!(c.http_tg_field_name_null());
    assert_eq!(c, HttpTgFieldNameNull);

    let c = TaosCode::from(0x117B);
    assert!(c.http_tg_field_name_size());
    assert_eq!(c, HttpTgFieldNameSize);

    let c = TaosCode::from(0x117C);
    assert!(c.http_tg_field_value_type());
    assert_eq!(c, HttpTgFieldValueType);

    let c = TaosCode::from(0x117D);
    assert!(c.http_tg_field_value_null());
    assert_eq!(c, HttpTgFieldValueNull);

    let c = TaosCode::from(0x117E);
    assert!(c.http_tg_host_not_string());
    assert_eq!(c, HttpTgHostNotString);

    let c = TaosCode::from(0x117F);
    assert!(c.http_tg_stable_not_exist());
    assert_eq!(c, HttpTgStableNotExist);

    let c = TaosCode::from(0x1190);
    assert!(c.http_op_db_not_input());
    assert_eq!(c, HttpOpDbNotInput);

    let c = TaosCode::from(0x1191);
    assert!(c.http_op_db_too_long());
    assert_eq!(c, HttpOpDbTooLong);

    let c = TaosCode::from(0x1192);
    assert!(c.http_op_invalid_json());
    assert_eq!(c, HttpOpInvalidJson);

    let c = TaosCode::from(0x1193);
    assert!(c.http_op_metrics_null());
    assert_eq!(c, HttpOpMetricsNull);

    let c = TaosCode::from(0x1194);
    assert!(c.http_op_metrics_size());
    assert_eq!(c, HttpOpMetricsSize);

    let c = TaosCode::from(0x1195);
    assert!(c.http_op_metric_null());
    assert_eq!(c, HttpOpMetricNull);

    let c = TaosCode::from(0x1196);
    assert!(c.http_op_metric_type());
    assert_eq!(c, HttpOpMetricType);

    let c = TaosCode::from(0x1197);
    assert!(c.http_op_metric_name_null());
    assert_eq!(c, HttpOpMetricNameNull);

    let c = TaosCode::from(0x1198);
    assert!(c.http_op_metric_name_long());
    assert_eq!(c, HttpOpMetricNameLong);

    let c = TaosCode::from(0x1199);
    assert!(c.http_op_timestamp_null());
    assert_eq!(c, HttpOpTimestampNull);

    let c = TaosCode::from(0x119A);
    assert!(c.http_op_timestamp_type());
    assert_eq!(c, HttpOpTimestampType);

    let c = TaosCode::from(0x119B);
    assert!(c.http_op_timestamp_val_null());
    assert_eq!(c, HttpOpTimestampValNull);

    let c = TaosCode::from(0x119C);
    assert!(c.http_op_tags_null());
    assert_eq!(c, HttpOpTagsNull);

    let c = TaosCode::from(0x119D);
    assert!(c.http_op_tags_size_0());
    assert_eq!(c, HttpOpTagsSize0);

    let c = TaosCode::from(0x119E);
    assert!(c.http_op_tags_size_long());
    assert_eq!(c, HttpOpTagsSizeLong);

    let c = TaosCode::from(0x119F);
    assert!(c.http_op_tag_null());
    assert_eq!(c, HttpOpTagNull);

    let c = TaosCode::from(0x11A0);
    assert!(c.http_op_tag_name_null());
    assert_eq!(c, HttpOpTagNameNull);

    let c = TaosCode::from(0x11A1);
    assert!(c.http_op_tag_name_size());
    assert_eq!(c, HttpOpTagNameSize);

    let c = TaosCode::from(0x11A2);
    assert!(c.http_op_tag_value_type());
    assert_eq!(c, HttpOpTagValueType);

    let c = TaosCode::from(0x11A3);
    assert!(c.http_op_tag_value_null());
    assert_eq!(c, HttpOpTagValueNull);

    let c = TaosCode::from(0x11A4);
    assert!(c.http_op_tag_value_too_long());
    assert_eq!(c, HttpOpTagValueTooLong);

    let c = TaosCode::from(0x11A5);
    assert!(c.http_op_value_null());
    assert_eq!(c, HttpOpValueNull);

    let c = TaosCode::from(0x11A6);
    assert!(c.http_op_value_type());
    assert_eq!(c, HttpOpValueType);

    let c = TaosCode::from(0x1F00);
    assert!(c.http_request_json_error());
    assert_eq!(c, HttpRequestJsonError);

    let c = TaosCode::from(0x2100);
    assert!(c.odbc_oom());
    assert_eq!(c, OdbcOom);

    let c = TaosCode::from(0x2101);
    assert!(c.odbc_conv_char_not_num());
    assert_eq!(c, OdbcConvCharNotNum);

    let c = TaosCode::from(0x2102);
    assert!(c.odbc_conv_undef());
    assert_eq!(c, OdbcConvUndef);

    let c = TaosCode::from(0x2103);
    assert!(c.odbc_conv_trunc_frac());
    assert_eq!(c, OdbcConvTruncFrac);

    let c = TaosCode::from(0x2104);
    assert!(c.odbc_conv_trunc());
    assert_eq!(c, OdbcConvTrunc);

    let c = TaosCode::from(0x2105);
    assert!(c.odbc_conv_not_support());
    assert_eq!(c, OdbcConvNotSupport);

    let c = TaosCode::from(0x2106);
    assert!(c.odbc_conv_oor());
    assert_eq!(c, OdbcConvOor);

    let c = TaosCode::from(0x2107);
    assert!(c.odbc_out_of_range());
    assert_eq!(c, OdbcOutOfRange);

    let c = TaosCode::from(0x2108);
    assert!(c.odbc_not_support());
    assert_eq!(c, OdbcNotSupport);

    let c = TaosCode::from(0x2109);
    assert!(c.odbc_invalid_handle());
    assert_eq!(c, OdbcInvalidHandle);

    let c = TaosCode::from(0x210a);
    assert!(c.odbc_no_result());
    assert_eq!(c, OdbcNoResult);

    let c = TaosCode::from(0x210b);
    assert!(c.odbc_no_fields());
    assert_eq!(c, OdbcNoFields);

    let c = TaosCode::from(0x210c);
    assert!(c.odbc_invalid_cursor());
    assert_eq!(c, OdbcInvalidCursor);

    let c = TaosCode::from(0x210d);
    assert!(c.odbc_statement_not_ready());
    assert_eq!(c, OdbcStatementNotReady);

    let c = TaosCode::from(0x210e);
    assert!(c.odbc_connection_busy());
    assert_eq!(c, OdbcConnectionBusy);

    let c = TaosCode::from(0x210f);
    assert!(c.odbc_bad_connstr());
    assert_eq!(c, OdbcBadConnstr);

    let c = TaosCode::from(0x2110);
    assert!(c.odbc_bad_arg());
    assert_eq!(c, OdbcBadArg);

    let c = TaosCode::from(0x2111);
    assert!(c.odbc_conv_not_valid_ts());
    assert_eq!(c, OdbcConvNotValidTs);

    let c = TaosCode::from(0x2112);
    assert!(c.odbc_conv_src_too_large());
    assert_eq!(c, OdbcConvSrcTooLarge);

    let c = TaosCode::from(0x2113);
    assert!(c.odbc_conv_src_bad_seq());
    assert_eq!(c, OdbcConvSrcBadSeq);

    let c = TaosCode::from(0x2114);
    assert!(c.odbc_conv_src_incomplete());
    assert_eq!(c, OdbcConvSrcIncomplete);

    let c = TaosCode::from(0x2115);
    assert!(c.odbc_conv_src_general());
    assert_eq!(c, OdbcConvSrcGeneral);

    let c = TaosCode::from(0x2200);
    assert!(c.fs_out_of_memory());
    assert_eq!(c, FsOutOfMemory);

    let c = TaosCode::from(0x2201);
    assert!(c.fs_invld_cfg());
    assert_eq!(c, FsInvldCfg);

    let c = TaosCode::from(0x2202);
    assert!(c.fs_too_many_mount());
    assert_eq!(c, FsTooManyMount);

    let c = TaosCode::from(0x2203);
    assert!(c.fs_dup_primary());
    assert_eq!(c, FsDupPrimary);

    let c = TaosCode::from(0x2204);
    assert!(c.fs_no_primary_disk());
    assert_eq!(c, FsNoPrimaryDisk);

    let c = TaosCode::from(0x2205);
    assert!(c.fs_no_mount_at_tier());
    assert_eq!(c, FsNoMountAtTier);

    let c = TaosCode::from(0x2206);
    assert!(c.fs_file_already_exists());
    assert_eq!(c, FsFileAlreadyExists);

    let c = TaosCode::from(0x2207);
    assert!(c.fs_invld_level());
    assert_eq!(c, FsInvldLevel);

    let c = TaosCode::from(0x2208);
    assert!(c.fs_no_valid_disk());
    assert_eq!(c, FsNoValidDisk);

    let c = TaosCode::from(0x2300);
    assert!(c.mon_connection_invalid());
    assert_eq!(c, MonConnectionInvalid);
}
