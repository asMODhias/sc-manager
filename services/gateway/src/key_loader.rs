use base64::engine::general_purpose::STANDARD as base64_std;
use base64::Engine as _;
use std::fs;

/// Load Keypair in order:
/// 1. GATEWAY_PRIVATE_KEY_FILE -> raw bytes
/// 2. GATEWAY_PRIVATE_KEY -> base64 of 32 bytes
/// 3. fallback: deterministic test keypair (for local dev/test only)
pub fn load_gateway_keypair() -> Result<sc_manager_core::events::KeyPair, String> {
    if let Ok(path) = std::env::var("GATEWAY_PRIVATE_KEY_FILE") {
        let data = fs::read(&path).map_err(|e| format!("read file: {}", e))?;
        return parse_key_bytes(&data);
    }

    if let Ok(b64) = std::env::var("GATEWAY_PRIVATE_KEY") {
        let bytes = base64_std.decode(b64).map_err(|e| format!("base64 decode: {}", e))?;
        return parse_key_bytes(&bytes);
    }

    sc_manager_core::events::generate_test_keypair()
}

fn parse_key_bytes(bytes: &[u8]) -> Result<sc_manager_core::events::KeyPair, String> {
    if bytes.len() == 32 {
        Ok(sc_manager_core::events::KeyPair::from_secret_bytes(bytes).map_err(|e| format!("secretkey: {}", e))?)
    } else {
        Err(format!("invalid key length: {}", bytes.len()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn loads_env_b64_key() {
        let seed = [1u8; 32];
        let b64 = base64::engine::general_purpose::STANDARD.encode(&seed);
        env::set_var("GATEWAY_PRIVATE_KEY", b64);
        let kp = load_gateway_keypair().expect("load");
        assert_eq!(kp.secret_bytes, seed);
        env::remove_var("GATEWAY_PRIVATE_KEY");
    }

    #[test]
    fn fallback_deterministic() {
        env::remove_var("GATEWAY_PRIVATE_KEY");
        env::remove_var("GATEWAY_PRIVATE_KEY_FILE");
        let kp = load_gateway_keypair().expect("load");
        let expected = sc_manager_core::events::generate_test_keypair().expect("generate test keypair");
        assert_eq!(kp.public_key_b64, expected.public_key_b64);
    }
}
