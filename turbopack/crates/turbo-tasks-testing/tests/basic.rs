#![feature(arbitrary_self_types)]
#![feature(arbitrary_self_types_pointers)]
#![allow(clippy::needless_return)] // clippy bug causes false positive

use anyhow::Result;
use turbo_tasks::Vc;
use turbo_tasks_testing::{register, run, Registration};

static REGISTRATION: Registration = register!();

#[tokio::test]
async fn basic() {
    run(&REGISTRATION, || async {
        let output1 = func_without_args();
        assert_eq!(output1.await?.value, 123);

        let input = Value { value: 42 }.cell();
        let output2 = func(input);
        assert_eq!(output2.await?.value, 42);

        anyhow::Ok(())
    })
    .await
    .unwrap()
}

#[turbo_tasks::value]
struct Value {
    value: u32,
}

#[turbo_tasks::function]
async fn func(input: Vc<Value>) -> Result<Vc<Value>> {
    let value = input.await?.value;
    Ok(Value { value }.cell())
}

#[turbo_tasks::function]
async fn func_without_args() -> Result<Vc<Value>> {
    let value = 123;
    Ok(Value { value }.cell())
}
