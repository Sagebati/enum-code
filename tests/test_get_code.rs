use enum_code::Code;
#[derive(enum_code::Code)]
enum WrapperError {
    Test(TestError)
}
#[derive(enum_code::Code)]
#[code(100)]
enum WrapperWithCode {
    Test(TestError)
}

#[derive(enum_code::Code)]
enum TestError {
    #[code(1)]
    Tuple(String),
    #[code(2)]
    #[allow(dead_code)]
    Struct { message: String },
    #[code(3)]
    Simple,
}

#[derive(enum_code::Code)]
#[code(10)]
enum TestErrorConcat {
    #[code(1)]
    #[allow(dead_code)]
    Tuple(String),
    #[code(2)]
    #[allow(dead_code)]
    Struct { message: String },
    #[code(3)]
    Simple,
}

#[test]
fn test_code() {
    assert_eq!(TestError::Tuple("test".to_string()).get_code(), 1);
    assert_eq!(
        TestError::Struct {
            message: "test".to_string()
        }
        .get_code(),
        2
    );
    assert_eq!(TestError::Simple.get_code(), 3);

    assert_eq!(WrapperError::Test(TestError::Simple).get_code(), 3);

    assert_eq!(WrapperWithCode::Test(TestError::Simple).get_code(), 103);

    assert_eq!(TestErrorConcat::Simple.get_code(), 13);
}
