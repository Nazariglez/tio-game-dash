use chrono::{NaiveDateTime, Utc};
use schema::administrator_sessions;

#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct AdminSession {
    pub id: i32,
    pub administrator_id: i32,
    pub is_valid: bool,
    pub expire_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Insertable)]
#[table_name = "administrator_sessions"]
pub struct NewAdminSession {
    pub administrator_id: i32,
    pub is_valid: bool,
    pub expire_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

impl NewAdminSession {
    pub fn new(admin_id: i32, is_valid:bool, expire_at: NaiveDateTime) -> NewAdminSession {
        NewAdminSession {
            administrator_id: admin_id,
            is_valid: is_valid,
            expire_at: expire_at,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(AsChangeset)]
#[table_name = "administrator_sessions"]
pub struct EditAdminSession {
    pub administrator_id: i32,
    pub is_valid: bool,
    pub expire_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime
}

impl EditAdminSession {
    pub fn new(admin_id: i32, is_valid:bool, expire_at: Option<NaiveDateTime>) -> EditAdminSession {
        EditAdminSession {
            administrator_id: admin_id,
            is_valid: is_valid,
            expire_at: expire_at,
            updated_at: Utc::now().naive_utc()
        }
    }
}


// -- actix handlers
pub mod handlers {
    use super::*;
    use diesel;
    use diesel::prelude::*;
    use actix::{Message, Handler};
    use actix_web::Error;
    use actix_web::error::*;
    use ConnDsl;
    use models::negotiate_error;

    //-- Create or Update a session
    #[derive(Serialize, Deserialize, Debug)]
    pub struct SetAdminSession {
        pub admin_id: i32,
        pub is_valid: bool,
        pub expire_at: Option<NaiveDateTime>
    }

    impl Message for SetAdminSession {
        type Result = Result<(), Error>;
    }

    impl Handler<SetAdminSession> for ConnDsl {
        type Result = Result<(), Error>;

        fn handle(&mut self, set_session: SetAdminSession, _: &mut Self::Context) -> Self::Result {
            use schema::administrator_sessions;
            
            let conn = &self.0.get().map_err(ErrorInternalServerError)?;

            let o_session = administrator_sessions::table
                .filter(administrator_sessions::dsl::administrator_id.eq(set_session.admin_id))
                .get_result::<AdminSession>(conn)
                .optional()
                .map_err(negotiate_error)?;

            if let Some(session) = o_session {
                //update
                let target = administrator_sessions::table
                    .filter(administrator_sessions::dsl::id.eq(session.id));

                diesel::update(target)
                    .set(&EditAdminSession::new(
                        set_session.admin_id, 
                        set_session.is_valid,
                        set_session.expire_at
                    ))
                    .execute(conn)
                    .map_err(negotiate_error)?;

            } else {
                //create
                let new_session = NewAdminSession::new(
                    set_session.admin_id, 
                    set_session.is_valid, 
                    set_session.expire_at.unwrap_or(Utc::now().naive_utc())
                );

                diesel::insert_into(administrator_sessions::table)
                    .values(new_session)
                    .execute(conn)
                    .map_err(negotiate_error)?;
            }

            Ok(())
        }
    }

    //-- Check if a session is valid
    #[derive(Serialize, Deserialize, Debug)]
    pub struct IsValidAdminSession {
        pub admin_id: i32,
        pub update: bool
    }

    impl Message for IsValidAdminSession {
        type Result = Result<bool, Error>;
    }

    impl Handler<IsValidAdminSession> for ConnDsl {
        type Result = Result<bool, Error>;

        fn handle(&mut self, data: IsValidAdminSession, _: &mut Self::Context) -> Self::Result {
            use schema::administrator_sessions;
            
            let conn = &self.0.get().map_err(ErrorInternalServerError)?;

            let o_session = administrator_sessions::table
                .filter(administrator_sessions::dsl::administrator_id.eq(data.admin_id))
                .get_result::<AdminSession>(conn)
                .optional()
                .map_err(negotiate_error)?;

            if let Some(session) = o_session {
                if session.is_valid && session.expire_at > Utc::now().naive_utc() {
                    if data.update {
                        //add a new updated_at value with the date of this request
                        diesel::update(
                            administrator_sessions::table.filter(administrator_sessions::dsl::id.eq(session.id))
                        ).set(&EditAdminSession::new(session.administrator_id, true, None))
                        .execute(conn)
                        .map_err(negotiate_error)?;
                    }

                    return Ok(true);
                }
            }

            Ok(false)
        }
    }
}