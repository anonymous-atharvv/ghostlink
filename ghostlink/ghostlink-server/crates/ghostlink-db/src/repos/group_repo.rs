use scylla::Session;
use std::sync::Arc;
use uuid::Uuid;

/// Repository for group CRUD and membership.
#[derive(Clone)]
pub struct GroupRepo {
    session: Arc<Session>,
}

impl GroupRepo {
    pub fn new(session: Arc<Session>) -> Self {
        Self { session }
    }

    /// Create a new group.
    pub async fn create(
        &self,
        group_id: Uuid,
        name: &str,
        creator_id: Uuid,
    ) -> anyhow::Result<()> {
        self.session
            .query(
                "INSERT INTO groups (group_id, name, creator_id, created_at) VALUES (?, ?, ?, toTimestamp(now()))",
                (group_id, name, creator_id),
            )
            .await?;
        Ok(())
    }

    /// Add a member to a group.
    pub async fn add_member(
        &self,
        group_id: Uuid,
        member_id: Uuid,
        username: &str,
        role: u8,
    ) -> anyhow::Result<()> {
        self.session
            .query(
                "INSERT INTO group_members (group_id, member_id, username, role, joined_at) VALUES (?, ?, ?, ?, toTimestamp(now()))",
                (group_id, member_id, username, role as i8),
            )
            .await?;
        Ok(())
    }

    /// Remove a member from a group.
    pub async fn remove_member(
        &self,
        group_id: Uuid,
        member_id: Uuid,
    ) -> anyhow::Result<()> {
        self.session
            .query(
                "DELETE FROM group_members WHERE group_id = ? AND member_id = ?",
                (group_id, member_id),
            )
            .await?;
        Ok(())
    }

    /// Delete a group and all its members.
    pub async fn delete(&self, group_id: Uuid) -> anyhow::Result<()> {
        self.session
            .query("DELETE FROM groups WHERE group_id = ?", (group_id,))
            .await?;
        self.session
            .query("DELETE FROM group_members WHERE group_id = ?", (group_id,))
            .await?;
        Ok(())
    }

    /// Count members in a group.
    pub async fn member_count(&self, group_id: Uuid) -> anyhow::Result<i64> {
        let result = self
            .session
            .query(
                "SELECT count(*) FROM group_members WHERE group_id = ?",
                (group_id,),
            )
            .await?;

        match result.rows_typed::<(i64,)>()?.next() {
            Some(Ok((count,))) => Ok(count),
            _ => Ok(0),
        }
    }
}
