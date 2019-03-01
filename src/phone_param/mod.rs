use rocket::http::RawStr;
use rocket::request::FromParam;

pub struct PhoneParam<'r> {
    pub phone_number: &'r str,
}

impl<'r> FromParam<'r> for PhoneParam<'r> {
    type Error = &'r RawStr;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        if param.len() != 10 || !param.chars().all(|c| c >= '0' && c <= '9') {
            return Err(param);
        }

        Ok(PhoneParam {
            phone_number: param,
        })
    }
}
