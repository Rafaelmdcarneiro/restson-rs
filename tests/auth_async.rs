use restson::{Error, RestClient, RestPath};
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct HttpBinBasicAuth {}

impl<'a> RestPath<(&'a str, &'a str)> for HttpBinBasicAuth {
    fn get_path(auth: (&str, &str)) -> Result<String, Error> {
        let (user, pass) = auth;
        Ok(format!("basic-auth/{}/{}", user, pass))
    }
}

#[tokio::test]
async fn basic_auth() {
    let mut client = RestClient::new("http://httpbin.org").unwrap();

    client.set_auth("username", "passwd");
    client
        .get::<_, HttpBinBasicAuth>(("username", "passwd"))
        .await
        .unwrap();
}

#[tokio::test]
async fn basic_auth_fail() {
    let mut client = RestClient::new("http://httpbin.org").unwrap();

    client.set_auth("username", "wrong_passwd");
    match client.get::<_, HttpBinBasicAuth>(("username", "passwd")).await {
        Err(Error::HttpError(s, _)) if s == 401 || s == 403 => (),
        _ => panic!("Expected Unauthorized/Forbidden HTTP error"),
    };
}
