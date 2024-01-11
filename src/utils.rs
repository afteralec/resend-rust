// TODO: Maybe move this to a function on the client?
pub(crate) mod url {
    pub(crate) mod emails {
        const EMAILS: &str = "emails";

        pub(crate) fn base(base_url: &str) -> String {
            format!("{}/{}", base_url, EMAILS)
        }

        pub(crate) fn with_id(base_url: &str, email_id: &str) -> String {
            format!("{}/{}/{}", base_url, EMAILS, email_id)
        }
    }

    pub(crate) mod domains {
        const DOMAINS: &str = "domains";

        pub(crate) fn base(base_url: &str) -> String {
            format!("{}/{}", base_url, DOMAINS)
        }

        pub(crate) fn with_id(base_url: &str, domain_id: &str) -> String {
            format!("{}/{}/{}", base_url, DOMAINS, domain_id)
        }
    }

    pub(crate) mod api_keys {
        const API_KEYS: &str = "api-keys";

        pub(crate) fn base(base_url: &str) -> String {
            format!("{}/{}", base_url, API_KEYS)
        }

        pub(crate) fn with_id(base_url: &str, domain_id: &str) -> String {
            format!("{}/{}/{}", base_url, API_KEYS, domain_id)
        }
    }
}
