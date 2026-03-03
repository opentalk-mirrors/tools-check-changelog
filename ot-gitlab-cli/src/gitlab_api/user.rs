use gitlab::{
    RestError,
    api::{Client, Query as _, users},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub name: String,
    // pub state: "active",
    // pub locked: false,
    // pub avatar_url: "http://localhost:3000/uploads/user/avatar/1/index.jpg",
    // pub web_url: "http://localhost:3000/john_smith",
    // pub created_at: "2012-05-23T08:00:58Z",
    // pub bio: "",
    // pub location: null,
    // pub public_email: "john@example.com",
    // pub skype: "",
    // pub linkedin: "",
    // pub twitter: "",
    // pub discord: "",
    // pub website_url: "",
    // pub organization: "",
    // pub job_title: "",
    // pub pronouns: "he/him",
    // pub bot: false,
    // pub work_information: null,
    // pub followers: 0,
    // pub following: 0,
    // pub local_time: "3:38 PM",
    // pub last_sign_in_at: "2012-06-01T11:41:01Z",
    // pub confirmed_at: "2012-05-23T09:05:22Z",
    // pub theme_id: 1,
    // pub last_activity_on: "2012-05-23",
    // pub color_scheme_id: 2,
    // pub projects_limit: 100,
    // pub current_sign_in_at: "2012-06-02T06:36:55Z",
    // pub identities: [],
    // pub can_create_group: true,
    // pub can_create_project: true,
    // pub two_factor_enabled: true,
    // pub external: false,
    // pub private_profile: false,
    // pub commit_email: "admin@example.com",
}

pub fn current_user<C: Client<Error = RestError>>(client: &C) -> anyhow::Result<User> {
    let call = users::CurrentUser::builder().build()?;

    let response: serde_json::Value = call.query(client)?;
    log::trace!("response: {response:#?}");
    let user = serde_json::from_value::<User>(response)?;
    Ok(user)
}
