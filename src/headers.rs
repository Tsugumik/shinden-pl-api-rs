use anyhow::Result;
use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue,
    ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CACHE_CONTROL,
    CONNECTION, CONTENT_TYPE, REFERER,
    UPGRADE_INSECURE_REQUESTS, USER_AGENT,
};

static SEC_FETCH_DEST: HeaderName = HeaderName::from_static("sec-fetch-dest");
static SEC_FETCH_MODE: HeaderName = HeaderName::from_static("sec-fetch-mode");
static SEC_FETCH_SITE: HeaderName = HeaderName::from_static("sec-fetch-site");
static SEC_FETCH_USER: HeaderName = HeaderName::from_static("sec-fetch-user");
static SEC_GPC: HeaderName = HeaderName::from_static("sec-gpc");

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RequestType {
    Frontend,
    Login,
    Api,
}

pub fn get_headers_for_type(req_type: RequestType, url_context: Option<&str>) -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();

    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.71 Safari/537.36"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("pl-PL,pl;q=0.9,en-US;q=0.8,en;q=0.7"));
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));

    match req_type {
        RequestType::Frontend => {
            headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"));
            headers.insert(CACHE_CONTROL, HeaderValue::from_static("max-age=0"));
            headers.insert(SEC_FETCH_DEST.clone(), HeaderValue::from_static("document"));
            headers.insert(SEC_FETCH_MODE.clone(), HeaderValue::from_static("navigate"));
            headers.insert(SEC_FETCH_SITE.clone(), HeaderValue::from_static("same-origin"));
            headers.insert(SEC_FETCH_USER.clone(), HeaderValue::from_static("?1"));
            headers.insert(SEC_GPC.clone(), HeaderValue::from_static("1"));
            headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));

            if let Some(url) = url_context {
                if url.starts_with("https://shinden.pl/") {
                    headers.insert(REFERER, HeaderValue::from_str(url)?);
                } else {
                    headers.insert(REFERER, HeaderValue::from_static("https://shinden.pl/"));
                }
            } else {
                headers.insert(REFERER, HeaderValue::from_static("https://shinden.pl/"));
            }
        }
        RequestType::Login => {
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
            headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
            headers.insert(REFERER, HeaderValue::from_static("https://shinden.pl/main/login"));
            headers.insert(SEC_FETCH_SITE.clone(), HeaderValue::from_static("same-origin"));
            headers.insert(SEC_FETCH_MODE.clone(), HeaderValue::from_static("navigate"));
            headers.insert(SEC_FETCH_DEST.clone(), HeaderValue::from_static("document"));
            headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
        }
        RequestType::Api => {
            headers.insert(ACCEPT, HeaderValue::from_static("application/json, text/plain, */*"));
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            if let Some(url) = url_context {
                if url.starts_with("https://shinden.pl/") {
                    headers.insert(REFERER, HeaderValue::from_str(url)?);
                }
            }
        }
    }

    Ok(headers)
}