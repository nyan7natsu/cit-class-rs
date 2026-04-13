use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusDto {
    pub success: bool,
    #[serde(alias = "messageList")]
    pub message_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponseData {
    #[serde(alias = "userId")]
    pub user_id: String,
    #[serde(alias = "encryptedPassword")]
    pub encrypted_password: String,
    #[serde(alias = "userName")]
    pub user_name: String,
    #[serde(alias = "userShkbtKbn")]
    pub user_shkbt_kbn: String,
    #[serde(alias = "shokuinUserKbn")]
    pub shokuin_user_kbn: Option<serde_json::Value>,
    #[serde(alias = "jinjiCd")]
    pub jinji_cd: Option<serde_json::Value>,
    #[serde(alias = "kanriNo")]
    pub kanri_no: i32,
    #[serde(alias = "gaksekiCd")]
    pub gakseki_cd: String,
    pub name: String,
    #[serde(alias = "nameKana")]
    pub name_kana: String,
    #[serde(alias = "nameEng")]
    pub name_eng: String,
    #[serde(alias = "nameDisp")]
    pub name_disp: String,
    #[serde(alias = "validKikanStartDatetime")]
    pub valid_kikan_start_datetime: Option<serde_json::Value>,
    #[serde(alias = "validKikanEndDatetime")]
    pub valid_kikan_end_datetime: Option<serde_json::Value>,
    #[serde(alias = "menuPtnCd")]
    pub menu_ptn_cd: Option<serde_json::Value>,
    #[serde(alias = "langCd")]
    pub lang_cd: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    #[serde(alias = "statusDto")]
    pub status_dto: StatusDto,
    pub data: Option<LoginResponseData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClassData {
    /** 授業年度 */
    pub nendo: i32,

    /** 授業コード */
    #[serde(alias = "jugyoCd")]
    pub jugyo_cd: String,

    /** 開講年度 */
    #[serde(alias = "kaikoNendo")]
    pub kaiko_nendo: i32,

    /** 開講学期 */
    #[serde(alias = "gakkiNo")]
    pub gakki_no: i32,

    /**
     * 授業区分
     *
     * `"1"`: 通常授業  
     * `"6"`: 集中授業
     */
    #[serde(alias = "jugyoKbn")]
    pub jugyo_kbn: String,

    /**
     * 授業終了時間
     *
     * 集中講義の場合は `null` になる
     */
    #[serde(alias = "jugyoEndTime")]
    pub jugyo_end_time: Option<String>,

    /** 指導教員名 */
    #[serde(alias = "kyoinName")]
    pub kyoin_name: String,

    /** 教室名 */
    #[serde(alias = "kyostName")]
    pub kyost_name: Option<String>,

    /** 掲示の未読数 */
    #[serde(alias = "keijiMidokCnt")]
    pub keiji_midok_cnt: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClassSchedule {
    pub nendo: i32,
    #[serde(alias = "gakkiNo")]
    pub gakki_no: i32,
    #[serde(alias = "gakkiName")]
    pub gakki_name: String,
    #[serde(alias = "jgkmDtoList")]
    pub jgkm_dto_list: Vec<ClassData>,
    #[serde(alias = "keijiCnt")]
    pub keiji_cnt: i32,
    #[serde(alias = "funcIdList")]
    pub func_id_list: Vec<String>,
    #[serde(alias = "maxGakkiNo")]
    pub max_gakki_no: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClassScheduleResponse {
    #[serde(alias = "statusDto")]
    pub status_dto: StatusDto,
    pub data: Option<ClassSchedule>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequestData {
    #[serde(rename = "loginUserId")]
    pub login_user_id: String,
    #[serde(rename = "plainLoginPassword")]
    pub plain_login_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub data: LoginRequestData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClassRequestData {
    #[serde(rename = "autoLoginAuthCd")]
    pub auto_login_auth_cd: String,
    #[serde(rename = "kaikoNendo")]
    pub kaiko_nendo: i32,
    #[serde(rename = "gakkiNo")]
    pub gakki_no: i32,
    #[serde(rename = "deviceId")]
    pub device_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClassRequest {
    #[serde(rename = "productCd")]
    pub product_cd: String,
    #[serde(rename = "langCd")]
    pub lang_cd: String,
    #[serde(rename = "loginUserId")]
    pub login_user_id: String,
    pub data: ClassRequestData,
    #[serde(rename = "plainLoginPassword")]
    pub plain_login_password: String,
    #[serde(rename = "subProductCd")]
    pub sub_product_cd: String,
    #[serde(rename = "encryptedLoginPassword")]
    pub encrypted_login_password: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
}
