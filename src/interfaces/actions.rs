pub enum IdentificationModuleServices {
    CreateUser,
    GetUser,
    GetUsers,
    UpdateUser,
}

impl IdentificationModuleServices {
    pub fn action(input: &str) -> Option<IdentificationModuleServices> {
        match input {
            "CREATE_USER" => Some(IdentificationModuleServices::CreateUser),
            "GET_USER" => Some(IdentificationModuleServices::GetUser),
            "GET_USERS" => Some(IdentificationModuleServices::GetUsers),
            "UPDATE_USER" => Some(IdentificationModuleServices::UpdateUser),
            _ => None,
        }
    }
}
