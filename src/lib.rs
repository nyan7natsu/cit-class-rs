pub mod types;

use reqwest;
use std::error::Error;

const MAINTENANCE_MESSAGE: &str =
    "申し訳ございません。ただいまサービス停止中です。（毎日 AM2：00～5：00）";

const LOGIN_URL: &str = "https://portal.chibatech.ac.jp/uprx/webapi/up/pk/Pky001Resource/login";
const CLASS_URL: &str =
    "https://portal.chibatech.ac.jp/uprx/webapi/up/ap/Apa004Resource/getJugyoKeijiMenuInfo";

pub async fn get_user_info(
    user_id: &str,
    password: &str,
) -> Result<types::LoginResponse, Box<dyn Error + Send + Sync>> {
    let body: types::LoginRequest = types::LoginRequest {
        data: types::LoginRequestData {
            login_user_id: user_id.to_string(),
            plain_login_password: password.to_string(),
        },
    };

    let client = reqwest::Client::new();
    let res = client
        .post(LOGIN_URL)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body)?)
        .send()
        .await?;
    let status = res.status();

    let body = res.text().await?;
    let decoded_body = urlencoding::decode(&body)?.into_owned();

    if status == 503 || decoded_body.contains(MAINTENANCE_MESSAGE) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "現在CITポータルは定期メンテナンス中です",
        )));
    }

    let parsed: types::LoginResponse = serde_json::from_str(&decoded_body)?;

    Ok(parsed)
}

pub async fn get_class_schedule(
    user_id: &str,
    password: &str,
    encrypted_password: &str,
) -> Result<types::ClassScheduleResponse, Box<dyn Error + Send + Sync>> {
    let body: types::ClassRequest = types::ClassRequest {
        product_cd: "ap".to_string(),
        lang_cd: "".to_string(),
        login_user_id: user_id.to_string(),
        data: types::ClassRequestData {
            auto_login_auth_cd: "".to_string(),
            kaiko_nendo: 0,
            gakki_no: 0,
            device_id: "".to_string(),
        },
        plain_login_password: password.to_string(),
        sub_product_cd: "apa".to_string(),
        encrypted_login_password: encrypted_password.to_string(),
        device_id: "".to_string(),
    };

    let client = reqwest::Client::new();
    let res = client
        .post(CLASS_URL)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body)?)
        .send()
        .await?;
    let status = res.status();

    let body = res.text().await?;
    let decoded_body = urlencoding::decode(&body)?.into_owned();

    if status == 503 || decoded_body.contains(MAINTENANCE_MESSAGE) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "現在CITポータルは定期メンテナンス中です",
        )));
    }

    let parsed: types::ClassScheduleResponse = serde_json::from_str(&decoded_body)?;

    Ok(parsed)
}
