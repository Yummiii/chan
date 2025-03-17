use crate::{
    models::user::User,
    utils::jwt::{_verify_token, create_token},
};

const JWT_TEST_KEY: &str = "jwt_test_key";

#[test]
fn jwt() {
    let user = User {
        id: "123abc".to_string(),
        username: "test".to_string(),
        pass_hash: "test".to_string(),
    };

    let token = create_token(user, JWT_TEST_KEY);
    let verify = _verify_token(&token, JWT_TEST_KEY);

    assert!(verify.is_ok());

    let verify = verify.unwrap();
    assert_eq!(verify.id, "123abc");
    assert_eq!(verify.username, "test");
}
