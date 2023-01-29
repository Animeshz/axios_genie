use super::Db;
use anyhow::Result;

impl Db {
    pub async fn set_codeforces_id(&self, discord_id: u64, codeforces_id: String) -> Result<()> {
        let discord_id = discord_id as i64;
        let mut conn = self.pool.acquire().await?;

        sqlx::query!("INSERT INTO user_challange_id (discord_id, codeforces_id) values (?1, ?2) ON CONFLICT(discord_id) DO UPDATE SET codeforces_id=?2",
            discord_id, codeforces_id)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn set_typeracer_id(&self, discord_id: u64, typeracer_id: String) -> Result<()> {
        let discord_id = discord_id as i64;
        let mut conn = self.pool.acquire().await?;

        sqlx::query!("INSERT INTO user_challange_id (discord_id, typeracer_id) values (?1, ?2) ON CONFLICT(discord_id) DO UPDATE SET typeracer_id=?2",
            discord_id, typeracer_id)
            .execute(&mut conn)
            .await?;

        Ok(())
    }


    pub async fn get_codeforces_id(&self, discord_id: u64) -> Result<Option<String>> {
        let discord_id = discord_id as i64;
        let mut conn = self.pool.acquire().await?;

        let result = sqlx::query!("SELECT codeforces_id FROM user_challange_id WHERE discord_id=?", discord_id)
            .fetch_optional(&mut conn)
            .await?
            .map(|x| x.codeforces_id);

        Ok(result.unwrap())
    }

    pub async fn get_typeracer_id(&self, discord_id: u64) -> Result<Option<String>> {
        let discord_id = discord_id as i64;
        let mut conn = self.pool.acquire().await?;

        let result = sqlx::query!("SELECT typeracer_id FROM user_challange_id WHERE discord_id=?", discord_id)
            .fetch_optional(&mut conn)
            .await?
            .map(|x| x.typeracer_id);

            Ok(result.unwrap())
    }

}
