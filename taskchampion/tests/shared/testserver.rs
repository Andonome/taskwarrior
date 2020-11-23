use std::collections::HashMap;
use taskchampion::server::{Blob, Server, VersionAdd};

pub struct TestServer {
    users: HashMap<String, User>,
}

struct User {
    // versions, indexed at v-1
    versions: Vec<Blob>,
    snapshots: HashMap<u64, Blob>,
}

impl TestServer {
    pub fn new() -> TestServer {
        TestServer {
            users: HashMap::new(),
        }
    }

    fn get_user_mut(&mut self, username: &str) -> &mut User {
        self.users
            .entry(username.to_string())
            .or_insert_with(User::new)
    }
}

impl Server for TestServer {
    /// Get a vector of all versions after `since_version`
    fn get_versions(&self, username: &str, since_version: u64) -> Vec<Blob> {
        self.users
            .get(username)
            .map(|user| user.get_versions(since_version))
            .unwrap_or_else(|| vec![])
    }

    /// Add a new version.  If the given version number is incorrect, this responds with the
    /// appropriate version and expects the caller to try again.
    fn add_version(&mut self, username: &str, version: u64, blob: Blob) -> VersionAdd {
        self.get_user_mut(username).add_version(version, blob)
    }

    fn add_snapshot(&mut self, username: &str, version: u64, blob: Blob) {
        self.get_user_mut(username).add_snapshot(version, blob);
    }
}

impl User {
    fn new() -> User {
        User {
            versions: vec![],
            snapshots: HashMap::new(),
        }
    }

    fn get_versions(&self, since_version: u64) -> Vec<Blob> {
        let last_version = self.versions.len();
        if last_version == since_version as usize {
            return vec![];
        }
        self.versions[since_version as usize..last_version]
            .iter()
            .map(|r| r.clone())
            .collect::<Vec<Blob>>()
    }

    fn add_version(&mut self, version: u64, blob: Blob) -> VersionAdd {
        // of by one here: client wants to send version 1 first
        let expected_version = self.versions.len() as u64 + 1;
        if version != expected_version {
            return VersionAdd::ExpectedVersion(expected_version);
        }
        self.versions.push(blob);

        VersionAdd::Ok
    }

    fn add_snapshot(&mut self, version: u64, blob: Blob) {
        self.snapshots.insert(version, blob);
    }
}
