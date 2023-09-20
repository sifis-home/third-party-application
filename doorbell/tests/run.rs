use anyhow::Result;
use assert_cmd::prelude::*;
use std::{collections::HashSet, path::PathBuf, process::Command};
use tempfile::{tempdir, TempDir};

struct Mock {
    sock: PathBuf,
    _dir: TempDir,
}

impl Mock {
    fn new() -> Result<Mock> {
        let dir = tempdir()?;
        let sock = dir.path().join("sifis.sock");

        let _server = Command::new("sifis-runtime-mock")
            .env("SIFIS_SERVER", &sock)
            .spawn()?;
        Ok(Mock { sock, _dir: dir })
    }
}

#[test]
fn default_mock() {
    let mock = Mock::new().unwrap();

    let mut cmd = Command::cargo_bin("doorbell").unwrap();

    let out = cmd.env("SIFIS_SERVER", &mock.sock).output().unwrap();
    let s = String::from_utf8(out.stdout).unwrap();

    let lines = s.lines().collect::<HashSet<_>>();

    let expected: HashSet<&str> = [
        "lamp1           Off     0     ",
        "lamp2           Off     0     ",
    ]
    .into();
    assert_eq!(expected, lines);
}
