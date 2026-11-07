//! Authentication model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthMethod { None, Basic, Bearer, Digest, Ntlm }

#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credentials() {
        let c = Credentials { username: "user".into(), password: "pass".into() };
        assert_eq!(c.username, "user");
    }
}
