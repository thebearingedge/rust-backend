use actix_web::actix::{Actor, Addr, SyncArbiter, SyncContext};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use std::env;

pub struct DbActor {
    pub conn: Pool<ConnectionManager<PgConnection>>,
}

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

pub fn get_addr() -> Addr<DbActor> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let db_pool = Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(db_url))
        .expect("Failed to create database connection pool.");

    SyncArbiter::start(num_cpus::get(), move || DbActor {
        conn: db_pool.clone(),
    })
}

pub mod functions {
    use diesel::sql_types::Text;

    sql_function!(fn lower(x: Text) -> Text);
}
