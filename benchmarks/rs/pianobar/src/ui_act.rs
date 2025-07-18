use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type gcry_cipher_handle;
    pub type ao_device;
    pub type AVOptionRanges;
    pub type AVOption;
    pub type AVBuffer;
    pub type AVDictionary;
    pub type AVCodecInternal;
    pub type MpegEncContext;
    pub type AVCodecHWConfigInternal;
    pub type AVCodecDefault;
    pub type AVFormatInternal;
    pub type AVStreamInternal;
    pub type AVDeviceCapabilitiesQuery;
    pub type AVDeviceInfoList;
    pub type AVCodecTag;
    pub type AVFilterCommand;
    pub type AVFilterInternal;
    pub type AVFilterChannelLayouts;
    pub type AVFilterFormats;
    pub type AVFilterPad;
    pub type AVFilterGraphInternal;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn PianoListGet(l: *mut PianoListHead_t, n: size_t) -> *mut libc::c_void;
    fn PianoDestroyPlaylist(_: *mut PianoSong_t);
    fn PianoDestroyStationInfo(_: *mut PianoStationInfo_t);
    fn PianoDestroyStationMode(_: *mut PianoStationMode_t);
    fn PianoFindStationById(
        _: *mut PianoStation_t,
        _: *const libc::c_char,
    ) -> *mut PianoStation_t;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn BarPlayerSetVolume(player: *mut player_t);
    fn BarReadline(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: *mut BarReadlineFds_t,
        _: BarReadlineFlags_t,
        _: libc::c_int,
    ) -> size_t;
    fn BarReadlineStr(
        _: *mut libc::c_char,
        _: size_t,
        _: *mut BarReadlineFds_t,
        _: BarReadlineFlags_t,
    ) -> size_t;
    fn BarReadlineInt(_: *mut libc::c_int, _: *mut BarReadlineFds_t) -> size_t;
    fn BarReadlineYesNo(_: bool, _: *mut BarReadlineFds_t) -> bool;
    fn BarUiMsg(_: *const BarSettings_t, _: BarUiMsg_t, _: *const libc::c_char, _: ...);
    fn BarUiSelectStation(
        _: *mut BarApp_t,
        _: *mut PianoStation_t,
        _: *const libc::c_char,
        _: BarUiSelectStationCallback_t,
        _: bool,
    ) -> *mut PianoStation_t;
    fn BarUiSelectSong(
        app: *const BarApp_t,
        startSong: *mut PianoSong_t,
        input: *mut BarReadlineFds_t,
    ) -> *mut PianoSong_t;
    fn BarUiSelectArtist(_: *mut BarApp_t, _: *mut PianoArtist_t) -> *mut PianoArtist_t;
    fn BarUiSelectMusicId(
        _: *mut BarApp_t,
        _: *mut PianoStation_t,
        _: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn BarUiPrintStation(_: *const BarSettings_t, _: *mut PianoStation_t);
    fn BarUiPrintSong(
        _: *const BarSettings_t,
        _: *const PianoSong_t,
        _: *const PianoStation_t,
    );
    fn BarUiListSongs(
        app: *const BarApp_t,
        song: *const PianoSong_t,
        filter: *const libc::c_char,
    ) -> size_t;
    fn BarUiStartEventCmd(
        _: *const BarSettings_t,
        _: *const libc::c_char,
        _: *const PianoStation_t,
        _: *const PianoSong_t,
        _: *mut player_t,
        _: *mut PianoStation_t,
        _: PianoReturn_t,
        _: CURLcode,
    );
    fn BarUiPianoCall(
        _: *mut BarApp_t,
        _: PianoRequestType_t,
        _: *mut libc::c_void,
        _: *mut PianoReturn_t,
        _: *mut CURLcode,
    ) -> bool;
    fn BarUiDispatch(
        _: *mut BarApp_t,
        _: libc::c_char,
        _: *mut PianoStation_t,
        _: *mut PianoSong_t,
        _: bool,
        _: BarUiDispatchContext_t,
    ) -> BarKeyShortcutId_t;
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __sig_atomic_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type int8_t = __int8_t;
pub type int64_t = __int64_t;
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
pub type gcry_cipher_hd_t = *mut gcry_cipher_handle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoListHead {
    pub next: *mut PianoListHead,
}
pub type PianoListHead_t = PianoListHead;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoUserInfo {
    pub listenerId: *mut libc::c_char,
    pub authToken: *mut libc::c_char,
}
pub type PianoUserInfo_t = PianoUserInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoStation {
    pub head: PianoListHead_t,
    pub isCreator: libc::c_char,
    pub isQuickMix: libc::c_char,
    pub useQuickMix: libc::c_char,
    pub name: *mut libc::c_char,
    pub id: *mut libc::c_char,
    pub seedId: *mut libc::c_char,
}
pub type PianoStation_t = PianoStation;
pub type PianoSongRating_t = libc::c_uint;
pub const PIANO_RATE_TIRED: PianoSongRating_t = 3;
pub const PIANO_RATE_BAN: PianoSongRating_t = 2;
pub const PIANO_RATE_LOVE: PianoSongRating_t = 1;
pub const PIANO_RATE_NONE: PianoSongRating_t = 0;
pub type PianoAudioFormat_t = libc::c_uint;
pub const PIANO_AF_MP3: PianoAudioFormat_t = 2;
pub const PIANO_AF_AACPLUS: PianoAudioFormat_t = 1;
pub const PIANO_AF_UNKNOWN: PianoAudioFormat_t = 0;
pub type PianoAudioQuality_t = libc::c_uint;
pub const PIANO_AQ_HIGH: PianoAudioQuality_t = 3;
pub const PIANO_AQ_MEDIUM: PianoAudioQuality_t = 2;
pub const PIANO_AQ_LOW: PianoAudioQuality_t = 1;
pub const PIANO_AQ_UNKNOWN: PianoAudioQuality_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoSong {
    pub head: PianoListHead_t,
    pub artist: *mut libc::c_char,
    pub stationId: *mut libc::c_char,
    pub album: *mut libc::c_char,
    pub audioUrl: *mut libc::c_char,
    pub coverArt: *mut libc::c_char,
    pub musicId: *mut libc::c_char,
    pub title: *mut libc::c_char,
    pub seedId: *mut libc::c_char,
    pub feedbackId: *mut libc::c_char,
    pub detailUrl: *mut libc::c_char,
    pub trackToken: *mut libc::c_char,
    pub fileGain: libc::c_float,
    pub length: libc::c_uint,
    pub rating: PianoSongRating_t,
    pub audioFormat: PianoAudioFormat_t,
}
pub type PianoSong_t = PianoSong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoArtist {
    pub head: PianoListHead_t,
    pub name: *mut libc::c_char,
    pub musicId: *mut libc::c_char,
    pub seedId: *mut libc::c_char,
    pub score: libc::c_int,
}
pub type PianoArtist_t = PianoArtist;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoGenre {
    pub head: PianoListHead_t,
    pub name: *mut libc::c_char,
    pub musicId: *mut libc::c_char,
}
pub type PianoGenre_t = PianoGenre;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoGenreCategory {
    pub head: PianoListHead_t,
    pub name: *mut libc::c_char,
    pub genres: *mut PianoGenre_t,
}
pub type PianoGenreCategory_t = PianoGenreCategory;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoPartner {
    pub in_0: gcry_cipher_hd_t,
    pub out: gcry_cipher_hd_t,
    pub authToken: *mut libc::c_char,
    pub device: *mut libc::c_char,
    pub user: *mut libc::c_char,
    pub password: *mut libc::c_char,
    pub id: libc::c_uint,
}
pub type PianoPartner_t = PianoPartner;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoHandle {
    pub user: PianoUserInfo_t,
    pub stations: *mut PianoStation_t,
    pub genreStations: *mut PianoGenreCategory_t,
    pub partner: PianoPartner_t,
    pub timeOffset: libc::c_int,
}
pub type PianoHandle_t = PianoHandle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoStationInfo_t {
    pub songSeeds: *mut PianoSong_t,
    pub artistSeeds: *mut PianoArtist_t,
    pub stationSeeds: *mut PianoStation_t,
    pub feedback: *mut PianoSong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoSettings_t {
    pub username: *mut libc::c_char,
    pub explicitContentFilter: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoStationMode_t {
    pub head: PianoListHead_t,
    pub name: *mut libc::c_char,
    pub description: *mut libc::c_char,
    pub isAlgorithmic: bool,
    pub isTakeover: bool,
    pub active: bool,
    pub id: libc::c_int,
}
pub type PianoRequestType_t = libc::c_uint;
pub const PIANO_REQUEST_SET_STATION_MODE: PianoRequestType_t = 26;
pub const PIANO_REQUEST_GET_STATION_MODES: PianoRequestType_t = 25;
pub const PIANO_REQUEST_CHANGE_SETTINGS: PianoRequestType_t = 24;
pub const PIANO_REQUEST_GET_SETTINGS: PianoRequestType_t = 23;
pub const PIANO_REQUEST_DELETE_SEED: PianoRequestType_t = 22;
pub const PIANO_REQUEST_DELETE_FEEDBACK: PianoRequestType_t = 21;
pub const PIANO_REQUEST_GET_STATION_INFO: PianoRequestType_t = 20;
pub const PIANO_REQUEST_BOOKMARK_ARTIST: PianoRequestType_t = 19;
pub const PIANO_REQUEST_BOOKMARK_SONG: PianoRequestType_t = 18;
pub const PIANO_REQUEST_EXPLAIN: PianoRequestType_t = 16;
pub const PIANO_REQUEST_TRANSFORM_STATION: PianoRequestType_t = 15;
pub const PIANO_REQUEST_GET_GENRE_STATIONS: PianoRequestType_t = 14;
pub const PIANO_REQUEST_SET_QUICKMIX: PianoRequestType_t = 13;
pub const PIANO_REQUEST_ADD_TIRED_SONG: PianoRequestType_t = 12;
pub const PIANO_REQUEST_ADD_SEED: PianoRequestType_t = 11;
pub const PIANO_REQUEST_CREATE_STATION: PianoRequestType_t = 10;
pub const PIANO_REQUEST_SEARCH: PianoRequestType_t = 9;
pub const PIANO_REQUEST_DELETE_STATION: PianoRequestType_t = 8;
pub const PIANO_REQUEST_RENAME_STATION: PianoRequestType_t = 7;
pub const PIANO_REQUEST_ADD_FEEDBACK: PianoRequestType_t = 5;
pub const PIANO_REQUEST_RATE_SONG: PianoRequestType_t = 4;
pub const PIANO_REQUEST_GET_PLAYLIST: PianoRequestType_t = 3;
pub const PIANO_REQUEST_GET_STATIONS: PianoRequestType_t = 2;
pub const PIANO_REQUEST_LOGIN: PianoRequestType_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoRequestDataRateSong_t {
    pub song: *mut PianoSong_t,
    pub rating: PianoSongRating_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoRequestDataRenameStation_t {
    pub station: *mut PianoStation_t,
    pub newName: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoRequestDataCreateStation_t {
    pub token: *mut libc::c_char,
    pub type_0: C2RustUnnamed_3,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const PIANO_MUSICTYPE_ARTIST: C2RustUnnamed_3 = 2;
pub const PIANO_MUSICTYPE_SONG: C2RustUnnamed_3 = 1;
pub const PIANO_MUSICTYPE_INVALID: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoRequestDataAddSeed_t {
    pub station: *mut PianoStation_t,
    pub musicId: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoRequestDataExplain_t {
    pub song: *mut PianoSong_t,
    pub retExplain: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoRequestDataGetStationInfo_t {
    pub station: *mut PianoStation_t,
    pub info: PianoStationInfo_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoRequestDataDeleteSeed_t {
    pub song: *mut PianoSong_t,
    pub artist: *mut PianoArtist_t,
    pub station: *mut PianoStation_t,
}
pub type PianoTristate_t = libc::c_uint;
pub const PIANO_TRUE: PianoTristate_t = 2;
pub const PIANO_FALSE: PianoTristate_t = 1;
pub const PIANO_UNDEFINED: PianoTristate_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoRequestDataChangeSettings_t {
    pub currentUsername: *mut libc::c_char,
    pub newUsername: *mut libc::c_char,
    pub currentPassword: *mut libc::c_char,
    pub newPassword: *mut libc::c_char,
    pub explicitContentFilter: PianoTristate_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoRequestDataGetStationModes_t {
    pub station: *mut PianoStation_t,
    pub retModes: *mut PianoStationMode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoRequestDataSetStationMode_t {
    pub station: *mut PianoStation_t,
    pub id: libc::c_uint,
}
pub type PianoReturn_t = libc::c_uint;
pub const PIANO_RET_P_RATE_LIMIT: PianoReturn_t = 2063;
pub const PIANO_RET_P_ZIP_CODE_INVALID: PianoReturn_t = 2048;
pub const PIANO_RET_P_USER_NOT_AUTHORIZED: PianoReturn_t = 2028;
pub const PIANO_RET_P_LISTENER_NOT_AUTHORIZED: PianoReturn_t = 2027;
pub const PIANO_RET_P_USER_ALREADY_USED_TRIAL: PianoReturn_t = 2061;
pub const PIANO_RET_P_USERNAME_ALREADY_EXISTS: PianoReturn_t = 2037;
pub const PIANO_RET_P_URL_PARAM_MISSING_USER_ID: PianoReturn_t = 1029;
pub const PIANO_RET_P_URL_PARAM_MISSING_PARTNER_ID: PianoReturn_t = 1028;
pub const PIANO_RET_P_URL_PARAM_MISSING_METHOD: PianoReturn_t = 1026;
pub const PIANO_RET_P_URL_PARAM_MISSING_AUTH_TOKEN: PianoReturn_t = 1027;
pub const PIANO_RET_P_UPGRADE_DEVICE_MODEL_INVALID: PianoReturn_t = 2039;
pub const PIANO_RET_P_STATION_DOES_NOT_EXIST: PianoReturn_t = 2030;
pub const PIANO_RET_P_SECURE_PROTOCOL_REQUIRED: PianoReturn_t = 1030;
pub const PIANO_RET_P_READ_ONLY_MODE: PianoReturn_t = 2024;
pub const PIANO_RET_P_PARTNER_NOT_AUTHORIZED: PianoReturn_t = 2034;
pub const PIANO_RET_P_PARAMETER_VALUE_INVALID: PianoReturn_t = 1034;
pub const PIANO_RET_P_PARAMETER_TYPE_MISMATCH: PianoReturn_t = 1032;
pub const PIANO_RET_P_PARAMETER_MISSING: PianoReturn_t = 1033;
pub const PIANO_RET_P_MAX_STATIONS_REACHED: PianoReturn_t = 2029;
pub const PIANO_RET_P_MAINTENANCE_MODE: PianoReturn_t = 1025;
pub const PIANO_RET_P_LICENSING_RESTRICTIONS: PianoReturn_t = 1036;
pub const PIANO_RET_P_INVALID_USERNAME: PianoReturn_t = 2035;
pub const PIANO_RET_P_INVALID_SPONSOR: PianoReturn_t = 2060;
pub const PIANO_RET_P_INVALID_PASSWORD: PianoReturn_t = 2036;
pub const PIANO_RET_P_INVALID_PARTNER_LOGIN: PianoReturn_t = 2026;
pub const PIANO_RET_P_INVALID_GENDER: PianoReturn_t = 2051;
pub const PIANO_RET_P_INVALID_COUNTRY_CODE: PianoReturn_t = 2051;
pub const PIANO_RET_P_INVALID_AUTH_TOKEN: PianoReturn_t = 2025;
pub const PIANO_RET_P_INSUFFICIENT_CONNECTIVITY: PianoReturn_t = 1037;
pub const PIANO_RET_P_EXPLICIT_PIN_MALFORMED: PianoReturn_t = 2044;
pub const PIANO_RET_P_EXPLICIT_PIN_INCORRECT: PianoReturn_t = 2042;
pub const PIANO_RET_P_DEVICE_NOT_FOUND: PianoReturn_t = 2033;
pub const PIANO_RET_P_DEVICE_MODEL_INVALID: PianoReturn_t = 2047;
pub const PIANO_RET_P_DEVICE_DISABLED: PianoReturn_t = 2058;
pub const PIANO_RET_P_DEVICE_ALREADY_ASSOCIATED_TO_ACCOUNT: PianoReturn_t = 2038;
pub const PIANO_RET_P_DAILY_TRIAL_LIMIT_REACHED: PianoReturn_t = 2059;
pub const PIANO_RET_P_COMPLIMENTARY_PERIOD_ALREADY_IN_USE: PianoReturn_t = 2031;
pub const PIANO_RET_P_CERTIFICATE_REQUIRED: PianoReturn_t = 1031;
pub const PIANO_RET_P_CALL_NOT_ALLOWED: PianoReturn_t = 2032;
pub const PIANO_RET_P_BIRTH_YEAR_TOO_YOUNG: PianoReturn_t = 2050;
pub const PIANO_RET_P_BIRTH_YEAR_INVALID: PianoReturn_t = 2049;
pub const PIANO_RET_P_API_VERSION_NOT_SUPPORTED: PianoReturn_t = 1035;
pub const PIANO_RET_P_INTERNAL: PianoReturn_t = 1024;
pub const PIANO_RET_GCRY_ERR: PianoReturn_t = 7;
pub const PIANO_RET_QUALITY_UNAVAILABLE: PianoReturn_t = 6;
pub const PIANO_RET_INVALID_LOGIN: PianoReturn_t = 5;
pub const PIANO_RET_OUT_OF_MEMORY: PianoReturn_t = 4;
pub const PIANO_RET_CONTINUE_REQUEST: PianoReturn_t = 3;
pub const PIANO_RET_INVALID_RESPONSE: PianoReturn_t = 2;
pub const PIANO_RET_OK: PianoReturn_t = 1;
pub const PIANO_RET_ERR: PianoReturn_t = 0;
pub type BarKeyShortcutId_t = libc::c_uint;
pub const BAR_KS_COUNT: BarKeyShortcutId_t = 30;
pub const BAR_KS_SETTINGS: BarKeyShortcutId_t = 29;
pub const BAR_KS_VOLRESET: BarKeyShortcutId_t = 28;
pub const BAR_KS_PAUSE: BarKeyShortcutId_t = 27;
pub const BAR_KS_PLAY: BarKeyShortcutId_t = 26;
pub const BAR_KS_CREATESTATIONFROMSONG: BarKeyShortcutId_t = 25;
pub const BAR_KS_PLAYPAUSE2: BarKeyShortcutId_t = 24;
pub const BAR_KS_MANAGESTATION: BarKeyShortcutId_t = 23;
pub const BAR_KS_VOLUP: BarKeyShortcutId_t = 22;
pub const BAR_KS_VOLDOWN: BarKeyShortcutId_t = 21;
pub const BAR_KS_BOOKMARK: BarKeyShortcutId_t = 20;
pub const BAR_KS_DEBUG: BarKeyShortcutId_t = 19;
pub const BAR_KS_SELECTQUICKMIX: BarKeyShortcutId_t = 18;
pub const BAR_KS_UPCOMING: BarKeyShortcutId_t = 17;
pub const BAR_KS_TIRED: BarKeyShortcutId_t = 16;
pub const BAR_KS_SELECTSTATION: BarKeyShortcutId_t = 15;
pub const BAR_KS_RENAMESTATION: BarKeyShortcutId_t = 14;
pub const BAR_KS_QUIT: BarKeyShortcutId_t = 13;
pub const BAR_KS_PLAYPAUSE: BarKeyShortcutId_t = 12;
pub const BAR_KS_SKIP: BarKeyShortcutId_t = 11;
pub const BAR_KS_ADDSHARED: BarKeyShortcutId_t = 10;
pub const BAR_KS_INFO: BarKeyShortcutId_t = 9;
pub const BAR_KS_HISTORY: BarKeyShortcutId_t = 8;
pub const BAR_KS_GENRESTATION: BarKeyShortcutId_t = 7;
pub const BAR_KS_EXPLAIN: BarKeyShortcutId_t = 6;
pub const BAR_KS_DELETESTATION: BarKeyShortcutId_t = 5;
pub const BAR_KS_CREATESTATION: BarKeyShortcutId_t = 4;
pub const BAR_KS_ADDMUSIC: BarKeyShortcutId_t = 3;
pub const BAR_KS_BAN: BarKeyShortcutId_t = 2;
pub const BAR_KS_LOVE: BarKeyShortcutId_t = 1;
pub const BAR_KS_HELP: BarKeyShortcutId_t = 0;
pub type BarStationSorting_t = libc::c_uint;
pub const BAR_SORT_COUNT: BarStationSorting_t = 6;
pub const BAR_SORT_QUICKMIX_10_NAME_ZA: BarStationSorting_t = 5;
pub const BAR_SORT_QUICKMIX_10_NAME_AZ: BarStationSorting_t = 4;
pub const BAR_SORT_QUICKMIX_01_NAME_ZA: BarStationSorting_t = 3;
pub const BAR_SORT_QUICKMIX_01_NAME_AZ: BarStationSorting_t = 2;
pub const BAR_SORT_NAME_ZA: BarStationSorting_t = 1;
pub const BAR_SORT_NAME_AZ: BarStationSorting_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BarMsgFormatStr_t {
    pub prefix: *mut libc::c_char,
    pub postfix: *mut libc::c_char,
}
pub type BarUiMsg_t = libc::c_uint;
pub const MSG_COUNT: BarUiMsg_t = 7;
pub const MSG_LIST: BarUiMsg_t = 6;
pub const MSG_QUESTION: BarUiMsg_t = 5;
pub const MSG_ERR: BarUiMsg_t = 4;
pub const MSG_TIME: BarUiMsg_t = 3;
pub const MSG_PLAYING: BarUiMsg_t = 2;
pub const MSG_INFO: BarUiMsg_t = 1;
pub const MSG_NONE: BarUiMsg_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BarSettings_t {
    pub autoselect: bool,
    pub history: libc::c_uint,
    pub maxRetry: libc::c_uint,
    pub timeout: libc::c_uint,
    pub bufferSecs: libc::c_uint,
    pub volume: libc::c_int,
    pub gainMul: libc::c_float,
    pub sortOrder: BarStationSorting_t,
    pub audioQuality: PianoAudioQuality_t,
    pub username: *mut libc::c_char,
    pub password: *mut libc::c_char,
    pub passwordCmd: *mut libc::c_char,
    pub controlProxy: *mut libc::c_char,
    pub proxy: *mut libc::c_char,
    pub bindTo: *mut libc::c_char,
    pub autostartStation: *mut libc::c_char,
    pub eventCmd: *mut libc::c_char,
    pub loveIcon: *mut libc::c_char,
    pub banIcon: *mut libc::c_char,
    pub tiredIcon: *mut libc::c_char,
    pub atIcon: *mut libc::c_char,
    pub npSongFormat: *mut libc::c_char,
    pub npStationFormat: *mut libc::c_char,
    pub listSongFormat: *mut libc::c_char,
    pub timeFormat: *mut libc::c_char,
    pub fifo: *mut libc::c_char,
    pub rpcHost: *mut libc::c_char,
    pub rpcTlsPort: *mut libc::c_char,
    pub partnerUser: *mut libc::c_char,
    pub partnerPassword: *mut libc::c_char,
    pub device: *mut libc::c_char,
    pub inkey: *mut libc::c_char,
    pub outkey: *mut libc::c_char,
    pub caBundle: *mut libc::c_char,
    pub audioPipe: *mut libc::c_char,
    pub keys: [libc::c_char; 30],
    pub sampleRate: libc::c_int,
    pub msgFormat: [BarMsgFormatStr_t; 7],
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type sig_atomic_t = __sig_atomic_t;
pub type AVMediaType = libc::c_int;
pub const AVMEDIA_TYPE_NB: AVMediaType = 5;
pub const AVMEDIA_TYPE_ATTACHMENT: AVMediaType = 4;
pub const AVMEDIA_TYPE_SUBTITLE: AVMediaType = 3;
pub const AVMEDIA_TYPE_DATA: AVMediaType = 2;
pub const AVMEDIA_TYPE_AUDIO: AVMediaType = 1;
pub const AVMEDIA_TYPE_VIDEO: AVMediaType = 0;
pub const AVMEDIA_TYPE_UNKNOWN: AVMediaType = -1;
pub type AVPictureType = libc::c_uint;
pub const AV_PICTURE_TYPE_BI: AVPictureType = 7;
pub const AV_PICTURE_TYPE_SP: AVPictureType = 6;
pub const AV_PICTURE_TYPE_SI: AVPictureType = 5;
pub const AV_PICTURE_TYPE_S: AVPictureType = 4;
pub const AV_PICTURE_TYPE_B: AVPictureType = 3;
pub const AV_PICTURE_TYPE_P: AVPictureType = 2;
pub const AV_PICTURE_TYPE_I: AVPictureType = 1;
pub const AV_PICTURE_TYPE_NONE: AVPictureType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: libc::c_int,
    pub den: libc::c_int,
}
pub type AVClassCategory = libc::c_uint;
pub const AV_CLASS_CATEGORY_NB: AVClassCategory = 46;
pub const AV_CLASS_CATEGORY_DEVICE_INPUT: AVClassCategory = 45;
pub const AV_CLASS_CATEGORY_DEVICE_OUTPUT: AVClassCategory = 44;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT: AVClassCategory = 43;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT: AVClassCategory = 42;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT: AVClassCategory = 41;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT: AVClassCategory = 40;
pub const AV_CLASS_CATEGORY_SWRESAMPLER: AVClassCategory = 10;
pub const AV_CLASS_CATEGORY_SWSCALER: AVClassCategory = 9;
pub const AV_CLASS_CATEGORY_BITSTREAM_FILTER: AVClassCategory = 8;
pub const AV_CLASS_CATEGORY_FILTER: AVClassCategory = 7;
pub const AV_CLASS_CATEGORY_DECODER: AVClassCategory = 6;
pub const AV_CLASS_CATEGORY_ENCODER: AVClassCategory = 5;
pub const AV_CLASS_CATEGORY_DEMUXER: AVClassCategory = 4;
pub const AV_CLASS_CATEGORY_MUXER: AVClassCategory = 3;
pub const AV_CLASS_CATEGORY_OUTPUT: AVClassCategory = 2;
pub const AV_CLASS_CATEGORY_INPUT: AVClassCategory = 1;
pub const AV_CLASS_CATEGORY_NA: AVClassCategory = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVClass {
    pub class_name: *const libc::c_char,
    pub item_name: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *const libc::c_char,
    >,
    pub option: *const AVOption,
    pub version: libc::c_int,
    pub log_level_offset_offset: libc::c_int,
    pub parent_log_context_offset: libc::c_int,
    pub child_next: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
    >,
    pub child_class_next: Option::<
        unsafe extern "C" fn(*const AVClass) -> *const AVClass,
    >,
    pub category: AVClassCategory,
    pub get_category: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> AVClassCategory,
    >,
    pub query_ranges: Option::<
        unsafe extern "C" fn(
            *mut *mut AVOptionRanges,
            *mut libc::c_void,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
}
pub type AVPixelFormat = libc::c_int;
pub const AV_PIX_FMT_NB: AVPixelFormat = 193;
pub const AV_PIX_FMT_NV42: AVPixelFormat = 192;
pub const AV_PIX_FMT_NV24: AVPixelFormat = 191;
pub const AV_PIX_FMT_YUVA444P12LE: AVPixelFormat = 190;
pub const AV_PIX_FMT_YUVA444P12BE: AVPixelFormat = 189;
pub const AV_PIX_FMT_YUVA422P12LE: AVPixelFormat = 188;
pub const AV_PIX_FMT_YUVA422P12BE: AVPixelFormat = 187;
pub const AV_PIX_FMT_GRAYF32LE: AVPixelFormat = 186;
pub const AV_PIX_FMT_GRAYF32BE: AVPixelFormat = 185;
pub const AV_PIX_FMT_GRAY14LE: AVPixelFormat = 184;
pub const AV_PIX_FMT_GRAY14BE: AVPixelFormat = 183;
pub const AV_PIX_FMT_OPENCL: AVPixelFormat = 182;
pub const AV_PIX_FMT_DRM_PRIME: AVPixelFormat = 181;
pub const AV_PIX_FMT_GBRAPF32LE: AVPixelFormat = 180;
pub const AV_PIX_FMT_GBRAPF32BE: AVPixelFormat = 179;
pub const AV_PIX_FMT_GBRPF32LE: AVPixelFormat = 178;
pub const AV_PIX_FMT_GBRPF32BE: AVPixelFormat = 177;
pub const AV_PIX_FMT_GRAY9LE: AVPixelFormat = 176;
pub const AV_PIX_FMT_GRAY9BE: AVPixelFormat = 175;
pub const AV_PIX_FMT_D3D11: AVPixelFormat = 174;
pub const AV_PIX_FMT_P016BE: AVPixelFormat = 173;
pub const AV_PIX_FMT_P016LE: AVPixelFormat = 172;
pub const AV_PIX_FMT_GRAY10LE: AVPixelFormat = 171;
pub const AV_PIX_FMT_GRAY10BE: AVPixelFormat = 170;
pub const AV_PIX_FMT_GRAY12LE: AVPixelFormat = 169;
pub const AV_PIX_FMT_GRAY12BE: AVPixelFormat = 168;
pub const AV_PIX_FMT_MEDIACODEC: AVPixelFormat = 167;
pub const AV_PIX_FMT_GBRAP10LE: AVPixelFormat = 166;
pub const AV_PIX_FMT_GBRAP10BE: AVPixelFormat = 165;
pub const AV_PIX_FMT_GBRAP12LE: AVPixelFormat = 164;
pub const AV_PIX_FMT_GBRAP12BE: AVPixelFormat = 163;
pub const AV_PIX_FMT_P010BE: AVPixelFormat = 162;
pub const AV_PIX_FMT_P010LE: AVPixelFormat = 161;
pub const AV_PIX_FMT_VIDEOTOOLBOX: AVPixelFormat = 160;
pub const AV_PIX_FMT_AYUV64BE: AVPixelFormat = 159;
pub const AV_PIX_FMT_AYUV64LE: AVPixelFormat = 158;
pub const AV_PIX_FMT_YUV440P12BE: AVPixelFormat = 157;
pub const AV_PIX_FMT_YUV440P12LE: AVPixelFormat = 156;
pub const AV_PIX_FMT_YUV440P10BE: AVPixelFormat = 155;
pub const AV_PIX_FMT_YUV440P10LE: AVPixelFormat = 154;
pub const AV_PIX_FMT_XVMC: AVPixelFormat = 153;
pub const AV_PIX_FMT_BAYER_GRBG16BE: AVPixelFormat = 152;
pub const AV_PIX_FMT_BAYER_GRBG16LE: AVPixelFormat = 151;
pub const AV_PIX_FMT_BAYER_GBRG16BE: AVPixelFormat = 150;
pub const AV_PIX_FMT_BAYER_GBRG16LE: AVPixelFormat = 149;
pub const AV_PIX_FMT_BAYER_RGGB16BE: AVPixelFormat = 148;
pub const AV_PIX_FMT_BAYER_RGGB16LE: AVPixelFormat = 147;
pub const AV_PIX_FMT_BAYER_BGGR16BE: AVPixelFormat = 146;
pub const AV_PIX_FMT_BAYER_BGGR16LE: AVPixelFormat = 145;
pub const AV_PIX_FMT_BAYER_GRBG8: AVPixelFormat = 144;
pub const AV_PIX_FMT_BAYER_GBRG8: AVPixelFormat = 143;
pub const AV_PIX_FMT_BAYER_RGGB8: AVPixelFormat = 142;
pub const AV_PIX_FMT_BAYER_BGGR8: AVPixelFormat = 141;
pub const AV_PIX_FMT_YUVJ411P: AVPixelFormat = 140;
pub const AV_PIX_FMT_GBRP14LE: AVPixelFormat = 139;
pub const AV_PIX_FMT_GBRP14BE: AVPixelFormat = 138;
pub const AV_PIX_FMT_GBRP12LE: AVPixelFormat = 137;
pub const AV_PIX_FMT_GBRP12BE: AVPixelFormat = 136;
pub const AV_PIX_FMT_YUV444P14LE: AVPixelFormat = 135;
pub const AV_PIX_FMT_YUV444P14BE: AVPixelFormat = 134;
pub const AV_PIX_FMT_YUV444P12LE: AVPixelFormat = 133;
pub const AV_PIX_FMT_YUV444P12BE: AVPixelFormat = 132;
pub const AV_PIX_FMT_YUV422P14LE: AVPixelFormat = 131;
pub const AV_PIX_FMT_YUV422P14BE: AVPixelFormat = 130;
pub const AV_PIX_FMT_YUV422P12LE: AVPixelFormat = 129;
pub const AV_PIX_FMT_YUV422P12BE: AVPixelFormat = 128;
pub const AV_PIX_FMT_YUV420P14LE: AVPixelFormat = 127;
pub const AV_PIX_FMT_YUV420P14BE: AVPixelFormat = 126;
pub const AV_PIX_FMT_YUV420P12LE: AVPixelFormat = 125;
pub const AV_PIX_FMT_YUV420P12BE: AVPixelFormat = 124;
pub const AV_PIX_FMT_BGR0: AVPixelFormat = 123;
pub const AV_PIX_FMT_0BGR: AVPixelFormat = 122;
pub const AV_PIX_FMT_RGB0: AVPixelFormat = 121;
pub const AV_PIX_FMT_0RGB: AVPixelFormat = 120;
pub const AV_PIX_FMT_CUDA: AVPixelFormat = 119;
pub const AV_PIX_FMT_D3D11VA_VLD: AVPixelFormat = 118;
pub const AV_PIX_FMT_MMAL: AVPixelFormat = 117;
pub const AV_PIX_FMT_QSV: AVPixelFormat = 116;
pub const AV_PIX_FMT_GBRAP16LE: AVPixelFormat = 115;
pub const AV_PIX_FMT_GBRAP16BE: AVPixelFormat = 114;
pub const AV_PIX_FMT_GBRAP: AVPixelFormat = 113;
pub const AV_PIX_FMT_YA16LE: AVPixelFormat = 112;
pub const AV_PIX_FMT_YA16BE: AVPixelFormat = 111;
pub const AV_PIX_FMT_YVYU422: AVPixelFormat = 110;
pub const AV_PIX_FMT_BGRA64LE: AVPixelFormat = 109;
pub const AV_PIX_FMT_BGRA64BE: AVPixelFormat = 108;
pub const AV_PIX_FMT_RGBA64LE: AVPixelFormat = 107;
pub const AV_PIX_FMT_RGBA64BE: AVPixelFormat = 106;
pub const AV_PIX_FMT_NV20BE: AVPixelFormat = 105;
pub const AV_PIX_FMT_NV20LE: AVPixelFormat = 104;
pub const AV_PIX_FMT_NV16: AVPixelFormat = 103;
pub const AV_PIX_FMT_XYZ12BE: AVPixelFormat = 102;
pub const AV_PIX_FMT_XYZ12LE: AVPixelFormat = 101;
pub const AV_PIX_FMT_VDPAU: AVPixelFormat = 100;
pub const AV_PIX_FMT_YUVA444P16LE: AVPixelFormat = 99;
pub const AV_PIX_FMT_YUVA444P16BE: AVPixelFormat = 98;
pub const AV_PIX_FMT_YUVA422P16LE: AVPixelFormat = 97;
pub const AV_PIX_FMT_YUVA422P16BE: AVPixelFormat = 96;
pub const AV_PIX_FMT_YUVA420P16LE: AVPixelFormat = 95;
pub const AV_PIX_FMT_YUVA420P16BE: AVPixelFormat = 94;
pub const AV_PIX_FMT_YUVA444P10LE: AVPixelFormat = 93;
pub const AV_PIX_FMT_YUVA444P10BE: AVPixelFormat = 92;
pub const AV_PIX_FMT_YUVA422P10LE: AVPixelFormat = 91;
pub const AV_PIX_FMT_YUVA422P10BE: AVPixelFormat = 90;
pub const AV_PIX_FMT_YUVA420P10LE: AVPixelFormat = 89;
pub const AV_PIX_FMT_YUVA420P10BE: AVPixelFormat = 88;
pub const AV_PIX_FMT_YUVA444P9LE: AVPixelFormat = 87;
pub const AV_PIX_FMT_YUVA444P9BE: AVPixelFormat = 86;
pub const AV_PIX_FMT_YUVA422P9LE: AVPixelFormat = 85;
pub const AV_PIX_FMT_YUVA422P9BE: AVPixelFormat = 84;
pub const AV_PIX_FMT_YUVA420P9LE: AVPixelFormat = 83;
pub const AV_PIX_FMT_YUVA420P9BE: AVPixelFormat = 82;
pub const AV_PIX_FMT_YUVA444P: AVPixelFormat = 81;
pub const AV_PIX_FMT_YUVA422P: AVPixelFormat = 80;
pub const AV_PIX_FMT_GBRP16LE: AVPixelFormat = 79;
pub const AV_PIX_FMT_GBRP16BE: AVPixelFormat = 78;
pub const AV_PIX_FMT_GBRP10LE: AVPixelFormat = 77;
pub const AV_PIX_FMT_GBRP10BE: AVPixelFormat = 76;
pub const AV_PIX_FMT_GBRP9LE: AVPixelFormat = 75;
pub const AV_PIX_FMT_GBRP9BE: AVPixelFormat = 74;
pub const AV_PIX_FMT_GBR24P: AVPixelFormat = 73;
pub const AV_PIX_FMT_GBRP: AVPixelFormat = 73;
pub const AV_PIX_FMT_YUV422P9LE: AVPixelFormat = 72;
pub const AV_PIX_FMT_YUV422P9BE: AVPixelFormat = 71;
pub const AV_PIX_FMT_YUV444P10LE: AVPixelFormat = 70;
pub const AV_PIX_FMT_YUV444P10BE: AVPixelFormat = 69;
pub const AV_PIX_FMT_YUV444P9LE: AVPixelFormat = 68;
pub const AV_PIX_FMT_YUV444P9BE: AVPixelFormat = 67;
pub const AV_PIX_FMT_YUV422P10LE: AVPixelFormat = 66;
pub const AV_PIX_FMT_YUV422P10BE: AVPixelFormat = 65;
pub const AV_PIX_FMT_YUV420P10LE: AVPixelFormat = 64;
pub const AV_PIX_FMT_YUV420P10BE: AVPixelFormat = 63;
pub const AV_PIX_FMT_YUV420P9LE: AVPixelFormat = 62;
pub const AV_PIX_FMT_YUV420P9BE: AVPixelFormat = 61;
pub const AV_PIX_FMT_BGR48LE: AVPixelFormat = 60;
pub const AV_PIX_FMT_BGR48BE: AVPixelFormat = 59;
pub const AV_PIX_FMT_GRAY8A: AVPixelFormat = 58;
pub const AV_PIX_FMT_Y400A: AVPixelFormat = 58;
pub const AV_PIX_FMT_YA8: AVPixelFormat = 58;
pub const AV_PIX_FMT_BGR444BE: AVPixelFormat = 57;
pub const AV_PIX_FMT_BGR444LE: AVPixelFormat = 56;
pub const AV_PIX_FMT_RGB444BE: AVPixelFormat = 55;
pub const AV_PIX_FMT_RGB444LE: AVPixelFormat = 54;
pub const AV_PIX_FMT_DXVA2_VLD: AVPixelFormat = 53;
pub const AV_PIX_FMT_YUV444P16BE: AVPixelFormat = 52;
pub const AV_PIX_FMT_YUV444P16LE: AVPixelFormat = 51;
pub const AV_PIX_FMT_YUV422P16BE: AVPixelFormat = 50;
pub const AV_PIX_FMT_YUV422P16LE: AVPixelFormat = 49;
pub const AV_PIX_FMT_YUV420P16BE: AVPixelFormat = 48;
pub const AV_PIX_FMT_YUV420P16LE: AVPixelFormat = 47;
pub const AV_PIX_FMT_VAAPI: AVPixelFormat = 46;
pub const AV_PIX_FMT_VAAPI_VLD: AVPixelFormat = 46;
pub const AV_PIX_FMT_VAAPI_IDCT: AVPixelFormat = 45;
pub const AV_PIX_FMT_VAAPI_MOCO: AVPixelFormat = 44;
pub const AV_PIX_FMT_BGR555LE: AVPixelFormat = 43;
pub const AV_PIX_FMT_BGR555BE: AVPixelFormat = 42;
pub const AV_PIX_FMT_BGR565LE: AVPixelFormat = 41;
pub const AV_PIX_FMT_BGR565BE: AVPixelFormat = 40;
pub const AV_PIX_FMT_RGB555LE: AVPixelFormat = 39;
pub const AV_PIX_FMT_RGB555BE: AVPixelFormat = 38;
pub const AV_PIX_FMT_RGB565LE: AVPixelFormat = 37;
pub const AV_PIX_FMT_RGB565BE: AVPixelFormat = 36;
pub const AV_PIX_FMT_RGB48LE: AVPixelFormat = 35;
pub const AV_PIX_FMT_RGB48BE: AVPixelFormat = 34;
pub const AV_PIX_FMT_YUVA420P: AVPixelFormat = 33;
pub const AV_PIX_FMT_YUVJ440P: AVPixelFormat = 32;
pub const AV_PIX_FMT_YUV440P: AVPixelFormat = 31;
pub const AV_PIX_FMT_GRAY16LE: AVPixelFormat = 30;
pub const AV_PIX_FMT_GRAY16BE: AVPixelFormat = 29;
pub const AV_PIX_FMT_BGRA: AVPixelFormat = 28;
pub const AV_PIX_FMT_ABGR: AVPixelFormat = 27;
pub const AV_PIX_FMT_RGBA: AVPixelFormat = 26;
pub const AV_PIX_FMT_ARGB: AVPixelFormat = 25;
pub const AV_PIX_FMT_NV21: AVPixelFormat = 24;
pub const AV_PIX_FMT_NV12: AVPixelFormat = 23;
pub const AV_PIX_FMT_RGB4_BYTE: AVPixelFormat = 22;
pub const AV_PIX_FMT_RGB4: AVPixelFormat = 21;
pub const AV_PIX_FMT_RGB8: AVPixelFormat = 20;
pub const AV_PIX_FMT_BGR4_BYTE: AVPixelFormat = 19;
pub const AV_PIX_FMT_BGR4: AVPixelFormat = 18;
pub const AV_PIX_FMT_BGR8: AVPixelFormat = 17;
pub const AV_PIX_FMT_UYYVYY411: AVPixelFormat = 16;
pub const AV_PIX_FMT_UYVY422: AVPixelFormat = 15;
pub const AV_PIX_FMT_YUVJ444P: AVPixelFormat = 14;
pub const AV_PIX_FMT_YUVJ422P: AVPixelFormat = 13;
pub const AV_PIX_FMT_YUVJ420P: AVPixelFormat = 12;
pub const AV_PIX_FMT_PAL8: AVPixelFormat = 11;
pub const AV_PIX_FMT_MONOBLACK: AVPixelFormat = 10;
pub const AV_PIX_FMT_MONOWHITE: AVPixelFormat = 9;
pub const AV_PIX_FMT_GRAY8: AVPixelFormat = 8;
pub const AV_PIX_FMT_YUV411P: AVPixelFormat = 7;
pub const AV_PIX_FMT_YUV410P: AVPixelFormat = 6;
pub const AV_PIX_FMT_YUV444P: AVPixelFormat = 5;
pub const AV_PIX_FMT_YUV422P: AVPixelFormat = 4;
pub const AV_PIX_FMT_BGR24: AVPixelFormat = 3;
pub const AV_PIX_FMT_RGB24: AVPixelFormat = 2;
pub const AV_PIX_FMT_YUYV422: AVPixelFormat = 1;
pub const AV_PIX_FMT_YUV420P: AVPixelFormat = 0;
pub const AV_PIX_FMT_NONE: AVPixelFormat = -1;
pub type AVColorPrimaries = libc::c_uint;
pub const AVCOL_PRI_NB: AVColorPrimaries = 23;
pub const AVCOL_PRI_JEDEC_P22: AVColorPrimaries = 22;
pub const AVCOL_PRI_SMPTE432: AVColorPrimaries = 12;
pub const AVCOL_PRI_SMPTE431: AVColorPrimaries = 11;
pub const AVCOL_PRI_SMPTEST428_1: AVColorPrimaries = 10;
pub const AVCOL_PRI_SMPTE428: AVColorPrimaries = 10;
pub const AVCOL_PRI_BT2020: AVColorPrimaries = 9;
pub const AVCOL_PRI_FILM: AVColorPrimaries = 8;
pub const AVCOL_PRI_SMPTE240M: AVColorPrimaries = 7;
pub const AVCOL_PRI_SMPTE170M: AVColorPrimaries = 6;
pub const AVCOL_PRI_BT470BG: AVColorPrimaries = 5;
pub const AVCOL_PRI_BT470M: AVColorPrimaries = 4;
pub const AVCOL_PRI_RESERVED: AVColorPrimaries = 3;
pub const AVCOL_PRI_UNSPECIFIED: AVColorPrimaries = 2;
pub const AVCOL_PRI_BT709: AVColorPrimaries = 1;
pub const AVCOL_PRI_RESERVED0: AVColorPrimaries = 0;
pub type AVColorTransferCharacteristic = libc::c_uint;
pub const AVCOL_TRC_NB: AVColorTransferCharacteristic = 19;
pub const AVCOL_TRC_ARIB_STD_B67: AVColorTransferCharacteristic = 18;
pub const AVCOL_TRC_SMPTEST428_1: AVColorTransferCharacteristic = 17;
pub const AVCOL_TRC_SMPTE428: AVColorTransferCharacteristic = 17;
pub const AVCOL_TRC_SMPTEST2084: AVColorTransferCharacteristic = 16;
pub const AVCOL_TRC_SMPTE2084: AVColorTransferCharacteristic = 16;
pub const AVCOL_TRC_BT2020_12: AVColorTransferCharacteristic = 15;
pub const AVCOL_TRC_BT2020_10: AVColorTransferCharacteristic = 14;
pub const AVCOL_TRC_IEC61966_2_1: AVColorTransferCharacteristic = 13;
pub const AVCOL_TRC_BT1361_ECG: AVColorTransferCharacteristic = 12;
pub const AVCOL_TRC_IEC61966_2_4: AVColorTransferCharacteristic = 11;
pub const AVCOL_TRC_LOG_SQRT: AVColorTransferCharacteristic = 10;
pub const AVCOL_TRC_LOG: AVColorTransferCharacteristic = 9;
pub const AVCOL_TRC_LINEAR: AVColorTransferCharacteristic = 8;
pub const AVCOL_TRC_SMPTE240M: AVColorTransferCharacteristic = 7;
pub const AVCOL_TRC_SMPTE170M: AVColorTransferCharacteristic = 6;
pub const AVCOL_TRC_GAMMA28: AVColorTransferCharacteristic = 5;
pub const AVCOL_TRC_GAMMA22: AVColorTransferCharacteristic = 4;
pub const AVCOL_TRC_RESERVED: AVColorTransferCharacteristic = 3;
pub const AVCOL_TRC_UNSPECIFIED: AVColorTransferCharacteristic = 2;
pub const AVCOL_TRC_BT709: AVColorTransferCharacteristic = 1;
pub const AVCOL_TRC_RESERVED0: AVColorTransferCharacteristic = 0;
pub type AVColorSpace = libc::c_uint;
pub const AVCOL_SPC_NB: AVColorSpace = 15;
pub const AVCOL_SPC_ICTCP: AVColorSpace = 14;
pub const AVCOL_SPC_CHROMA_DERIVED_CL: AVColorSpace = 13;
pub const AVCOL_SPC_CHROMA_DERIVED_NCL: AVColorSpace = 12;
pub const AVCOL_SPC_SMPTE2085: AVColorSpace = 11;
pub const AVCOL_SPC_BT2020_CL: AVColorSpace = 10;
pub const AVCOL_SPC_BT2020_NCL: AVColorSpace = 9;
pub const AVCOL_SPC_YCOCG: AVColorSpace = 8;
pub const AVCOL_SPC_YCGCO: AVColorSpace = 8;
pub const AVCOL_SPC_SMPTE240M: AVColorSpace = 7;
pub const AVCOL_SPC_SMPTE170M: AVColorSpace = 6;
pub const AVCOL_SPC_BT470BG: AVColorSpace = 5;
pub const AVCOL_SPC_FCC: AVColorSpace = 4;
pub const AVCOL_SPC_RESERVED: AVColorSpace = 3;
pub const AVCOL_SPC_UNSPECIFIED: AVColorSpace = 2;
pub const AVCOL_SPC_BT709: AVColorSpace = 1;
pub const AVCOL_SPC_RGB: AVColorSpace = 0;
pub type AVColorRange = libc::c_uint;
pub const AVCOL_RANGE_NB: AVColorRange = 3;
pub const AVCOL_RANGE_JPEG: AVColorRange = 2;
pub const AVCOL_RANGE_MPEG: AVColorRange = 1;
pub const AVCOL_RANGE_UNSPECIFIED: AVColorRange = 0;
pub type AVChromaLocation = libc::c_uint;
pub const AVCHROMA_LOC_NB: AVChromaLocation = 7;
pub const AVCHROMA_LOC_BOTTOM: AVChromaLocation = 6;
pub const AVCHROMA_LOC_BOTTOMLEFT: AVChromaLocation = 5;
pub const AVCHROMA_LOC_TOP: AVChromaLocation = 4;
pub const AVCHROMA_LOC_TOPLEFT: AVChromaLocation = 3;
pub const AVCHROMA_LOC_CENTER: AVChromaLocation = 2;
pub const AVCHROMA_LOC_LEFT: AVChromaLocation = 1;
pub const AVCHROMA_LOC_UNSPECIFIED: AVChromaLocation = 0;
pub type AVSampleFormat = libc::c_int;
pub const AV_SAMPLE_FMT_NB: AVSampleFormat = 12;
pub const AV_SAMPLE_FMT_S64P: AVSampleFormat = 11;
pub const AV_SAMPLE_FMT_S64: AVSampleFormat = 10;
pub const AV_SAMPLE_FMT_DBLP: AVSampleFormat = 9;
pub const AV_SAMPLE_FMT_FLTP: AVSampleFormat = 8;
pub const AV_SAMPLE_FMT_S32P: AVSampleFormat = 7;
pub const AV_SAMPLE_FMT_S16P: AVSampleFormat = 6;
pub const AV_SAMPLE_FMT_U8P: AVSampleFormat = 5;
pub const AV_SAMPLE_FMT_DBL: AVSampleFormat = 4;
pub const AV_SAMPLE_FMT_FLT: AVSampleFormat = 3;
pub const AV_SAMPLE_FMT_S32: AVSampleFormat = 2;
pub const AV_SAMPLE_FMT_S16: AVSampleFormat = 1;
pub const AV_SAMPLE_FMT_U8: AVSampleFormat = 0;
pub const AV_SAMPLE_FMT_NONE: AVSampleFormat = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: libc::c_int,
}
pub type AVFrameSideDataType = libc::c_uint;
pub const AV_FRAME_DATA_REGIONS_OF_INTEREST: AVFrameSideDataType = 20;
pub const AV_FRAME_DATA_DYNAMIC_HDR_PLUS: AVFrameSideDataType = 19;
pub const AV_FRAME_DATA_S12M_TIMECODE: AVFrameSideDataType = 18;
pub const AV_FRAME_DATA_QP_TABLE_DATA: AVFrameSideDataType = 17;
pub const AV_FRAME_DATA_QP_TABLE_PROPERTIES: AVFrameSideDataType = 16;
pub const AV_FRAME_DATA_ICC_PROFILE: AVFrameSideDataType = 15;
pub const AV_FRAME_DATA_CONTENT_LIGHT_LEVEL: AVFrameSideDataType = 14;
pub const AV_FRAME_DATA_SPHERICAL: AVFrameSideDataType = 13;
pub const AV_FRAME_DATA_GOP_TIMECODE: AVFrameSideDataType = 12;
pub const AV_FRAME_DATA_MASTERING_DISPLAY_METADATA: AVFrameSideDataType = 11;
pub const AV_FRAME_DATA_AUDIO_SERVICE_TYPE: AVFrameSideDataType = 10;
pub const AV_FRAME_DATA_SKIP_SAMPLES: AVFrameSideDataType = 9;
pub const AV_FRAME_DATA_MOTION_VECTORS: AVFrameSideDataType = 8;
pub const AV_FRAME_DATA_AFD: AVFrameSideDataType = 7;
pub const AV_FRAME_DATA_DISPLAYMATRIX: AVFrameSideDataType = 6;
pub const AV_FRAME_DATA_REPLAYGAIN: AVFrameSideDataType = 5;
pub const AV_FRAME_DATA_DOWNMIX_INFO: AVFrameSideDataType = 4;
pub const AV_FRAME_DATA_MATRIXENCODING: AVFrameSideDataType = 3;
pub const AV_FRAME_DATA_STEREO3D: AVFrameSideDataType = 2;
pub const AV_FRAME_DATA_A53_CC: AVFrameSideDataType = 1;
pub const AV_FRAME_DATA_PANSCAN: AVFrameSideDataType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFrameSideData {
    pub type_0: AVFrameSideDataType,
    pub data: *mut uint8_t,
    pub size: libc::c_int,
    pub metadata: *mut AVDictionary,
    pub buf: *mut AVBufferRef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFrame {
    pub data: [*mut uint8_t; 8],
    pub linesize: [libc::c_int; 8],
    pub extended_data: *mut *mut uint8_t,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub nb_samples: libc::c_int,
    pub format: libc::c_int,
    pub key_frame: libc::c_int,
    pub pict_type: AVPictureType,
    pub sample_aspect_ratio: AVRational,
    pub pts: int64_t,
    pub pkt_pts: int64_t,
    pub pkt_dts: int64_t,
    pub coded_picture_number: libc::c_int,
    pub display_picture_number: libc::c_int,
    pub quality: libc::c_int,
    pub opaque: *mut libc::c_void,
    pub error: [uint64_t; 8],
    pub repeat_pict: libc::c_int,
    pub interlaced_frame: libc::c_int,
    pub top_field_first: libc::c_int,
    pub palette_has_changed: libc::c_int,
    pub reordered_opaque: int64_t,
    pub sample_rate: libc::c_int,
    pub channel_layout: uint64_t,
    pub buf: [*mut AVBufferRef; 8],
    pub extended_buf: *mut *mut AVBufferRef,
    pub nb_extended_buf: libc::c_int,
    pub side_data: *mut *mut AVFrameSideData,
    pub nb_side_data: libc::c_int,
    pub flags: libc::c_int,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub best_effort_timestamp: int64_t,
    pub pkt_pos: int64_t,
    pub pkt_duration: int64_t,
    pub metadata: *mut AVDictionary,
    pub decode_error_flags: libc::c_int,
    pub channels: libc::c_int,
    pub pkt_size: libc::c_int,
    pub qscale_table: *mut int8_t,
    pub qstride: libc::c_int,
    pub qscale_type: libc::c_int,
    pub qp_table_buf: *mut AVBufferRef,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub opaque_ref: *mut AVBufferRef,
    pub crop_top: size_t,
    pub crop_bottom: size_t,
    pub crop_left: size_t,
    pub crop_right: size_t,
    pub private_ref: *mut AVBufferRef,
}
pub type AVCodecID = libc::c_uint;
pub const AV_CODEC_ID_WRAPPED_AVFRAME: AVCodecID = 135169;
pub const AV_CODEC_ID_FFMETADATA: AVCodecID = 135168;
pub const AV_CODEC_ID_MPEG4SYSTEMS: AVCodecID = 131073;
pub const AV_CODEC_ID_MPEG2TS: AVCodecID = 131072;
pub const AV_CODEC_ID_PROBE: AVCodecID = 102400;
pub const AV_CODEC_ID_BIN_DATA: AVCodecID = 100359;
pub const AV_CODEC_ID_TIMED_ID3: AVCodecID = 100358;
pub const AV_CODEC_ID_DVD_NAV: AVCodecID = 100357;
pub const AV_CODEC_ID_SMPTE_KLV: AVCodecID = 100356;
pub const AV_CODEC_ID_OTF: AVCodecID = 100355;
pub const AV_CODEC_ID_IDF: AVCodecID = 100354;
pub const AV_CODEC_ID_XBIN: AVCodecID = 100353;
pub const AV_CODEC_ID_BINTEXT: AVCodecID = 100352;
pub const AV_CODEC_ID_SCTE_35: AVCodecID = 98305;
pub const AV_CODEC_ID_TTF: AVCodecID = 98304;
pub const AV_CODEC_ID_FIRST_UNKNOWN: AVCodecID = 98304;
pub const AV_CODEC_ID_ARIB_CAPTION: AVCodecID = 96272;
pub const AV_CODEC_ID_TTML: AVCodecID = 96271;
pub const AV_CODEC_ID_HDMV_TEXT_SUBTITLE: AVCodecID = 96270;
pub const AV_CODEC_ID_ASS: AVCodecID = 96269;
pub const AV_CODEC_ID_PJS: AVCodecID = 96268;
pub const AV_CODEC_ID_VPLAYER: AVCodecID = 96267;
pub const AV_CODEC_ID_MPL2: AVCodecID = 96266;
pub const AV_CODEC_ID_WEBVTT: AVCodecID = 96265;
pub const AV_CODEC_ID_SUBRIP: AVCodecID = 96264;
pub const AV_CODEC_ID_SUBVIEWER: AVCodecID = 96263;
pub const AV_CODEC_ID_SUBVIEWER1: AVCodecID = 96262;
pub const AV_CODEC_ID_STL: AVCodecID = 96261;
pub const AV_CODEC_ID_REALTEXT: AVCodecID = 96260;
pub const AV_CODEC_ID_SAMI: AVCodecID = 96259;
pub const AV_CODEC_ID_JACOSUB: AVCodecID = 96258;
pub const AV_CODEC_ID_EIA_608: AVCodecID = 96257;
pub const AV_CODEC_ID_MICRODVD: AVCodecID = 96256;
pub const AV_CODEC_ID_SRT: AVCodecID = 94216;
pub const AV_CODEC_ID_DVB_TELETEXT: AVCodecID = 94215;
pub const AV_CODEC_ID_HDMV_PGS_SUBTITLE: AVCodecID = 94214;
pub const AV_CODEC_ID_MOV_TEXT: AVCodecID = 94213;
pub const AV_CODEC_ID_SSA: AVCodecID = 94212;
pub const AV_CODEC_ID_XSUB: AVCodecID = 94211;
pub const AV_CODEC_ID_TEXT: AVCodecID = 94210;
pub const AV_CODEC_ID_DVB_SUBTITLE: AVCodecID = 94209;
pub const AV_CODEC_ID_DVD_SUBTITLE: AVCodecID = 94208;
pub const AV_CODEC_ID_FIRST_SUBTITLE: AVCodecID = 94208;
pub const AV_CODEC_ID_HCOM: AVCodecID = 88085;
pub const AV_CODEC_ID_ATRAC9: AVCodecID = 88084;
pub const AV_CODEC_ID_SBC: AVCodecID = 88083;
pub const AV_CODEC_ID_APTX_HD: AVCodecID = 88082;
pub const AV_CODEC_ID_APTX: AVCodecID = 88081;
pub const AV_CODEC_ID_DOLBY_E: AVCodecID = 88080;
pub const AV_CODEC_ID_ATRAC3PAL: AVCodecID = 88079;
pub const AV_CODEC_ID_ATRAC3AL: AVCodecID = 88078;
pub const AV_CODEC_ID_DST: AVCodecID = 88077;
pub const AV_CODEC_ID_XMA2: AVCodecID = 88076;
pub const AV_CODEC_ID_XMA1: AVCodecID = 88075;
pub const AV_CODEC_ID_INTERPLAY_ACM: AVCodecID = 88074;
pub const AV_CODEC_ID_4GV: AVCodecID = 88073;
pub const AV_CODEC_ID_DSD_MSBF_PLANAR: AVCodecID = 88072;
pub const AV_CODEC_ID_DSD_LSBF_PLANAR: AVCodecID = 88071;
pub const AV_CODEC_ID_DSD_MSBF: AVCodecID = 88070;
pub const AV_CODEC_ID_DSD_LSBF: AVCodecID = 88069;
pub const AV_CODEC_ID_SMV: AVCodecID = 88068;
pub const AV_CODEC_ID_EVRC: AVCodecID = 88067;
pub const AV_CODEC_ID_SONIC_LS: AVCodecID = 88066;
pub const AV_CODEC_ID_SONIC: AVCodecID = 88065;
pub const AV_CODEC_ID_FFWAVESYNTH: AVCodecID = 88064;
pub const AV_CODEC_ID_CODEC2: AVCodecID = 86083;
pub const AV_CODEC_ID_DSS_SP: AVCodecID = 86082;
pub const AV_CODEC_ID_ON2AVC: AVCodecID = 86081;
pub const AV_CODEC_ID_PAF_AUDIO: AVCodecID = 86080;
pub const AV_CODEC_ID_METASOUND: AVCodecID = 86079;
pub const AV_CODEC_ID_TAK: AVCodecID = 86078;
pub const AV_CODEC_ID_COMFORT_NOISE: AVCodecID = 86077;
pub const AV_CODEC_ID_OPUS: AVCodecID = 86076;
pub const AV_CODEC_ID_ILBC: AVCodecID = 86075;
pub const AV_CODEC_ID_IAC: AVCodecID = 86074;
pub const AV_CODEC_ID_RALF: AVCodecID = 86073;
pub const AV_CODEC_ID_BMV_AUDIO: AVCodecID = 86072;
pub const AV_CODEC_ID_8SVX_FIB: AVCodecID = 86071;
pub const AV_CODEC_ID_8SVX_EXP: AVCodecID = 86070;
pub const AV_CODEC_ID_G729: AVCodecID = 86069;
pub const AV_CODEC_ID_G723_1: AVCodecID = 86068;
pub const AV_CODEC_ID_CELT: AVCodecID = 86067;
pub const AV_CODEC_ID_QDMC: AVCodecID = 86066;
pub const AV_CODEC_ID_AAC_LATM: AVCodecID = 86065;
pub const AV_CODEC_ID_BINKAUDIO_DCT: AVCodecID = 86064;
pub const AV_CODEC_ID_BINKAUDIO_RDFT: AVCodecID = 86063;
pub const AV_CODEC_ID_ATRAC1: AVCodecID = 86062;
pub const AV_CODEC_ID_MP4ALS: AVCodecID = 86061;
pub const AV_CODEC_ID_TRUEHD: AVCodecID = 86060;
pub const AV_CODEC_ID_TWINVQ: AVCodecID = 86059;
pub const AV_CODEC_ID_MP1: AVCodecID = 86058;
pub const AV_CODEC_ID_SIPR: AVCodecID = 86057;
pub const AV_CODEC_ID_EAC3: AVCodecID = 86056;
pub const AV_CODEC_ID_ATRAC3P: AVCodecID = 86055;
pub const AV_CODEC_ID_WMALOSSLESS: AVCodecID = 86054;
pub const AV_CODEC_ID_WMAPRO: AVCodecID = 86053;
pub const AV_CODEC_ID_WMAVOICE: AVCodecID = 86052;
pub const AV_CODEC_ID_SPEEX: AVCodecID = 86051;
pub const AV_CODEC_ID_MUSEPACK8: AVCodecID = 86050;
pub const AV_CODEC_ID_NELLYMOSER: AVCodecID = 86049;
pub const AV_CODEC_ID_APE: AVCodecID = 86048;
pub const AV_CODEC_ID_ATRAC3: AVCodecID = 86047;
pub const AV_CODEC_ID_GSM_MS: AVCodecID = 86046;
pub const AV_CODEC_ID_MLP: AVCodecID = 86045;
pub const AV_CODEC_ID_MUSEPACK7: AVCodecID = 86044;
pub const AV_CODEC_ID_IMC: AVCodecID = 86043;
pub const AV_CODEC_ID_DSICINAUDIO: AVCodecID = 86042;
pub const AV_CODEC_ID_WAVPACK: AVCodecID = 86041;
pub const AV_CODEC_ID_QCELP: AVCodecID = 86040;
pub const AV_CODEC_ID_SMACKAUDIO: AVCodecID = 86039;
pub const AV_CODEC_ID_TTA: AVCodecID = 86038;
pub const AV_CODEC_ID_TRUESPEECH: AVCodecID = 86037;
pub const AV_CODEC_ID_COOK: AVCodecID = 86036;
pub const AV_CODEC_ID_QDM2: AVCodecID = 86035;
pub const AV_CODEC_ID_GSM: AVCodecID = 86034;
pub const AV_CODEC_ID_WESTWOOD_SND1: AVCodecID = 86033;
pub const AV_CODEC_ID_ALAC: AVCodecID = 86032;
pub const AV_CODEC_ID_SHORTEN: AVCodecID = 86031;
pub const AV_CODEC_ID_MP3ON4: AVCodecID = 86030;
pub const AV_CODEC_ID_MP3ADU: AVCodecID = 86029;
pub const AV_CODEC_ID_FLAC: AVCodecID = 86028;
pub const AV_CODEC_ID_VMDAUDIO: AVCodecID = 86027;
pub const AV_CODEC_ID_MACE6: AVCodecID = 86026;
pub const AV_CODEC_ID_MACE3: AVCodecID = 86025;
pub const AV_CODEC_ID_WMAV2: AVCodecID = 86024;
pub const AV_CODEC_ID_WMAV1: AVCodecID = 86023;
pub const AV_CODEC_ID_DVAUDIO: AVCodecID = 86022;
pub const AV_CODEC_ID_VORBIS: AVCodecID = 86021;
pub const AV_CODEC_ID_DTS: AVCodecID = 86020;
pub const AV_CODEC_ID_AC3: AVCodecID = 86019;
pub const AV_CODEC_ID_AAC: AVCodecID = 86018;
pub const AV_CODEC_ID_MP3: AVCodecID = 86017;
pub const AV_CODEC_ID_MP2: AVCodecID = 86016;
pub const AV_CODEC_ID_GREMLIN_DPCM: AVCodecID = 83969;
pub const AV_CODEC_ID_SDX2_DPCM: AVCodecID = 83968;
pub const AV_CODEC_ID_SOL_DPCM: AVCodecID = 81923;
pub const AV_CODEC_ID_XAN_DPCM: AVCodecID = 81922;
pub const AV_CODEC_ID_INTERPLAY_DPCM: AVCodecID = 81921;
pub const AV_CODEC_ID_ROQ_DPCM: AVCodecID = 81920;
pub const AV_CODEC_ID_RA_288: AVCodecID = 77825;
pub const AV_CODEC_ID_RA_144: AVCodecID = 77824;
pub const AV_CODEC_ID_AMR_WB: AVCodecID = 73729;
pub const AV_CODEC_ID_AMR_NB: AVCodecID = 73728;
pub const AV_CODEC_ID_ADPCM_AGM: AVCodecID = 71690;
pub const AV_CODEC_ID_ADPCM_MTAF: AVCodecID = 71689;
pub const AV_CODEC_ID_ADPCM_IMA_DAT4: AVCodecID = 71688;
pub const AV_CODEC_ID_ADPCM_AICA: AVCodecID = 71687;
pub const AV_CODEC_ID_ADPCM_PSX: AVCodecID = 71686;
pub const AV_CODEC_ID_ADPCM_THP_LE: AVCodecID = 71685;
pub const AV_CODEC_ID_ADPCM_G726LE: AVCodecID = 71684;
pub const AV_CODEC_ID_ADPCM_IMA_RAD: AVCodecID = 71683;
pub const AV_CODEC_ID_ADPCM_DTK: AVCodecID = 71682;
pub const AV_CODEC_ID_ADPCM_IMA_OKI: AVCodecID = 71681;
pub const AV_CODEC_ID_ADPCM_AFC: AVCodecID = 71680;
pub const AV_CODEC_ID_ADPCM_VIMA: AVCodecID = 69662;
pub const AV_CODEC_ID_ADPCM_IMA_APC: AVCodecID = 69661;
pub const AV_CODEC_ID_ADPCM_G722: AVCodecID = 69660;
pub const AV_CODEC_ID_ADPCM_IMA_ISS: AVCodecID = 69659;
pub const AV_CODEC_ID_ADPCM_EA_MAXIS_XA: AVCodecID = 69658;
pub const AV_CODEC_ID_ADPCM_EA_XAS: AVCodecID = 69657;
pub const AV_CODEC_ID_ADPCM_IMA_EA_EACS: AVCodecID = 69656;
pub const AV_CODEC_ID_ADPCM_IMA_EA_SEAD: AVCodecID = 69655;
pub const AV_CODEC_ID_ADPCM_EA_R2: AVCodecID = 69654;
pub const AV_CODEC_ID_ADPCM_EA_R3: AVCodecID = 69653;
pub const AV_CODEC_ID_ADPCM_EA_R1: AVCodecID = 69652;
pub const AV_CODEC_ID_ADPCM_IMA_AMV: AVCodecID = 69651;
pub const AV_CODEC_ID_ADPCM_THP: AVCodecID = 69650;
pub const AV_CODEC_ID_ADPCM_SBPRO_2: AVCodecID = 69649;
pub const AV_CODEC_ID_ADPCM_SBPRO_3: AVCodecID = 69648;
pub const AV_CODEC_ID_ADPCM_SBPRO_4: AVCodecID = 69647;
pub const AV_CODEC_ID_ADPCM_YAMAHA: AVCodecID = 69646;
pub const AV_CODEC_ID_ADPCM_SWF: AVCodecID = 69645;
pub const AV_CODEC_ID_ADPCM_CT: AVCodecID = 69644;
pub const AV_CODEC_ID_ADPCM_G726: AVCodecID = 69643;
pub const AV_CODEC_ID_ADPCM_EA: AVCodecID = 69642;
pub const AV_CODEC_ID_ADPCM_ADX: AVCodecID = 69641;
pub const AV_CODEC_ID_ADPCM_XA: AVCodecID = 69640;
pub const AV_CODEC_ID_ADPCM_4XM: AVCodecID = 69639;
pub const AV_CODEC_ID_ADPCM_MS: AVCodecID = 69638;
pub const AV_CODEC_ID_ADPCM_IMA_SMJPEG: AVCodecID = 69637;
pub const AV_CODEC_ID_ADPCM_IMA_WS: AVCodecID = 69636;
pub const AV_CODEC_ID_ADPCM_IMA_DK4: AVCodecID = 69635;
pub const AV_CODEC_ID_ADPCM_IMA_DK3: AVCodecID = 69634;
pub const AV_CODEC_ID_ADPCM_IMA_WAV: AVCodecID = 69633;
pub const AV_CODEC_ID_ADPCM_IMA_QT: AVCodecID = 69632;
pub const AV_CODEC_ID_PCM_VIDC: AVCodecID = 67588;
pub const AV_CODEC_ID_PCM_F24LE: AVCodecID = 67587;
pub const AV_CODEC_ID_PCM_F16LE: AVCodecID = 67586;
pub const AV_CODEC_ID_PCM_S64BE: AVCodecID = 67585;
pub const AV_CODEC_ID_PCM_S64LE: AVCodecID = 67584;
pub const AV_CODEC_ID_PCM_S16BE_PLANAR: AVCodecID = 65566;
pub const AV_CODEC_ID_PCM_S32LE_PLANAR: AVCodecID = 65565;
pub const AV_CODEC_ID_PCM_S24LE_PLANAR: AVCodecID = 65564;
pub const AV_CODEC_ID_PCM_S8_PLANAR: AVCodecID = 65563;
pub const AV_CODEC_ID_S302M: AVCodecID = 65562;
pub const AV_CODEC_ID_PCM_LXF: AVCodecID = 65561;
pub const AV_CODEC_ID_PCM_BLURAY: AVCodecID = 65560;
pub const AV_CODEC_ID_PCM_F64LE: AVCodecID = 65559;
pub const AV_CODEC_ID_PCM_F64BE: AVCodecID = 65558;
pub const AV_CODEC_ID_PCM_F32LE: AVCodecID = 65557;
pub const AV_CODEC_ID_PCM_F32BE: AVCodecID = 65556;
pub const AV_CODEC_ID_PCM_DVD: AVCodecID = 65555;
pub const AV_CODEC_ID_PCM_S16LE_PLANAR: AVCodecID = 65554;
pub const AV_CODEC_ID_PCM_ZORK: AVCodecID = 65553;
pub const AV_CODEC_ID_PCM_S24DAUD: AVCodecID = 65552;
pub const AV_CODEC_ID_PCM_U24BE: AVCodecID = 65551;
pub const AV_CODEC_ID_PCM_U24LE: AVCodecID = 65550;
pub const AV_CODEC_ID_PCM_S24BE: AVCodecID = 65549;
pub const AV_CODEC_ID_PCM_S24LE: AVCodecID = 65548;
pub const AV_CODEC_ID_PCM_U32BE: AVCodecID = 65547;
pub const AV_CODEC_ID_PCM_U32LE: AVCodecID = 65546;
pub const AV_CODEC_ID_PCM_S32BE: AVCodecID = 65545;
pub const AV_CODEC_ID_PCM_S32LE: AVCodecID = 65544;
pub const AV_CODEC_ID_PCM_ALAW: AVCodecID = 65543;
pub const AV_CODEC_ID_PCM_MULAW: AVCodecID = 65542;
pub const AV_CODEC_ID_PCM_U8: AVCodecID = 65541;
pub const AV_CODEC_ID_PCM_S8: AVCodecID = 65540;
pub const AV_CODEC_ID_PCM_U16BE: AVCodecID = 65539;
pub const AV_CODEC_ID_PCM_U16LE: AVCodecID = 65538;
pub const AV_CODEC_ID_PCM_S16BE: AVCodecID = 65537;
pub const AV_CODEC_ID_PCM_S16LE: AVCodecID = 65536;
pub const AV_CODEC_ID_FIRST_AUDIO: AVCodecID = 65536;
pub const AV_CODEC_ID_VP4: AVCodecID = 32813;
pub const AV_CODEC_ID_LSCR: AVCodecID = 32812;
pub const AV_CODEC_ID_AGM: AVCodecID = 32811;
pub const AV_CODEC_ID_ARBC: AVCodecID = 32810;
pub const AV_CODEC_ID_HYMT: AVCodecID = 32809;
pub const AV_CODEC_ID_RASC: AVCodecID = 32808;
pub const AV_CODEC_ID_WCMV: AVCodecID = 32807;
pub const AV_CODEC_ID_MWSC: AVCodecID = 32806;
pub const AV_CODEC_ID_PROSUMER: AVCodecID = 32805;
pub const AV_CODEC_ID_IMM4: AVCodecID = 32804;
pub const AV_CODEC_ID_FITS: AVCodecID = 32803;
pub const AV_CODEC_ID_GDV: AVCodecID = 32802;
pub const AV_CODEC_ID_SVG: AVCodecID = 32801;
pub const AV_CODEC_ID_SRGC: AVCodecID = 32800;
pub const AV_CODEC_ID_MSCC: AVCodecID = 32799;
pub const AV_CODEC_ID_BITPACKED: AVCodecID = 32798;
pub const AV_CODEC_ID_AV1: AVCodecID = 32797;
pub const AV_CODEC_ID_XPM: AVCodecID = 32796;
pub const AV_CODEC_ID_CLEARVIDEO: AVCodecID = 32795;
pub const AV_CODEC_ID_SCPR: AVCodecID = 32794;
pub const AV_CODEC_ID_FMVC: AVCodecID = 32793;
pub const AV_CODEC_ID_SPEEDHQ: AVCodecID = 32792;
pub const AV_CODEC_ID_PIXLET: AVCodecID = 32791;
pub const AV_CODEC_ID_PSD: AVCodecID = 32790;
pub const AV_CODEC_ID_YLC: AVCodecID = 32789;
pub const AV_CODEC_ID_SHEERVIDEO: AVCodecID = 32788;
pub const AV_CODEC_ID_MAGICYUV: AVCodecID = 32787;
pub const AV_CODEC_ID_M101: AVCodecID = 32786;
pub const AV_CODEC_ID_TRUEMOTION2RT: AVCodecID = 32785;
pub const AV_CODEC_ID_CFHD: AVCodecID = 32784;
pub const AV_CODEC_ID_DAALA: AVCodecID = 32783;
pub const AV_CODEC_ID_APNG: AVCodecID = 32782;
pub const AV_CODEC_ID_SMVJPEG: AVCodecID = 32781;
pub const AV_CODEC_ID_SNOW: AVCodecID = 32780;
pub const AV_CODEC_ID_XFACE: AVCodecID = 32779;
pub const AV_CODEC_ID_CPIA: AVCodecID = 32778;
pub const AV_CODEC_ID_AVRN: AVCodecID = 32777;
pub const AV_CODEC_ID_YUV4: AVCodecID = 32776;
pub const AV_CODEC_ID_V408: AVCodecID = 32775;
pub const AV_CODEC_ID_V308: AVCodecID = 32774;
pub const AV_CODEC_ID_TARGA_Y216: AVCodecID = 32773;
pub const AV_CODEC_ID_AYUV: AVCodecID = 32772;
pub const AV_CODEC_ID_AVUI: AVCodecID = 32771;
pub const AV_CODEC_ID_012V: AVCodecID = 32770;
pub const AV_CODEC_ID_AVRP: AVCodecID = 32769;
pub const AV_CODEC_ID_Y41P: AVCodecID = 32768;
pub const AV_CODEC_ID_AVS2: AVCodecID = 192;
pub const AV_CODEC_ID_RSCC: AVCodecID = 191;
pub const AV_CODEC_ID_SCREENPRESSO: AVCodecID = 190;
pub const AV_CODEC_ID_DXV: AVCodecID = 189;
pub const AV_CODEC_ID_DDS: AVCodecID = 188;
pub const AV_CODEC_ID_HAP: AVCodecID = 187;
pub const AV_CODEC_ID_HQ_HQA: AVCodecID = 186;
pub const AV_CODEC_ID_TDSC: AVCodecID = 185;
pub const AV_CODEC_ID_HQX: AVCodecID = 184;
pub const AV_CODEC_ID_MVC2: AVCodecID = 183;
pub const AV_CODEC_ID_MVC1: AVCodecID = 182;
pub const AV_CODEC_ID_SGIRLE: AVCodecID = 181;
pub const AV_CODEC_ID_SANM: AVCodecID = 180;
pub const AV_CODEC_ID_VP7: AVCodecID = 179;
pub const AV_CODEC_ID_EXR: AVCodecID = 178;
pub const AV_CODEC_ID_PAF_VIDEO: AVCodecID = 177;
pub const AV_CODEC_ID_BRENDER_PIX: AVCodecID = 176;
pub const AV_CODEC_ID_ALIAS_PIX: AVCodecID = 175;
pub const AV_CODEC_ID_FIC: AVCodecID = 174;
pub const AV_CODEC_ID_HEVC: AVCodecID = 173;
pub const AV_CODEC_ID_HNM4_VIDEO: AVCodecID = 172;
pub const AV_CODEC_ID_WEBP: AVCodecID = 171;
pub const AV_CODEC_ID_G2M: AVCodecID = 170;
pub const AV_CODEC_ID_ESCAPE130: AVCodecID = 169;
pub const AV_CODEC_ID_AIC: AVCodecID = 168;
pub const AV_CODEC_ID_VP9: AVCodecID = 167;
pub const AV_CODEC_ID_MSS2: AVCodecID = 166;
pub const AV_CODEC_ID_CLLC: AVCodecID = 165;
pub const AV_CODEC_ID_MTS2: AVCodecID = 164;
pub const AV_CODEC_ID_TSCC2: AVCodecID = 163;
pub const AV_CODEC_ID_MSA1: AVCodecID = 162;
pub const AV_CODEC_ID_MSS1: AVCodecID = 161;
pub const AV_CODEC_ID_ZEROCODEC: AVCodecID = 160;
pub const AV_CODEC_ID_XBM: AVCodecID = 159;
pub const AV_CODEC_ID_CDXL: AVCodecID = 158;
pub const AV_CODEC_ID_XWD: AVCodecID = 157;
pub const AV_CODEC_ID_V410: AVCodecID = 156;
pub const AV_CODEC_ID_DXTORY: AVCodecID = 155;
pub const AV_CODEC_ID_VBLE: AVCodecID = 154;
pub const AV_CODEC_ID_BMV_VIDEO: AVCodecID = 153;
pub const AV_CODEC_ID_UTVIDEO: AVCodecID = 152;
pub const AV_CODEC_ID_VC1IMAGE: AVCodecID = 151;
pub const AV_CODEC_ID_WMV3IMAGE: AVCodecID = 150;
pub const AV_CODEC_ID_DFA: AVCodecID = 149;
pub const AV_CODEC_ID_JV: AVCodecID = 148;
pub const AV_CODEC_ID_PRORES: AVCodecID = 147;
pub const AV_CODEC_ID_LAGARITH: AVCodecID = 146;
pub const AV_CODEC_ID_MXPEG: AVCodecID = 145;
pub const AV_CODEC_ID_R10K: AVCodecID = 144;
pub const AV_CODEC_ID_A64_MULTI5: AVCodecID = 143;
pub const AV_CODEC_ID_A64_MULTI: AVCodecID = 142;
pub const AV_CODEC_ID_ANSI: AVCodecID = 141;
pub const AV_CODEC_ID_PICTOR: AVCodecID = 140;
pub const AV_CODEC_ID_VP8: AVCodecID = 139;
pub const AV_CODEC_ID_YOP: AVCodecID = 138;
pub const AV_CODEC_ID_KGV1: AVCodecID = 137;
pub const AV_CODEC_ID_IFF_ILBM: AVCodecID = 136;
pub const AV_CODEC_ID_BINKVIDEO: AVCodecID = 135;
pub const AV_CODEC_ID_ANM: AVCodecID = 134;
pub const AV_CODEC_ID_R210: AVCodecID = 133;
pub const AV_CODEC_ID_CDGRAPHICS: AVCodecID = 132;
pub const AV_CODEC_ID_FLASHSV2: AVCodecID = 131;
pub const AV_CODEC_ID_FRWU: AVCodecID = 130;
pub const AV_CODEC_ID_MAD: AVCodecID = 129;
pub const AV_CODEC_ID_DPX: AVCodecID = 128;
pub const AV_CODEC_ID_V210: AVCodecID = 127;
pub const AV_CODEC_ID_TMV: AVCodecID = 126;
pub const AV_CODEC_ID_V210X: AVCodecID = 125;
pub const AV_CODEC_ID_AURA2: AVCodecID = 124;
pub const AV_CODEC_ID_AURA: AVCodecID = 123;
pub const AV_CODEC_ID_TQI: AVCodecID = 122;
pub const AV_CODEC_ID_TGQ: AVCodecID = 121;
pub const AV_CODEC_ID_TGV: AVCodecID = 120;
pub const AV_CODEC_ID_MOTIONPIXELS: AVCodecID = 119;
pub const AV_CODEC_ID_CMV: AVCodecID = 118;
pub const AV_CODEC_ID_BFI: AVCodecID = 117;
pub const AV_CODEC_ID_DIRAC: AVCodecID = 116;
pub const AV_CODEC_ID_ESCAPE124: AVCodecID = 115;
pub const AV_CODEC_ID_RL2: AVCodecID = 114;
pub const AV_CODEC_ID_MIMIC: AVCodecID = 113;
pub const AV_CODEC_ID_INDEO5: AVCodecID = 112;
pub const AV_CODEC_ID_INDEO4: AVCodecID = 111;
pub const AV_CODEC_ID_SUNRAST: AVCodecID = 110;
pub const AV_CODEC_ID_PCX: AVCodecID = 109;
pub const AV_CODEC_ID_VB: AVCodecID = 108;
pub const AV_CODEC_ID_AMV: AVCodecID = 107;
pub const AV_CODEC_ID_VP6A: AVCodecID = 106;
pub const AV_CODEC_ID_TXD: AVCodecID = 105;
pub const AV_CODEC_ID_PTX: AVCodecID = 104;
pub const AV_CODEC_ID_BETHSOFTVID: AVCodecID = 103;
pub const AV_CODEC_ID_C93: AVCodecID = 102;
pub const AV_CODEC_ID_SGI: AVCodecID = 101;
pub const AV_CODEC_ID_THP: AVCodecID = 100;
pub const AV_CODEC_ID_DNXHD: AVCodecID = 99;
pub const AV_CODEC_ID_DXA: AVCodecID = 98;
pub const AV_CODEC_ID_GIF: AVCodecID = 97;
pub const AV_CODEC_ID_TIFF: AVCodecID = 96;
pub const AV_CODEC_ID_TIERTEXSEQVIDEO: AVCodecID = 95;
pub const AV_CODEC_ID_DSICINVIDEO: AVCodecID = 94;
pub const AV_CODEC_ID_TARGA: AVCodecID = 93;
pub const AV_CODEC_ID_VP6F: AVCodecID = 92;
pub const AV_CODEC_ID_VP6: AVCodecID = 91;
pub const AV_CODEC_ID_VP5: AVCodecID = 90;
pub const AV_CODEC_ID_VMNC: AVCodecID = 89;
pub const AV_CODEC_ID_JPEG2000: AVCodecID = 88;
pub const AV_CODEC_ID_CAVS: AVCodecID = 87;
pub const AV_CODEC_ID_FLASHSV: AVCodecID = 86;
pub const AV_CODEC_ID_KMVC: AVCodecID = 85;
pub const AV_CODEC_ID_NUV: AVCodecID = 84;
pub const AV_CODEC_ID_SMACKVIDEO: AVCodecID = 83;
pub const AV_CODEC_ID_AVS: AVCodecID = 82;
pub const AV_CODEC_ID_ZMBV: AVCodecID = 81;
pub const AV_CODEC_ID_MMVIDEO: AVCodecID = 80;
pub const AV_CODEC_ID_CSCD: AVCodecID = 79;
pub const AV_CODEC_ID_BMP: AVCodecID = 78;
pub const AV_CODEC_ID_TRUEMOTION2: AVCodecID = 77;
pub const AV_CODEC_ID_FRAPS: AVCodecID = 76;
pub const AV_CODEC_ID_INDEO2: AVCodecID = 75;
pub const AV_CODEC_ID_AASC: AVCodecID = 74;
pub const AV_CODEC_ID_WNV1: AVCodecID = 73;
pub const AV_CODEC_ID_LOCO: AVCodecID = 72;
pub const AV_CODEC_ID_WMV3: AVCodecID = 71;
pub const AV_CODEC_ID_VC1: AVCodecID = 70;
pub const AV_CODEC_ID_RV40: AVCodecID = 69;
pub const AV_CODEC_ID_RV30: AVCodecID = 68;
pub const AV_CODEC_ID_FFVHUFF: AVCodecID = 67;
pub const AV_CODEC_ID_PAM: AVCodecID = 66;
pub const AV_CODEC_ID_PGMYUV: AVCodecID = 65;
pub const AV_CODEC_ID_PGM: AVCodecID = 64;
pub const AV_CODEC_ID_PBM: AVCodecID = 63;
pub const AV_CODEC_ID_PPM: AVCodecID = 62;
pub const AV_CODEC_ID_PNG: AVCodecID = 61;
pub const AV_CODEC_ID_QPEG: AVCodecID = 60;
pub const AV_CODEC_ID_VIXL: AVCodecID = 59;
pub const AV_CODEC_ID_QDRAW: AVCodecID = 58;
pub const AV_CODEC_ID_ULTI: AVCodecID = 57;
pub const AV_CODEC_ID_TSCC: AVCodecID = 56;
pub const AV_CODEC_ID_QTRLE: AVCodecID = 55;
pub const AV_CODEC_ID_ZLIB: AVCodecID = 54;
pub const AV_CODEC_ID_MSZH: AVCodecID = 53;
pub const AV_CODEC_ID_VMDVIDEO: AVCodecID = 52;
pub const AV_CODEC_ID_TRUEMOTION1: AVCodecID = 51;
pub const AV_CODEC_ID_FLIC: AVCodecID = 50;
pub const AV_CODEC_ID_SMC: AVCodecID = 49;
pub const AV_CODEC_ID_8BPS: AVCodecID = 48;
pub const AV_CODEC_ID_IDCIN: AVCodecID = 47;
pub const AV_CODEC_ID_MSVIDEO1: AVCodecID = 46;
pub const AV_CODEC_ID_MSRLE: AVCodecID = 45;
pub const AV_CODEC_ID_WS_VQA: AVCodecID = 44;
pub const AV_CODEC_ID_CINEPAK: AVCodecID = 43;
pub const AV_CODEC_ID_RPZA: AVCodecID = 42;
pub const AV_CODEC_ID_XAN_WC4: AVCodecID = 41;
pub const AV_CODEC_ID_XAN_WC3: AVCodecID = 40;
pub const AV_CODEC_ID_INTERPLAY_VIDEO: AVCodecID = 39;
pub const AV_CODEC_ID_ROQ: AVCodecID = 38;
pub const AV_CODEC_ID_MDEC: AVCodecID = 37;
pub const AV_CODEC_ID_CLJR: AVCodecID = 36;
pub const AV_CODEC_ID_VCR1: AVCodecID = 35;
pub const AV_CODEC_ID_4XM: AVCodecID = 34;
pub const AV_CODEC_ID_FFV1: AVCodecID = 33;
pub const AV_CODEC_ID_ASV2: AVCodecID = 32;
pub const AV_CODEC_ID_ASV1: AVCodecID = 31;
pub const AV_CODEC_ID_THEORA: AVCodecID = 30;
pub const AV_CODEC_ID_VP3: AVCodecID = 29;
pub const AV_CODEC_ID_INDEO3: AVCodecID = 28;
pub const AV_CODEC_ID_H264: AVCodecID = 27;
pub const AV_CODEC_ID_CYUV: AVCodecID = 26;
pub const AV_CODEC_ID_HUFFYUV: AVCodecID = 25;
pub const AV_CODEC_ID_DVVIDEO: AVCodecID = 24;
pub const AV_CODEC_ID_SVQ3: AVCodecID = 23;
pub const AV_CODEC_ID_SVQ1: AVCodecID = 22;
pub const AV_CODEC_ID_FLV1: AVCodecID = 21;
pub const AV_CODEC_ID_H263I: AVCodecID = 20;
pub const AV_CODEC_ID_H263P: AVCodecID = 19;
pub const AV_CODEC_ID_WMV2: AVCodecID = 18;
pub const AV_CODEC_ID_WMV1: AVCodecID = 17;
pub const AV_CODEC_ID_MSMPEG4V3: AVCodecID = 16;
pub const AV_CODEC_ID_MSMPEG4V2: AVCodecID = 15;
pub const AV_CODEC_ID_MSMPEG4V1: AVCodecID = 14;
pub const AV_CODEC_ID_RAWVIDEO: AVCodecID = 13;
pub const AV_CODEC_ID_MPEG4: AVCodecID = 12;
pub const AV_CODEC_ID_JPEGLS: AVCodecID = 11;
pub const AV_CODEC_ID_SP5X: AVCodecID = 10;
pub const AV_CODEC_ID_LJPEG: AVCodecID = 9;
pub const AV_CODEC_ID_MJPEGB: AVCodecID = 8;
pub const AV_CODEC_ID_MJPEG: AVCodecID = 7;
pub const AV_CODEC_ID_RV20: AVCodecID = 6;
pub const AV_CODEC_ID_RV10: AVCodecID = 5;
pub const AV_CODEC_ID_H263: AVCodecID = 4;
pub const AV_CODEC_ID_H261: AVCodecID = 3;
pub const AV_CODEC_ID_MPEG2VIDEO: AVCodecID = 2;
pub const AV_CODEC_ID_MPEG1VIDEO: AVCodecID = 1;
pub const AV_CODEC_ID_NONE: AVCodecID = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecDescriptor {
    pub id: AVCodecID,
    pub type_0: AVMediaType,
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub props: libc::c_int,
    pub mime_types: *const *const libc::c_char,
    pub profiles: *const AVProfile,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProfile {
    pub profile: libc::c_int,
    pub name: *const libc::c_char,
}
pub type AVDiscard = libc::c_int;
pub const AVDISCARD_ALL: AVDiscard = 48;
pub const AVDISCARD_NONKEY: AVDiscard = 32;
pub const AVDISCARD_NONINTRA: AVDiscard = 24;
pub const AVDISCARD_BIDIR: AVDiscard = 16;
pub const AVDISCARD_NONREF: AVDiscard = 8;
pub const AVDISCARD_DEFAULT: AVDiscard = 0;
pub const AVDISCARD_NONE: AVDiscard = -16;
pub type AVAudioServiceType = libc::c_uint;
pub const AV_AUDIO_SERVICE_TYPE_NB: AVAudioServiceType = 9;
pub const AV_AUDIO_SERVICE_TYPE_KARAOKE: AVAudioServiceType = 8;
pub const AV_AUDIO_SERVICE_TYPE_VOICE_OVER: AVAudioServiceType = 7;
pub const AV_AUDIO_SERVICE_TYPE_EMERGENCY: AVAudioServiceType = 6;
pub const AV_AUDIO_SERVICE_TYPE_COMMENTARY: AVAudioServiceType = 5;
pub const AV_AUDIO_SERVICE_TYPE_DIALOGUE: AVAudioServiceType = 4;
pub const AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED: AVAudioServiceType = 3;
pub const AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED: AVAudioServiceType = 2;
pub const AV_AUDIO_SERVICE_TYPE_EFFECTS: AVAudioServiceType = 1;
pub const AV_AUDIO_SERVICE_TYPE_MAIN: AVAudioServiceType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RcOverride {
    pub start_frame: libc::c_int,
    pub end_frame: libc::c_int,
    pub qscale: libc::c_int,
    pub quality_factor: libc::c_float,
}
pub type AVPacketSideDataType = libc::c_uint;
pub const AV_PKT_DATA_NB: AVPacketSideDataType = 27;
pub const AV_PKT_DATA_AFD: AVPacketSideDataType = 26;
pub const AV_PKT_DATA_ENCRYPTION_INFO: AVPacketSideDataType = 25;
pub const AV_PKT_DATA_ENCRYPTION_INIT_INFO: AVPacketSideDataType = 24;
pub const AV_PKT_DATA_A53_CC: AVPacketSideDataType = 23;
pub const AV_PKT_DATA_CONTENT_LIGHT_LEVEL: AVPacketSideDataType = 22;
pub const AV_PKT_DATA_SPHERICAL: AVPacketSideDataType = 21;
pub const AV_PKT_DATA_MASTERING_DISPLAY_METADATA: AVPacketSideDataType = 20;
pub const AV_PKT_DATA_MPEGTS_STREAM_ID: AVPacketSideDataType = 19;
pub const AV_PKT_DATA_METADATA_UPDATE: AVPacketSideDataType = 18;
pub const AV_PKT_DATA_WEBVTT_SETTINGS: AVPacketSideDataType = 17;
pub const AV_PKT_DATA_WEBVTT_IDENTIFIER: AVPacketSideDataType = 16;
pub const AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL: AVPacketSideDataType = 15;
pub const AV_PKT_DATA_SUBTITLE_POSITION: AVPacketSideDataType = 14;
pub const AV_PKT_DATA_STRINGS_METADATA: AVPacketSideDataType = 13;
pub const AV_PKT_DATA_JP_DUALMONO: AVPacketSideDataType = 12;
pub const AV_PKT_DATA_SKIP_SAMPLES: AVPacketSideDataType = 11;
pub const AV_PKT_DATA_CPB_PROPERTIES: AVPacketSideDataType = 10;
pub const AV_PKT_DATA_FALLBACK_TRACK: AVPacketSideDataType = 9;
pub const AV_PKT_DATA_QUALITY_STATS: AVPacketSideDataType = 8;
pub const AV_PKT_DATA_AUDIO_SERVICE_TYPE: AVPacketSideDataType = 7;
pub const AV_PKT_DATA_STEREO3D: AVPacketSideDataType = 6;
pub const AV_PKT_DATA_DISPLAYMATRIX: AVPacketSideDataType = 5;
pub const AV_PKT_DATA_REPLAYGAIN: AVPacketSideDataType = 4;
pub const AV_PKT_DATA_H263_MB_INFO: AVPacketSideDataType = 3;
pub const AV_PKT_DATA_PARAM_CHANGE: AVPacketSideDataType = 2;
pub const AV_PKT_DATA_NEW_EXTRADATA: AVPacketSideDataType = 1;
pub const AV_PKT_DATA_PALETTE: AVPacketSideDataType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacketSideData {
    pub data: *mut uint8_t,
    pub size: libc::c_int,
    pub type_0: AVPacketSideDataType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacket {
    pub buf: *mut AVBufferRef,
    pub pts: int64_t,
    pub dts: int64_t,
    pub data: *mut uint8_t,
    pub size: libc::c_int,
    pub stream_index: libc::c_int,
    pub flags: libc::c_int,
    pub side_data: *mut AVPacketSideData,
    pub side_data_elems: libc::c_int,
    pub duration: int64_t,
    pub pos: int64_t,
    pub convergence_duration: int64_t,
}
pub type AVFieldOrder = libc::c_uint;
pub const AV_FIELD_BT: AVFieldOrder = 5;
pub const AV_FIELD_TB: AVFieldOrder = 4;
pub const AV_FIELD_BB: AVFieldOrder = 3;
pub const AV_FIELD_TT: AVFieldOrder = 2;
pub const AV_FIELD_PROGRESSIVE: AVFieldOrder = 1;
pub const AV_FIELD_UNKNOWN: AVFieldOrder = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecContext {
    pub av_class: *const AVClass,
    pub log_level_offset: libc::c_int,
    pub codec_type: AVMediaType,
    pub codec: *const AVCodec,
    pub codec_id: AVCodecID,
    pub codec_tag: libc::c_uint,
    pub priv_data: *mut libc::c_void,
    pub internal: *mut AVCodecInternal,
    pub opaque: *mut libc::c_void,
    pub bit_rate: int64_t,
    pub bit_rate_tolerance: libc::c_int,
    pub global_quality: libc::c_int,
    pub compression_level: libc::c_int,
    pub flags: libc::c_int,
    pub flags2: libc::c_int,
    pub extradata: *mut uint8_t,
    pub extradata_size: libc::c_int,
    pub time_base: AVRational,
    pub ticks_per_frame: libc::c_int,
    pub delay: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub coded_width: libc::c_int,
    pub coded_height: libc::c_int,
    pub gop_size: libc::c_int,
    pub pix_fmt: AVPixelFormat,
    pub draw_horiz_band: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const AVFrame,
            *mut libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub get_format: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *const AVPixelFormat) -> AVPixelFormat,
    >,
    pub max_b_frames: libc::c_int,
    pub b_quant_factor: libc::c_float,
    pub b_frame_strategy: libc::c_int,
    pub b_quant_offset: libc::c_float,
    pub has_b_frames: libc::c_int,
    pub mpeg_quant: libc::c_int,
    pub i_quant_factor: libc::c_float,
    pub i_quant_offset: libc::c_float,
    pub lumi_masking: libc::c_float,
    pub temporal_cplx_masking: libc::c_float,
    pub spatial_cplx_masking: libc::c_float,
    pub p_masking: libc::c_float,
    pub dark_masking: libc::c_float,
    pub slice_count: libc::c_int,
    pub prediction_method: libc::c_int,
    pub slice_offset: *mut libc::c_int,
    pub sample_aspect_ratio: AVRational,
    pub me_cmp: libc::c_int,
    pub me_sub_cmp: libc::c_int,
    pub mb_cmp: libc::c_int,
    pub ildct_cmp: libc::c_int,
    pub dia_size: libc::c_int,
    pub last_predictor_count: libc::c_int,
    pub pre_me: libc::c_int,
    pub me_pre_cmp: libc::c_int,
    pub pre_dia_size: libc::c_int,
    pub me_subpel_quality: libc::c_int,
    pub me_range: libc::c_int,
    pub slice_flags: libc::c_int,
    pub mb_decision: libc::c_int,
    pub intra_matrix: *mut uint16_t,
    pub inter_matrix: *mut uint16_t,
    pub scenechange_threshold: libc::c_int,
    pub noise_reduction: libc::c_int,
    pub intra_dc_precision: libc::c_int,
    pub skip_top: libc::c_int,
    pub skip_bottom: libc::c_int,
    pub mb_lmin: libc::c_int,
    pub mb_lmax: libc::c_int,
    pub me_penalty_compensation: libc::c_int,
    pub bidir_refine: libc::c_int,
    pub brd_scale: libc::c_int,
    pub keyint_min: libc::c_int,
    pub refs: libc::c_int,
    pub chromaoffset: libc::c_int,
    pub mv0_threshold: libc::c_int,
    pub b_sensitivity: libc::c_int,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub color_range: AVColorRange,
    pub chroma_sample_location: AVChromaLocation,
    pub slices: libc::c_int,
    pub field_order: AVFieldOrder,
    pub sample_rate: libc::c_int,
    pub channels: libc::c_int,
    pub sample_fmt: AVSampleFormat,
    pub frame_size: libc::c_int,
    pub frame_number: libc::c_int,
    pub block_align: libc::c_int,
    pub cutoff: libc::c_int,
    pub channel_layout: uint64_t,
    pub request_channel_layout: uint64_t,
    pub audio_service_type: AVAudioServiceType,
    pub request_sample_fmt: AVSampleFormat,
    pub get_buffer2: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVFrame,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub refcounted_frames: libc::c_int,
    pub qcompress: libc::c_float,
    pub qblur: libc::c_float,
    pub qmin: libc::c_int,
    pub qmax: libc::c_int,
    pub max_qdiff: libc::c_int,
    pub rc_buffer_size: libc::c_int,
    pub rc_override_count: libc::c_int,
    pub rc_override: *mut RcOverride,
    pub rc_max_rate: int64_t,
    pub rc_min_rate: int64_t,
    pub rc_max_available_vbv_use: libc::c_float,
    pub rc_min_vbv_overflow_use: libc::c_float,
    pub rc_initial_buffer_occupancy: libc::c_int,
    pub coder_type: libc::c_int,
    pub context_model: libc::c_int,
    pub frame_skip_threshold: libc::c_int,
    pub frame_skip_factor: libc::c_int,
    pub frame_skip_exp: libc::c_int,
    pub frame_skip_cmp: libc::c_int,
    pub trellis: libc::c_int,
    pub min_prediction_order: libc::c_int,
    pub max_prediction_order: libc::c_int,
    pub timecode_frame_start: int64_t,
    pub rtp_callback: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub rtp_payload_size: libc::c_int,
    pub mv_bits: libc::c_int,
    pub header_bits: libc::c_int,
    pub i_tex_bits: libc::c_int,
    pub p_tex_bits: libc::c_int,
    pub i_count: libc::c_int,
    pub p_count: libc::c_int,
    pub skip_count: libc::c_int,
    pub misc_bits: libc::c_int,
    pub frame_bits: libc::c_int,
    pub stats_out: *mut libc::c_char,
    pub stats_in: *mut libc::c_char,
    pub workaround_bugs: libc::c_int,
    pub strict_std_compliance: libc::c_int,
    pub error_concealment: libc::c_int,
    pub debug: libc::c_int,
    pub err_recognition: libc::c_int,
    pub reordered_opaque: int64_t,
    pub hwaccel: *const AVHWAccel,
    pub hwaccel_context: *mut libc::c_void,
    pub error: [uint64_t; 8],
    pub dct_algo: libc::c_int,
    pub idct_algo: libc::c_int,
    pub bits_per_coded_sample: libc::c_int,
    pub bits_per_raw_sample: libc::c_int,
    pub lowres: libc::c_int,
    pub coded_frame: *mut AVFrame,
    pub thread_count: libc::c_int,
    pub thread_type: libc::c_int,
    pub active_thread_type: libc::c_int,
    pub thread_safe_callbacks: libc::c_int,
    pub execute: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            Option::<
                unsafe extern "C" fn(
                    *mut AVCodecContext,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            *mut libc::c_void,
            *mut libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub execute2: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            Option::<
                unsafe extern "C" fn(
                    *mut AVCodecContext,
                    *mut libc::c_void,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
            >,
            *mut libc::c_void,
            *mut libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub nsse_weight: libc::c_int,
    pub profile: libc::c_int,
    pub level: libc::c_int,
    pub skip_loop_filter: AVDiscard,
    pub skip_idct: AVDiscard,
    pub skip_frame: AVDiscard,
    pub subtitle_header: *mut uint8_t,
    pub subtitle_header_size: libc::c_int,
    pub vbv_delay: uint64_t,
    pub side_data_only_packets: libc::c_int,
    pub initial_padding: libc::c_int,
    pub framerate: AVRational,
    pub sw_pix_fmt: AVPixelFormat,
    pub pkt_timebase: AVRational,
    pub codec_descriptor: *const AVCodecDescriptor,
    pub pts_correction_num_faulty_pts: int64_t,
    pub pts_correction_num_faulty_dts: int64_t,
    pub pts_correction_last_pts: int64_t,
    pub pts_correction_last_dts: int64_t,
    pub sub_charenc: *mut libc::c_char,
    pub sub_charenc_mode: libc::c_int,
    pub skip_alpha: libc::c_int,
    pub seek_preroll: libc::c_int,
    pub debug_mv: libc::c_int,
    pub chroma_intra_matrix: *mut uint16_t,
    pub dump_separator: *mut uint8_t,
    pub codec_whitelist: *mut libc::c_char,
    pub properties: libc::c_uint,
    pub coded_side_data: *mut AVPacketSideData,
    pub nb_coded_side_data: libc::c_int,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub sub_text_format: libc::c_int,
    pub trailing_padding: libc::c_int,
    pub max_pixels: int64_t,
    pub hw_device_ctx: *mut AVBufferRef,
    pub hwaccel_flags: libc::c_int,
    pub apply_cropping: libc::c_int,
    pub extra_hw_frames: libc::c_int,
    pub discard_damaged_percentage: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVHWAccel {
    pub name: *const libc::c_char,
    pub type_0: AVMediaType,
    pub id: AVCodecID,
    pub pix_fmt: AVPixelFormat,
    pub capabilities: libc::c_int,
    pub alloc_frame: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVFrame) -> libc::c_int,
    >,
    pub start_frame: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const uint8_t,
            uint32_t,
        ) -> libc::c_int,
    >,
    pub decode_params: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            libc::c_int,
            *const uint8_t,
            uint32_t,
        ) -> libc::c_int,
    >,
    pub decode_slice: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const uint8_t,
            uint32_t,
        ) -> libc::c_int,
    >,
    pub end_frame: Option::<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub frame_priv_data_size: libc::c_int,
    pub decode_mb: Option::<unsafe extern "C" fn(*mut MpegEncContext) -> ()>,
    pub init: Option::<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub uninit: Option::<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub priv_data_size: libc::c_int,
    pub caps_internal: libc::c_int,
    pub frame_params: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVBufferRef) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodec {
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub type_0: AVMediaType,
    pub id: AVCodecID,
    pub capabilities: libc::c_int,
    pub supported_framerates: *const AVRational,
    pub pix_fmts: *const AVPixelFormat,
    pub supported_samplerates: *const libc::c_int,
    pub sample_fmts: *const AVSampleFormat,
    pub channel_layouts: *const uint64_t,
    pub max_lowres: uint8_t,
    pub priv_class: *const AVClass,
    pub profiles: *const AVProfile,
    pub wrapper_name: *const libc::c_char,
    pub priv_data_size: libc::c_int,
    pub next: *mut AVCodec,
    pub init_thread_copy: Option::<
        unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int,
    >,
    pub update_thread_context: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *const AVCodecContext) -> libc::c_int,
    >,
    pub defaults: *const AVCodecDefault,
    pub init_static_data: Option::<unsafe extern "C" fn(*mut AVCodec) -> ()>,
    pub init: Option::<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub encode_sub: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut uint8_t,
            libc::c_int,
            *const AVSubtitle,
        ) -> libc::c_int,
    >,
    pub encode2: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVPacket,
            *const AVFrame,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub decode: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut libc::c_void,
            *mut libc::c_int,
            *mut AVPacket,
        ) -> libc::c_int,
    >,
    pub close: Option::<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub send_frame: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *const AVFrame) -> libc::c_int,
    >,
    pub receive_packet: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVPacket) -> libc::c_int,
    >,
    pub receive_frame: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVFrame) -> libc::c_int,
    >,
    pub flush: Option::<unsafe extern "C" fn(*mut AVCodecContext) -> ()>,
    pub caps_internal: libc::c_int,
    pub bsfs: *const libc::c_char,
    pub hw_configs: *mut *const AVCodecHWConfigInternal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVSubtitle {
    pub format: uint16_t,
    pub start_display_time: uint32_t,
    pub end_display_time: uint32_t,
    pub num_rects: libc::c_uint,
    pub rects: *mut *mut AVSubtitleRect,
    pub pts: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVSubtitleRect {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub nb_colors: libc::c_int,
    pub pict: AVPicture,
    pub data: [*mut uint8_t; 4],
    pub linesize: [libc::c_int; 4],
    pub type_0: AVSubtitleType,
    pub text: *mut libc::c_char,
    pub ass: *mut libc::c_char,
    pub flags: libc::c_int,
}
pub type AVSubtitleType = libc::c_uint;
pub const SUBTITLE_ASS: AVSubtitleType = 3;
pub const SUBTITLE_TEXT: AVSubtitleType = 2;
pub const SUBTITLE_BITMAP: AVSubtitleType = 1;
pub const SUBTITLE_NONE: AVSubtitleType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPicture {
    pub data: [*mut uint8_t; 8],
    pub linesize: [libc::c_int; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecParameters {
    pub codec_type: AVMediaType,
    pub codec_id: AVCodecID,
    pub codec_tag: uint32_t,
    pub extradata: *mut uint8_t,
    pub extradata_size: libc::c_int,
    pub format: libc::c_int,
    pub bit_rate: int64_t,
    pub bits_per_coded_sample: libc::c_int,
    pub bits_per_raw_sample: libc::c_int,
    pub profile: libc::c_int,
    pub level: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub sample_aspect_ratio: AVRational,
    pub field_order: AVFieldOrder,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub color_space: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub video_delay: libc::c_int,
    pub channel_layout: uint64_t,
    pub channels: libc::c_int,
    pub sample_rate: libc::c_int,
    pub block_align: libc::c_int,
    pub frame_size: libc::c_int,
    pub initial_padding: libc::c_int,
    pub trailing_padding: libc::c_int,
    pub seek_preroll: libc::c_int,
}
pub type AVPictureStructure = libc::c_uint;
pub const AV_PICTURE_STRUCTURE_FRAME: AVPictureStructure = 3;
pub const AV_PICTURE_STRUCTURE_BOTTOM_FIELD: AVPictureStructure = 2;
pub const AV_PICTURE_STRUCTURE_TOP_FIELD: AVPictureStructure = 1;
pub const AV_PICTURE_STRUCTURE_UNKNOWN: AVPictureStructure = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecParserContext {
    pub priv_data: *mut libc::c_void,
    pub parser: *mut AVCodecParser,
    pub frame_offset: int64_t,
    pub cur_offset: int64_t,
    pub next_frame_offset: int64_t,
    pub pict_type: libc::c_int,
    pub repeat_pict: libc::c_int,
    pub pts: int64_t,
    pub dts: int64_t,
    pub last_pts: int64_t,
    pub last_dts: int64_t,
    pub fetch_timestamp: libc::c_int,
    pub cur_frame_start_index: libc::c_int,
    pub cur_frame_offset: [int64_t; 4],
    pub cur_frame_pts: [int64_t; 4],
    pub cur_frame_dts: [int64_t; 4],
    pub flags: libc::c_int,
    pub offset: int64_t,
    pub cur_frame_end: [int64_t; 4],
    pub key_frame: libc::c_int,
    pub convergence_duration: int64_t,
    pub dts_sync_point: libc::c_int,
    pub dts_ref_dts_delta: libc::c_int,
    pub pts_dts_delta: libc::c_int,
    pub cur_frame_pos: [int64_t; 4],
    pub pos: int64_t,
    pub last_pos: int64_t,
    pub duration: libc::c_int,
    pub field_order: AVFieldOrder,
    pub picture_structure: AVPictureStructure,
    pub output_picture_number: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub coded_width: libc::c_int,
    pub coded_height: libc::c_int,
    pub format: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecParser {
    pub codec_ids: [libc::c_int; 5],
    pub priv_data_size: libc::c_int,
    pub parser_init: Option::<
        unsafe extern "C" fn(*mut AVCodecParserContext) -> libc::c_int,
    >,
    pub parser_parse: Option::<
        unsafe extern "C" fn(
            *mut AVCodecParserContext,
            *mut AVCodecContext,
            *mut *const uint8_t,
            *mut libc::c_int,
            *const uint8_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub parser_close: Option::<unsafe extern "C" fn(*mut AVCodecParserContext) -> ()>,
    pub split: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const uint8_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub next: *mut AVCodecParser,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVIOInterruptCB {
    pub callback: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub opaque: *mut libc::c_void,
}
pub type AVIODataMarkerType = libc::c_uint;
pub const AVIO_DATA_MARKER_FLUSH_POINT: AVIODataMarkerType = 5;
pub const AVIO_DATA_MARKER_TRAILER: AVIODataMarkerType = 4;
pub const AVIO_DATA_MARKER_UNKNOWN: AVIODataMarkerType = 3;
pub const AVIO_DATA_MARKER_BOUNDARY_POINT: AVIODataMarkerType = 2;
pub const AVIO_DATA_MARKER_SYNC_POINT: AVIODataMarkerType = 1;
pub const AVIO_DATA_MARKER_HEADER: AVIODataMarkerType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVIOContext {
    pub av_class: *const AVClass,
    pub buffer: *mut libc::c_uchar,
    pub buffer_size: libc::c_int,
    pub buf_ptr: *mut libc::c_uchar,
    pub buf_end: *mut libc::c_uchar,
    pub opaque: *mut libc::c_void,
    pub read_packet: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut uint8_t, libc::c_int) -> libc::c_int,
    >,
    pub write_packet: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut uint8_t, libc::c_int) -> libc::c_int,
    >,
    pub seek: Option::<
        unsafe extern "C" fn(*mut libc::c_void, int64_t, libc::c_int) -> int64_t,
    >,
    pub pos: int64_t,
    pub eof_reached: libc::c_int,
    pub write_flag: libc::c_int,
    pub max_packet_size: libc::c_int,
    pub checksum: libc::c_ulong,
    pub checksum_ptr: *mut libc::c_uchar,
    pub update_checksum: Option::<
        unsafe extern "C" fn(
            libc::c_ulong,
            *const uint8_t,
            libc::c_uint,
        ) -> libc::c_ulong,
    >,
    pub error: libc::c_int,
    pub read_pause: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> libc::c_int,
    >,
    pub read_seek: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            int64_t,
            libc::c_int,
        ) -> int64_t,
    >,
    pub seekable: libc::c_int,
    pub maxsize: int64_t,
    pub direct: libc::c_int,
    pub bytes_read: int64_t,
    pub seek_count: libc::c_int,
    pub writeout_count: libc::c_int,
    pub orig_buffer_size: libc::c_int,
    pub short_seek_threshold: libc::c_int,
    pub protocol_whitelist: *const libc::c_char,
    pub protocol_blacklist: *const libc::c_char,
    pub write_data_type: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut uint8_t,
            libc::c_int,
            AVIODataMarkerType,
            int64_t,
        ) -> libc::c_int,
    >,
    pub ignore_boundary_point: libc::c_int,
    pub current_type: AVIODataMarkerType,
    pub last_time: int64_t,
    pub short_seek_get: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub written: int64_t,
    pub buf_ptr_max: *mut libc::c_uchar,
    pub min_packet_size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFormatContext {
    pub av_class: *const AVClass,
    pub iformat: *mut AVInputFormat,
    pub oformat: *mut AVOutputFormat,
    pub priv_data: *mut libc::c_void,
    pub pb: *mut AVIOContext,
    pub ctx_flags: libc::c_int,
    pub nb_streams: libc::c_uint,
    pub streams: *mut *mut AVStream,
    pub filename: [libc::c_char; 1024],
    pub url: *mut libc::c_char,
    pub start_time: int64_t,
    pub duration: int64_t,
    pub bit_rate: int64_t,
    pub packet_size: libc::c_uint,
    pub max_delay: libc::c_int,
    pub flags: libc::c_int,
    pub probesize: int64_t,
    pub max_analyze_duration: int64_t,
    pub key: *const uint8_t,
    pub keylen: libc::c_int,
    pub nb_programs: libc::c_uint,
    pub programs: *mut *mut AVProgram,
    pub video_codec_id: AVCodecID,
    pub audio_codec_id: AVCodecID,
    pub subtitle_codec_id: AVCodecID,
    pub max_index_size: libc::c_uint,
    pub max_picture_buffer: libc::c_uint,
    pub nb_chapters: libc::c_uint,
    pub chapters: *mut *mut AVChapter,
    pub metadata: *mut AVDictionary,
    pub start_time_realtime: int64_t,
    pub fps_probe_size: libc::c_int,
    pub error_recognition: libc::c_int,
    pub interrupt_callback: AVIOInterruptCB,
    pub debug: libc::c_int,
    pub max_interleave_delta: int64_t,
    pub strict_std_compliance: libc::c_int,
    pub event_flags: libc::c_int,
    pub max_ts_probe: libc::c_int,
    pub avoid_negative_ts: libc::c_int,
    pub ts_id: libc::c_int,
    pub audio_preload: libc::c_int,
    pub max_chunk_duration: libc::c_int,
    pub max_chunk_size: libc::c_int,
    pub use_wallclock_as_timestamps: libc::c_int,
    pub avio_flags: libc::c_int,
    pub duration_estimation_method: AVDurationEstimationMethod,
    pub skip_initial_bytes: int64_t,
    pub correct_ts_overflow: libc::c_uint,
    pub seek2any: libc::c_int,
    pub flush_packets: libc::c_int,
    pub probe_score: libc::c_int,
    pub format_probesize: libc::c_int,
    pub codec_whitelist: *mut libc::c_char,
    pub format_whitelist: *mut libc::c_char,
    pub internal: *mut AVFormatInternal,
    pub io_repositioned: libc::c_int,
    pub video_codec: *mut AVCodec,
    pub audio_codec: *mut AVCodec,
    pub subtitle_codec: *mut AVCodec,
    pub data_codec: *mut AVCodec,
    pub metadata_header_padding: libc::c_int,
    pub opaque: *mut libc::c_void,
    pub control_message_cb: av_format_control_message,
    pub output_ts_offset: int64_t,
    pub dump_separator: *mut uint8_t,
    pub data_codec_id: AVCodecID,
    pub open_cb: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut *mut AVIOContext,
            *const libc::c_char,
            libc::c_int,
            *const AVIOInterruptCB,
            *mut *mut AVDictionary,
        ) -> libc::c_int,
    >,
    pub protocol_whitelist: *mut libc::c_char,
    pub io_open: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut *mut AVIOContext,
            *const libc::c_char,
            libc::c_int,
            *mut *mut AVDictionary,
        ) -> libc::c_int,
    >,
    pub io_close: Option::<
        unsafe extern "C" fn(*mut AVFormatContext, *mut AVIOContext) -> (),
    >,
    pub protocol_blacklist: *mut libc::c_char,
    pub max_streams: libc::c_int,
    pub skip_estimate_duration_from_pts: libc::c_int,
}
pub type av_format_control_message = Option::<
    unsafe extern "C" fn(
        *mut AVFormatContext,
        libc::c_int,
        *mut libc::c_void,
        size_t,
    ) -> libc::c_int,
>;
pub type AVDurationEstimationMethod = libc::c_uint;
pub const AVFMT_DURATION_FROM_BITRATE: AVDurationEstimationMethod = 2;
pub const AVFMT_DURATION_FROM_STREAM: AVDurationEstimationMethod = 1;
pub const AVFMT_DURATION_FROM_PTS: AVDurationEstimationMethod = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVChapter {
    pub id: libc::c_int,
    pub time_base: AVRational,
    pub start: int64_t,
    pub end: int64_t,
    pub metadata: *mut AVDictionary,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProgram {
    pub id: libc::c_int,
    pub flags: libc::c_int,
    pub discard: AVDiscard,
    pub stream_index: *mut libc::c_uint,
    pub nb_stream_indexes: libc::c_uint,
    pub metadata: *mut AVDictionary,
    pub program_num: libc::c_int,
    pub pmt_pid: libc::c_int,
    pub pcr_pid: libc::c_int,
    pub pmt_version: libc::c_int,
    pub start_time: int64_t,
    pub end_time: int64_t,
    pub pts_wrap_reference: int64_t,
    pub pts_wrap_behavior: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVStream {
    pub index: libc::c_int,
    pub id: libc::c_int,
    pub codec: *mut AVCodecContext,
    pub priv_data: *mut libc::c_void,
    pub time_base: AVRational,
    pub start_time: int64_t,
    pub duration: int64_t,
    pub nb_frames: int64_t,
    pub disposition: libc::c_int,
    pub discard: AVDiscard,
    pub sample_aspect_ratio: AVRational,
    pub metadata: *mut AVDictionary,
    pub avg_frame_rate: AVRational,
    pub attached_pic: AVPacket,
    pub side_data: *mut AVPacketSideData,
    pub nb_side_data: libc::c_int,
    pub event_flags: libc::c_int,
    pub r_frame_rate: AVRational,
    pub recommended_encoder_configuration: *mut libc::c_char,
    pub codecpar: *mut AVCodecParameters,
    pub info: *mut C2RustUnnamed_4,
    pub pts_wrap_bits: libc::c_int,
    pub first_dts: int64_t,
    pub cur_dts: int64_t,
    pub last_IP_pts: int64_t,
    pub last_IP_duration: libc::c_int,
    pub probe_packets: libc::c_int,
    pub codec_info_nb_frames: libc::c_int,
    pub need_parsing: AVStreamParseType,
    pub parser: *mut AVCodecParserContext,
    pub last_in_packet_buffer: *mut AVPacketList,
    pub probe_data: AVProbeData,
    pub pts_buffer: [int64_t; 17],
    pub index_entries: *mut AVIndexEntry,
    pub nb_index_entries: libc::c_int,
    pub index_entries_allocated_size: libc::c_uint,
    pub stream_identifier: libc::c_int,
    pub program_num: libc::c_int,
    pub pmt_version: libc::c_int,
    pub pmt_stream_idx: libc::c_int,
    pub interleaver_chunk_size: int64_t,
    pub interleaver_chunk_duration: int64_t,
    pub request_probe: libc::c_int,
    pub skip_to_keyframe: libc::c_int,
    pub skip_samples: libc::c_int,
    pub start_skip_samples: int64_t,
    pub first_discard_sample: int64_t,
    pub last_discard_sample: int64_t,
    pub nb_decoded_frames: libc::c_int,
    pub mux_ts_offset: int64_t,
    pub pts_wrap_reference: int64_t,
    pub pts_wrap_behavior: libc::c_int,
    pub update_initial_durations_done: libc::c_int,
    pub pts_reorder_error: [int64_t; 17],
    pub pts_reorder_error_count: [uint8_t; 17],
    pub last_dts_for_order_check: int64_t,
    pub dts_ordered: uint8_t,
    pub dts_misordered: uint8_t,
    pub inject_global_side_data: libc::c_int,
    pub display_aspect_ratio: AVRational,
    pub internal: *mut AVStreamInternal,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct AVIndexEntry {
    pub pos: int64_t,
    pub timestamp: int64_t,
    #[bitfield(name = "flags", ty = "libc::c_int", bits = "0..=1")]
    #[bitfield(name = "size", ty = "libc::c_int", bits = "2..=31")]
    pub flags_size: [u8; 4],
    pub min_distance: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProbeData {
    pub filename: *const libc::c_char,
    pub buf: *mut libc::c_uchar,
    pub buf_size: libc::c_int,
    pub mime_type: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacketList {
    pub pkt: AVPacket,
    pub next: *mut AVPacketList,
}
pub type AVStreamParseType = libc::c_uint;
pub const AVSTREAM_PARSE_FULL_RAW: AVStreamParseType = 5;
pub const AVSTREAM_PARSE_FULL_ONCE: AVStreamParseType = 4;
pub const AVSTREAM_PARSE_TIMESTAMPS: AVStreamParseType = 3;
pub const AVSTREAM_PARSE_HEADERS: AVStreamParseType = 2;
pub const AVSTREAM_PARSE_FULL: AVStreamParseType = 1;
pub const AVSTREAM_PARSE_NONE: AVStreamParseType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub last_dts: int64_t,
    pub duration_gcd: int64_t,
    pub duration_count: libc::c_int,
    pub rfps_duration_sum: int64_t,
    pub duration_error: *mut [[libc::c_double; 399]; 2],
    pub codec_info_duration: int64_t,
    pub codec_info_duration_fields: int64_t,
    pub frame_delay_evidence: libc::c_int,
    pub found_decoder: libc::c_int,
    pub last_duration: int64_t,
    pub fps_first_dts: int64_t,
    pub fps_first_dts_idx: libc::c_int,
    pub fps_last_dts: int64_t,
    pub fps_last_dts_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOutputFormat {
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub mime_type: *const libc::c_char,
    pub extensions: *const libc::c_char,
    pub audio_codec: AVCodecID,
    pub video_codec: AVCodecID,
    pub subtitle_codec: AVCodecID,
    pub flags: libc::c_int,
    pub codec_tag: *const *const AVCodecTag,
    pub priv_class: *const AVClass,
    pub next: *mut AVOutputFormat,
    pub priv_data_size: libc::c_int,
    pub write_header: Option::<
        unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int,
    >,
    pub write_packet: Option::<
        unsafe extern "C" fn(*mut AVFormatContext, *mut AVPacket) -> libc::c_int,
    >,
    pub write_trailer: Option::<
        unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int,
    >,
    pub interleave_packet: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut AVPacket,
            *mut AVPacket,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub query_codec: Option::<
        unsafe extern "C" fn(AVCodecID, libc::c_int) -> libc::c_int,
    >,
    pub get_output_timestamp: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            *mut int64_t,
            *mut int64_t,
        ) -> (),
    >,
    pub control_message: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            *mut libc::c_void,
            size_t,
        ) -> libc::c_int,
    >,
    pub write_uncoded_frame: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            *mut *mut AVFrame,
            libc::c_uint,
        ) -> libc::c_int,
    >,
    pub get_device_list: Option::<
        unsafe extern "C" fn(*mut AVFormatContext, *mut AVDeviceInfoList) -> libc::c_int,
    >,
    pub create_device_capabilities: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut AVDeviceCapabilitiesQuery,
        ) -> libc::c_int,
    >,
    pub free_device_capabilities: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut AVDeviceCapabilitiesQuery,
        ) -> libc::c_int,
    >,
    pub data_codec: AVCodecID,
    pub init: Option::<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub deinit: Option::<unsafe extern "C" fn(*mut AVFormatContext) -> ()>,
    pub check_bitstream: Option::<
        unsafe extern "C" fn(*mut AVFormatContext, *const AVPacket) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVInputFormat {
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub flags: libc::c_int,
    pub extensions: *const libc::c_char,
    pub codec_tag: *const *const AVCodecTag,
    pub priv_class: *const AVClass,
    pub mime_type: *const libc::c_char,
    pub next: *mut AVInputFormat,
    pub raw_codec_id: libc::c_int,
    pub priv_data_size: libc::c_int,
    pub read_probe: Option::<unsafe extern "C" fn(*const AVProbeData) -> libc::c_int>,
    pub read_header: Option::<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub read_packet: Option::<
        unsafe extern "C" fn(*mut AVFormatContext, *mut AVPacket) -> libc::c_int,
    >,
    pub read_close: Option::<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub read_seek: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            int64_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub read_timestamp: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            *mut int64_t,
            int64_t,
        ) -> int64_t,
    >,
    pub read_play: Option::<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub read_pause: Option::<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub read_seek2: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            int64_t,
            int64_t,
            int64_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub get_device_list: Option::<
        unsafe extern "C" fn(*mut AVFormatContext, *mut AVDeviceInfoList) -> libc::c_int,
    >,
    pub create_device_capabilities: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut AVDeviceCapabilitiesQuery,
        ) -> libc::c_int,
    >,
    pub free_device_capabilities: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut AVDeviceCapabilitiesQuery,
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilterContext {
    pub av_class: *const AVClass,
    pub filter: *const AVFilter,
    pub name: *mut libc::c_char,
    pub input_pads: *mut AVFilterPad,
    pub inputs: *mut *mut AVFilterLink,
    pub nb_inputs: libc::c_uint,
    pub output_pads: *mut AVFilterPad,
    pub outputs: *mut *mut AVFilterLink,
    pub nb_outputs: libc::c_uint,
    pub priv_0: *mut libc::c_void,
    pub graph: *mut AVFilterGraph,
    pub thread_type: libc::c_int,
    pub internal: *mut AVFilterInternal,
    pub command_queue: *mut AVFilterCommand,
    pub enable_str: *mut libc::c_char,
    pub enable: *mut libc::c_void,
    pub var_values: *mut libc::c_double,
    pub is_disabled: libc::c_int,
    pub hw_device_ctx: *mut AVBufferRef,
    pub nb_threads: libc::c_int,
    pub ready: libc::c_uint,
    pub extra_hw_frames: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilterGraph {
    pub av_class: *const AVClass,
    pub filters: *mut *mut AVFilterContext,
    pub nb_filters: libc::c_uint,
    pub scale_sws_opts: *mut libc::c_char,
    pub resample_lavr_opts: *mut libc::c_char,
    pub thread_type: libc::c_int,
    pub nb_threads: libc::c_int,
    pub internal: *mut AVFilterGraphInternal,
    pub opaque: *mut libc::c_void,
    pub execute: Option::<avfilter_execute_func>,
    pub aresample_swr_opts: *mut libc::c_char,
    pub sink_links: *mut *mut AVFilterLink,
    pub sink_links_count: libc::c_int,
    pub disable_auto_convert: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilterLink {
    pub src: *mut AVFilterContext,
    pub srcpad: *mut AVFilterPad,
    pub dst: *mut AVFilterContext,
    pub dstpad: *mut AVFilterPad,
    pub type_0: AVMediaType,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub sample_aspect_ratio: AVRational,
    pub channel_layout: uint64_t,
    pub sample_rate: libc::c_int,
    pub format: libc::c_int,
    pub time_base: AVRational,
    pub in_formats: *mut AVFilterFormats,
    pub out_formats: *mut AVFilterFormats,
    pub in_samplerates: *mut AVFilterFormats,
    pub out_samplerates: *mut AVFilterFormats,
    pub in_channel_layouts: *mut AVFilterChannelLayouts,
    pub out_channel_layouts: *mut AVFilterChannelLayouts,
    pub request_samples: libc::c_int,
    pub init_state: C2RustUnnamed_5,
    pub graph: *mut AVFilterGraph,
    pub current_pts: int64_t,
    pub current_pts_us: int64_t,
    pub age_index: libc::c_int,
    pub frame_rate: AVRational,
    pub partial_buf: *mut AVFrame,
    pub partial_buf_size: libc::c_int,
    pub min_samples: libc::c_int,
    pub max_samples: libc::c_int,
    pub channels: libc::c_int,
    pub flags: libc::c_uint,
    pub frame_count_in: int64_t,
    pub frame_count_out: int64_t,
    pub frame_pool: *mut libc::c_void,
    pub frame_wanted_out: libc::c_int,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub reserved: [libc::c_char; 61440],
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const AVLINK_INIT: C2RustUnnamed_5 = 2;
pub const AVLINK_STARTINIT: C2RustUnnamed_5 = 1;
pub const AVLINK_UNINIT: C2RustUnnamed_5 = 0;
pub type avfilter_execute_func = unsafe extern "C" fn(
    *mut AVFilterContext,
    Option::<avfilter_action_func>,
    *mut libc::c_void,
    *mut libc::c_int,
    libc::c_int,
) -> libc::c_int;
pub type avfilter_action_func = unsafe extern "C" fn(
    *mut AVFilterContext,
    *mut libc::c_void,
    libc::c_int,
    libc::c_int,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilter {
    pub name: *const libc::c_char,
    pub description: *const libc::c_char,
    pub inputs: *const AVFilterPad,
    pub outputs: *const AVFilterPad,
    pub priv_class: *const AVClass,
    pub flags: libc::c_int,
    pub preinit: Option::<unsafe extern "C" fn(*mut AVFilterContext) -> libc::c_int>,
    pub init: Option::<unsafe extern "C" fn(*mut AVFilterContext) -> libc::c_int>,
    pub init_dict: Option::<
        unsafe extern "C" fn(*mut AVFilterContext, *mut *mut AVDictionary) -> libc::c_int,
    >,
    pub uninit: Option::<unsafe extern "C" fn(*mut AVFilterContext) -> ()>,
    pub query_formats: Option::<
        unsafe extern "C" fn(*mut AVFilterContext) -> libc::c_int,
    >,
    pub priv_size: libc::c_int,
    pub flags_internal: libc::c_int,
    pub next: *mut AVFilter,
    pub process_command: Option::<
        unsafe extern "C" fn(
            *mut AVFilterContext,
            *const libc::c_char,
            *const libc::c_char,
            *mut libc::c_char,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub init_opaque: Option::<
        unsafe extern "C" fn(*mut AVFilterContext, *mut libc::c_void) -> libc::c_int,
    >,
    pub activate: Option::<unsafe extern "C" fn(*mut AVFilterContext) -> libc::c_int>,
}
pub type BarPlayerMode = libc::c_uint;
pub const PLAYER_FINISHED: BarPlayerMode = 3;
pub const PLAYER_PLAYING: BarPlayerMode = 2;
pub const PLAYER_WAITING: BarPlayerMode = 1;
pub const PLAYER_DEAD: BarPlayerMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_t {
    pub lock: pthread_mutex_t,
    pub aoplayLock: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub aoplayCond: pthread_cond_t,
    pub doQuit: bool,
    pub doPause: bool,
    pub songDuration: libc::c_uint,
    pub songPlayed: libc::c_uint,
    pub mode: BarPlayerMode,
    pub fvolume: *mut AVFilterContext,
    pub fgraph: *mut AVFilterGraph,
    pub fctx: *mut AVFormatContext,
    pub st: *mut AVStream,
    pub cctx: *mut AVCodecContext,
    pub fbufsink: *mut AVFilterContext,
    pub fabuf: *mut AVFilterContext,
    pub streamIdx: libc::c_int,
    pub lastTimestamp: int64_t,
    pub interrupted: sig_atomic_t,
    pub aoDev: *mut ao_device,
    pub gain: libc::c_double,
    pub url: *mut libc::c_char,
    pub settings: *const BarSettings_t,
}
pub type CURL = ();
pub type CURLcode = libc::c_uint;
pub const CURL_LAST: CURLcode = 96;
pub const CURLE_HTTP3: CURLcode = 95;
pub const CURLE_AUTH_ERROR: CURLcode = 94;
pub const CURLE_RECURSIVE_API_CALL: CURLcode = 93;
pub const CURLE_HTTP2_STREAM: CURLcode = 92;
pub const CURLE_SSL_INVALIDCERTSTATUS: CURLcode = 91;
pub const CURLE_SSL_PINNEDPUBKEYNOTMATCH: CURLcode = 90;
pub const CURLE_NO_CONNECTION_AVAILABLE: CURLcode = 89;
pub const CURLE_CHUNK_FAILED: CURLcode = 88;
pub const CURLE_FTP_BAD_FILE_LIST: CURLcode = 87;
pub const CURLE_RTSP_SESSION_ERROR: CURLcode = 86;
pub const CURLE_RTSP_CSEQ_ERROR: CURLcode = 85;
pub const CURLE_FTP_PRET_FAILED: CURLcode = 84;
pub const CURLE_SSL_ISSUER_ERROR: CURLcode = 83;
pub const CURLE_SSL_CRL_BADFILE: CURLcode = 82;
pub const CURLE_AGAIN: CURLcode = 81;
pub const CURLE_SSL_SHUTDOWN_FAILED: CURLcode = 80;
pub const CURLE_SSH: CURLcode = 79;
pub const CURLE_REMOTE_FILE_NOT_FOUND: CURLcode = 78;
pub const CURLE_SSL_CACERT_BADFILE: CURLcode = 77;
pub const CURLE_CONV_REQD: CURLcode = 76;
pub const CURLE_CONV_FAILED: CURLcode = 75;
pub const CURLE_TFTP_NOSUCHUSER: CURLcode = 74;
pub const CURLE_REMOTE_FILE_EXISTS: CURLcode = 73;
pub const CURLE_TFTP_UNKNOWNID: CURLcode = 72;
pub const CURLE_TFTP_ILLEGAL: CURLcode = 71;
pub const CURLE_REMOTE_DISK_FULL: CURLcode = 70;
pub const CURLE_TFTP_PERM: CURLcode = 69;
pub const CURLE_TFTP_NOTFOUND: CURLcode = 68;
pub const CURLE_LOGIN_DENIED: CURLcode = 67;
pub const CURLE_SSL_ENGINE_INITFAILED: CURLcode = 66;
pub const CURLE_SEND_FAIL_REWIND: CURLcode = 65;
pub const CURLE_USE_SSL_FAILED: CURLcode = 64;
pub const CURLE_FILESIZE_EXCEEDED: CURLcode = 63;
pub const CURLE_LDAP_INVALID_URL: CURLcode = 62;
pub const CURLE_BAD_CONTENT_ENCODING: CURLcode = 61;
pub const CURLE_PEER_FAILED_VERIFICATION: CURLcode = 60;
pub const CURLE_SSL_CIPHER: CURLcode = 59;
pub const CURLE_SSL_CERTPROBLEM: CURLcode = 58;
pub const CURLE_OBSOLETE57: CURLcode = 57;
pub const CURLE_RECV_ERROR: CURLcode = 56;
pub const CURLE_SEND_ERROR: CURLcode = 55;
pub const CURLE_SSL_ENGINE_SETFAILED: CURLcode = 54;
pub const CURLE_SSL_ENGINE_NOTFOUND: CURLcode = 53;
pub const CURLE_GOT_NOTHING: CURLcode = 52;
pub const CURLE_OBSOLETE51: CURLcode = 51;
pub const CURLE_OBSOLETE50: CURLcode = 50;
pub const CURLE_TELNET_OPTION_SYNTAX: CURLcode = 49;
pub const CURLE_UNKNOWN_OPTION: CURLcode = 48;
pub const CURLE_TOO_MANY_REDIRECTS: CURLcode = 47;
pub const CURLE_OBSOLETE46: CURLcode = 46;
pub const CURLE_INTERFACE_FAILED: CURLcode = 45;
pub const CURLE_OBSOLETE44: CURLcode = 44;
pub const CURLE_BAD_FUNCTION_ARGUMENT: CURLcode = 43;
pub const CURLE_ABORTED_BY_CALLBACK: CURLcode = 42;
pub const CURLE_FUNCTION_NOT_FOUND: CURLcode = 41;
pub const CURLE_OBSOLETE40: CURLcode = 40;
pub const CURLE_LDAP_SEARCH_FAILED: CURLcode = 39;
pub const CURLE_LDAP_CANNOT_BIND: CURLcode = 38;
pub const CURLE_FILE_COULDNT_READ_FILE: CURLcode = 37;
pub const CURLE_BAD_DOWNLOAD_RESUME: CURLcode = 36;
pub const CURLE_SSL_CONNECT_ERROR: CURLcode = 35;
pub const CURLE_HTTP_POST_ERROR: CURLcode = 34;
pub const CURLE_RANGE_ERROR: CURLcode = 33;
pub const CURLE_OBSOLETE32: CURLcode = 32;
pub const CURLE_FTP_COULDNT_USE_REST: CURLcode = 31;
pub const CURLE_FTP_PORT_FAILED: CURLcode = 30;
pub const CURLE_OBSOLETE29: CURLcode = 29;
pub const CURLE_OPERATION_TIMEDOUT: CURLcode = 28;
pub const CURLE_OUT_OF_MEMORY: CURLcode = 27;
pub const CURLE_READ_ERROR: CURLcode = 26;
pub const CURLE_UPLOAD_FAILED: CURLcode = 25;
pub const CURLE_OBSOLETE24: CURLcode = 24;
pub const CURLE_WRITE_ERROR: CURLcode = 23;
pub const CURLE_HTTP_RETURNED_ERROR: CURLcode = 22;
pub const CURLE_QUOTE_ERROR: CURLcode = 21;
pub const CURLE_OBSOLETE20: CURLcode = 20;
pub const CURLE_FTP_COULDNT_RETR_FILE: CURLcode = 19;
pub const CURLE_PARTIAL_FILE: CURLcode = 18;
pub const CURLE_FTP_COULDNT_SET_TYPE: CURLcode = 17;
pub const CURLE_HTTP2: CURLcode = 16;
pub const CURLE_FTP_CANT_GET_HOST: CURLcode = 15;
pub const CURLE_FTP_WEIRD_227_FORMAT: CURLcode = 14;
pub const CURLE_FTP_WEIRD_PASV_REPLY: CURLcode = 13;
pub const CURLE_FTP_ACCEPT_TIMEOUT: CURLcode = 12;
pub const CURLE_FTP_WEIRD_PASS_REPLY: CURLcode = 11;
pub const CURLE_FTP_ACCEPT_FAILED: CURLcode = 10;
pub const CURLE_REMOTE_ACCESS_DENIED: CURLcode = 9;
pub const CURLE_WEIRD_SERVER_REPLY: CURLcode = 8;
pub const CURLE_COULDNT_CONNECT: CURLcode = 7;
pub const CURLE_COULDNT_RESOLVE_HOST: CURLcode = 6;
pub const CURLE_COULDNT_RESOLVE_PROXY: CURLcode = 5;
pub const CURLE_NOT_BUILT_IN: CURLcode = 4;
pub const CURLE_URL_MALFORMAT: CURLcode = 3;
pub const CURLE_FAILED_INIT: CURLcode = 2;
pub const CURLE_UNSUPPORTED_PROTOCOL: CURLcode = 1;
pub const CURLE_OK: CURLcode = 0;
pub type BarReadlineFlags_t = libc::c_uint;
pub const BAR_RL_NOINT: BarReadlineFlags_t = 4;
pub const BAR_RL_NOECHO: BarReadlineFlags_t = 2;
pub const BAR_RL_FULLRETURN: BarReadlineFlags_t = 1;
pub const BAR_RL_DEFAULT: BarReadlineFlags_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BarReadlineFds_t {
    pub set: fd_set,
    pub maxfd: libc::c_int,
    pub fds: [libc::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BarApp_t {
    pub ph: PianoHandle_t,
    pub http: *mut libc::c_void,
    pub player: player_t,
    pub settings: BarSettings_t,
    pub playlist: *mut PianoSong_t,
    pub songHistory: *mut PianoSong_t,
    pub curStation: *mut PianoStation_t,
    pub nextStation: *mut PianoStation_t,
    pub doQuit: sig_atomic_t,
    pub input: BarReadlineFds_t,
    pub playerErrors: libc::c_uint,
}
pub type BarUiSelectStationCallback_t = Option::<
    unsafe extern "C" fn(*mut BarApp_t, *mut libc::c_char) -> (),
>;
pub type BarUiDispatchContext_t = libc::c_uint;
pub const BAR_DC_SONG: BarUiDispatchContext_t = 4;
pub const BAR_DC_STATION: BarUiDispatchContext_t = 2;
pub const BAR_DC_GLOBAL: BarUiDispatchContext_t = 1;
pub const BAR_DC_UNDEFINED: BarUiDispatchContext_t = 0;
pub type BarKeyShortcutFunc_t = Option::<
    unsafe extern "C" fn(
        *mut BarApp_t,
        *mut PianoStation_t,
        *mut PianoSong_t,
        BarUiDispatchContext_t,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BarUiDispatchAction_t {
    pub defaultKey: libc::c_char,
    pub context: BarUiDispatchContext_t,
    pub function: BarKeyShortcutFunc_t,
    pub helpText: *const libc::c_char,
    pub configKey: *const libc::c_char,
}
static mut dispatchActions: [BarUiDispatchAction_t; 30] = unsafe {
    [
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: '?' as i32 as libc::c_char,
                context: BAR_DC_UNDEFINED,
                function: Some(
                    BarUiActHelp
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: 0 as *const libc::c_char,
                configKey: b"act_help\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: '+' as i32 as libc::c_char,
                context: BAR_DC_SONG,
                function: Some(
                    BarUiActLoveSong
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"love song\0" as *const u8 as *const libc::c_char,
                configKey: b"act_songlove\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: '-' as i32 as libc::c_char,
                context: BAR_DC_SONG,
                function: Some(
                    BarUiActBanSong
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"ban song\0" as *const u8 as *const libc::c_char,
                configKey: b"act_songban\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: 'a' as i32 as libc::c_char,
                context: BAR_DC_STATION,
                function: Some(
                    BarUiActAddMusic
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"add music to station\0" as *const u8 as *const libc::c_char,
                configKey: b"act_stationaddmusic\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: 'c' as i32 as libc::c_char,
                context: BAR_DC_GLOBAL,
                function: Some(
                    BarUiActCreateStation
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"create new station\0" as *const u8 as *const libc::c_char,
                configKey: b"act_stationcreate\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: 'd' as i32 as libc::c_char,
                context: BAR_DC_STATION,
                function: Some(
                    BarUiActDeleteStation
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"delete station\0" as *const u8 as *const libc::c_char,
                configKey: b"act_stationdelete\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: 'e' as i32 as libc::c_char,
                context: BAR_DC_SONG,
                function: Some(
                    BarUiActExplain
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"explain why this song is played\0" as *const u8
                    as *const libc::c_char,
                configKey: b"act_songexplain\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: 'g' as i32 as libc::c_char,
                context: BAR_DC_GLOBAL,
                function: Some(
                    BarUiActStationFromGenre
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"add genre station\0" as *const u8 as *const libc::c_char,
                configKey: b"act_stationaddbygenre\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: 'h' as i32 as libc::c_char,
                context: BAR_DC_GLOBAL,
                function: Some(
                    BarUiActHistory
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"song history\0" as *const u8 as *const libc::c_char,
                configKey: b"act_history\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: 'i' as i32 as libc::c_char,
                context: (BAR_DC_GLOBAL as libc::c_int | BAR_DC_STATION as libc::c_int
                    | BAR_DC_SONG as libc::c_int) as BarUiDispatchContext_t,
                function: Some(
                    BarUiActSongInfo
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"print information about song/station\0" as *const u8
                    as *const libc::c_char,
                configKey: b"act_songinfo\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: 'j' as i32 as libc::c_char,
                context: BAR_DC_GLOBAL,
                function: Some(
                    BarUiActAddSharedStation
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"add shared station\0" as *const u8 as *const libc::c_char,
                configKey: b"act_addshared\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: 'n' as i32 as libc::c_char,
                context: (BAR_DC_GLOBAL as libc::c_int | BAR_DC_STATION as libc::c_int)
                    as BarUiDispatchContext_t,
                function: Some(
                    BarUiActSkipSong
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"next song\0" as *const u8 as *const libc::c_char,
                configKey: b"act_songnext\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: 'p' as i32 as libc::c_char,
                context: (BAR_DC_GLOBAL as libc::c_int | BAR_DC_STATION as libc::c_int)
                    as BarUiDispatchContext_t,
                function: Some(
                    BarUiActTogglePause
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"pause/resume playback\0" as *const u8 as *const libc::c_char,
                configKey: b"act_songpausetoggle\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: 'q' as i32 as libc::c_char,
                context: BAR_DC_GLOBAL,
                function: Some(
                    BarUiActQuit
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"quit\0" as *const u8 as *const libc::c_char,
                configKey: b"act_quit\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: 'r' as i32 as libc::c_char,
                context: BAR_DC_STATION,
                function: Some(
                    BarUiActRenameStation
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"rename station\0" as *const u8 as *const libc::c_char,
                configKey: b"act_stationrename\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: 's' as i32 as libc::c_char,
                context: BAR_DC_GLOBAL,
                function: Some(
                    BarUiActSelectStation
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"change station\0" as *const u8 as *const libc::c_char,
                configKey: b"act_stationchange\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: 't' as i32 as libc::c_char,
                context: BAR_DC_SONG,
                function: Some(
                    BarUiActTempBanSong
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"tired (ban song for 1 month)\0" as *const u8
                    as *const libc::c_char,
                configKey: b"act_songtired\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: 'u' as i32 as libc::c_char,
                context: (BAR_DC_GLOBAL as libc::c_int | BAR_DC_STATION as libc::c_int)
                    as BarUiDispatchContext_t,
                function: Some(
                    BarUiActPrintUpcoming
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"upcoming songs\0" as *const u8 as *const libc::c_char,
                configKey: b"act_upcoming\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: 'x' as i32 as libc::c_char,
                context: BAR_DC_STATION,
                function: Some(
                    BarUiActSelectQuickMix
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"select quickmix stations\0" as *const u8
                    as *const libc::c_char,
                configKey: b"act_stationselectquickmix\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: '$' as i32 as libc::c_char,
                context: BAR_DC_SONG,
                function: Some(
                    BarUiActDebug
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: 0 as *const libc::c_char,
                configKey: b"act_debug\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: 'b' as i32 as libc::c_char,
                context: BAR_DC_SONG,
                function: Some(
                    BarUiActBookmark
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"bookmark song/artist\0" as *const u8 as *const libc::c_char,
                configKey: b"act_bookmark\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: '(' as i32 as libc::c_char,
                context: BAR_DC_GLOBAL,
                function: Some(
                    BarUiActVolDown
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"decrease volume\0" as *const u8 as *const libc::c_char,
                configKey: b"act_voldown\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: ')' as i32 as libc::c_char,
                context: BAR_DC_GLOBAL,
                function: Some(
                    BarUiActVolUp
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"increase volume\0" as *const u8 as *const libc::c_char,
                configKey: b"act_volup\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: '=' as i32 as libc::c_char,
                context: BAR_DC_STATION,
                function: Some(
                    BarUiActManageStation
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"manage station seeds/feedback/mode\0" as *const u8
                    as *const libc::c_char,
                configKey: b"act_managestation\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: ' ' as i32 as libc::c_char,
                context: (BAR_DC_GLOBAL as libc::c_int | BAR_DC_STATION as libc::c_int)
                    as BarUiDispatchContext_t,
                function: Some(
                    BarUiActTogglePause
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: 0 as *const libc::c_char,
                configKey: b"act_songpausetoggle2\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: 'v' as i32 as libc::c_char,
                context: BAR_DC_SONG,
                function: Some(
                    BarUiActCreateStationFromSong
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"create new station from song or artist\0" as *const u8
                    as *const libc::c_char,
                configKey: b"act_stationcreatefromsong\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: 'P' as i32 as libc::c_char,
                context: (BAR_DC_GLOBAL as libc::c_int | BAR_DC_STATION as libc::c_int)
                    as BarUiDispatchContext_t,
                function: Some(
                    BarUiActPlay
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"resume playback\0" as *const u8 as *const libc::c_char,
                configKey: b"act_songplay\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: 'S' as i32 as libc::c_char,
                context: (BAR_DC_GLOBAL as libc::c_int | BAR_DC_STATION as libc::c_int)
                    as BarUiDispatchContext_t,
                function: Some(
                    BarUiActPause
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"pause playback\0" as *const u8 as *const libc::c_char,
                configKey: b"act_songpause\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: '^' as i32 as libc::c_char,
                context: BAR_DC_GLOBAL,
                function: Some(
                    BarUiActVolReset
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"reset volume\0" as *const u8 as *const libc::c_char,
                configKey: b"act_volreset\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = BarUiDispatchAction_t {
                defaultKey: '!' as i32 as libc::c_char,
                context: BAR_DC_GLOBAL,
                function: Some(
                    BarUiActSettings
                        as unsafe extern "C" fn(
                            *mut BarApp_t,
                            *mut PianoStation_t,
                            *mut PianoSong_t,
                            BarUiDispatchContext_t,
                        ) -> (),
                ),
                helpText: b"change settings\0" as *const u8 as *const libc::c_char,
                configKey: b"act_settings\0" as *const u8 as *const libc::c_char,
            };
            init
        },
    ]
};
#[inline]
unsafe extern "C" fn BarUiDoSkipSong(player: *mut player_t) {
    pthread_mutex_lock(&mut (*player).lock);
    (*player).doQuit = 1 as libc::c_int != 0;
    (*player).doPause = 0 as libc::c_int != 0;
    pthread_cond_broadcast(&mut (*player).cond);
    pthread_mutex_unlock(&mut (*player).lock);
    pthread_mutex_lock(&mut (*player).aoplayLock);
    pthread_cond_broadcast(&mut (*player).aoplayCond);
    pthread_mutex_unlock(&mut (*player).aoplayLock);
}
unsafe extern "C" fn BarTransformIfShared(
    mut app: *mut BarApp_t,
    mut station: *mut PianoStation_t,
) -> libc::c_int {
    let mut pRet: PianoReturn_t = PIANO_RET_ERR;
    let mut wRet: CURLcode = CURLE_OK;
    if (*station).isCreator == 0 {
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_INFO,
            b"Transforming station... \0" as *const u8 as *const libc::c_char,
        );
        if !BarUiPianoCall(
            app,
            PIANO_REQUEST_TRANSFORM_STATION,
            station as *mut libc::c_void,
            &mut pRet,
            &mut wRet,
        ) {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn BarUiActHelp(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_NONE,
        b"\r\0" as *const u8 as *const libc::c_char,
    );
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < BAR_KS_COUNT as libc::c_int as libc::c_ulong {
        if !(dispatchActions[i as usize].helpText).is_null()
            && context as libc::c_uint
                & dispatchActions[i as usize].context as libc::c_uint
                == dispatchActions[i as usize].context as libc::c_uint
            && (*app).settings.keys[i as usize] as libc::c_int != '\0' as i32
        {
            BarUiMsg(
                &mut (*app).settings as *mut BarSettings_t,
                MSG_LIST,
                b"%c    %s\n\0" as *const u8 as *const libc::c_char,
                (*app).settings.keys[i as usize] as libc::c_int,
                dispatchActions[i as usize].helpText,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn BarUiActAddMusic(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    let mut pRet: PianoReturn_t = PIANO_RET_ERR;
    let mut wRet: CURLcode = CURLE_OK;
    let mut reqData: PianoRequestDataAddSeed_t = PianoRequestDataAddSeed_t {
        station: 0 as *mut PianoStation_t,
        musicId: 0 as *mut libc::c_char,
    };
    reqData
        .musicId = BarUiSelectMusicId(
        app,
        selStation,
        b"Add artist or title to station: \0" as *const u8 as *const libc::c_char,
    );
    if !(reqData.musicId).is_null() {
        if BarTransformIfShared(app, selStation) == 0 {
            return;
        }
        reqData.station = selStation;
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_INFO,
            b"Adding music to station... \0" as *const u8 as *const libc::c_char,
        );
        BarUiPianoCall(
            app,
            PIANO_REQUEST_ADD_SEED,
            &mut reqData as *mut PianoRequestDataAddSeed_t as *mut libc::c_void,
            &mut pRet,
            &mut wRet,
        );
        free(reqData.musicId as *mut libc::c_void);
        BarUiStartEventCmd(
            &mut (*app).settings,
            b"stationaddmusic\0" as *const u8 as *const libc::c_char,
            selStation,
            selSong,
            &mut (*app).player,
            (*app).ph.stations,
            pRet,
            wRet,
        );
    }
}
pub unsafe extern "C" fn BarUiActBanSong(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    let mut pRet: PianoReturn_t = PIANO_RET_ERR;
    let mut wRet: CURLcode = CURLE_OK;
    let mut realStation: *mut PianoStation_t = 0 as *mut PianoStation_t;
    realStation = PianoFindStationById((*app).ph.stations, (*selSong).stationId);
    if realStation.is_null() {
        return;
    }
    if BarTransformIfShared(app, realStation) == 0 {
        return;
    }
    let mut reqData: PianoRequestDataRateSong_t = PianoRequestDataRateSong_t {
        song: 0 as *mut PianoSong_t,
        rating: PIANO_RATE_NONE,
    };
    reqData.song = selSong;
    reqData.rating = PIANO_RATE_BAN;
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_INFO,
        b"Banning song... \0" as *const u8 as *const libc::c_char,
    );
    if BarUiPianoCall(
        app,
        PIANO_REQUEST_RATE_SONG,
        &mut reqData as *mut PianoRequestDataRateSong_t as *mut libc::c_void,
        &mut pRet,
        &mut wRet,
    ) as libc::c_int != 0 && selSong == (*app).playlist
    {
        BarUiDoSkipSong(&mut (*app).player);
    }
    BarUiStartEventCmd(
        &mut (*app).settings,
        b"songban\0" as *const u8 as *const libc::c_char,
        selStation,
        selSong,
        &mut (*app).player,
        (*app).ph.stations,
        pRet,
        wRet,
    );
}
pub unsafe extern "C" fn BarUiActCreateStation(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    let mut pRet: PianoReturn_t = PIANO_RET_ERR;
    let mut wRet: CURLcode = CURLE_OK;
    let mut reqData: PianoRequestDataCreateStation_t = PianoRequestDataCreateStation_t {
        token: 0 as *mut libc::c_char,
        type_0: PIANO_MUSICTYPE_INVALID,
    };
    reqData.type_0 = PIANO_MUSICTYPE_INVALID;
    reqData
        .token = BarUiSelectMusicId(
        app,
        0 as *mut PianoStation_t,
        b"Create station from artist or title: \0" as *const u8 as *const libc::c_char,
    );
    if !(reqData.token).is_null() {
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_INFO,
            b"Creating station... \0" as *const u8 as *const libc::c_char,
        );
        BarUiPianoCall(
            app,
            PIANO_REQUEST_CREATE_STATION,
            &mut reqData as *mut PianoRequestDataCreateStation_t as *mut libc::c_void,
            &mut pRet,
            &mut wRet,
        );
        free(reqData.token as *mut libc::c_void);
        BarUiStartEventCmd(
            &mut (*app).settings,
            b"stationcreate\0" as *const u8 as *const libc::c_char,
            selStation,
            selSong,
            &mut (*app).player,
            (*app).ph.stations,
            pRet,
            wRet,
        );
    }
}
pub unsafe extern "C" fn BarUiActCreateStationFromSong(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    let mut pRet: PianoReturn_t = PIANO_RET_ERR;
    let mut wRet: CURLcode = CURLE_OK;
    let mut reqData: PianoRequestDataCreateStation_t = PianoRequestDataCreateStation_t {
        token: 0 as *mut libc::c_char,
        type_0: PIANO_MUSICTYPE_INVALID,
    };
    let mut selectBuf: [libc::c_char; 2] = [0; 2];
    reqData.token = (*selSong).trackToken;
    reqData.type_0 = PIANO_MUSICTYPE_INVALID;
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_QUESTION,
        b"Create station from [s]ong or [a]rtist? \0" as *const u8 as *const libc::c_char,
    );
    BarReadline(
        selectBuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
        b"sa\0" as *const u8 as *const libc::c_char,
        &mut (*app).input,
        BAR_RL_FULLRETURN,
        -(1 as libc::c_int),
    );
    match selectBuf[0 as libc::c_int as usize] as libc::c_int {
        115 => {
            reqData.type_0 = PIANO_MUSICTYPE_SONG;
        }
        97 => {
            reqData.type_0 = PIANO_MUSICTYPE_ARTIST;
        }
        _ => {}
    }
    if reqData.type_0 as libc::c_uint
        != PIANO_MUSICTYPE_INVALID as libc::c_int as libc::c_uint
    {
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_INFO,
            b"Creating station... \0" as *const u8 as *const libc::c_char,
        );
        BarUiPianoCall(
            app,
            PIANO_REQUEST_CREATE_STATION,
            &mut reqData as *mut PianoRequestDataCreateStation_t as *mut libc::c_void,
            &mut pRet,
            &mut wRet,
        );
        BarUiStartEventCmd(
            &mut (*app).settings,
            b"stationcreate\0" as *const u8 as *const libc::c_char,
            selStation,
            selSong,
            &mut (*app).player,
            (*app).ph.stations,
            pRet,
            wRet,
        );
    }
}
pub unsafe extern "C" fn BarUiActAddSharedStation(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    let mut pRet: PianoReturn_t = PIANO_RET_ERR;
    let mut wRet: CURLcode = CURLE_OK;
    let mut stationId: [libc::c_char; 50] = [0; 50];
    let mut reqData: PianoRequestDataCreateStation_t = PianoRequestDataCreateStation_t {
        token: 0 as *mut libc::c_char,
        type_0: PIANO_MUSICTYPE_INVALID,
    };
    reqData.token = stationId.as_mut_ptr();
    reqData.type_0 = PIANO_MUSICTYPE_INVALID;
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_QUESTION,
        b"Station id: \0" as *const u8 as *const libc::c_char,
    );
    if BarReadline(
        stationId.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong,
        b"0123456789\0" as *const u8 as *const libc::c_char,
        &mut (*app).input,
        BAR_RL_DEFAULT,
        -(1 as libc::c_int),
    ) > 0 as libc::c_int as libc::c_ulong
    {
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_INFO,
            b"Adding shared station... \0" as *const u8 as *const libc::c_char,
        );
        BarUiPianoCall(
            app,
            PIANO_REQUEST_CREATE_STATION,
            &mut reqData as *mut PianoRequestDataCreateStation_t as *mut libc::c_void,
            &mut pRet,
            &mut wRet,
        );
        BarUiStartEventCmd(
            &mut (*app).settings,
            b"stationaddshared\0" as *const u8 as *const libc::c_char,
            selStation,
            selSong,
            &mut (*app).player,
            (*app).ph.stations,
            pRet,
            wRet,
        );
    }
}
unsafe extern "C" fn drainPlaylist(app: *mut BarApp_t) {
    BarUiDoSkipSong(&mut (*app).player);
    if !((*app).playlist).is_null() {
        PianoDestroyPlaylist(
            (if ((*app).playlist).is_null() {
                0 as *mut libc::c_void
            } else {
                (*(*app).playlist).head.next as *mut libc::c_void
            }) as *mut PianoSong_t,
        );
        (*(*app).playlist).head.next = 0 as *mut PianoListHead;
    }
}
pub unsafe extern "C" fn BarUiActDeleteStation(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    let mut pRet: PianoReturn_t = PIANO_RET_ERR;
    let mut wRet: CURLcode = CURLE_OK;
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_QUESTION,
        b"Really delete \"%s\"? [yN] \0" as *const u8 as *const libc::c_char,
        (*selStation).name,
    );
    if BarReadlineYesNo(0 as libc::c_int != 0, &mut (*app).input) {
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_INFO,
            b"Deleting station... \0" as *const u8 as *const libc::c_char,
        );
        if BarUiPianoCall(
            app,
            PIANO_REQUEST_DELETE_STATION,
            selStation as *mut libc::c_void,
            &mut pRet,
            &mut wRet,
        ) as libc::c_int != 0 && selStation == (*app).curStation
        {
            drainPlaylist(app);
            (*app).nextStation = 0 as *mut PianoStation_t;
            (*app).curStation = 0 as *mut PianoStation_t;
            selStation = 0 as *mut PianoStation_t;
        }
        BarUiStartEventCmd(
            &mut (*app).settings,
            b"stationdelete\0" as *const u8 as *const libc::c_char,
            selStation,
            selSong,
            &mut (*app).player,
            (*app).ph.stations,
            pRet,
            wRet,
        );
    }
}
pub unsafe extern "C" fn BarUiActExplain(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    let mut pRet: PianoReturn_t = PIANO_RET_ERR;
    let mut wRet: CURLcode = CURLE_OK;
    let mut reqData: PianoRequestDataExplain_t = PianoRequestDataExplain_t {
        song: 0 as *mut PianoSong_t,
        retExplain: 0 as *mut libc::c_char,
    };
    reqData.song = selSong;
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_INFO,
        b"Receiving explanation... \0" as *const u8 as *const libc::c_char,
    );
    if BarUiPianoCall(
        app,
        PIANO_REQUEST_EXPLAIN,
        &mut reqData as *mut PianoRequestDataExplain_t as *mut libc::c_void,
        &mut pRet,
        &mut wRet,
    ) {
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_INFO,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            reqData.retExplain,
        );
        free(reqData.retExplain as *mut libc::c_void);
    }
    BarUiStartEventCmd(
        &mut (*app).settings,
        b"songexplain\0" as *const u8 as *const libc::c_char,
        selStation,
        selSong,
        &mut (*app).player,
        (*app).ph.stations,
        pRet,
        wRet,
    );
}
pub unsafe extern "C" fn BarUiActStationFromGenre(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    let mut pRet: PianoReturn_t = PIANO_RET_ERR;
    let mut wRet: CURLcode = CURLE_OK;
    let mut curCat: *const PianoGenreCategory_t = 0 as *const PianoGenreCategory_t;
    let mut curGenre: *const PianoGenre_t = 0 as *const PianoGenre_t;
    let mut i: libc::c_int = 0;
    if ((*app).ph.genreStations).is_null() {
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_INFO,
            b"Receiving genre stations... \0" as *const u8 as *const libc::c_char,
        );
        let ret: bool = BarUiPianoCall(
            app,
            PIANO_REQUEST_GET_GENRE_STATIONS,
            0 as *mut libc::c_void,
            &mut pRet,
            &mut wRet,
        );
        BarUiStartEventCmd(
            &mut (*app).settings,
            b"stationfetchgenre\0" as *const u8 as *const libc::c_char,
            selStation,
            selSong,
            &mut (*app).player,
            (*app).ph.stations,
            pRet,
            wRet,
        );
        if !ret {
            return;
        }
    }
    curCat = (*app).ph.genreStations;
    i = 0 as libc::c_int;
    while !curCat.is_null() {
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_LIST,
            b"%2i) %s\n\0" as *const u8 as *const libc::c_char,
            i,
            (*curCat).name,
        );
        i += 1;
        i;
        curCat = (*curCat).head.next as *mut libc::c_void as *const PianoGenreCategory_t;
    }
    loop {
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_QUESTION,
            b"Select category: \0" as *const u8 as *const libc::c_char,
        );
        if BarReadlineInt(&mut i, &mut (*app).input) == 0 as libc::c_int as libc::c_ulong
        {
            return;
        }
        curCat = PianoListGet(&mut (*(*app).ph.genreStations).head, i as size_t)
            as *const PianoGenreCategory_t;
        if !curCat.is_null() {
            break;
        }
    }
    i = 0 as libc::c_int;
    curGenre = (*curCat).genres;
    while !curGenre.is_null() {
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_LIST,
            b"%2i) %s\n\0" as *const u8 as *const libc::c_char,
            i,
            (*curGenre).name,
        );
        i += 1;
        i;
        curGenre = (*curGenre).head.next as *mut libc::c_void as *const PianoGenre_t;
    }
    loop {
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_QUESTION,
            b"Select genre: \0" as *const u8 as *const libc::c_char,
        );
        if BarReadlineInt(&mut i, &mut (*app).input) == 0 as libc::c_int as libc::c_ulong
        {
            return;
        }
        curGenre = PianoListGet(&mut (*(*curCat).genres).head, i as size_t)
            as *const PianoGenre_t;
        if !curGenre.is_null() {
            break;
        }
    }
    let mut reqData: PianoRequestDataCreateStation_t = PianoRequestDataCreateStation_t {
        token: 0 as *mut libc::c_char,
        type_0: PIANO_MUSICTYPE_INVALID,
    };
    reqData.token = (*curGenre).musicId;
    reqData.type_0 = PIANO_MUSICTYPE_INVALID;
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_INFO,
        b"Adding genre station \"%s\"... \0" as *const u8 as *const libc::c_char,
        (*curGenre).name,
    );
    BarUiPianoCall(
        app,
        PIANO_REQUEST_CREATE_STATION,
        &mut reqData as *mut PianoRequestDataCreateStation_t as *mut libc::c_void,
        &mut pRet,
        &mut wRet,
    );
    BarUiStartEventCmd(
        &mut (*app).settings,
        b"stationaddgenre\0" as *const u8 as *const libc::c_char,
        selStation,
        selSong,
        &mut (*app).player,
        (*app).ph.stations,
        pRet,
        wRet,
    );
}
pub unsafe extern "C" fn BarUiActSongInfo(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    BarUiPrintStation(&mut (*app).settings, selStation);
    BarUiPrintSong(
        &mut (*app).settings,
        selSong,
        if (*selStation).isQuickMix as libc::c_int != 0 {
            PianoFindStationById((*app).ph.stations, (*selSong).stationId)
        } else {
            0 as *mut PianoStation_t
        },
    );
}
pub unsafe extern "C" fn BarUiActDebug(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_NONE,
        b"album:\t%s\nartist:\t%s\naudioFormat:\t%i\naudioUrl:\t%s\ncoverArt:\t%s\ndetailUrl:\t%s\nfileGain:\t%f\nmusicId:\t%s\nrating:\t%i\nstationId:\t%s\ntitle:\t%s\ntrackToken:\t%s\n\0"
            as *const u8 as *const libc::c_char,
        (*selSong).album,
        (*selSong).artist,
        (*selSong).audioFormat as libc::c_uint,
        (*selSong).audioUrl,
        (*selSong).coverArt,
        (*selSong).detailUrl,
        (*selSong).fileGain as libc::c_double,
        (*selSong).musicId,
        (*selSong).rating as libc::c_uint,
        (*selSong).stationId,
        (*selSong).title,
        (*selSong).trackToken,
    );
}
pub unsafe extern "C" fn BarUiActLoveSong(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    let mut pRet: PianoReturn_t = PIANO_RET_ERR;
    let mut wRet: CURLcode = CURLE_OK;
    let mut realStation: *mut PianoStation_t = 0 as *mut PianoStation_t;
    realStation = PianoFindStationById((*app).ph.stations, (*selSong).stationId);
    if realStation.is_null() {
        return;
    }
    if BarTransformIfShared(app, realStation) == 0 {
        return;
    }
    let mut reqData: PianoRequestDataRateSong_t = PianoRequestDataRateSong_t {
        song: 0 as *mut PianoSong_t,
        rating: PIANO_RATE_NONE,
    };
    reqData.song = selSong;
    reqData.rating = PIANO_RATE_LOVE;
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_INFO,
        b"Loving song... \0" as *const u8 as *const libc::c_char,
    );
    BarUiPianoCall(
        app,
        PIANO_REQUEST_RATE_SONG,
        &mut reqData as *mut PianoRequestDataRateSong_t as *mut libc::c_void,
        &mut pRet,
        &mut wRet,
    );
    BarUiStartEventCmd(
        &mut (*app).settings,
        b"songlove\0" as *const u8 as *const libc::c_char,
        selStation,
        selSong,
        &mut (*app).player,
        (*app).ph.stations,
        pRet,
        wRet,
    );
}
pub unsafe extern "C" fn BarUiActSkipSong(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    BarUiDoSkipSong(&mut (*app).player);
}
pub unsafe extern "C" fn BarUiActPlay(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    pthread_mutex_lock(&mut (*app).player.lock);
    (*app).player.doPause = 0 as libc::c_int != 0;
    pthread_cond_broadcast(&mut (*app).player.cond);
    pthread_mutex_unlock(&mut (*app).player.lock);
}
pub unsafe extern "C" fn BarUiActPause(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    pthread_mutex_lock(&mut (*app).player.lock);
    (*app).player.doPause = 1 as libc::c_int != 0;
    pthread_cond_broadcast(&mut (*app).player.cond);
    pthread_mutex_unlock(&mut (*app).player.lock);
}
pub unsafe extern "C" fn BarUiActTogglePause(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    pthread_mutex_lock(&mut (*app).player.lock);
    (*app).player.doPause = !(*app).player.doPause;
    pthread_cond_broadcast(&mut (*app).player.cond);
    pthread_mutex_unlock(&mut (*app).player.lock);
}
pub unsafe extern "C" fn BarUiActRenameStation(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    let mut pRet: PianoReturn_t = PIANO_RET_ERR;
    let mut wRet: CURLcode = CURLE_OK;
    let mut lineBuf: [libc::c_char; 100] = [0; 100];
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_QUESTION,
        b"New name: \0" as *const u8 as *const libc::c_char,
    );
    if BarReadlineStr(
        lineBuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
        &mut (*app).input,
        BAR_RL_DEFAULT,
    ) > 0 as libc::c_int as libc::c_ulong
    {
        let mut reqData: PianoRequestDataRenameStation_t = PianoRequestDataRenameStation_t {
            station: 0 as *mut PianoStation_t,
            newName: 0 as *mut libc::c_char,
        };
        if BarTransformIfShared(app, selStation) == 0 {
            return;
        }
        reqData.station = selStation;
        reqData.newName = lineBuf.as_mut_ptr();
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_INFO,
            b"Renaming station... \0" as *const u8 as *const libc::c_char,
        );
        BarUiPianoCall(
            app,
            PIANO_REQUEST_RENAME_STATION,
            &mut reqData as *mut PianoRequestDataRenameStation_t as *mut libc::c_void,
            &mut pRet,
            &mut wRet,
        );
        BarUiStartEventCmd(
            &mut (*app).settings,
            b"stationrename\0" as *const u8 as *const libc::c_char,
            selStation,
            selSong,
            &mut (*app).player,
            (*app).ph.stations,
            pRet,
            wRet,
        );
    }
}
pub unsafe extern "C" fn BarUiActSelectStation(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    let mut newStation: *mut PianoStation_t = BarUiSelectStation(
        app,
        (*app).ph.stations,
        b"Select station: \0" as *const u8 as *const libc::c_char,
        None,
        (*app).settings.autoselect,
    );
    if !newStation.is_null() {
        (*app).nextStation = newStation;
        drainPlaylist(app);
    }
}
pub unsafe extern "C" fn BarUiActTempBanSong(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    let mut pRet: PianoReturn_t = PIANO_RET_ERR;
    let mut wRet: CURLcode = CURLE_OK;
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_INFO,
        b"Putting song on shelf... \0" as *const u8 as *const libc::c_char,
    );
    if BarUiPianoCall(
        app,
        PIANO_REQUEST_ADD_TIRED_SONG,
        selSong as *mut libc::c_void,
        &mut pRet,
        &mut wRet,
    ) as libc::c_int != 0 && selSong == (*app).playlist
    {
        BarUiDoSkipSong(&mut (*app).player);
    }
    BarUiStartEventCmd(
        &mut (*app).settings,
        b"songshelf\0" as *const u8 as *const libc::c_char,
        selStation,
        selSong,
        &mut (*app).player,
        (*app).ph.stations,
        pRet,
        wRet,
    );
}
pub unsafe extern "C" fn BarUiActPrintUpcoming(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    let nextSong: *mut PianoSong_t = (if selSong.is_null() {
        0 as *mut libc::c_void
    } else {
        (*selSong).head.next as *mut libc::c_void
    }) as *mut PianoSong_t;
    if !nextSong.is_null() {
        BarUiListSongs(app, nextSong, 0 as *const libc::c_char);
    } else {
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_INFO,
            b"No songs in queue.\n\0" as *const u8 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn BarUiActQuickmixCallback(
    mut app: *mut BarApp_t,
    mut buf: *mut libc::c_char,
) {
    let mut curStation: *mut PianoStation_t = (*app).ph.stations;
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || *buf.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        return;
    }
    match *buf as libc::c_int {
        116 => {
            while !curStation.is_null() {
                (*curStation)
                    .useQuickMix = ((*curStation).useQuickMix == 0) as libc::c_int
                    as libc::c_char;
                curStation = (*curStation).head.next as *mut libc::c_void
                    as *mut PianoStation_t;
            }
            *buf = '\0' as i32 as libc::c_char;
        }
        97 => {
            while !curStation.is_null() {
                (*curStation).useQuickMix = 1 as libc::c_int as libc::c_char;
                curStation = (*curStation).head.next as *mut libc::c_void
                    as *mut PianoStation_t;
            }
            *buf = '\0' as i32 as libc::c_char;
        }
        110 => {
            while !curStation.is_null() {
                (*curStation).useQuickMix = 0 as libc::c_int as libc::c_char;
                curStation = (*curStation).head.next as *mut libc::c_void
                    as *mut PianoStation_t;
            }
            *buf = '\0' as i32 as libc::c_char;
        }
        _ => {}
    };
}
pub unsafe extern "C" fn BarUiActSelectQuickMix(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    let mut pRet: PianoReturn_t = PIANO_RET_ERR;
    let mut wRet: CURLcode = CURLE_OK;
    if (*selStation).isQuickMix != 0 {
        let mut toggleStation: *mut PianoStation_t = 0 as *mut PianoStation_t;
        loop {
            toggleStation = BarUiSelectStation(
                app,
                (*app).ph.stations,
                b"Toggle QuickMix for station: \0" as *const u8 as *const libc::c_char,
                Some(
                    BarUiActQuickmixCallback
                        as unsafe extern "C" fn(*mut BarApp_t, *mut libc::c_char) -> (),
                ),
                0 as libc::c_int != 0,
            );
            if toggleStation.is_null() {
                break;
            }
            (*toggleStation)
                .useQuickMix = ((*toggleStation).useQuickMix == 0) as libc::c_int
                as libc::c_char;
        }
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_INFO,
            b"Setting QuickMix stations... \0" as *const u8 as *const libc::c_char,
        );
        BarUiPianoCall(
            app,
            PIANO_REQUEST_SET_QUICKMIX,
            0 as *mut libc::c_void,
            &mut pRet,
            &mut wRet,
        );
        BarUiStartEventCmd(
            &mut (*app).settings,
            b"stationquickmixtoggle\0" as *const u8 as *const libc::c_char,
            selStation,
            selSong,
            &mut (*app).player,
            (*app).ph.stations,
            pRet,
            wRet,
        );
    } else {
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_ERR,
            b"Please select a QuickMix station first.\n\0" as *const u8
                as *const libc::c_char,
        );
    };
}
pub unsafe extern "C" fn BarUiActQuit(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    (*app).doQuit = 1 as libc::c_int;
    BarUiDoSkipSong(&mut (*app).player);
}
pub unsafe extern "C" fn BarUiActHistory(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    let mut buf: [libc::c_char; 2] = [0; 2];
    let mut histSong: *mut PianoSong_t = 0 as *mut PianoSong_t;
    if !((*app).songHistory).is_null() {
        histSong = BarUiSelectSong(app, (*app).songHistory, &mut (*app).input);
        if !histSong.is_null() {
            let mut action: BarKeyShortcutId_t = BAR_KS_HELP;
            let mut songStation: *mut PianoStation_t = PianoFindStationById(
                (*app).ph.stations,
                (*histSong).stationId,
            );
            if songStation.is_null() {
                BarUiMsg(
                    &mut (*app).settings as *mut BarSettings_t,
                    MSG_ERR,
                    b"Station does not exist any more.\n\0" as *const u8
                        as *const libc::c_char,
                );
                return;
            }
            loop {
                action = BAR_KS_COUNT;
                BarUiMsg(
                    &mut (*app).settings as *mut BarSettings_t,
                    MSG_QUESTION,
                    b"What to do with this song? \0" as *const u8 as *const libc::c_char,
                );
                if BarReadline(
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
                    0 as *const libc::c_char,
                    &mut (*app).input,
                    BAR_RL_FULLRETURN,
                    -(1 as libc::c_int),
                ) > 0 as libc::c_int as libc::c_ulong
                {
                    action = BarUiDispatch(
                        app,
                        buf[0 as libc::c_int as usize],
                        songStation,
                        histSong,
                        0 as libc::c_int != 0,
                        BAR_DC_UNDEFINED,
                    );
                }
                if !(action as libc::c_uint
                    == BAR_KS_HELP as libc::c_int as libc::c_uint)
                {
                    break;
                }
            }
        }
    } else {
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_INFO,
            if (*app).settings.history == 0 as libc::c_int as libc::c_uint {
                b"History disabled.\n\0" as *const u8 as *const libc::c_char
            } else {
                b"No history yet.\n\0" as *const u8 as *const libc::c_char
            },
        );
    };
}
pub unsafe extern "C" fn BarUiActBookmark(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    let mut pRet: PianoReturn_t = PIANO_RET_ERR;
    let mut wRet: CURLcode = CURLE_OK;
    let mut selectBuf: [libc::c_char; 2] = [0; 2];
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_QUESTION,
        b"Bookmark [s]ong or [a]rtist? \0" as *const u8 as *const libc::c_char,
    );
    BarReadline(
        selectBuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
        b"sa\0" as *const u8 as *const libc::c_char,
        &mut (*app).input,
        BAR_RL_FULLRETURN,
        -(1 as libc::c_int),
    );
    if selectBuf[0 as libc::c_int as usize] as libc::c_int == 's' as i32 {
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_INFO,
            b"Bookmarking song... \0" as *const u8 as *const libc::c_char,
        );
        BarUiPianoCall(
            app,
            PIANO_REQUEST_BOOKMARK_SONG,
            selSong as *mut libc::c_void,
            &mut pRet,
            &mut wRet,
        );
        BarUiStartEventCmd(
            &mut (*app).settings,
            b"songbookmark\0" as *const u8 as *const libc::c_char,
            selStation,
            selSong,
            &mut (*app).player,
            (*app).ph.stations,
            pRet,
            wRet,
        );
    } else if selectBuf[0 as libc::c_int as usize] as libc::c_int == 'a' as i32 {
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_INFO,
            b"Bookmarking artist... \0" as *const u8 as *const libc::c_char,
        );
        BarUiPianoCall(
            app,
            PIANO_REQUEST_BOOKMARK_ARTIST,
            selSong as *mut libc::c_void,
            &mut pRet,
            &mut wRet,
        );
        BarUiStartEventCmd(
            &mut (*app).settings,
            b"artistbookmark\0" as *const u8 as *const libc::c_char,
            selStation,
            selSong,
            &mut (*app).player,
            (*app).ph.stations,
            pRet,
            wRet,
        );
    }
}
pub unsafe extern "C" fn BarUiActVolDown(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    (*app).settings.volume -= 1;
    (*app).settings.volume;
    BarPlayerSetVolume(&mut (*app).player);
}
pub unsafe extern "C" fn BarUiActVolUp(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    (*app).settings.volume += 1;
    (*app).settings.volume;
    BarPlayerSetVolume(&mut (*app).player);
}
pub unsafe extern "C" fn BarUiActVolReset(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    (*app).settings.volume = 0 as libc::c_int;
    BarPlayerSetVolume(&mut (*app).player);
}
unsafe extern "C" fn boolToYesNo(value: bool) -> *const libc::c_char {
    return if value as libc::c_int != 0 {
        b"yes\0" as *const u8 as *const libc::c_char
    } else {
        b"no\0" as *const u8 as *const libc::c_char
    };
}
pub unsafe extern "C" fn BarUiActSettings(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    let mut pRet: PianoReturn_t = PIANO_RET_ERR;
    let mut wRet: CURLcode = CURLE_OK;
    let mut settings: PianoSettings_t = PianoSettings_t {
        username: 0 as *mut libc::c_char,
        explicitContentFilter: false,
    };
    let mut reqData: PianoRequestDataChangeSettings_t = PianoRequestDataChangeSettings_t {
        currentUsername: 0 as *mut libc::c_char,
        newUsername: 0 as *mut libc::c_char,
        currentPassword: 0 as *mut libc::c_char,
        newPassword: 0 as *mut libc::c_char,
        explicitContentFilter: PIANO_UNDEFINED,
    };
    let mut modified: bool = 0 as libc::c_int != 0;
    memset(
        &mut settings as *mut PianoSettings_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<PianoSettings_t>() as libc::c_ulong,
    );
    memset(
        &mut reqData as *mut PianoRequestDataChangeSettings_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<PianoRequestDataChangeSettings_t>() as libc::c_ulong,
    );
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_INFO,
        b"Retrieving settings... \0" as *const u8 as *const libc::c_char,
    );
    let mut bret: bool = BarUiPianoCall(
        app,
        PIANO_REQUEST_GET_SETTINGS,
        &mut settings as *mut PianoSettings_t as *mut libc::c_void,
        &mut pRet,
        &mut wRet,
    );
    BarUiStartEventCmd(
        &mut (*app).settings,
        b"settingsget\0" as *const u8 as *const libc::c_char,
        selStation,
        selSong,
        &mut (*app).player,
        (*app).ph.stations,
        pRet,
        wRet,
    );
    if !bret {
        return;
    }
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_LIST,
        b" 0) Username (%s)\n\0" as *const u8 as *const libc::c_char,
        settings.username,
    );
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_LIST,
        b" 1) Password (*****)\n\0" as *const u8 as *const libc::c_char,
    );
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_LIST,
        b" 2) Explicit content filter (%s)\n\0" as *const u8 as *const libc::c_char,
        boolToYesNo(settings.explicitContentFilter),
    );
    loop {
        let mut val: libc::c_int = 0;
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_QUESTION,
            b"Change setting: \0" as *const u8 as *const libc::c_char,
        );
        if BarReadlineInt(&mut val, &mut (*app).input)
            == 0 as libc::c_int as libc::c_ulong
        {
            break;
        }
        match val {
            0 => {
                let mut buf: [libc::c_char; 80] = [0; 80];
                BarUiMsg(
                    &mut (*app).settings as *mut BarSettings_t,
                    MSG_QUESTION,
                    b"New username: \0" as *const u8 as *const libc::c_char,
                );
                if BarReadlineStr(
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong,
                    &mut (*app).input,
                    BAR_RL_DEFAULT,
                ) > 0 as libc::c_int as libc::c_ulong
                {
                    reqData.newUsername = strdup(buf.as_mut_ptr());
                    modified = 1 as libc::c_int != 0;
                }
            }
            1 => {
                let mut buf_0: [libc::c_char; 80] = [0; 80];
                BarUiMsg(
                    &mut (*app).settings as *mut BarSettings_t,
                    MSG_QUESTION,
                    b"New password: \0" as *const u8 as *const libc::c_char,
                );
                if BarReadlineStr(
                    buf_0.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong,
                    &mut (*app).input,
                    BAR_RL_NOECHO,
                ) > 0 as libc::c_int as libc::c_ulong
                {
                    reqData.newPassword = strdup(buf_0.as_mut_ptr());
                    modified = 1 as libc::c_int != 0;
                }
                puts(b"\0" as *const u8 as *const libc::c_char);
            }
            2 => {
                BarUiMsg(
                    &mut (*app).settings as *mut BarSettings_t,
                    MSG_QUESTION,
                    b"Enable explicit content filter? [yn] \0" as *const u8
                        as *const libc::c_char,
                );
                reqData
                    .explicitContentFilter = (if BarReadlineYesNo(
                    settings.explicitContentFilter,
                    &mut (*app).input,
                ) as libc::c_int != 0
                {
                    PIANO_TRUE as libc::c_int
                } else {
                    PIANO_FALSE as libc::c_int
                }) as PianoTristate_t;
                modified = 1 as libc::c_int != 0;
            }
            _ => {}
        }
    }
    if modified {
        reqData.currentUsername = (*app).settings.username;
        reqData.currentPassword = (*app).settings.password;
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_INFO,
            b"Changing settings... \0" as *const u8 as *const libc::c_char,
        );
        BarUiPianoCall(
            app,
            PIANO_REQUEST_CHANGE_SETTINGS,
            &mut reqData as *mut PianoRequestDataChangeSettings_t as *mut libc::c_void,
            &mut pRet,
            &mut wRet,
        );
        BarUiStartEventCmd(
            &mut (*app).settings,
            b"settingschange\0" as *const u8 as *const libc::c_char,
            selStation,
            selSong,
            &mut (*app).player,
            (*app).ph.stations,
            pRet,
            wRet,
        );
        if !(reqData.newUsername).is_null() {
            free((*app).settings.username as *mut libc::c_void);
            (*app).settings.username = reqData.newUsername;
        }
        if !(reqData.newPassword).is_null() {
            free((*app).settings.password as *mut libc::c_void);
            (*app).settings.password = reqData.newPassword;
        }
    }
}
pub unsafe extern "C" fn BarUiActManageStation(
    mut app: *mut BarApp_t,
    mut selStation: *mut PianoStation_t,
    mut selSong: *mut PianoSong_t,
    mut context: BarUiDispatchContext_t,
) {
    let mut pRet: PianoReturn_t = PIANO_RET_ERR;
    let mut wRet: CURLcode = CURLE_OK;
    let mut reqData: PianoRequestDataGetStationInfo_t = PianoRequestDataGetStationInfo_t {
        station: 0 as *mut PianoStation_t,
        info: PianoStationInfo_t {
            songSeeds: 0 as *mut PianoSong_t,
            artistSeeds: 0 as *mut PianoArtist_t,
            stationSeeds: 0 as *mut PianoStation_t,
            feedback: 0 as *mut PianoSong_t,
        },
    };
    let mut selectBuf: [libc::c_char; 2] = [0; 2];
    let mut allowedActions: [libc::c_char; 6] = [0; 6];
    let mut allowedPos: *mut libc::c_char = allowedActions.as_mut_ptr();
    let mut question: [libc::c_char; 128] = [0; 128];
    memset(
        &mut reqData as *mut PianoRequestDataGetStationInfo_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<PianoRequestDataGetStationInfo_t>() as libc::c_ulong,
    );
    reqData.station = selStation;
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_INFO,
        b"Fetching station info... \0" as *const u8 as *const libc::c_char,
    );
    let bret: bool = BarUiPianoCall(
        app,
        PIANO_REQUEST_GET_STATION_INFO,
        &mut reqData as *mut PianoRequestDataGetStationInfo_t as *mut libc::c_void,
        &mut pRet,
        &mut wRet,
    );
    BarUiStartEventCmd(
        &mut (*app).settings,
        b"stationfetchinfo\0" as *const u8 as *const libc::c_char,
        selStation,
        selSong,
        &mut (*app).player,
        (*app).ph.stations,
        pRet,
        wRet,
    );
    if !bret {
        return;
    }
    if !(reqData.info.artistSeeds).is_null() || !(reqData.info.songSeeds).is_null()
        || !(reqData.info.stationSeeds).is_null() || !(reqData.info.feedback).is_null()
    {
        strcpy(question.as_mut_ptr(), b"Delete \0" as *const u8 as *const libc::c_char);
    }
    if !(reqData.info.artistSeeds).is_null() {
        strcat(question.as_mut_ptr(), b"[a]rtist\0" as *const u8 as *const libc::c_char);
        let fresh0 = allowedPos;
        allowedPos = allowedPos.offset(1);
        *fresh0 = 'a' as i32 as libc::c_char;
    }
    if !(reqData.info.songSeeds).is_null() {
        if allowedPos != allowedActions.as_mut_ptr() {
            strcat(question.as_mut_ptr(), b"/\0" as *const u8 as *const libc::c_char);
        }
        strcat(question.as_mut_ptr(), b"[s]ong\0" as *const u8 as *const libc::c_char);
        let fresh1 = allowedPos;
        allowedPos = allowedPos.offset(1);
        *fresh1 = 's' as i32 as libc::c_char;
    }
    if !(reqData.info.stationSeeds).is_null() {
        if allowedPos != allowedActions.as_mut_ptr() {
            strcat(question.as_mut_ptr(), b"/\0" as *const u8 as *const libc::c_char);
        }
        strcat(
            question.as_mut_ptr(),
            b"s[t]ation\0" as *const u8 as *const libc::c_char,
        );
        let fresh2 = allowedPos;
        allowedPos = allowedPos.offset(1);
        *fresh2 = 't' as i32 as libc::c_char;
    }
    if allowedPos != allowedActions.as_mut_ptr() {
        strcat(question.as_mut_ptr(), b" seeds\0" as *const u8 as *const libc::c_char);
    }
    if !(reqData.info.feedback).is_null() {
        if allowedPos != allowedActions.as_mut_ptr() {
            strcat(question.as_mut_ptr(), b" or \0" as *const u8 as *const libc::c_char);
        }
        strcat(
            question.as_mut_ptr(),
            b"[f]eedback\0" as *const u8 as *const libc::c_char,
        );
        let fresh3 = allowedPos;
        allowedPos = allowedPos.offset(1);
        *fresh3 = 'f' as i32 as libc::c_char;
    }
    if allowedPos != allowedActions.as_mut_ptr() {
        strcat(question.as_mut_ptr(), b"? \0" as *const u8 as *const libc::c_char);
    }
    if (*selStation).isQuickMix == 0 {
        strcat(
            question.as_mut_ptr(),
            b"Manage [m]ode? \0" as *const u8 as *const libc::c_char,
        );
        let fresh4 = allowedPos;
        allowedPos = allowedPos.offset(1);
        *fresh4 = 'm' as i32 as libc::c_char;
    }
    *allowedPos = '\0' as i32 as libc::c_char;
    if allowedPos == allowedActions.as_mut_ptr() {
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_INFO,
            b"No actions available.\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_QUESTION,
        b"%s\0" as *const u8 as *const libc::c_char,
        question.as_mut_ptr(),
    );
    if BarReadline(
        selectBuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
        allowedActions.as_mut_ptr(),
        &mut (*app).input,
        BAR_RL_FULLRETURN,
        -(1 as libc::c_int),
    ) != 0
    {
        if selectBuf[0 as libc::c_int as usize] as libc::c_int == 'a' as i32 {
            let mut artist: *mut PianoArtist_t = BarUiSelectArtist(
                app,
                reqData.info.artistSeeds,
            );
            if !artist.is_null() {
                let mut subReqData: PianoRequestDataDeleteSeed_t = PianoRequestDataDeleteSeed_t {
                    song: 0 as *mut PianoSong_t,
                    artist: 0 as *mut PianoArtist_t,
                    station: 0 as *mut PianoStation_t,
                };
                memset(
                    &mut subReqData as *mut PianoRequestDataDeleteSeed_t
                        as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<PianoRequestDataDeleteSeed_t>()
                        as libc::c_ulong,
                );
                subReqData.artist = artist;
                BarUiMsg(
                    &mut (*app).settings as *mut BarSettings_t,
                    MSG_INFO,
                    b"Deleting artist seed... \0" as *const u8 as *const libc::c_char,
                );
                BarUiPianoCall(
                    app,
                    PIANO_REQUEST_DELETE_SEED,
                    &mut subReqData as *mut PianoRequestDataDeleteSeed_t
                        as *mut libc::c_void,
                    &mut pRet,
                    &mut wRet,
                );
                BarUiStartEventCmd(
                    &mut (*app).settings,
                    b"stationdeleteartistseed\0" as *const u8 as *const libc::c_char,
                    selStation,
                    selSong,
                    &mut (*app).player,
                    (*app).ph.stations,
                    pRet,
                    wRet,
                );
            }
        } else if selectBuf[0 as libc::c_int as usize] as libc::c_int == 's' as i32 {
            let mut song: *mut PianoSong_t = BarUiSelectSong(
                app,
                reqData.info.songSeeds,
                &mut (*app).input,
            );
            if !song.is_null() {
                let mut subReqData_0: PianoRequestDataDeleteSeed_t = PianoRequestDataDeleteSeed_t {
                    song: 0 as *mut PianoSong_t,
                    artist: 0 as *mut PianoArtist_t,
                    station: 0 as *mut PianoStation_t,
                };
                memset(
                    &mut subReqData_0 as *mut PianoRequestDataDeleteSeed_t
                        as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<PianoRequestDataDeleteSeed_t>()
                        as libc::c_ulong,
                );
                subReqData_0.song = song;
                BarUiMsg(
                    &mut (*app).settings as *mut BarSettings_t,
                    MSG_INFO,
                    b"Deleting song seed... \0" as *const u8 as *const libc::c_char,
                );
                BarUiPianoCall(
                    app,
                    PIANO_REQUEST_DELETE_SEED,
                    &mut subReqData_0 as *mut PianoRequestDataDeleteSeed_t
                        as *mut libc::c_void,
                    &mut pRet,
                    &mut wRet,
                );
                BarUiStartEventCmd(
                    &mut (*app).settings,
                    b"stationdeletesongseed\0" as *const u8 as *const libc::c_char,
                    selStation,
                    selSong,
                    &mut (*app).player,
                    (*app).ph.stations,
                    pRet,
                    wRet,
                );
            }
        } else if selectBuf[0 as libc::c_int as usize] as libc::c_int == 't' as i32 {
            let mut station: *mut PianoStation_t = BarUiSelectStation(
                app,
                reqData.info.stationSeeds,
                b"Delete seed station: \0" as *const u8 as *const libc::c_char,
                None,
                0 as libc::c_int != 0,
            );
            if !station.is_null() {
                let mut subReqData_1: PianoRequestDataDeleteSeed_t = PianoRequestDataDeleteSeed_t {
                    song: 0 as *mut PianoSong_t,
                    artist: 0 as *mut PianoArtist_t,
                    station: 0 as *mut PianoStation_t,
                };
                memset(
                    &mut subReqData_1 as *mut PianoRequestDataDeleteSeed_t
                        as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<PianoRequestDataDeleteSeed_t>()
                        as libc::c_ulong,
                );
                subReqData_1.station = station;
                BarUiMsg(
                    &mut (*app).settings as *mut BarSettings_t,
                    MSG_INFO,
                    b"Deleting station seed... \0" as *const u8 as *const libc::c_char,
                );
                BarUiPianoCall(
                    app,
                    PIANO_REQUEST_DELETE_SEED,
                    &mut subReqData_1 as *mut PianoRequestDataDeleteSeed_t
                        as *mut libc::c_void,
                    &mut pRet,
                    &mut wRet,
                );
                BarUiStartEventCmd(
                    &mut (*app).settings,
                    b"stationdeletestationseed\0" as *const u8 as *const libc::c_char,
                    selStation,
                    selSong,
                    &mut (*app).player,
                    (*app).ph.stations,
                    pRet,
                    wRet,
                );
            }
        } else if selectBuf[0 as libc::c_int as usize] as libc::c_int == 'f' as i32 {
            let mut song_0: *mut PianoSong_t = BarUiSelectSong(
                app,
                reqData.info.feedback,
                &mut (*app).input,
            );
            if !song_0.is_null() {
                BarUiMsg(
                    &mut (*app).settings as *mut BarSettings_t,
                    MSG_INFO,
                    b"Deleting feedback... \0" as *const u8 as *const libc::c_char,
                );
                BarUiPianoCall(
                    app,
                    PIANO_REQUEST_DELETE_FEEDBACK,
                    song_0 as *mut libc::c_void,
                    &mut pRet,
                    &mut wRet,
                );
                BarUiStartEventCmd(
                    &mut (*app).settings,
                    b"stationdeletefeedback\0" as *const u8 as *const libc::c_char,
                    selStation,
                    selSong,
                    &mut (*app).player,
                    (*app).ph.stations,
                    pRet,
                    wRet,
                );
            }
        } else if selectBuf[0 as libc::c_int as usize] as libc::c_int == 'm' as i32 {
            let mut subReqData_2: PianoRequestDataGetStationModes_t = {
                let mut init = PianoRequestDataGetStationModes_t {
                    station: selStation,
                    retModes: 0 as *mut PianoStationMode_t,
                };
                init
            };
            BarUiMsg(
                &mut (*app).settings as *mut BarSettings_t,
                MSG_INFO,
                b"Fetching modes... \0" as *const u8 as *const libc::c_char,
            );
            BarUiPianoCall(
                app,
                PIANO_REQUEST_GET_STATION_MODES,
                &mut subReqData_2 as *mut PianoRequestDataGetStationModes_t
                    as *mut libc::c_void,
                &mut pRet,
                &mut wRet,
            );
            BarUiStartEventCmd(
                &mut (*app).settings,
                b"stationgetmodes\0" as *const u8 as *const libc::c_char,
                selStation,
                selSong,
                &mut (*app).player,
                (*app).ph.stations,
                pRet,
                wRet,
            );
            let mut curMode: *const PianoStationMode_t = subReqData_2.retModes;
            let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while !curMode.is_null() {
                BarUiMsg(
                    &mut (*app).settings as *mut BarSettings_t,
                    MSG_LIST,
                    b"%2i) %s: %s%s\n\0" as *const u8 as *const libc::c_char,
                    i,
                    (*curMode).name,
                    (*curMode).description,
                    if (*curMode).active as libc::c_int != 0 {
                        b" (active)\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                );
                i = i.wrapping_add(1);
                i;
                curMode = (*curMode).head.next as *mut libc::c_void
                    as *const PianoStationMode_t;
            }
            BarUiMsg(
                &mut (*app).settings as *mut BarSettings_t,
                MSG_QUESTION,
                b"Pick a new mode: \0" as *const u8 as *const libc::c_char,
            );
            let mut selected: libc::c_int = 0;
            while !(BarReadlineInt(&mut selected, &mut (*app).input)
                == 0 as libc::c_int as libc::c_ulong)
            {
                let selMode: *const PianoStationMode_t = PianoListGet(
                    &mut (*subReqData_2.retModes).head,
                    selected as size_t,
                ) as *const PianoStationMode_t;
                if selMode.is_null() {
                    continue;
                }
                let mut subReqDataSet: PianoRequestDataSetStationMode_t = {
                    let mut init = PianoRequestDataSetStationMode_t {
                        station: selStation,
                        id: selected as libc::c_uint,
                    };
                    init
                };
                BarUiMsg(
                    &mut (*app).settings as *mut BarSettings_t,
                    MSG_INFO,
                    b"Selecting mode \"%s\"... \0" as *const u8 as *const libc::c_char,
                    (*selMode).name,
                );
                if BarUiPianoCall(
                    app,
                    PIANO_REQUEST_SET_STATION_MODE,
                    &mut subReqDataSet as *mut PianoRequestDataSetStationMode_t
                        as *mut libc::c_void,
                    &mut pRet,
                    &mut wRet,
                ) {
                    drainPlaylist(app);
                }
                BarUiStartEventCmd(
                    &mut (*app).settings,
                    b"stationsetmode\0" as *const u8 as *const libc::c_char,
                    selStation,
                    selSong,
                    &mut (*app).player,
                    (*app).ph.stations,
                    pRet,
                    wRet,
                );
                break;
            }
            PianoDestroyStationMode(subReqData_2.retModes);
        }
    }
    PianoDestroyStationInfo(&mut reqData.info);
}
