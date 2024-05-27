use restson::{Error, RestClient, RestPath};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct HttpBinDelete {
    data: String,
}

#[derive(Deserialize)]
struct HttpBinDeleteResp {
    json: HttpBinDelete,
    url: String,
}

impl RestPath<()> for HttpBinDelete {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from("delete"))
    }
}


#[tokio::test]
async fn basic_delete() {
    let client = RestClient::new("http://httpbin.org").unwrap();

    client.delete::<(), HttpBinDelete>(()).await.unwrap();
}

#[tokio::test]
async fn delete_with() {
    let client = RestClient::new("http://httpbin.org").unwrap();

    let params = vec![("a", "2"), ("b", "abcd")];
    let data = HttpBinDelete {
        data: String::from("test data"),
    };
    client.delete_with((), &data, &params).await.unwrap();

    client.delete_with((), &data, &vec![]).await.unwrap();
}

#[tokio::test]
async fn delete_capture() {
    let client = RestClient::new("https://httpbin.org").unwrap();

    let data = HttpBinDelete {
        data: String::from("test data"),
    };
    let resp = client.delete_capture::<_, _, HttpBinDeleteResp>((), &data).await.unwrap();

    assert_eq!(resp.json.data, "test data");
    assert_eq!(resp.url, "https://httpbin.org/delete");
}

#[tokio::test]
async fn delete_capture_query_params() {
    let client = RestClient::new("https://httpbin.org").unwrap();

    let params = vec![("a", "2"), ("b", "abcd")];
    let data = HttpBinDelete {
        data: String::from("test data"),
    };
    let resp =
        client.delete_capture_with::<_, _, HttpBinDeleteResp>((), &data, &params).await.unwrap();

    assert_eq!(resp.json.data, "test data");
    assert_eq!(resp.url, "https://httpbin.org/delete?a=2&b=abcd");
}