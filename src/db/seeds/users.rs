use crate::models::NewUser;

pub fn users<'a>() -> Vec<NewUser<'a>> {
    vec![
        NewUser {
            email: "miles.davis@trumpet.com",
            password: password_auth::generate_hash("passw0rd"),
        },
        NewUser {
            email: "marcus.miller@bass.com",
            password: password_auth::generate_hash("secr3t"),
        }
    ]
}