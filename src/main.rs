use std::{path::PathBuf, env, time::{SystemTime, UNIX_EPOCH, Duration}};

use clap::{Parser, Subcommand};
use diesel::{ExpressionMethods, SqliteConnection, Connection, RunQueryDsl, QueryDsl};
use dotenvy::dotenv;
use totp_rs::{Algorithm, TOTP, Secret};
mod schema;
mod models;
mod error;


use error::TOTPError;
use crate::{error::Result, models::{NewTotp, Totp}};
use crate::schema::totp_keys;
#[macro_use]
extern crate log;

fn get_default_data_path() -> Result<PathBuf> {
    env::current_dir().map(|path| path.join("data.db")).map_err(TOTPError::Io)
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(env = "TOTP_DATABASE_URL", short, long)]
    data_path: Option<PathBuf>,

    #[arg(value_name = "LogLevel",
        long,
        default_value = "info",
    )]
    log_level: String,

    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    /// Add Key
    Add {
        /// Key Label
        taget: String,
        /// Key
        secret_key: String,
    },
    /// List ALL Key
    List {
        /// Key ID in sqlite
        id: Option<i32>,

        /// Show Key
        #[arg(short, long, default_value_t = false)]
        show_key: bool,
    },
    /// Show Code
    Show {
        /// Key ID in sqlite
        id: i32,

        /// Key ID in sqlite
        #[arg(short, long, default_value_t = 6)]
        digits: usize,
        
        /// Key ID in sqlite
        #[arg(short, long, default_value_t = 30)]
        step: u64,
    },
    // Remove {
    //     id: i32,
    // },
}

pub fn establish_connection(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


fn main() -> Result<()> {
    dotenv().ok();
    let args = Args::parse();
    env::set_var("RUST_LOG", &args.log_level);
    env_logger::init();
    info!("args {:?}", args);

    let data_path = args.data_path.unwrap_or_else(|| get_default_data_path().unwrap());
    info!("sqlite path {:?}", data_path);

    let mut conn = establish_connection(&data_path.to_string_lossy());

    match args.action {
        Action::Add { taget, secret_key } => {
            let new_totp = NewTotp {
                taget,
                secret_key,
            };
        diesel::insert_into(totp_keys::table)
            .values(&new_totp)
            .execute(&mut conn)
            .expect("Error saving new totp");
        },
        Action::List { id , show_key}=> {
            let table = totp_keys::dsl::totp_keys;
            let mut query = table.into_boxed();
            if let Some(id) = id {
                query = query.filter(totp_keys::dsl::id.eq(id));
            }
            let results = query.load::<Totp>(&mut conn).expect("Error list totp");

            info!("record size: {:?}", results.len());
            println!("{:^20} | {:^20} | {:^20}", "ID", "LABEL", "KEY");
            for totp in results {
                println!("{:^20} | {:^20} | {:^20}", totp.id, totp.taget, match show_key {
                    true => totp.secret_key,
                    false => "***".to_owned(),
                });
            }
        },
        Action::Show { id , digits, step} => {
            let totp_info = totp_keys::dsl::totp_keys
                .filter(totp_keys::dsl::id.eq(id))
                .first::<Totp>(&mut conn)
                .expect("Error list totp");
            debug!("find record: {:?}", totp_info);
            // let a = Secret::Encoded(totp_info.secret_key.to_owned()).to_bytes().unwrap();
            // debug!("{:?}", a);
            let totp = TOTP::new_unchecked(
                Algorithm::SHA1,
                digits,
                1,
                step,
                Secret::Encoded(totp_info.secret_key.to_owned()).to_bytes().unwrap(),
            );
            for _ in 0..10 {
                let token = totp.generate_current().unwrap();
                println!("label: {:?}, token is: {}", totp_info.taget, token);
                let time = totp.next_step_current().map_or(10, |time| {
                    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                    time - now
                });
                std::thread::sleep(Duration::from_secs(time));
            }
        },
        // Action::Remove { id } => todo!(),
    }

    Ok(())
}
