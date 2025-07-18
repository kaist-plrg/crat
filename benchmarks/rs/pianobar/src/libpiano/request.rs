use ::libc;
extern "C" {
    pub type json_object;
    pub type gcry_cipher_handle;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn curl_easy_escape(
        handle: *mut libc::c_void,
        string: *const libc::c_char,
        length: libc::c_int,
    ) -> *mut libc::c_char;
    fn curl_free(p: *mut libc::c_void);
    fn curl_easy_init() -> *mut libc::c_void;
    fn curl_easy_cleanup(curl: *mut libc::c_void);
    fn json_object_put(obj: *mut json_object) -> libc::c_int;
    fn json_object_to_json_string(obj: *mut json_object) -> *const libc::c_char;
    fn json_object_new_object() -> *mut json_object;
    fn json_object_object_add(
        obj: *mut json_object,
        key: *const libc::c_char,
        val: *mut json_object,
    ) -> libc::c_int;
    fn json_object_new_array() -> *mut json_object;
    fn json_object_array_add(
        obj: *mut json_object,
        val: *mut json_object,
    ) -> libc::c_int;
    fn json_object_new_boolean(b: json_bool) -> *mut json_object;
    fn json_object_new_int(i: int32_t) -> *mut json_object;
    fn json_object_new_string(s: *const libc::c_char) -> *mut json_object;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn PianoEncryptString(
        _: gcry_cipher_hd_t,
        _: *const libc::c_char,
    ) -> *mut libc::c_char;
}
pub type __int32_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type int32_t = __int32_t;
pub type CURL = ();
pub type json_bool = libc::c_int;
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
pub struct PianoSearchResult {
    pub songs: *mut PianoSong_t,
    pub artists: *mut PianoArtist_t,
}
pub type PianoSearchResult_t = PianoSearchResult;
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
pub struct PianoRequest {
    pub type_0: PianoRequestType_t,
    pub secure: bool,
    pub data: *mut libc::c_void,
    pub urlPath: [libc::c_char; 1024],
    pub postData: *mut libc::c_char,
    pub responseData: *mut libc::c_char,
}
pub type PianoRequest_t = PianoRequest;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoRequestDataLogin_t {
    pub user: *mut libc::c_char,
    pub password: *mut libc::c_char,
    pub step: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoRequestDataGetPlaylist_t {
    pub station: *mut PianoStation_t,
    pub quality: PianoAudioQuality_t,
    pub retPlaylist: *mut PianoSong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoRequestDataRateSong_t {
    pub song: *mut PianoSong_t,
    pub rating: PianoSongRating_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoRequestDataAddFeedback_t {
    pub stationId: *mut libc::c_char,
    pub trackToken: *mut libc::c_char,
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
pub struct PianoRequestDataSearch_t {
    pub searchStr: *mut libc::c_char,
    pub searchResult: PianoSearchResult_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoRequestDataCreateStation_t {
    pub token: *mut libc::c_char,
    pub type_0: C2RustUnnamed,
}
pub type C2RustUnnamed = libc::c_uint;
pub const PIANO_MUSICTYPE_ARTIST: C2RustUnnamed = 2;
pub const PIANO_MUSICTYPE_SONG: C2RustUnnamed = 1;
pub const PIANO_MUSICTYPE_INVALID: C2RustUnnamed = 0;
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
pub unsafe extern "C" fn PianoRequest(
    mut ph: *mut PianoHandle_t,
    mut req: *mut PianoRequest_t,
    mut type_0: PianoRequestType_t,
) -> PianoReturn_t {
    let mut current_block: u64;
    let mut ret: PianoReturn_t = PIANO_RET_OK;
    let mut jsonSendBuf: *const libc::c_char = 0 as *const libc::c_char;
    let mut method: *const libc::c_char = 0 as *const libc::c_char;
    let mut j: *mut json_object = json_object_new_object();
    let mut timestamp: time_t = time(0 as *mut time_t)
        - (*ph).timeOffset as libc::c_long;
    let mut encrypted: bool = 1 as libc::c_int != 0;
    (*req).type_0 = type_0;
    (*req).secure = 0 as libc::c_int != 0;
    match (*req).type_0 as libc::c_uint {
        1 => {
            let mut logindata: *mut PianoRequestDataLogin_t = (*req).data
                as *mut PianoRequestDataLogin_t;
            match (*logindata).step as libc::c_int {
                0 => {
                    encrypted = 0 as libc::c_int != 0;
                    (*req).secure = 1 as libc::c_int != 0;
                    json_object_object_add(
                        j,
                        b"username\0" as *const u8 as *const libc::c_char,
                        json_object_new_string((*ph).partner.user),
                    );
                    json_object_object_add(
                        j,
                        b"password\0" as *const u8 as *const libc::c_char,
                        json_object_new_string((*ph).partner.password),
                    );
                    json_object_object_add(
                        j,
                        b"deviceModel\0" as *const u8 as *const libc::c_char,
                        json_object_new_string((*ph).partner.device),
                    );
                    json_object_object_add(
                        j,
                        b"version\0" as *const u8 as *const libc::c_char,
                        json_object_new_string(
                            b"5\0" as *const u8 as *const libc::c_char,
                        ),
                    );
                    json_object_object_add(
                        j,
                        b"includeUrls\0" as *const u8 as *const libc::c_char,
                        json_object_new_boolean(1 as libc::c_int),
                    );
                    snprintf(
                        ((*req).urlPath).as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                        b"/services/json/?method=auth.partnerLogin\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                1 => {
                    let mut urlencAuthToken: *mut libc::c_char = 0 as *mut libc::c_char;
                    (*req).secure = 1 as libc::c_int != 0;
                    json_object_object_add(
                        j,
                        b"loginType\0" as *const u8 as *const libc::c_char,
                        json_object_new_string(
                            b"user\0" as *const u8 as *const libc::c_char,
                        ),
                    );
                    json_object_object_add(
                        j,
                        b"username\0" as *const u8 as *const libc::c_char,
                        json_object_new_string((*logindata).user),
                    );
                    json_object_object_add(
                        j,
                        b"password\0" as *const u8 as *const libc::c_char,
                        json_object_new_string((*logindata).password),
                    );
                    json_object_object_add(
                        j,
                        b"partnerAuthToken\0" as *const u8 as *const libc::c_char,
                        json_object_new_string((*ph).partner.authToken),
                    );
                    json_object_object_add(
                        j,
                        b"syncTime\0" as *const u8 as *const libc::c_char,
                        json_object_new_int(timestamp as int32_t),
                    );
                    let curl: *mut libc::c_void = curl_easy_init();
                    urlencAuthToken = curl_easy_escape(
                        curl,
                        (*ph).partner.authToken,
                        0 as libc::c_int,
                    );
                    snprintf(
                        ((*req).urlPath).as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                        b"/services/json/?method=auth.userLogin&auth_token=%s&partner_id=%i\0"
                            as *const u8 as *const libc::c_char,
                        urlencAuthToken,
                        (*ph).partner.id,
                    );
                    curl_free(urlencAuthToken as *mut libc::c_void);
                    curl_easy_cleanup(curl);
                }
                _ => {}
            }
            current_block = 12223373342341601825;
        }
        2 => {
            json_object_object_add(
                j,
                b"returnAllStations\0" as *const u8 as *const libc::c_char,
                json_object_new_boolean(1 as libc::c_int),
            );
            method = b"user.getStationList\0" as *const u8 as *const libc::c_char;
            current_block = 12223373342341601825;
        }
        3 => {
            let mut reqData: *mut PianoRequestDataGetPlaylist_t = (*req).data
                as *mut PianoRequestDataGetPlaylist_t;
            (*req).secure = 1 as libc::c_int != 0;
            json_object_object_add(
                j,
                b"stationToken\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*(*reqData).station).id),
            );
            json_object_object_add(
                j,
                b"includeTrackLength\0" as *const u8 as *const libc::c_char,
                json_object_new_boolean(1 as libc::c_int),
            );
            method = b"station.getPlaylist\0" as *const u8 as *const libc::c_char;
            current_block = 12223373342341601825;
        }
        5 => {
            let mut reqData_0: *mut PianoRequestDataAddFeedback_t = (*req).data
                as *mut PianoRequestDataAddFeedback_t;
            json_object_object_add(
                j,
                b"stationToken\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*reqData_0).stationId),
            );
            json_object_object_add(
                j,
                b"trackToken\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*reqData_0).trackToken),
            );
            json_object_object_add(
                j,
                b"isPositive\0" as *const u8 as *const libc::c_char,
                json_object_new_boolean(
                    ((*reqData_0).rating as libc::c_uint
                        == PIANO_RATE_LOVE as libc::c_int as libc::c_uint) as libc::c_int,
                ),
            );
            method = b"station.addFeedback\0" as *const u8 as *const libc::c_char;
            current_block = 12223373342341601825;
        }
        7 => {
            let mut reqData_1: *mut PianoRequestDataRenameStation_t = (*req).data
                as *mut PianoRequestDataRenameStation_t;
            json_object_object_add(
                j,
                b"stationToken\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*(*reqData_1).station).id),
            );
            json_object_object_add(
                j,
                b"stationName\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*reqData_1).newName),
            );
            method = b"station.renameStation\0" as *const u8 as *const libc::c_char;
            current_block = 12223373342341601825;
        }
        8 => {
            let mut station: *mut PianoStation_t = (*req).data as *mut PianoStation_t;
            json_object_object_add(
                j,
                b"stationToken\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*station).id),
            );
            method = b"station.deleteStation\0" as *const u8 as *const libc::c_char;
            current_block = 12223373342341601825;
        }
        9 => {
            let mut reqData_2: *mut PianoRequestDataSearch_t = (*req).data
                as *mut PianoRequestDataSearch_t;
            json_object_object_add(
                j,
                b"searchText\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*reqData_2).searchStr),
            );
            method = b"music.search\0" as *const u8 as *const libc::c_char;
            current_block = 12223373342341601825;
        }
        10 => {
            let mut reqData_3: *mut PianoRequestDataCreateStation_t = (*req).data
                as *mut PianoRequestDataCreateStation_t;
            if (*reqData_3).type_0 as libc::c_uint
                == PIANO_MUSICTYPE_INVALID as libc::c_int as libc::c_uint
            {
                json_object_object_add(
                    j,
                    b"musicToken\0" as *const u8 as *const libc::c_char,
                    json_object_new_string((*reqData_3).token),
                );
            } else {
                json_object_object_add(
                    j,
                    b"trackToken\0" as *const u8 as *const libc::c_char,
                    json_object_new_string((*reqData_3).token),
                );
                match (*reqData_3).type_0 as libc::c_uint {
                    1 => {
                        json_object_object_add(
                            j,
                            b"musicType\0" as *const u8 as *const libc::c_char,
                            json_object_new_string(
                                b"song\0" as *const u8 as *const libc::c_char,
                            ),
                        );
                    }
                    2 => {
                        json_object_object_add(
                            j,
                            b"musicType\0" as *const u8 as *const libc::c_char,
                            json_object_new_string(
                                b"artist\0" as *const u8 as *const libc::c_char,
                            ),
                        );
                    }
                    _ => {}
                }
            }
            method = b"station.createStation\0" as *const u8 as *const libc::c_char;
            current_block = 12223373342341601825;
        }
        11 => {
            let mut reqData_4: *mut PianoRequestDataAddSeed_t = (*req).data
                as *mut PianoRequestDataAddSeed_t;
            json_object_object_add(
                j,
                b"musicToken\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*reqData_4).musicId),
            );
            json_object_object_add(
                j,
                b"stationToken\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*(*reqData_4).station).id),
            );
            method = b"station.addMusic\0" as *const u8 as *const libc::c_char;
            current_block = 12223373342341601825;
        }
        12 => {
            let mut song: *mut PianoSong_t = (*req).data as *mut PianoSong_t;
            json_object_object_add(
                j,
                b"trackToken\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*song).trackToken),
            );
            method = b"user.sleepSong\0" as *const u8 as *const libc::c_char;
            current_block = 12223373342341601825;
        }
        13 => {
            let mut curStation: *mut PianoStation_t = (*ph).stations;
            let mut a: *mut json_object = json_object_new_array();
            while !curStation.is_null() {
                if (*curStation).useQuickMix as libc::c_int != 0
                    && (*curStation).isQuickMix == 0
                {
                    json_object_array_add(a, json_object_new_string((*curStation).id));
                }
                curStation = (*curStation).head.next as *mut libc::c_void
                    as *mut PianoStation_t;
            }
            json_object_object_add(
                j,
                b"quickMixStationIds\0" as *const u8 as *const libc::c_char,
                a,
            );
            method = b"user.setQuickMix\0" as *const u8 as *const libc::c_char;
            current_block = 12223373342341601825;
        }
        14 => {
            method = b"station.getGenreStations\0" as *const u8 as *const libc::c_char;
            current_block = 12223373342341601825;
        }
        15 => {
            let mut station_0: *mut PianoStation_t = (*req).data as *mut PianoStation_t;
            json_object_object_add(
                j,
                b"stationToken\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*station_0).id),
            );
            method = b"station.transformSharedStation\0" as *const u8
                as *const libc::c_char;
            current_block = 12223373342341601825;
        }
        16 => {
            let mut reqData_5: *mut PianoRequestDataExplain_t = (*req).data
                as *mut PianoRequestDataExplain_t;
            json_object_object_add(
                j,
                b"trackToken\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*(*reqData_5).song).trackToken),
            );
            method = b"track.explainTrack\0" as *const u8 as *const libc::c_char;
            current_block = 12223373342341601825;
        }
        18 => {
            let mut song_0: *mut PianoSong_t = (*req).data as *mut PianoSong_t;
            json_object_object_add(
                j,
                b"trackToken\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*song_0).trackToken),
            );
            method = b"bookmark.addSongBookmark\0" as *const u8 as *const libc::c_char;
            current_block = 12223373342341601825;
        }
        19 => {
            let mut song_1: *mut PianoSong_t = (*req).data as *mut PianoSong_t;
            json_object_object_add(
                j,
                b"trackToken\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*song_1).trackToken),
            );
            method = b"bookmark.addArtistBookmark\0" as *const u8 as *const libc::c_char;
            current_block = 12223373342341601825;
        }
        20 => {
            let mut reqData_6: *mut PianoRequestDataGetStationInfo_t = (*req).data
                as *mut PianoRequestDataGetStationInfo_t;
            json_object_object_add(
                j,
                b"stationToken\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*(*reqData_6).station).id),
            );
            json_object_object_add(
                j,
                b"includeExtendedAttributes\0" as *const u8 as *const libc::c_char,
                json_object_new_boolean(1 as libc::c_int),
            );
            json_object_object_add(
                j,
                b"includeExtraParams\0" as *const u8 as *const libc::c_char,
                json_object_new_boolean(1 as libc::c_int),
            );
            method = b"station.getStation\0" as *const u8 as *const libc::c_char;
            current_block = 12223373342341601825;
        }
        25 => {
            let mut reqData_7: *mut PianoRequestDataGetStationModes_t = (*req).data
                as *mut PianoRequestDataGetStationModes_t;
            let station_1: *mut PianoStation_t = (*reqData_7).station;
            json_object_object_add(
                j,
                b"stationId\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*station_1).id),
            );
            method = b"interactiveradio.v1.getAvailableModesSimple\0" as *const u8
                as *const libc::c_char;
            (*req).secure = 1 as libc::c_int != 0;
            current_block = 12223373342341601825;
        }
        26 => {
            let mut reqData_8: *mut PianoRequestDataSetStationMode_t = (*req).data
                as *mut PianoRequestDataSetStationMode_t;
            let station_2: *mut PianoStation_t = (*reqData_8).station;
            json_object_object_add(
                j,
                b"stationId\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*station_2).id),
            );
            json_object_object_add(
                j,
                b"modeId\0" as *const u8 as *const libc::c_char,
                json_object_new_int((*reqData_8).id as int32_t),
            );
            method = b"interactiveradio.v1.setAndGetAvailableModes\0" as *const u8
                as *const libc::c_char;
            (*req).secure = 1 as libc::c_int != 0;
            current_block = 12223373342341601825;
        }
        21 => {
            let mut song_2: *mut PianoSong_t = (*req).data as *mut PianoSong_t;
            json_object_object_add(
                j,
                b"feedbackId\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*song_2).feedbackId),
            );
            method = b"station.deleteFeedback\0" as *const u8 as *const libc::c_char;
            current_block = 12223373342341601825;
        }
        22 => {
            let mut reqData_9: *mut PianoRequestDataDeleteSeed_t = (*req).data
                as *mut PianoRequestDataDeleteSeed_t;
            let mut seedId: *mut libc::c_char = 0 as *mut libc::c_char;
            if !((*reqData_9).song).is_null() {
                seedId = (*(*reqData_9).song).seedId;
            } else if !((*reqData_9).artist).is_null() {
                seedId = (*(*reqData_9).artist).seedId;
            } else if !((*reqData_9).station).is_null() {
                seedId = (*(*reqData_9).station).seedId;
            }
            json_object_object_add(
                j,
                b"seedId\0" as *const u8 as *const libc::c_char,
                json_object_new_string(seedId),
            );
            method = b"station.deleteMusic\0" as *const u8 as *const libc::c_char;
            current_block = 12223373342341601825;
        }
        23 => {
            method = b"user.getSettings\0" as *const u8 as *const libc::c_char;
            current_block = 12223373342341601825;
        }
        24 => {
            let mut reqData_10: *mut PianoRequestDataChangeSettings_t = (*req).data
                as *mut PianoRequestDataChangeSettings_t;
            json_object_object_add(
                j,
                b"userInitiatedChange\0" as *const u8 as *const libc::c_char,
                json_object_new_boolean(1 as libc::c_int),
            );
            json_object_object_add(
                j,
                b"currentUsername\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*reqData_10).currentUsername),
            );
            json_object_object_add(
                j,
                b"currentPassword\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*reqData_10).currentPassword),
            );
            if (*reqData_10).explicitContentFilter as libc::c_uint
                != PIANO_UNDEFINED as libc::c_int as libc::c_uint
            {
                json_object_object_add(
                    j,
                    b"isExplicitContentFilterEnabled\0" as *const u8
                        as *const libc::c_char,
                    json_object_new_boolean(
                        ((*reqData_10).explicitContentFilter as libc::c_uint
                            == PIANO_TRUE as libc::c_int as libc::c_uint) as libc::c_int,
                    ),
                );
            }
            if !((*reqData_10).newUsername).is_null() {
                json_object_object_add(
                    j,
                    b"newUsername\0" as *const u8 as *const libc::c_char,
                    json_object_new_string((*reqData_10).newUsername),
                );
            }
            if !((*reqData_10).newPassword).is_null() {
                json_object_object_add(
                    j,
                    b"newPassword\0" as *const u8 as *const libc::c_char,
                    json_object_new_string((*reqData_10).newPassword),
                );
            }
            (*req).secure = 1 as libc::c_int != 0;
            method = b"user.changeSettings\0" as *const u8 as *const libc::c_char;
            current_block = 12223373342341601825;
        }
        4 => {
            let mut reqData_11: *mut PianoRequestDataRateSong_t = (*req).data
                as *mut PianoRequestDataRateSong_t;
            let mut transformedReqData: PianoRequestDataAddFeedback_t = PianoRequestDataAddFeedback_t {
                stationId: 0 as *mut libc::c_char,
                trackToken: 0 as *mut libc::c_char,
                rating: PIANO_RATE_NONE,
            };
            transformedReqData.stationId = (*(*reqData_11).song).stationId;
            transformedReqData.trackToken = (*(*reqData_11).song).trackToken;
            transformedReqData.rating = (*reqData_11).rating;
            (*req)
                .data = &mut transformedReqData as *mut PianoRequestDataAddFeedback_t
                as *mut libc::c_void;
            ret = PianoRequest(ph, req, PIANO_REQUEST_ADD_FEEDBACK);
            (*req).type_0 = PIANO_REQUEST_RATE_SONG;
            (*req).data = reqData_11 as *mut libc::c_void;
            current_block = 6156042887766821020;
        }
        _ => {
            current_block = 12223373342341601825;
        }
    }
    match current_block {
        12223373342341601825 => {
            if !method.is_null() {
                let mut urlencAuthToken_0: *mut libc::c_char = 0 as *mut libc::c_char;
                let curl_0: *mut libc::c_void = curl_easy_init();
                urlencAuthToken_0 = curl_easy_escape(
                    curl_0,
                    (*ph).user.authToken,
                    0 as libc::c_int,
                );
                snprintf(
                    ((*req).urlPath).as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                    b"/services/json/?method=%s&auth_token=%s&partner_id=%i&user_id=%s\0"
                        as *const u8 as *const libc::c_char,
                    method,
                    urlencAuthToken_0,
                    (*ph).partner.id,
                    (*ph).user.listenerId,
                );
                curl_free(urlencAuthToken_0 as *mut libc::c_void);
                curl_easy_cleanup(curl_0);
                json_object_object_add(
                    j,
                    b"userAuthToken\0" as *const u8 as *const libc::c_char,
                    json_object_new_string((*ph).user.authToken),
                );
                json_object_object_add(
                    j,
                    b"syncTime\0" as *const u8 as *const libc::c_char,
                    json_object_new_int(timestamp as int32_t),
                );
            }
            jsonSendBuf = json_object_to_json_string(j);
            if encrypted {
                (*req).postData = PianoEncryptString((*ph).partner.out, jsonSendBuf);
                if ((*req).postData).is_null() {
                    ret = PIANO_RET_OUT_OF_MEMORY;
                }
            } else {
                (*req).postData = strdup(jsonSendBuf);
            }
        }
        _ => {}
    }
    json_object_put(j);
    return ret;
}
