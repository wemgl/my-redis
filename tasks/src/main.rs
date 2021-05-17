use tokio::task;

#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3];

    // Without `move` this doesn't compile because the task needs to own its data
    // and `v` wouldn't belong to it otherwise—it would still be owned by `main`.
    // task::spawn(async {
    task::spawn(async move {
        println!("Here's a vec: {:?}", v);
    });
}

#[allow(dead_code)]
async fn return_value_example() {
    let handle = tokio::spawn(async {
        // Do some work
        "return value"
    });

    // Do some other work


    let out = handle.await.unwrap();
    println!("GOT {}", out);
}
