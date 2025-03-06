# static-wrapper
a basic macro to enable a wrapper to prevent the need to dynamic dispatch for single structs.

## Motivation

I recently came across a codebase that use the following:

```rust
struct SomeStruct {
    pub handle: Arc<dyn SomeTrait>
}
```

The author explained that he didn't want loads of generic params in his structs. This makes sense but sadly, we get all the downsides of dynamic dispatch to reduce the amount of code typed. In this repo a basic macro for async functions is housed to avoid dynamic dispatch without all the generic params. 

## Solution

Lets say we have the following trait:

```rust
use std::future::Future;

pub trait UtilTrait {

    fn add(&self, one: i32, two: i32) -> impl Future<Output = i32> + Send;

    fn say_hello(&self, name: String) -> impl Future<Output = ()> + Send;

    fn call_to_get_code(&self, url: String) -> impl Future<Output = i32>;

}
```

This macro is just defined for async functions in a trait. We can implement this trait for a couple of structs as seen below:

```rust
pub struct One;
pub struct Two;

impl UtilTrait for One {

    fn add(&self, one: i32, two: i32) -> impl Future<Output = i32> + Send {
        async move {
            one + two
        }
    }

    fn say_hello(&self, name: String) -> impl Future<Output = ()> + Send {
        async move {
            println!("hello {} from one", name);
        }
    }

    fn call_to_get_code(&self, url: String) -> impl Future<Output = i32> {
        async move {
            1
        }
    }
}

impl UtilTrait for Two {

    fn add(&self, one: i32, two: i32) -> impl Future<Output = i32> + Send {
        async move {
            one + two
        }
    }

    fn say_hello(&self, name: String) -> impl Future<Output = ()> + Send {
        async move {
            println!("hello {} from two", name);
        }
    }

    fn call_to_get_code(&self, url: String) -> impl Future<Output = i32> {
        async move {
            2
        }
    }
}
```

We are now ready to define our enum and map all the functions with the following macro call:

```rust
define_wrapper!(
    Wrapper,
    [One, Two],
    fn add(&self, one: i32, two: i32) -> i32 {},
    fn say_hello(&self, name: String) {},
    fn call_to_get_code(&self, url: String) -> i32 {},
);
```

Which expands out to the code below:

```rust
pub enum Wrapper {
    One(One),
    Two(Two),
}
impl Wrapper {
    pub async fn add(&self, one: i32, two: i32) -> i32 {
        match self {
            Self::One(inner) => inner.add(one, two).await,
            Self::Two(inner) => inner.add(one, two).await,
        }
    }
    pub async fn say_hello(&self, name: String) {
        match self {
            Self::One(inner) => inner.say_hello(name).await,
            Self::Two(inner) => inner.say_hello(name).await,
        }
    }
    pub async fn call_to_get_code(&self, url: String) -> i32 {
        match self {
            Self::One(inner) => inner.call_to_get_code(url).await,
            Self::Two(inner) => inner.call_to_get_code(url).await,
        }
    }
}
```

We must `await` because although the return types are the same, the futures might be different sizes and different opaque types. However, if we `await` the future and return the types, then our arms match. We can test our wrapper enum with the following code:

```rust
#[tokio::main]
async fn main() {
    let one = One;
    let wrapper = Wrapper::One(one);
    let outcome: i32 = wrapper.call_to_get_code("some url".to_string()).await;
    println!("here is the outcome: {}", outcome);
    wrapper.say_hello("test name".to_string()).await;
}
```

This means that our struct now takes the form below:

```rust
struct SomeStruct {
    pub handle: Arc<Wrapper>
}
```

And we don't need to `match`, we just call the functions directly. If we want to add another handle, as long as that handle has implemented the trait, we can just add it to the list like the following adding the struct `AnotherHandle`:

```rust
define_wrapper!(
    Wrapper,
    [One, Twom AnotherHandle],
    fn add(&self, one: i32, two: i32) -> i32 {},
    fn say_hello(&self, name: String) {},
    fn call_to_get_code(&self, url: String) -> i32 {},
);
```