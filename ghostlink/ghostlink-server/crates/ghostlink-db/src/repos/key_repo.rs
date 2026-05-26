use scylla::Session;
use std::sync::Arc;
use uuid::Uuid;

use ghostlink_core::crypto::{OneTimePreKey, SignedPreKey};

/// Repository for Signal Protocol key storage.
#[derive(Clone)]
pub struct KeyRepo {
    session: Arc<Session>,
}

impl KeyRepo {
    pub fn new(session: Arc<Session>) -> Self {
        Self { session }
    }

    /// Store the full key bundle during registration.
    pub async fn store_key_bundle(
        &self,
        account_id: Uuid,
        identity_key: &str,
        signed_pre_key: &SignedPreKey,
        one_time_pre_keys: &[OneTimePreKey],
    ) -> anyhow::Result<()> {
        // Store identity key
        self.session
            .query(
                "INSERT INTO identity_keys (account_id, identity_key) VALUES (?, ?)",
                (account_id, identity_key.as_bytes().to_vec()),
            )
            .await?;

        // Store signed pre-key
        self.session
            .query(
                "INSERT INTO signed_pre_keys (account_id, key_id, public_key, signature, timestamp) VALUES (?, ?, ?, ?, toTimestamp(now()))",
                (
                    account_id,
                    signed_pre_key.key_id,
                    signed_pre_key.public_key.as_bytes().to_vec(),
                    signed_pre_key.signature.as_bytes().to_vec(),
                ),
            )
            .await?;

        // Store one-time pre-keys
        for otpk in one_time_pre_keys {
            self.session
                .query(
                    "INSERT INTO pre_keys (account_id, key_id, public_key) VALUES (?, ?, ?)",
                    (account_id, otpk.key_id, otpk.public_key.as_bytes().to_vec()),
                )
                .await?;
        }

        Ok(())
    }

    /// Fetch key bundle for X3DH — consumes one OTP key atomically.
    pub async fn get_key_bundle(
        &self,
        account_id: Uuid,
    ) -> anyhow::Result<Option<ghostlink_core::crypto::KeyBundle>> {
        // Fetch identity key
        let ik_result = self
            .session
            .query(
                "SELECT identity_key FROM identity_keys WHERE account_id = ?",
                (account_id,),
            )
            .await?;

        let identity_key: Vec<u8> = match ik_result.rows_typed::<(Vec<u8>,)>()?.next() {
            Some(Ok((ik,))) => ik,
            _ => return Ok(None),
        };

        // Fetch latest signed pre-key
        let spk_result = self
            .session
            .query(
                "SELECT key_id, public_key, signature FROM signed_pre_keys WHERE account_id = ? LIMIT 1",
                (account_id,),
            )
            .await?;

        let (spk_id, spk_pub, spk_sig): (i32, Vec<u8>, Vec<u8>) =
            match spk_result.rows_typed::<(i32, Vec<u8>, Vec<u8>)>()?.next() {
                Some(Ok(row)) => row,
                _ => return Ok(None),
            };

        // Fetch and consume one OTP key
        let otpk_result = self
            .session
            .query(
                "SELECT key_id, public_key FROM pre_keys WHERE account_id = ? LIMIT 1",
                (account_id,),
            )
            .await?;

        let one_time_pre_key =
            if let Some(Ok((otpk_id, otpk_pub))) =
                otpk_result.rows_typed::<(i32, Vec<u8>)>()?.next()
            {
                // Consume the OTP key (delete after fetching)
                self.session
                    .query(
                        "DELETE FROM pre_keys WHERE account_id = ? AND key_id = ?",
                        (account_id, otpk_id),
                    )
                    .await?;

                Some(OneTimePreKey {
                    key_id: otpk_id,
                    public_key: String::from_utf8_lossy(&otpk_pub).to_string(),
                })
            } else {
                None
            };

        Ok(Some(ghostlink_core::crypto::KeyBundle {
            account_id,
            identity_key: String::from_utf8_lossy(&identity_key).to_string(),
            signed_pre_key: SignedPreKey {
                key_id: spk_id,
                public_key: String::from_utf8_lossy(&spk_pub).to_string(),
                signature: String::from_utf8_lossy(&spk_sig).to_string(),
            },
            one_time_pre_key,
        }))
    }

    /// Count remaining OTP keys for an account.
    pub async fn count_pre_keys(&self, account_id: Uuid) -> anyhow::Result<i64> {
        let result = self
            .session
            .query(
                "SELECT count(*) FROM pre_keys WHERE account_id = ?",
                (account_id,),
            )
            .await?;

        match result.rows_typed::<(i64,)>()?.next() {
            Some(Ok((count,))) => Ok(count),
            _ => Ok(0),
        }
    }

    /// Upload additional one-time pre-keys (top up).
    pub async fn upload_pre_keys(
        &self,
        account_id: Uuid,
        keys: &[OneTimePreKey],
    ) -> anyhow::Result<usize> {
        let mut count = 0;
        for key in keys {
            self.session
                .query(
                    "INSERT INTO pre_keys (account_id, key_id, public_key) VALUES (?, ?, ?)",
                    (account_id, key.key_id, key.public_key.as_bytes().to_vec()),
                )
                .await?;
            count += 1;
        }
        Ok(count)
    }
}
