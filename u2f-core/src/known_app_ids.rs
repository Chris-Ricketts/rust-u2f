use ring::digest;
use std::collections::HashMap;

use app_id::AppId;

pub fn try_reverse_app_id(app_id: &AppId) -> Option<String> {
    KNOWN_APP_IDS.get(app_id).map(|s| String::from(*s))
}

lazy_static! {
    static ref KNOWN_APP_IDS: HashMap<AppId, &'static str> = {
        let mut map = HashMap::new();

        // Should be kept in sync with https://github.com/github/SoftU2F/blob/master/SoftU2FTool/KnownFacets.swift
        map.insert(from_url("https://github.com/u2f/trusted_facets"), "github.com");
        map.insert(from_url("https://demo.yubico.com"), "demo.yubico.com");
        map.insert(from_url("https://www.dropbox.com/u2f-app-id.json"), "dropbox.com");
        map.insert(from_url("https://www.gstatic.com/securitykey/origins.json"), "google.com");
        map.insert(from_url("https://vault.bitwarden.com/app-id.json"), "vault.bitwarden.com");
        map.insert(from_url("https://keepersecurity.com"), "keepersecurity.com");
        map.insert(from_url("https://api-9dcf9b83.duosecurity.com"), "duosecurity.com");
        map.insert(from_url("https://dashboard.stripe.com"), "dashboard.stripe.com");
        map.insert(from_url("https://id.fedoraproject.org/u2f-origins.json"), "id.fedoraproject.org");
        map.insert(from_url("https://lastpass.com"), "lastpass.com");

        // Additional known app IDs not yet in KnownFacets.swift
        map.insert(from_url("demo.yubico.com"), "demo.yubico.com");
        map.insert(from_url("https://gitlab.com"), "gitlab.com");
        map.insert(from_url("https://www.fastmail.com"), "www.fastmail.com");

        map
    };
}

fn from_url(url: &str) -> AppId {
    AppId::from_bytes(digest::digest(&digest::SHA256, url.as_bytes()).as_ref())
}
