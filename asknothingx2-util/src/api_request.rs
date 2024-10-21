use http::{header::CONTENT_TYPE, HeaderMap};

use crate::{
    tqpe::{CONTENT_TYPE_FORMENCODED, CONTENT_TYPE_JSON},
    treit::APIRequest,
};

pub async fn oauth_request<T: APIRequest>(request: T) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let headers = request.headers();
    let client = client.request(request.method(), request.url());

    let client = if check_header_json(&headers) && request.json().is_some() {
        client.headers(headers).body(request.json().unwrap())
    } else if check_header_formenecoded(&headers) && request.urlencoded().is_some() {
        client.headers(headers).body(request.urlencoded().unwrap())
    } else {
        client
    };

    let response = client.send().await?;

    Ok(response)
}

pub fn check_header_json(headers: &HeaderMap) -> bool {
    headers.get(CONTENT_TYPE).is_some() && headers.get(CONTENT_TYPE).unwrap() == CONTENT_TYPE_JSON()
}
pub fn check_header_formenecoded(headers: &HeaderMap) -> bool {
    headers.get(CONTENT_TYPE).is_some()
        && headers.get(CONTENT_TYPE).unwrap() == CONTENT_TYPE_FORMENCODED()
}

#[cfg(test)]
mod tests {

    use http::{header::CONTENT_TYPE, HeaderMap};

    use crate::{
        check_header_formenecoded, check_header_json,
        tqpe::{CONTENT_TYPE_FORMENCODED, CONTENT_TYPE_JSON},
    };

    #[test]
    fn check_header() {
        let mut headers1 = HeaderMap::new();
        headers1.append(CONTENT_TYPE, CONTENT_TYPE_JSON());

        let mut headers2 = HeaderMap::new();
        headers2.append(CONTENT_TYPE, CONTENT_TYPE_FORMENCODED());

        assert!(check_header_json(&headers1));
        assert!(check_header_formenecoded(&headers2));
    }
}
