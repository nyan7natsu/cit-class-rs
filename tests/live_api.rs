use std::env;

use cit_class_rs::{get_class_schedule, get_user_info};

#[tokio::test]
#[ignore = "Runs against the real CIT portal API and requires credentials in env vars"]
async fn live_login_and_class_schedule() {
    let user_id = env::var("CIT_USER_ID").expect("CIT_USER_ID is not set");
    let password = env::var("CIT_PASSWORD").expect("CIT_PASSWORD is not set");

    let login = get_user_info(&user_id, &password)
        .await
        .expect("login request failed");

    println!(
        "[live_api] login response:\n{}",
        serde_json::to_string_pretty(&login).expect("failed to serialize login response")
    );

    assert!(
        login.status_dto.success,
        "login did not return success: {:?}",
        login.status_dto.message_list
    );

    let encrypted_password = login
        .data
        .as_ref()
        .map(|d| d.encrypted_password.clone())
        .expect("encrypted_password is missing in login response data");

    let schedule = get_class_schedule(&user_id, &password, &encrypted_password)
        .await
        .expect("class schedule request failed");

    println!(
        "[live_api] class schedule response:\n{}",
        serde_json::to_string_pretty(&schedule).expect("failed to serialize schedule response")
    );

    assert!(
        schedule.status_dto.success,
        "schedule did not return success: {:?}",
        schedule.status_dto.message_list
    );
}
