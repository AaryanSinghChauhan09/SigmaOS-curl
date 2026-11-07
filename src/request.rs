//! Curl request model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurlMethod { Get, Post, Put, Delete, Head, Options, Patch }

#[derive(Debug, Clone)]
pub struct CurlRequest {
    pub method: CurlMethod,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curl_request() {
        let r = CurlRequest { method: CurlMethod::Get, url: "https://example.com".into(), headers: vec![], body: None };
        assert_eq!(r.method, CurlMethod::Get);
    }
}
