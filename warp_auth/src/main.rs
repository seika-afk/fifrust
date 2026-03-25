use serde::{Deserialize,Serialize}

pub struct User{
    pub uid :String,
    pub email : String,
    pub pw : String,
    pub role :String,

}



#[derive(Deserialize)]
pub struct LoginRequest{

    pub email :String,
    pub pw :String,


}

#[derive(Serialize)]
pub struct LoginResponse{

    pub token :String



}
