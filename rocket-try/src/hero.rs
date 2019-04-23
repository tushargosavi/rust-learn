

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::schema::heroes;

#[table_name = "heroes" ]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Hero {
    pub id : Option<i32>,
    pub name : String,
    pub identity : String,
    pub hometown : String,
    pub age : i32
}

impl Hero {
    pub fn create(hero: Hero, conn : &PgConnection) -> Hero {
        diesel::insert_into(heroes::table)
            .values(&hero)
            .execute(conn)
            .expect("Error creating new hero");

        heroes::table.order(heroes::id.desc()).first(conn).unwrap()
    }

    pub fn read(conn : &PgConnection) -> Vec<Hero> {
        heroes::table.order(heroes::id.asc()).load::<Hero>(conn).unwrap()
    }

    pub fn update(id : i32, hero: Hero, conn : &PgConnection) -> bool {
        diesel::update(heroes::table.find(id)).set(&hero).execute(conn).is_ok()
    }

    pub fn delete(id: i32, conn : &PgConnection) -> bool {
        diesel::delete(heroes::table.find(id)).execute(conn).is_ok()
    }
}