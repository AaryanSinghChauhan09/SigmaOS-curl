//! Cookie model.

#[derive(Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub secure: bool,
    pub http_only: bool,
}

#[derive(Debug, Clone)]
pub struct CookieJar {
    pub cookies: Vec<Cookie>,
}

impl CookieJar {
    pub fn new() -> Self {
        Self { cookies: vec![] }
    }

    pub fn add(&mut self, cookie: Cookie) {
        self.cookies.push(cookie);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cookie_jar() {
        let mut jar = CookieJar::new();
        jar.add(Cookie { name: "session".into(), value: "abc".into(), domain: "example.com".into(), path: "/".into(), secure: true, http_only: true });
        assert_eq!(jar.cookies.len(), 1);
    }
}
