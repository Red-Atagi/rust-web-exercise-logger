use argon2::password_hash::rand_core::{OsRng, RngCore};
use core::convert::TryFrom;
use deadpool_redis::redis::AsyncCommands;
use hex;
use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::keys::SymmetricKey;
use pasetors::token::UntrustedToken;
use pasetors::{local, version4::V4, Local};

use std::env;

const SESSION_KEY_PREFIX: &str = "valid_session_key_for_{}";

pub async fn issue_confirmation_token_pasetors(
    user_id: uuid::Uuid,
    redis_connection: &mut deadpool_redis::redis::aio::Connection,
    is_for_password_reset: bool,
) -> Result<String, deadpool_redis::redis::RedisError> {

    //generate session key
    let sesssion_key: String = {
        let mut buf = [0u8; 128];
        OsRng.fill_bytes(&mut buf); 
        hex::encode(buf)
    };

    //build redis key
    let redis_key = {
        if is_for_password_reset {
            format!("{}{}is_for_password_reset", SESSION_KEY_PREFIX, sesssion_key)
        } else {
            format!("{}{}", SESSION_KEY_PREFIX, sesssion_key)
        }
    };

    //load token expiration from env
    let token_expiration = env::var("TOKEN_EXPIRATION")
        .unwrap().expect("TOKEN_EXPIRATION must be set")
        .parse::<usize>()
        .unwrap();

    //save to redis
    let _: () = redis_connection
        .set(&redis_key, user_id.to_string())
        .await?;

    let time_to_live = {
        if is_for_password_reset {
            chrono::Duration::hours(1).num_seconds()
        } else {
            chrono::Duration::minutes(token_expiration).num_seconds()
        }
    };

    let _: () = redis_connection
        .expire(&redis_key, time_to_live.try_into().unwrap())
        .await?;

    //build token claim
    let mut claims = Claims::new_expires_in(time_to_live).unwrap();
    claims.add_additional("user_id", serde_json::json!(user_id)).unwrap();
    claims.add_additional("session_key", serde_json::json!(sesssion_key)).unwrap();

    //Encrypt with PASETO
    let secret_key = env::var("SECRET_KEY").unwrap().expect("SECRET_KEY must be set");
    let hmac_secret = env::var("HMAC_SECRET").unwrap().expect("HMAC_SECRET must be set");

    let sk = SymmetricKey::from_bytes(secret_key.as_bytes()).unwrap();

    Ok(local::encrypt(
        &sk,
        &claims,
        None,
        Some(hmac_secret.as_bytes()),
    )
    .unwrap())
}

pub async fn verify_confirmation_token_pasetor(
    token: String,
    redis_connection: &mut deadpool_redis::redis::aio::Connection,
    is_password: Option<bool>,
) -> Result<crate::types::ConfirmationToken, String> {
    //decrypt with PASETO
    let secret_key = env::var("SECRET_KEY").unwrap().expect("SECRET_KEY must be set");
    let hmac_secret = env::var("HMAC_SECRET").unwrap().expect("HMAC_SECRET must be set");
    let sk = SymmetricKey::from_bytes(secret_key.as_bytes()).unwrap();

    let untrusted_token = UntrustedToken::<V4, Local>::try_from(token.as_str())
        .map_err(|e| format!("TokenValiation: {}", e))?;

    let validation_rules = ClaimsValidationRules::new();
    let trusted_token = UntrustedToken::decrypt(
        &sk,
        &untrusted_token,
        &validation_rules,
        None,
        Some(hmac_secret.as_bytes()),
    ).map_err(|e| format!("Paseto: {}", e))?;

    //verify claims
    let claims = trusted_token.payload_claims().unwrap();

    let uid = serde_json::to_value(claims.get("user_id")).unwrap().unwrap();

    match serde_json::from_value::<String>(uid) {
        Ok(user_id_string) => {
            match uuid::Uuid::parse_str(&user_id_string) {
                Ok(user_id) => {
                    //get session key from claims
                    let sss_key = serde_json::to_value(claims.get("session_key")).unwrap().unwrap();
                    let session_key = match serde_json::from_value::<String>(sss_key) {
                        Ok(sk) => sk,
                        Err(e) => return Err(format!("SessionKeyParse: {}", e)),
                    };
                    //build redis key
                    let redis_key = {
                        if is_password.is_some() {
                            format!("{}{}is_for_password_reset", SESSION_KEY_PREFIX, sesssion_key)
                        } else {
                            format!("{}{}", SESSION_KEY_PREFIX, session_key)
                        }
                    };
                    //verify token from redis
                    if redis_connection.get::<_, Option<String>>(redis_key.clone()).await.map_err(|e| format!("RedisGet: {}", e))?.is_none() {
                        Err("Invalid or expired token".to_string())
                    }
                    redis_connection.del(redis_key.clone()).await.map_err(|e| format!("RedisDel: {}", e))?;
                    Ok(crate::types::ConfirmationToken { user_id: user_uuid })
                },
                Err(e) => Err(format!("UUIDParse: {}", e)),
            }
        },
        Err(e) => Err(format!("UserIDParse String: {}", e)),
    }
}

