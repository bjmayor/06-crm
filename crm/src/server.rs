use anyhow::Result;
use crm::pb::{
    user_service_server::{UserService, UserServiceServer},
    CreateUserRequest, GetUserRequest, User,
};
use tonic::{async_trait, transport::Server, Request, Response, Status};

#[derive(Debug)]
pub struct UserServer;

#[async_trait]
impl UserService for UserServer {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        let input = request.into_inner();
        println!("create_user: {:?}", input);
        Ok(Response::new(User::new(1, &input.name, &input.email)))
    }

    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<User>, Status> {
        let input = request.into_inner();
        println!("create_user: {:?}", input);
        Ok(Response::new(User::default()))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::1]:50051".parse()?;
    let user_server = UserServer {};
    println!("UserServer listening on {}", addr);
    Server::builder()
        .add_service(UserServiceServer::new(user_server))
        .serve(addr)
        .await?;
    Ok(())
}
