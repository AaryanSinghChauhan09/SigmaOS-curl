//! Curl client model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurlOption {
    FollowRedirects,
    Verbose,
    Silent,
    ShowHeaders,
}

#[derive(Debug, Clone)]
pub struct CurlClient {
    pub options: Vec<CurlOption>,
}

impl CurlClient {
    pub fn new() -> Self {
        Self { options: vec![] }
    }

    pub fn with_option(mut self, opt: CurlOption) -> Self {
        self.options.push(opt);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curl_client() {
        let c = CurlClient::new().with_option(CurlOption::Verbose);
        assert!(c.options.contains(&CurlOption::Verbose));
    }
}
