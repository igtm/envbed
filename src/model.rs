#[derive(Debug)]
pub struct EnvVar {
    pub key: String,
    pub val: String,
}

#[allow(dead_code)]
impl EnvVar {
    pub fn new(key: String, val: String) -> EnvVar {
        EnvVar { key, val }
    }
}
