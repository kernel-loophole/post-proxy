use std::sync::Arc;
use async_trait::async_trait;
use tokio_postgres::{Client, NoTls, SimpleQueryMessage};
use pgwire::api::ClientInfo;
use pgwire::api::query::SimpleQueryHandler;
use pgwire::error::{PgWireError, PgWireResult};
use pgwire::api::results::{Response, Tag};
use tokio::net::TcpListener;
use pgwire::tokio::process_socket;
use pgwire::api::auth::noop::NoopStartupHandler;
use pgwire::api::{ PgWireHandlerFactory};

pub struct ProxyProcessor {
    upstream_client: Client,
}

#[async_trait]
impl SimpleQueryHandler for ProxyProcessor {
    async fn do_query<'a, C>(&self, _client: &mut C, query: &'a str) -> PgWireResult<Vec<Response<'a>>>
    where
        C: ClientInfo + Unpin + Send + Sync,
    {
        self.upstream_client
            .simple_query(query)
            .await
            .map_err(|e| PgWireError::ApiError(Box::new(e)))
            .map(|resp_msgs| {
                let mut responses = Vec::new();
                for message in resp_msgs {
                    match message {
                        SimpleQueryMessage::CommandComplete(count) => {
                            let tag = Tag::new("EXECUTE");
                            responses.push(Response::Execution(tag));
                        }
                        SimpleQueryMessage::Row(row) => {
                            // Handle row responses if needed
                            // You might need to convert the row into a pgwire DataRow
                        }
                        _ => {}
                    }
                }
                responses
            })
    }
}

// Implementing PgWireHandlerFactory for ProxyProcessor
impl PgWireHandlerFactory for ProxyProcessor {
    type StartupHandler = ();
    type SimpleQueryHandler = ();
    type ExtendedQueryHandler = ();
    type CopyHandler = ();

    fn simple_query_handler(&self) -> Arc<Self::SimpleQueryHandler> {
        println!("simple_query_handler");
        Arc::new(())

    }

    fn extended_query_handler(&self) -> Arc<Self::ExtendedQueryHandler> {
        println!("extended_query_handler");
        Arc::new(())
    }

    fn startup_handler(&self) -> Arc<Self::StartupHandler> {
        Arc::new(())
    }

    fn copy_handler(&self) -> Arc<Self::CopyHandler> {
        Arc::new(())
    }
}

#[tokio::main]
pub async fn main() {
    // Establishing connection to the upstream PostgreSQL
    let (client, connection) = tokio_postgres::connect("host=127.0.0.1 user=postgres", NoTls)
        .await
        .expect("Failed to connect to upstream PostgreSQL");

    // Spawn a task to manage the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Creating the proxy processor
    let processor = Arc::new(ProxyProcessor {
        upstream_client: client,
    });

    // Setting up the authenticator
    let authenticator = Arc::new(NoopStartupHandler);

    // Bind the server to the specified address
    let server_addr = "127.0.0.1:5431";
    let listener = TcpListener::bind(server_addr).await.unwrap();
    println!("Listening on {}", server_addr);

    // Accepting incoming connections in a loop
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let processor_ref = processor.clone();
        let authenticator_ref = authenticator.clone();

        // Spawn a new task for each incoming connection
        tokio::spawn(async move {
            // Pass None for the TLS acceptor and use the correct authenticator and processor references
            if let Err(e) = process_socket(socket,  None, None).await {
                eprintln!("Error processing socket: {}", e);
            }
        });
    }
}
