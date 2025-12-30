use thiserror::Error;

use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;

#[derive(Debug, Error)]
pub enum SignatureError {
    #[error("Signature mismatch")]
    Mismatch,

    #[error("Key error: {0}")]
    KeyError(String),

    #[error("Signature decode error")]
    DecodeError,
}

/// Generate a fresh ed25519 keypair (public, secret bytes)
pub fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
    let mut csprng = OsRng;
    let kp: Keypair = Keypair::generate(&mut csprng);
    (kp.public.to_bytes().to_vec(), kp.secret.to_bytes().to_vec())
}

/// Sign a message using secret + public key bytes (both 32 bytes each)
pub fn sign_with_keypair(message: &str, secret: &[u8], public: &[u8]) -> Result<Vec<u8>, SignatureError> {
    if secret.len() != 32 || public.len() != 32 {
        return Err(SignatureError::KeyError("invalid key length".into()));
    }
    let secret = SecretKey::from_bytes(secret).map_err(|e| SignatureError::KeyError(e.to_string()))?;
    let public = PublicKey::from_bytes(public).map_err(|e| SignatureError::KeyError(e.to_string()))?;
    let kp = Keypair { secret, public };
    let sig: Signature = kp.sign(message.as_bytes());
    Ok(sig.to_bytes().to_vec())
}

/// Verify a signature bytes against a message and public key bytes
pub fn verify_signature_ed25519(message: &str, signature: &[u8], public: &[u8]) -> Result<bool, SignatureError> {
    if public.len() != 32 {
        return Err(SignatureError::KeyError("invalid public key length".into()));
    }
    let public = PublicKey::from_bytes(public).map_err(|e| SignatureError::KeyError(e.to_string()))?;
    let sig = Signature::from_bytes(signature).map_err(|_| SignatureError::DecodeError)?;
    public.verify(message.as_bytes(), &sig).map_err(|_| SignatureError::Mismatch)?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_verify_roundtrip() {
        let (pubk, seck) = generate_keypair();
        let msg = "hello";
        let sig = sign_with_keypair(msg, &seck, &pubk).unwrap();
        assert!(verify_signature_ed25519(msg, &sig, &pubk).unwrap());
    }

    #[test]
    fn verify_fail_bad_sig() {
        let (pubk, seck) = generate_keypair();
        let msg = "hello";
        let mut sig = sign_with_keypair(msg, &seck, &pubk).unwrap();
        // Corrupt signature
        sig[0] ^= 0xff;
        assert!(verify_signature_ed25519(msg, &sig, &pubk).is_err());
    }
}
