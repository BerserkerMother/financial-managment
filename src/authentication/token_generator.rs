use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn random_token() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    rand_string
}
