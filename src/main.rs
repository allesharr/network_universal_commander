// use axum::{
//     extract::{Json, Extension},
//     response::IntoResponse,
//     routing::post,
//     Router,
//     http::StatusCode,
// };
 
// use axum::extract::State;
// use std::sync::Mutex;
// use axum::*;
// use once_cell::sync::OnceCell;

// struct DeviceMap {
//     devices:Vec<Device>
// }

// // Структура для устройства
// #[derive(Serialize, Deserialize, FromRow, Default)]
// struct Device {
//     ip: String,
//     admin_username: String,
//     admin_password_hash: String,
// }

// // Структура для получаемых данных
// #[derive(Deserialize, Default)]
// struct DevicePayload {
//     ip: String,
//     admin_username: String,
//     admin_password: String,
// }

// static POOL: OnceCell<Mutex<PgPool>> = OnceCell::new();

// // Пример проверки доступности IP-адреса (тестовый код)
// async fn check_device_online(ip: &str) -> bool {
//     let addr = format!("{}:80", ip).parse::<SocketAddr>().unwrap();
//     match TcpStream::connect(addr).await {
//         Ok(_) => true,
//         Err(_) => false,
//     }
// }
// async fn init_pool() {
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let pool = PgPool::connect(&database_url)
//         .await
//         .expect("Failed to connect to the database");

//     // Устанавливаем пул в глобальную переменную
//     POOL.set(Mutex::new(pool)).expect("Pool is already initialized");
// }

// fn get_pool() -> &'static Mutex<PgPool> {
//     POOL.get().expect("Pool is not initialized yet")
// }

// // Функция для создания таблицы
// async fn create_table(pool: &PgPool) -> Result<(), sqlx::Error> {
//     sqlx::query(
//         "CREATE TABLE IF NOT EXISTS devices (
//             id SERIAL PRIMARY KEY,
//             ip VARCHAR(255) NOT NULL,
//             admin_username VARCHAR(255) NOT NULL,
//             admin_password_hash VARCHAR(255) NOT NULL
//         )"
//     )
//     .execute(pool)
//     .await?;
//     Ok(())
// }

// // Обработчик для добавления устройства
// async fn add_device(
//     Json(payload): Json<DevicePayload>,
// ) -> impl IntoResponse {
//     // Хэшируем пароль
//     let password_hash = match hash(&payload.admin_password, DEFAULT_COST) {
//         Ok(hash) => hash,
//         Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//     };

//     // Вставка данных в базу
//     let result = sqlx::query!(
//         "INSERT INTO devices (ip, admin_username, admin_password_hash) VALUES ($1, $2, $3)",
//         payload.ip,
//         payload.admin_username,
//         password_hash
//     )
//     .execute(&POOL)
//     .await;

//     match result {
//         Ok(_) => StatusCode::CREATED.into_response(),
//         Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//     }
// }

// #[tokio::main]
// async fn main() {
//     // Загружаем переменные окружения
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

//     init_pool().await;

//     let pool = get_pool().lock().unwrap();

//     // Создание таблицы
//     if let Err(e) = create_table(&pool).await {
//         eprintln!("Failed to create table: {}", e);
//         return;
//     }

//     // Создание маршрутизатора
//     let app = Router::new()
//         .route("/add_device", post(add_device))
//         .layer(Extension(pool));

//     // Запуск сервера
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     println!("Server running at http://{}", addr);
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

mod server_local;
mod models;
use server_local::*;
fn main() {
    main_local::start();
}

#[cfg(test)]
mod tests{
    use super::*;
    #[tokio::test]
    async fn test1() {
        println!("data")
        // create_data().await;
    }
    // async fn create_data(){
    //     dotenv().ok();
    //     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    //     // Создание пула соединений PostgreSQL
    //     let pool = PgPool::connect(&database_url)
    //         .await
    //         .expect("Failed to connect to the database");
    //     create_table(&pool).await;
    // }
}


