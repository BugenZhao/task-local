use task_local::task_local;

task_local! {
    static NUMBER: u32;
    static MESSAGE: String;
}

#[tokio::test]
async fn test_basic_functionality() {
    // Test basic scope functionality
    NUMBER
        .scope(42, async {
            assert_eq!(NUMBER.get(), 42);
            assert_eq!(NUMBER.try_get(), Ok(42));
        })
        .await;
    NUMBER.try_get().unwrap_err();

    // Test nested scopes
    NUMBER
        .scope(1, async {
            assert_eq!(NUMBER.get(), 1);
            assert_eq!(NUMBER.try_get(), Ok(1));

            NUMBER
                .scope(2, async {
                    assert_eq!(NUMBER.get(), 2);
                    assert_eq!(NUMBER.try_get(), Ok(2));
                })
                .await;

            // Original value should be restored
            assert_eq!(NUMBER.get(), 1);
            assert_eq!(NUMBER.try_get(), Ok(1));
        })
        .await;
}

#[tokio::test]
async fn test_multiple_task_locals() {
    // Test using multiple task locals together
    NUMBER
        .scope(42, async {
            MESSAGE
                .scope("Hello".to_string(), async {
                    assert_eq!(NUMBER.get(), 42);
                    assert_eq!(MESSAGE.get(), "Hello");
                })
                .await;
        })
        .await;
}

#[tokio::test]
async fn test_across_await_points() {
    async fn inner_function() {
        assert_eq!(NUMBER.get(), 99);
    }

    NUMBER
        .scope(99, async {
            assert_eq!(NUMBER.get(), 99);
            inner_function().await;
            assert_eq!(NUMBER.get(), 99);
        })
        .await;
}

#[tokio::test]
async fn test_take_value() {
    let fut = NUMBER.scope(42, async {
        // Do some work
        NUMBER.get()
    });

    let mut pinned = Box::pin(fut);

    // Complete the future
    let result = pinned.as_mut().await;
    assert_eq!(result, 42);

    // Take the value
    let value = pinned.as_mut().take_value();
    assert_eq!(value, Some(42));

    // Value should be gone after taking
    let value = pinned.as_mut().take_value();
    assert_eq!(value, None);
}

#[test]
fn test_sync_scope() {
    // Test synchronous scope
    NUMBER.sync_scope(42, || {
        assert_eq!(NUMBER.get(), 42);
        assert_eq!(NUMBER.try_get(), Ok(42));
    });

    // Test nested synchronous scopes
    NUMBER.sync_scope(1, || {
        assert_eq!(NUMBER.get(), 1);

        NUMBER.sync_scope(2, || {
            assert_eq!(NUMBER.get(), 2);
        });

        // Original value should be restored
        assert_eq!(NUMBER.get(), 1);
    });
}
