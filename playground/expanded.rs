#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use static_wrapper::define_wrapper;
use std::future::Future;
pub trait UtilTrait {
    fn add(&self, one: i32, two: i32) -> impl Future<Output = i32> + Send;
    fn say_hello(&self, name: String) -> impl Future<Output = ()> + Send;
    fn call_to_get_code(&self, url: String) -> impl Future<Output = i32>;
}
pub struct One;
pub struct Two;
impl UtilTrait for One {
    fn add(&self, one: i32, two: i32) -> impl Future<Output = i32> + Send {
        async move { one + two }
    }
    fn say_hello(&self, name: String) -> impl Future<Output = ()> + Send {
        async move {
            {
                ::std::io::_print(format_args!("hello {0} from one\n", name));
            };
        }
    }
    fn call_to_get_code(&self, url: String) -> impl Future<Output = i32> {
        async move { 1 }
    }
}
impl UtilTrait for Two {
    fn add(&self, one: i32, two: i32) -> impl Future<Output = i32> + Send {
        async move { one + two }
    }
    fn say_hello(&self, name: String) -> impl Future<Output = ()> + Send {
        async move {
            {
                ::std::io::_print(format_args!("hello {0} from two\n", name));
            };
        }
    }
    fn call_to_get_code(&self, url: String) -> impl Future<Output = i32> {
        async move { 2 }
    }
}
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
fn main() {
    let body = async {
        let one = One;
        let wrapper = Wrapper::One(one);
        let outcome: i32 = wrapper.call_to_get_code("some url".to_string()).await;
        {
            ::std::io::_print(format_args!("here is the outcome: {0}\n", outcome));
        };
        wrapper.say_hello("test name".to_string()).await;
        {
            ::std::io::_print(format_args!("Hello, world!\n"));
        };
    };
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return
    )]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
