use tiberius::{AuthMethod, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let server = std::env::var("DB_HOST")?;
        let mut config = Config::new();
        config.host(server);
        config.database("master");
        config.authentication(AuthMethod::Integrated);
        #[cfg(target_os = "windows")]
        {
            config.trust_cert();
        }

        let mgr = bb8_tiberius::ConnectionManager::build::<Config>(config)?;

        let pool = bb8::Pool::builder().max_size(2).build(mgr).await?;
    
        let mut conn = pool.get().await?;
    
        let res = conn
            .simple_query("SELECT @@version")
            .await?
            .into_first_result()
            .await?
            .into_iter()
            .map(|row| {
                let val: &str = row.get(0).unwrap();
                String::from(val)
            })
            .collect::<Vec<_>>();
    
        println!("{:?}", &res);
    
        Ok(())
}
