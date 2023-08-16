use reqwest::{
    header::{COOKIE, SET_COOKIE},
    Client,
};
use serde::{Deserialize, Serialize};

#[tokio::test]
async fn force() {
    let client = Client::new();
    let res = client
        .post("https://www.zhihu.com/api/v4/questions/267165403/draft")
        .body(include_str!("data.json"))
        .header(COOKIE, include_str!("cookie.txt"))
        .send()
        .await
        .expect("fail send");
    let out = res.json::<Response>().await.expect("fail read");
    println!("{:#?}", out)
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    pub created_time: i64,
    pub updated_time: i64,
    pub content: String,
    pub title: Struct,
    pub excerpt: String,
    pub editable_content: String,
    pub url: String,
    pub r#type: String,
    pub answer_type: String,
    pub draft_type: String,
    pub settings: Settings,
}

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    pub disclaimer_status: String,
    pub disclaimer_type: String,
    pub comment_permission: String,
    pub is_copyable: bool,
    pub reshipment_settings: String,
    pub can_reward: bool,
    pub push_activity: bool,
    pub commercial_report_info: CommercialReportInfo,
    pub thank_inviter: String,
    pub thank_inviter_status: String,
    pub table_of_contents_enabled: bool,
    pub table_of_contents: Struct,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommercialReportInfo {
    pub is_report: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Struct {
    pub enabled: bool,
}
