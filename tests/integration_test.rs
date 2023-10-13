mod predicates;

use predicates::json::IsJson;

use assert_cmd::Command;
use ctor::{ctor, dtor};
use lazy_static::lazy_static;
use std::{env, fmt::Display, future::Future, net::IpAddr, thread};
use testcontainers::clients::Cli;
use testcontainers::core::WaitFor;
use testcontainers::images::generic::GenericImage;
use tokio::{
    runtime,
    sync::{
        mpsc::{self, UnboundedReceiver, UnboundedSender},
        Mutex,
    },
};

const METASTORE_HOST: &str = "localhost";
const METASTORE_PORT: u16 = 9083;
const HIVE_VERSION: &str = "4.0.0-beta-2-SNAPSHOT";
const SERVICE_NAME_VAR: &str = "SERVICE_NAME";
const SERVICE_NAME: &str = "metastore";
const SERVICE_OPTS_VAR: &str = "SERVICE_OPTS";
const SERVICE_OPTS: &str = "-Dhive.root.logger=console";
const READY_MSG: &str = "Successfully created a default database with name: default";

/// Channels for communication with container started on thread
struct Channel<T> {
    tx: UnboundedSender<T>,
    rx: Mutex<UnboundedReceiver<T>>,
}

/// Channel constructor
fn channel<T>() -> Channel<T> {
    let (tx, rx) = mpsc::unbounded_channel();
    Channel {
        tx,
        rx: Mutex::new(rx),
    }
}

/// Messages for communicating with container
#[derive(Debug)]
enum ContainerCommands {
    FetchPort,
    Stop,
}

struct HostPort {
    host: IpAddr,
    port: u16,
}

impl Display for HostPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

lazy_static! {
    static ref METASTORE_IN: Channel<ContainerCommands> = channel();
    static ref METASTORE_HOST_PORT: Channel<HostPort> = channel();
    static ref METASTORE_STOPPED: Channel<()> = channel();
}

/// Start testcontainers, open channels for communication
///
/// Necessary to support starting one long lived testcontainer
/// for all test methods.
async fn set_up() {
    let docker = Cli::default();
    let msg_await = WaitFor::message_on_stderr(READY_MSG.to_string());
    let hive_image = GenericImage::new("apache/hive", HIVE_VERSION)
        .with_wait_for(msg_await)
        .with_exposed_port(METASTORE_PORT)
        .with_env_var(SERVICE_NAME_VAR, SERVICE_NAME)
        .with_env_var(SERVICE_OPTS_VAR, SERVICE_OPTS);
    let hive_metastore = docker.run(hive_image);
    let host = match env::var("GITHUB_ACTIONS") {
        Ok(_) => hive_metastore.get_bridge_ip_address(),
        Err(_) => "127.0.0.1".parse().unwrap(),
    };
    let open_port = hive_metastore.ports().map_to_host_port_ipv4(METASTORE_PORT);

    log::info!("{}:{}", METASTORE_HOST, METASTORE_PORT);
    let mut rx = METASTORE_IN.rx.lock().await;
    while let Some(command) = rx.recv().await {
        println!("Received container command: {:?}", command);
        match command {
            ContainerCommands::FetchPort => METASTORE_HOST_PORT
                .tx
                .send(HostPort {
                    host: host,
                    port: open_port.unwrap(),
                })
                .unwrap(),
            ContainerCommands::Stop => {
                hive_metastore.stop();
                METASTORE_STOPPED.tx.send(()).unwrap();
                rx.close();
            }
        }
    }
}

/// Helper function for blocking run, used to start testcontainer
fn execute_blocking<F: Future>(f: F) {
    runtime::Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(f);
}

#[ctor]
/// Start container on process start
fn on_startup() {
    thread::spawn(|| execute_blocking(set_up()));
}

#[dtor]
/// Kill container on process end
fn on_shutdown() {
    execute_blocking(clean_up());
}

/// Destructor, kill container and notify channels
async fn clean_up() {
    METASTORE_IN.tx.send(ContainerCommands::Stop).unwrap();
    METASTORE_STOPPED.rx.lock().await.recv().await;
}

// BEGIN: test cases

#[tokio::test]
async fn test_get_default_catalog() {
    METASTORE_IN.tx.send(ContainerCommands::FetchPort).unwrap();
    let mut cmd = Command::cargo_bin("nektar").unwrap();
    let open_port = METASTORE_HOST_PORT.rx.lock().await.recv().await.unwrap();
    cmd.arg(format!("{}", open_port))
        .arg("get-catalog")
        .arg("hive")
        .assert()
        .success();
}

#[tokio::test]
async fn test_create_and_get_catalog() {
    METASTORE_IN.tx.send(ContainerCommands::FetchPort).unwrap();
    let mut create_cmd = Command::cargo_bin("nektar").unwrap();
    let open_port = METASTORE_HOST_PORT.rx.lock().await.recv().await.unwrap();
    let catalog_name = "test";
    create_cmd
        .arg(format!("{}", open_port))
        .arg("create-catalog")
        .arg(catalog_name)
        .arg("file:/opt/hive/data/warehouse")
        .arg("-d")
        .arg("a description")
        .assert()
        .success();

    let mut get_cmd = Command::cargo_bin("nektar").unwrap();
    get_cmd
        .arg(format!("{}", open_port))
        .arg("get-catalog")
        .arg(catalog_name)
        .assert()
        .stdout(IsJson)
        .success();
}
