pub use hash_map::UsersHashMapDao;
pub use mocked::UsersMockedDao;

use super::{dtos, errors, interface};

mod hash_map;
mod mocked;
