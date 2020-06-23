struct GitRemote {
    url: String,
    r#ref: String,
}

struct GitConfig {
    name: String,
    email: String
}

pub struct Git {
    remote: GitRemote,
    config: GitConfig
}

impl Git {
    fn new(remote: GitRemote, config: GitConfig) -> Self {
        Git {
            remote,
            config
        }
    }
}
