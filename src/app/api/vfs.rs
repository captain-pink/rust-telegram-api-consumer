use std::{sync::Arc, thread};
use teloxide::{prelude::AutoSend, Bot};
use url::form_urlencoded::byte_serialize;

use thirtyfour::{prelude::*, ChromeCapabilities};

use log::{debug, info};

use serde_json::{Result as JsonResult, Value};

use chrono::{DateTime, Duration as ChronoDuration, Utc};

use crate::{
  app::db::ConnectionStore,
  shared::{enums::env::Variables, utils::env_var},
};

use super::{
  enums::Route,
  models::{ApiConfig, ApiRuntimeConfig, AuthCredentials, Token},
  types::Routes,
};

#[derive(Clone)]
pub struct ChromeDriver {
  pub value: Arc<WebDriver>,
  pub config: ApiConfig,
  pub runtime: ApiRuntimeConfig,
}

impl ChromeDriver {
  pub async fn new() -> Self {
    let options = chrome_options();

    Self {
      value: ChromeDriver::web_driver(options).await,
      config: ChromeDriver::config(),
      runtime: ApiRuntimeConfig::default(),
    }
  }

  async fn web_driver(options: ChromeCapabilities) -> Arc<WebDriver> {
    let value = WebDriver::new("http://localhost:9515", options)
      .await
      .expect("VfsApi:selenium_dirver: error creating new driver");

    Arc::new(value)
  }

  fn config() -> ApiConfig {
    let host = Variables::VFS_HOST.into();
    let host = env_var(host);

    let routes = [
      (Route::Login, "/rus/en/fin/login".to_string()),
      (Route::Appointments, "/appointment/slots".to_string()),
    ];
    let routes: Routes = routes.into_iter().collect();

    let credentials = auth_creds();

    ApiConfig::new(host, routes, credentials)
  }

  async fn element_by_id<'a>(&self, id: &'a str) -> WebElement {
    let id = By::Id(id);

    self.element(id).await
  }

  async fn element_by_xpath<'a>(&self, xpath: &'a str) -> WebElement {
    let xpath = By::XPath(xpath);

    self.element(xpath).await
  }

  async fn element(&self, by: By) -> WebElement {
    let element = self
      .value
      .query(by.clone())
      .first()
      .await
      .expect(&format!("Unable to find element by: {:?}", &by));

    element
      .wait_until()
      .displayed()
      .await
      .expect(&format!("Unable to wait_untill element: {:?}", &by));

    element
  }

  async fn execute_script<'a>(&self, script: &'a str, args: Vec<Value>) -> ScriptRet {
    self
      .value
      .execute(script, args)
      .await
      .expect("Script execution failed")
  }
}

fn chrome_options() -> ChromeCapabilities {
  let mut options = DesiredCapabilities::chrome();
  options.add_chrome_arg("--incognito").unwrap();

  options
}

fn auth_creds() -> AuthCredentials {
  let username = Variables::VFS_USERNAME.into();
  let username = env_var(username);

  let password = Variables::VFS_PASSWORD.into();
  let password = env_var(password);

  AuthCredentials::new(username, password)
}

pub async fn check_slots(
  driver: Arc<ChromeDriver>,
  bot: Arc<AutoSend<Bot>>,
  connection: Arc<ConnectionStore>,
) -> () {
  let token = revalidate_jwt(&driver)
    .await
    .expect("Revalidation token error");

  let slots = fetch_slots(&driver.value, token)
    .await
    .expect("Fetching slots error");

  // bot.send_message(chat_id, text);

  info!("Slots info: {:?}", &slots);
}

async fn login(driver: &ChromeDriver) -> JsonResult<()> {
  let (username, password) = driver.config.credentials().read();

  let email_element = driver.element_by_id("mat-input-0").await;
  email_element.send_keys(username).await.unwrap();

  let password_element = driver.element_by_id("mat-input-1").await;
  password_element.send_keys(password).await.unwrap();

  thread::sleep(std::time::Duration::from_millis(100));
  let submit_element = driver
    .element_by_xpath("//span[contains(text(), 'Sign In')]")
    .await;
  submit_element.click().await.unwrap();

  Ok(())
}

async fn fetch_slots(driver: &WebDriver, token: String) -> JsonResult<Value> {
  info!("Fetch slots");

  let dt_start = Utc::now();
  let dt_end = dt_start.clone() + ChronoDuration::days(90);

  let dt_start = dt_start.format("%d/%m/%Y").to_string();
  let dt_start: String = byte_serialize(dt_start.as_bytes()).collect();
  let dt_end = dt_end.format("%d/%m/%Y").to_string();
  let dt_end: String = byte_serialize(dt_end.as_bytes()).collect();

  let prefetch_script = format!(
    r#"
      console.log('arguments', arguments);
      var done = arguments[0];

      var myHeaders = new Headers();
      myHeaders.append("Authorization", "{}");
      myHeaders.append("Content-type", "application/json;charset=utf-8");
      myHeaders.append("Content-length", "0");
      myHeaders.append("Accept", "application/json, text/plain, */*");
      myHeaders.append("accept-encoding", "gzip, deflate, br");
      myHeaders.append("accept-language", "en-GB,en;q=0.9");
      
      var requestOptions = {{
        method: 'GET',
        headers: myHeaders,
        redirect: 'follow'
      }};

      fetch("https://lift-api.vfsglobal.com/appointment/slots?countryCode=rus&missionCode=fin&centerCode=FIN-ROS&loginUser=akh81058%40gmail.com&visaCategoryCode=APPSUBM&languageCode=en-US&applicantsCount=1&days=90&slotType=2&fromDate={}&toDate={}", requestOptions)
        .then(response => {{
          const contentType = response.headers.get('Content-Type');
          console.log('Response content type:', contentType);
          return response.json();
        }})
        .then(result => {{
          console.log('this is result', result);
          return done(result);
        }})
        .catch(error => {{
          console.log('Error during execution:', error);
          return done({{
            message: typeof error === 'object' ? error.message : 'Unknown error',
            jsonError: JSON.stringify(error),
          }});
        }})
    "#,
    token, dt_start, dt_end
  );

  debug!("script: {}", &prefetch_script);

  let response = driver
    .execute_async(&prefetch_script, Vec::new())
    .await
    .expect("Unable to execute script");

  Ok(response.json().clone())
}

async fn parse_jwt(driver: &ChromeDriver) -> JsonResult<()> {
  let new_booking = driver
    .element_by_xpath("//div[contains(text(), 'No Application(s) Found.')]")
    .await;

  info!("new booking button: {:?}", new_booking);

  let token = driver
    .execute_script("return window.sessionStorage.JWT", Vec::new())
    .await;
  let token = token.json();
  let token = token.to_string();
  let token = token[1..token.len() - 1].to_string();
  let valid = Utc::now() + ChronoDuration::minutes(9);

  set_token(token, valid);

  Ok(())
}

async fn revalidate_jwt(driver: &ChromeDriver) -> JsonResult<String> {
  info!("Revalidating token");

  match driver.runtime.token() {
    Some(token) => {
      if !token.is_valid() {
        update_jwt(driver).await?;
      }
    }
    None => {
      update_jwt(driver).await?;
    }
  }

  Ok(driver.runtime.token().unwrap().value().clone())
}

async fn update_jwt(driver: &ChromeDriver) -> JsonResult<()> {
  open_login(driver).await.unwrap();
  login(driver).await.unwrap();

  thread::sleep(std::time::Duration::from_secs(10));

  parse_jwt(driver).await.unwrap();

  Ok(())
}

async fn open_login(driver: &ChromeDriver) -> Result<(), WebDriverError> {
  let url = build_url(&driver.config, Route::Login);

  driver.value.goto(url).await
}

fn build_url(config: &ApiConfig, route: Route) -> String {
  let route = config.route(&route);

  config.host() + route.as_str()
}

fn set_token(token: String, valid: DateTime<Utc>) -> () {
  let token = Token::new(token, valid);

  info!("VfsApi:parse_jwt: Token created: {:?}", &token);

  // RUNTIME.lock().unwrap().set_token(token);
}
