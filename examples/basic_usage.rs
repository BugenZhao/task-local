use task_local::task_local;

task_local! {
    static NUMBER: u32;
    static MESSAGE: String;
}

async fn nested_function() {
    // Access task-local values from the parent scope
    println!("In nested_function: NUMBER = {}", NUMBER.get());
    println!("In nested_function: MESSAGE = {}", MESSAGE.get());
    println!("In nested_function: MESSAGE = {:?}", MESSAGE.try_get());
}

async fn example() {
    // Set task-local values for the duration of this async block
    NUMBER
        .scope(42, async {
            MESSAGE
                .scope("Hello, task-local!".to_string(), async {
                    println!("NUMBER = {}", NUMBER.get());
                    println!("MESSAGE = {}", MESSAGE.get());

                    // Values are preserved across .await points
                    nested_function().await;

                    // Nested scopes
                    NUMBER
                        .scope(100, async {
                            println!("Inside nested scope: NUMBER = {}", NUMBER.get());
                            println!("Inside nested scope: MESSAGE = {}", MESSAGE.get());
                        })
                        .await;

                    // After the nested scope, the original value is restored
                    println!("After nested scope: NUMBER = {}", NUMBER.get());
                })
                .await;
        })
        .await;

    // Outside of the scope, the values are no longer accessible
    // Uncommenting the following line would panic:
    // println!("NUMBER = {}", NUMBER.get());
    println!("NUMBER = {:?}", NUMBER.try_get());
}

// Example of using sync_scope for synchronous code
fn sync_example() {
    NUMBER.sync_scope(99, || {
        println!("In sync_scope: NUMBER = {}", NUMBER.get());

        // Nested sync_scope
        NUMBER.sync_scope(999, || {
            println!("In nested sync_scope: NUMBER = {}", NUMBER.get());
        });

        // Original value is restored
        println!("After nested sync_scope: NUMBER = {}", NUMBER.get());
    });
}

#[tokio::main]
async fn main() {
    println!("Running async example:");
    example().await;

    println!("\nRunning sync example:");
    sync_example();
}
