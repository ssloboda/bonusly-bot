use errors::*;

use std;

use hyper;
use hyper_tls;
use serde_yaml;
use tokio_core;

use std::io::Write;
use futures::Stream;
use futures::Future;

const BONUS_URL: &'static str = "https://bonus.ly/api/v1/bonuses";
const USERS_URL: &'static str = "https://bonus.ly/api/v1/users";
const DEFAULT_HASHTAGS: &'static str = "#tupacisalive #bonuslybot";

header!{(ApplicationName, "Application-Name") => [String] }

#[derive(Debug, Deserialize)]
struct Config {
    email: String,
    access_token: String
}

#[derive(Debug, Deserialize)]
struct BonuslyConfig {
    bonusly: Config
}

type BonuslyBotClient = hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>;

pub struct BonuslyBot {
    config: Config,
    core: tokio_core::reactor::Core,
    client: BonuslyBotClient
}

impl BonuslyBot {
    pub fn from_config_file(config_file: &std::path::Path) -> BBResult<Self> {
        let reader = std::fs::File::open(config_file).chain_err(
            || "Cannot open config file"
        )?;
        let config: BonuslyConfig = serde_yaml::from_reader(reader).chain_err(
            || "Malformed configuration file"
        )?;
        BonuslyBot::new(config.bonusly)
    }

    fn new(config: Config) -> BBResult<Self> {
        let core = tokio_core::reactor::Core::new().chain_err(
            || "Cannot create core"
        )?;
        let client = hyper::Client::configure()
            .connector(hyper_tls::HttpsConnector::new(4, &core.handle()).expect(
                "Cannot get HttpsConnector"
            ))
            .build(&core.handle());

        Ok(BonuslyBot {
            config: config,
            core: core,
            client: client
        })
    }

    fn set_standard_headers(
        &self,
        headers: &mut hyper::Headers,
        content_length: usize,
    ) -> BBResult<()> {
        headers.set(hyper::header::Accept(
            vec![hyper::header::qitem(hyper::mime::APPLICATION_JSON)]
        ));
        headers.set(hyper::header::ContentType::json());
        headers.set(hyper::header::ContentLength(content_length as u64));
        headers.set(hyper::header::Authorization(hyper::header::Bearer {
            token: self.config.access_token.to_owned()
        }));
        headers.set(ApplicationName("BonuslyBot/0.1".to_owned()));
        Ok(())
    }

    // FIXME clean up
    pub fn give_raw_bonus(&mut self, message: &str) -> BBResult<()> {
        let uri = BONUS_URL.parse().expect("Invalid bonus URL");
        let mut request = hyper::Request::new(hyper::Method::Post, uri);

        let payload = format!("{{ \"reason\": \"{} {}\" }}", message, DEFAULT_HASHTAGS);

        self.set_standard_headers(
            request.headers_mut(),
            payload.len()
        )?;

        println!("payload: {}", payload); // FIXME remove
        request.set_body(payload);
        println!("request: {:?}", request); // FIXME remove

        let post = self.client
            .request(request)
            .and_then(|res| {
                println!("Response: {}", res.status());

                res.body().for_each(|chunk| {
                    std::io::stdout().write_all(&chunk).map(|_| ()).map_err(
                        From::from
                    )
                })
            })
            .map(|_| Ok(()));

        self.core.run(post).chain_err(|| "Request failed")?
    }

    // FIXME clean up
    fn get_users(&mut self) -> BBResult<()> {
        let uri = USERS_URL.parse().expect("Invalid users URL");
        let mut request = hyper::Request::new(hyper::Method::Get, uri);

        let payload = "";

        self.set_standard_headers(
            request.headers_mut(),
            payload.len()
        )?;

        request.set_body(payload);
        println!("request: {:?}", request); // FIXME remove

        let get = self.client
            .request(request)
            .and_then(|res| {
                println!("Response: {}", res.status());

                res.body().for_each(|chunk| {
                    std::io::stdout().write_all(&chunk).map(|_| ()).map_err(
                        From::from
                    )
                })
            })
            .map(|_| Ok(()));
        self.core.run(get).chain_err(|| "Request failed")?
    }

    // FIXME
    pub fn give_random_bonus(&mut self) -> BBResult<()> {
        bail!("give_random_bonus not implemented")
    }
}
