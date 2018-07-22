use chrono::{NaiveDateTime, Utc};
use schema::administrators;

#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct Admin {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub level: i16, 
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Insertable)]
#[table_name = "administrators"]
pub struct NewAdmin {
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

impl NewAdmin {
    pub fn new(email:String, pass: String) -> NewAdmin {
        NewAdmin {
            email: email,
            password: pass,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc()
        }
    }
}

//-- actix handlers --//
pub mod handlers {
    use super::*;
    use diesel;
    use diesel::prelude::*;
    use actix::{Message, Handler};
    use ConnDsl;
    use bcrypt;

    //-- Create Implementation
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateAdmin {
        pub email: String,
        pub password: String
    }

    impl Message for CreateAdmin  {
        type Result = Result<Admin, String>;
    }

    impl Handler<CreateAdmin> for ConnDsl {
        type Result = Result<Admin, String>;

        fn handle(&mut self, create_admin: CreateAdmin, _: &mut Self::Context) -> Self::Result {
            use schema::administrators;

            if create_admin.password.len() < 6 {
                return Err("Minimum password length is 6".to_string());
            }
            
            let conn = &self.0.get().map_err(|e|e.to_string())?;
            let pass = bcrypt::hash(create_admin.password.as_str(), bcrypt::DEFAULT_COST)
                .map_err(|e|e.to_string())?;

            let new_admin = NewAdmin::new(create_admin.email, pass);
            let mut inserted_users = diesel::insert_into(administrators::table)
                .values(&new_admin)
                .get_results(conn)
                .map_err(|e|e.to_string())?;

            Ok(inserted_users.pop().unwrap())
        }
    }

    //-- Read Implementation

    //-- Update Implementation

    //-- Delete Implementation

}