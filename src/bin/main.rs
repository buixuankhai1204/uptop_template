use identification::application::topic::app::UserApp;
use identification::infrastructure::persistence::IDRepositories;
use identification::interfaces::actions::IdentificationModuleServices;
use identification::interfaces::user_handler::{on_create_new_user, on_find_user, UserHandler};
use scylla::CachingSession;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};
use uptop_core::common::result::AppResult;
use uptop_core::common::trace::tracing_init;
use uptop_core::infrastructure::cassandra::{create_db_session, create_keyspace};

mod message {
    tonic::include_proto!("message");
}

use message::message_server::{Message, MessageServer};
use message::{MessageRequest, MessageResponse};

struct MessageService {
    repositories: Arc<IDRepositories>,
}

impl MessageService {
    fn new(repos: IDRepositories) -> Self {
        Self {
            repositories: Arc::new(repos),
        }
    }
}

#[tonic::async_trait]
impl Message for MessageService {
    async fn send_message(
        &self,
        request: Request<MessageRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        // Extract the inner message from the request
        let payload = request.into_inner();
        let command = payload.id;
        let message = payload.message;

        let mut status_code = "OK".to_string();
        let mut response: MessageResponse = MessageResponse {
            id: "Internal Server Error".to_owned(),
            message: "Please try again!".to_owned(),
        };

        let user_app = UserApp::new(Arc::new(self.repositories.user.clone()));
        let user_handler = UserHandler {
            user_app: Arc::new(user_app),
        };

        match IdentificationModuleServices::action(&command) {
            Some(IdentificationModuleServices::CreateUser) => {
                let message = match on_create_new_user(user_handler, message).await {
                    Ok(res) => res,
                    Err(err) => {
                        status_code = "ERROR".to_string();
                        err.to_string()
                    }
                };
                response = MessageResponse {
                    id: status_code,
                    message,
                };
            }
            Some(IdentificationModuleServices::GetUser) => {
                let message = match on_find_user(user_handler, message).await {
                    Ok(res) => res,
                    Err(err) => {
                        status_code = "ERROR".to_string();
                        err.to_string()
                    }
                };
                response = MessageResponse {
                    id: status_code,
                    message,
                };
            }
            Some(IdentificationModuleServices::GetUsers) => {
                todo!()
            }
            Some(IdentificationModuleServices::UpdateUser) => {
                todo!()
            }
            _none => (),
        }

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv::dotenv().ok();
    let _gaurd = tracing_init();

    let cassandra = create_db_session().await;
    create_keyspace(&cassandra).await?;
    let cache_session = CachingSession::from(cassandra, 1);
    let repos = IDRepositories::new(Arc::new(Mutex::new(cache_session)));
    repos.auto_mod_identification_migrate().await?;

    pub(crate) const FILE_MESSAGE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("message_descriptor");
    let reflect_sv = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_MESSAGE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    let server_addr = "0.0.0.0:3000".parse().unwrap();
    tracing::info!(message = "Starting server on", %server_addr);
    let msg_service = MessageService::new(repos);

    Server::builder()
        .add_service(reflect_sv)
        .add_service(MessageServer::new(msg_service))
        .serve(server_addr)
        .await
        .unwrap();

    Ok(())
}
