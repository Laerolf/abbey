const EXAMPLE_USER_EMAIL: &str = "ozzy@in.heaven";
const EXAMPLE_USER_PASSWORD: &str = "live";

pub const EXAMPLE_GAME_SEED: u64 = 666666;

/// Represents a User fixture used in tests.
pub struct TestUserFixture {
    /// The email of the test user.
    pub email: String,
    /// The password of the test user.
    pub password: String,
}

/// Returns a User fixture used in tests.
pub fn test_user_fixture() -> TestUserFixture {
    TestUserFixture {
        email: EXAMPLE_USER_EMAIL.into(),
        password: EXAMPLE_USER_PASSWORD.into(),
    }
}
