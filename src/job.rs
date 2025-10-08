use serde::{Deserialize, Serialize};

macro_rules! create_struct {
    ($st_name:ident, $( $st_fn:ident | $st_ft:ty )*) => {
        #[derive(Debug, Default, Clone, Serialize, Deserialize)]
        pub struct $st_name{
            $(
                pub $st_fn: $st_ft,
            )*
        }
    };
}

create_struct!(
    Job,
    name | String
    freq | String
    done_at | String
    output | bool
);

/*
#[derive(Debug, Default)]
pub struct Job {
    name: String,
    freq: String,
    // NaiveDate::from_ymd()
    done_at: String,
    output: bool,
}
*/

// entry, pub vis,
// thread spawn for tokio axum,
