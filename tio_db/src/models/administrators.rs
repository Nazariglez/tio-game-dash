use chrono::{NaiveDateTime, Utc};
use schema::administrators;

#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct Admin {
    pub id: i32,
    pub email: String,

    #[serde(skip_serializing)] 
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
    pub level: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

impl NewAdmin {
    pub fn new(email:String, pass: String, level: i16) -> NewAdmin {
        NewAdmin {
            email: email,
            password: pass,
            level: level,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc()
        }
    }
}

#[derive(AsChangeset)]
#[table_name = "administrators"]
pub struct EditAdmin {
    pub id: i32,
    pub email: Option<String>,
    pub password: Option<String>,
    pub level: Option<i16>,
    pub updated_at: NaiveDateTime
}

impl EditAdmin {
    pub fn new(id: i32) -> EditAdmin {
        EditAdmin {
            id: id,
            email: None,
            password: None,
            level: None,
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
    use actix_web::Error;
    use actix_web::error::*;
    use ConnDsl;
    use bcrypt;
    use models::negotiate_error;
    use tio_utils::is_valid_email;

    //-- Count
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CountAdmins;
    
    impl Message for CountAdmins {
        type Result = Result<i64, Error>;
    }

    impl Handler<CountAdmins> for ConnDsl {
        type Result = Result<i64, Error>;

        fn handle(&mut self, _: CountAdmins, _: &mut Self::Context) -> Self::Result {
            use schema::administrators;
            use diesel::dsl::*;
            
            let conn = &self.0.get().map_err(ErrorInternalServerError)?;
            let c = administrators::table.select(count_star())
                .first(conn)
                .map_err(negotiate_error)?;

            Ok(c)
        }
    }

    //-- List
    #[derive(Serialize, Deserialize, Debug)]
    pub struct ListAdmins {
        pub page: i64,
        pub limit: i64,
        //pub sort_type: String,
        //pub sort_by: String
    }

    impl Message for ListAdmins  {
        type Result = Result<Vec<Admin>, Error>;
    }

    impl Handler<ListAdmins> for ConnDsl {
        type Result = Result<Vec<Admin>, Error>;

        fn handle(&mut self, list_admins: ListAdmins, _: &mut Self::Context) -> Self::Result {
            use schema::administrators;
            
            let conn = &self.0.get().map_err(ErrorInternalServerError)?;
            let list = administrators::table
                    .offset(list_admins.page*list_admins.limit)
                    .limit(list_admins.limit)
                    .order(administrators::dsl::created_at.asc())
                    .get_results::<Admin>(conn)
                    .map_err(negotiate_error)?;

            if list.len() == 0 {
                Err(ErrorNotFound("Not Found"))
            } else {
                Ok(list)
            }

        }

    }

    //-- Create Implementation
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateAdmin {
        pub email: String,
        pub password: String,
        pub level: Option<i16>
    }

    impl Message for CreateAdmin  {
        type Result = Result<Admin, Error>;
    }

    impl Handler<CreateAdmin> for ConnDsl {
        type Result = Result<Admin, Error>;

        fn handle(&mut self, create_admin: CreateAdmin, _: &mut Self::Context) -> Self::Result {
            use schema::administrators;

            validate_password(create_admin.password.clone())?;
            validate_email(create_admin.email.clone())?;
            
            let conn = &self.0.get().map_err(ErrorInternalServerError)?;
            let pass = hash_password(create_admin.password.clone())?;

            let new_admin = NewAdmin::new(create_admin.email, pass, create_admin.level.unwrap_or(1));
            let mut inserted_users = diesel::insert_into(administrators::table)
                .values(&new_admin)
                .get_results(conn)
                .map_err(negotiate_error)?;

            Ok(inserted_users.pop().unwrap())
        }
    }

    //-- Read Implementation
    #[derive(Serialize, Deserialize, Debug)]
    pub struct ReadAdmin {
        pub id: Option<i32>,
        pub email: Option<String>
    }

    impl Message for ReadAdmin  {
        type Result = Result<Admin, Error>;
    }

    impl Handler<ReadAdmin> for ConnDsl {
        type Result = Result<Admin, Error>;

        fn handle(&mut self, read_admin: ReadAdmin, _: &mut Self::Context) -> Self::Result {
            use schema::administrators;

            let conn = &self.0.get().map_err(ErrorInternalServerError)?;

            if let Some(id) = read_admin.id {
                let mut admin = administrators::table
                    .filter(administrators::dsl::id.eq(id))
                    .get_result::<Admin>(conn)
                    .map_err(negotiate_error)?;

                Ok(admin)
            } else if let Some(email) = read_admin.email {
                let mut admin = administrators::table
                    .filter(administrators::dsl::email.eq(email))
                    .get_result::<Admin>(conn)
                    .map_err(negotiate_error)?;

                Ok(admin)
            } else {
                Err(ErrorBadRequest("Id or email must be provided."))
            }
        }
    }

    //-- Update Implementation
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateAdmin {
        pub id: i32,
        pub email: Option<String>,
        pub password: Option<String>,
        pub level: Option<i16>
    }

    impl Message for UpdateAdmin  {
        type Result = Result<Admin, Error>;
    }

    impl Handler<UpdateAdmin> for ConnDsl {
        type Result = Result<Admin, Error>;

        fn handle(&mut self, update_admin: UpdateAdmin, _: &mut Self::Context) -> Self::Result {
            use schema::administrators;

            if update_admin.email.is_none()
                && update_admin.password.is_none()
                && update_admin.level.is_none()
            {
                return Err(ErrorBadRequest("Empty request update. No one fields to set."));
            }

            if let Some(email) = update_admin.email.clone() {
                validate_email(email)?;
            }

            let pass = if let Some(pass) = update_admin.password.clone() {
                validate_password(pass.clone())?;
                Some(hash_password(pass)?)
            } else {
                None
            };

            let mut edit = EditAdmin::new(update_admin.id);
            edit.email = update_admin.email;
            edit.password = pass;
            edit.level = update_admin.level;

            let conn = &self.0.get().map_err(ErrorInternalServerError)?;
            let filter = administrators::table.filter(administrators::dsl::id.eq(update_admin.id));
            
            let edited_admin = diesel::update(filter)
                .set(&edit)
                .get_result::<Admin>(conn)
                //.optional()
                .map_err(negotiate_error)?;

            //if the password change, invalidate the session
            if edit.password.is_some() || edit.email.is_some() {
                use models::administrator_sessions::*;
                use schema::administrator_sessions;

                 diesel::update(
                     administrator_sessions::table.filter(administrator_sessions::dsl::administrator_id.eq(edited_admin.id))
                 ).set(&EditAdminSession::new(edited_admin.id, false, None))
                    .execute(conn)
                    .optional() //is necesarry to avoid the not_found error?
                    .map_err(negotiate_error)?;
            }

            Ok(edited_admin)
        }

    }

    //-- Delete Implementation
    #[derive(Serialize, Deserialize, Debug)]
    pub struct DeleteAdmin {
        pub id: i32,
    }

    impl Message for DeleteAdmin  {
        type Result = Result<(), Error>;
    }

    impl Handler<DeleteAdmin> for ConnDsl {
        type Result = Result<(), Error>;

        fn handle(&mut self, del_admin: DeleteAdmin, _: &mut Self::Context) -> Self::Result {
            use schema::administrators;

            let conn = &self.0.get().map_err(ErrorInternalServerError)?;
            
            /*{ //invalidate the auth session
                use models::administrator_sessions::*;
                use schema::administrator_sessions;

                 diesel::update(
                     administrator_sessions::table.filter(administrator_sessions::dsl::administrator_id.eq(del_admin.id))
                 ).set(&EditAdminSession::new(del_admin.id, false, None))
                    .execute(conn)
                    .optional() //is necesarry to avoid the not_found error?
                    .map_err(negotiate_error)?;
            }*/
        

            let filter = administrators::table.filter(administrators::dsl::id.eq(del_admin.id));
            diesel::delete(filter)
                .execute(conn)
                .map_err(negotiate_error)?;

            Ok(())
        }
    }


    //-- utils
    fn validate_password(pass:String) -> Result<(), Error> {
        if pass.len() < 6 {
            Err(ErrorBadRequest("Minimum password length is 6"))
        } else {
            Ok(())
        }
    }

    fn validate_email(email:String) -> Result<(), Error> {
        if !is_valid_email(&email) {
            Err(ErrorBadRequest("Invalid email format."))
        } else {
            Ok(())
        }
    }

    fn hash_password(pass:String) -> Result<String, Error> {
        bcrypt::hash(pass.as_str(), bcrypt::DEFAULT_COST)
            .map_err(ErrorInternalServerError)
    }

    pub fn compare_password(pass:String, hash:String) -> Result<bool, Error> {
        bcrypt::verify(pass.as_str(), hash.as_str())
            .map_err(ErrorInternalServerError)
    }
}