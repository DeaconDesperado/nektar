mod predicates;

#[cfg(ci)]
mod tests {

    use crate::predicates::json::IsJson;

    use assert_cmd::Command;

    const METASTORE_HOST: &str = "localhost";
    const METASTORE_PORT: u16 = 9083;

    #[test]
    fn test_get_default_catalog() {
        let _ = env_logger::builder().is_test(true).try_init();
        let mut cmd = Command::cargo_bin("nektar").unwrap();
        cmd.arg(format!("{}:{}", METASTORE_HOST, METASTORE_PORT))
            .arg("get-catalog")
            .arg("hive")
            .assert()
            .success();
    }

    #[test]
    fn test_create_and_get_catalog() {
        let _ = env_logger::builder().is_test(true).try_init();
        let mut create_cmd = Command::cargo_bin("nektar").unwrap();
        let catalog_name = "test";
        create_cmd
            .arg(format!("{}:{}", METASTORE_HOST, METASTORE_PORT))
            .arg("create-catalog")
            .arg(catalog_name)
            .arg("file:/opt/hive/data/warehouse")
            .arg("-d")
            .arg("a description")
            .assert()
            .success();

        let mut get_cmd = Command::cargo_bin("nektar").unwrap();
        get_cmd
            .arg(format!("{}:{}", METASTORE_HOST, METASTORE_PORT))
            .arg("get-catalog")
            .arg(catalog_name)
            .assert()
            .stdout(IsJson)
            .success();
    }
}
