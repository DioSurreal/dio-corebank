use anyhow::Result;


use crate::config::config_model::{CustomersSecret, OperatorsSecret};

use super::stage::Stage;

use super::config_model::{DotEnvyConfig, Server,Database};


pub fn load() ->Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();


    let server = Server {
        port: std::env::var("SERVER_PORT").expect("SERVER_PORT is invalid").parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT").expect("SERVER_BODY_LIMIT is invalid").parse()?,
        timeout:std::env::var("SERVER_TIMEOUT").expect("SERVER_TIMEOUT is invalid").parse()?,

    };
    let database =  Database {
        url : std::env::var("DATABASE_URL").expect("DATABASE_URL is invalid"),
    };

    Ok(DotEnvyConfig{server,database})
}

pub fn get_stage() -> Stage {
        dotenvy::dotenv().ok();
        let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
        Stage::try_form(&stage_str).unwrap_or_default()
}

pub fn get_adventurers_secret_env() -> Result<CustomersSecret>{
        dotenvy::dotenv().ok();

        Ok(CustomersSecret{
            secret: std::env::var("JWT_ADVENTURER_SECRET").expect("JWT_ADVENTURER_SECRET is invalid"),
            refresh_secret: std::env::var("JWT_ADVENTURER_REFRESH_SECRET").expect("JWT_ADVENTURER_REFRESH_SECRET is invalid"),
        })
}
pub fn get_guild_commanders_secret_env() -> Result<OperatorsSecret>{
        dotenvy::dotenv().ok();

        Ok(OperatorsSecret{
            secret: std::env::var("JWT_GUILD_COMMANDER_SECRET").expect("JWT_GUILD_COMMANDER_SECRET is invalid"),
            refresh_secret: std::env::var("JWT_GUILD_COMMANDER_REFRESH_SECRET").expect("JWT_GUILD_COMMANDER_REFRESH_SECRET is invalid"),
        })
}

