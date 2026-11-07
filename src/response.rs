//! Curl response model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurlStatus { Success, ConnectionError, Timeout, HttpError(u16) }

#[derive(Debug, Clone)]
pub struct CurlResponse {
    pub status: CurlStatus,
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curl_response() {
        let r = CurlResponse { status: CurlStatus::Success, status_code: 200, headers: vec![], body: vec![] };
        assert_eq!(r.status_code, 200);
    }
}
