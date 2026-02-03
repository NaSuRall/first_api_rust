use axum::{Router, routing::get};
use sqlx::MySqlPool;

mod handlers;

use crate::handlers::user::{create_user, delete_user, get_all_user, update_user};

#[tokio::main]
async fn main() {
    // Lien pour se connecter a la base de données
    let database_url =
        "mysql://u998935179_apiRust:TTrcbt12@193.203.168.188:3306/u998935179_apiRust";

    // Création de la pool de connexion à la base de données
    let pool = MySqlPool::connect(database_url)
        .await
        .expect("Failed to connect to Mysql");

    // Rooter de l'API ici :
    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route(
            "/users",
            get(get_all_user)
                .post(create_user)
                .put(update_user)
                .delete(delete_user),
        )
        .with_state(pool);

    // Lancement du serveur
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
