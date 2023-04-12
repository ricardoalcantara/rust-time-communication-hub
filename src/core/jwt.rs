pub struct Jwt {
    secret: Vec<u8>,
    algorithm: jsonwebtoken::Algorithm,
}

impl Jwt {
    pub fn from_env(name: &str) -> Self {
        let secret = std::env::var(name).unwrap_or_else(|_| panic!("{} must be set", name));
        let algorithm = jsonwebtoken::Algorithm::HS512;
        Jwt {
            secret: secret.into_bytes(),
            algorithm,
        }
    }

    pub fn new() -> Self {
        Jwt::from_env("JWT_SECRET")
    }

    pub fn encode<C: serde::Serialize>(
        &self,
        claims: C,
    ) -> std::result::Result<String, jsonwebtoken::errors::Error> {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(self.algorithm),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(&self.secret),
        )?;

        Ok(token)
    }

    pub fn decode<T>(&self, token: &str) -> std::result::Result<T, jsonwebtoken::errors::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let token_data = jsonwebtoken::decode::<T>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(&self.secret),
            &jsonwebtoken::Validation::new(self.algorithm),
        )?;

        Ok(token_data.claims)
    }
}
