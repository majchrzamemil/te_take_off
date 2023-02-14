//!src/configuration.rs 
//[...] implDatabaseSettings{
     pubfnconnection_string(&self)->String{ 
        format!( 
            "postgres://{}:{}@{}:{}/{}", 
            self.username,self.password,self.host,self.port,self.database_name 
        ) 
    } 
}