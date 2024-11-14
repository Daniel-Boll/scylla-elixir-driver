use std::{borrow::Borrow, ops::Deref, panic::RefUnwindSafe};

use lazy_static::lazy_static;
use rustler::{Env, NifResult, NifStruct, ResourceArc, Term};
use scylla::{SessionBuilder, prepared_statement::PreparedStatement};
use tokio::runtime::Runtime;

lazy_static! {
  pub static ref RUNTIME: Runtime = Runtime::new().unwrap();
}

pub(crate) struct SessionRef(scylla::Session);

#[derive(NifStruct)]
#[module = "Scylla.Session"]
pub struct Session {
  session: ResourceArc<SessionRef>,
}

impl RefUnwindSafe for Session {}

impl Deref for Session {
  type Target = scylla::Session;

  fn deref(&self) -> &Self::Target {
    &self.session.0
  }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn start_link(uri: String) -> NifResult<Session> {
  RUNTIME.block_on(async {
    let mut builder = SessionBuilder::new();

    builder = builder.known_node(uri);

    let session = builder
      .build()
      .await
      .map_err(|e| rustler::Error::Term(Box::new(e.to_string())))?;

    Ok(Session {
      session: ResourceArc::new(SessionRef(session)),
    })
  })
}

#[rustler::nif(schedule = "DirtyCpu")]
fn execute(session: Session, query: String) -> bool {
  RUNTIME.block_on(async {
    let prepared = session.prepare(query).await.unwrap();
    let result = session.execute_unpaged(&prepared, &[]).await.unwrap();

    println!("{result:?}");

    true
  })
}

fn load(env: Env, _info: Term) -> bool {
  rustler::resource!(SessionRef, env);
  true
}

rustler::init!("Elixir.ScyllaElixirDriver", load = load);
