use actix_cors::Cors;
use actix_web::{get, post, web, App, Error, error::ErrorInternalServerError, HttpResponse, HttpServer, Responder};
use chrono::DateTime;
use chrono::Local;
use dotenv::dotenv;
use reqwest::header::HeaderMap;
use rusqlite::{params, Connection, OptionalExtension, Result};
use serde::{Serialize, Deserialize};
use std::env;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct TokenTx {
  id: String,
  timestamp: i64
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenRx {
  id: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Status {
  status: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct Receive {
  id: String,
  items: Vec<Item>,
  total: String,
  payment: String,
  cash: String,
  change: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
  name: String,
  price: String,
  quantity: String
}

#[derive(Debug)]
struct Sale {
  id: String,
  timestamp: String,
  name: String,
  quantity: u16,
  subtotal: u16
}

#[derive(Debug)]
struct Payment {
  id: String,
  timestamp: String,
  method: String,
  total: u16,
  cash: u16,
  change: u16
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  HttpServer::new(|| {
    let cors = Cors::default()
      .allowed_origin("https://arkw.net")
      .allowed_methods(vec!["GET", "POST"])
      .allow_any_header()
      .max_age(3600);
    App::new()
    .wrap(cors).service(get_now).service(post_token_generate).service(post_token_verify).service(post_record)
  })
  .bind("0.0.0.0:8080")
  .expect("").run()
  .await
}

#[get("/now")]
async fn get_now() -> HttpResponse {
  let local_datetime: DateTime<Local> = Local::now();
  HttpResponse::Ok().content_type("text/plain").body(local_datetime.to_string())
}

#[post("/token/generate")]
async fn post_token_generate(key: web::Json<TokenRx>) -> Result<HttpResponse, Error> {
  let mut token = TokenTx { id: "".to_string(), timestamp: 0 };
  let password = env::var("TOKEN_PASSWORD").unwrap();
  if key.id == password {
    let db = open_db("./token.db");
    if let Ok(connection) = db {
      let _connection = connection;
      let local_datetime: DateTime<Local> = Local::now();
      token.id = Uuid::new_v4().to_string();
      token.timestamp = local_datetime.timestamp();
      let _insert_token = insert_token(&_connection, &token);
      let _ = discord_log(format!("[TOKEN] id={}, timestamp={}", token.id, token.timestamp).as_str()).await;
    }
  }
  Ok(HttpResponse::Ok().json(token))
}

#[post("/token/verify")]
async fn post_token_verify(token: web::Json<TokenRx>) -> Result<HttpResponse, Error> {
  let mut status = Status { status: false };
  let connection = open_db("./token.db").map_err(|e| ErrorInternalServerError(e))?;
  let row: Option<(String, i64)> = connection.query_row(
    "SELECT id, timestamp FROM token WHERE id = ?1",
    params![token.id],
    |row| Ok((row.get(0)?, row.get(1)?)),
  ).optional().map_err(|e| ErrorInternalServerError(e))?;
  if let Some((_id, timestamp)) = row {
    let now = Local::now().timestamp();
    let diff = now - timestamp;
    if (0..600).contains(&diff) {
        status.status = true;
    }
  }
  Ok(HttpResponse::Ok().json(status))
}

#[post("/record")]
async fn post_record(receive: web::Json<Receive>) -> impl Responder {
  let db = open_db("./record.db");
  let mut log = String::new();
  if let Ok(connection) = db {
    let _connection = connection;
    let local_datetime: DateTime<Local> = Local::now();
    let timestamp: String = local_datetime.to_string();
    for item in &receive.items {
      let quantity: u16 = item.quantity.parse::<u16>().unwrap();
      let price: u16 = item.price.parse::<u16>().unwrap();
      let subtotal = quantity * price;
      let _sale = Sale {
        id: receive.id.clone(),
        timestamp: timestamp.clone(),
        name: item.name.clone(),
        quantity: quantity,
        subtotal: subtotal
      };
      let _insert_sale = insert_sale(&_connection, &_sale);
      log.push_str(format!("[SALE] id={}, timestamp={}, name={}, quantity={}, subtotal={}\n", receive.id, timestamp, item.name, quantity, subtotal).as_str());
    }
    let _payment = Payment {
      id: receive.id.clone(),
      timestamp: timestamp.clone(),
      method: receive.payment.clone(),
      total: receive.total.parse::<u16>().unwrap(),
      cash: receive.cash.parse::<u16>().unwrap(),
      change: receive.change.parse::<u16>().unwrap()
    };
    let _insert_payment = insert_payment(&_connection, &_payment);
    log.push_str(format!("[PAYMENT] id={}, timestamp={}, method={}, total={}, cash={}, change={}", receive.id, timestamp, receive.payment, receive.total, receive.cash, receive.change).as_str());
  }
  let _ = discord_log(log.as_str()).await;
  HttpResponse::Ok().body("")
}

fn open_db(path: &str) -> Result<Connection, rusqlite::Error> {
  let connection = Connection::open(&path)?;
  println!("{}", connection.is_autocommit());
  Ok(connection)
}

fn insert_token(connection: &Connection, token: &TokenTx) -> Result<usize, rusqlite::Error> {
  return Ok(connection.execute(
    "insert into token (id, timestamp) values (?1, ?2)",
    params![token.id, token.timestamp]
  )?);
}

fn insert_sale(connection: &Connection, sale: &Sale) -> Result<usize, rusqlite::Error> {
  return Ok(connection.execute(
    "insert into sale (id, timestamp, name, quantity, subtotal) values (?1, ?2, ?3, ?4, ?5)",
    params![sale.id, sale.timestamp, sale.name, sale.quantity, sale.subtotal]
  )?);
}

fn insert_payment(connection: &Connection, payment: &Payment) -> Result<usize, rusqlite::Error> {
  return Ok(connection.execute(
    "insert into payment (id, timestamp, method, total, cash, change) values (?1, ?2, ?3, ?4, ?5, ?6)",
    params![payment.id, payment.timestamp, payment.method, payment.total, payment.cash, payment.change]
  )?);
}

async fn discord_log(s: &str) -> Result<(), reqwest::Error> {
  let url = env::var("DISCORD_WEBHOOK_URL").unwrap();
  let mut headers = HeaderMap::new();
  headers.append("Content-Type", "application/json".parse().expect(""));
  let payload = serde_json::json!({
    "content": s
  });
  let _client = reqwest::Client::new().post(url).headers(headers).json(&payload).send().await?.error_for_status()?;
  Ok(())
}
