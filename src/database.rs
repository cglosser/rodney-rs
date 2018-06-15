extern crate rusqlite;
use self::rusqlite::Connection;

pub fn open(fname: &str) -> Connection {
    let facts_db = Connection::open(fname).unwrap();
    facts_db
        .execute(
            "CREATE TABLE IF NOT EXISTS facts (
      id        INTEGER PRIMARY KEY AUTOINCREMENT,
      fact      TEXT NOT NULL,
      tidbit    TEXT NOT NULL,
      verb      TEXT NOT NULL DEFAULT 'is',
      regex     INTEGER NOT NULL DEFAULT 0,
      protected INTEGER NOT NULL,
      mood      INTEGER DEFAULT NULL,
      chance    INTEGER DEFAULT NULL,

      CONSTRAINT unique_fact UNIQUE (fact, tidbit, verb)
      )",
            &[],
        )
        .unwrap();

    facts_db
}
