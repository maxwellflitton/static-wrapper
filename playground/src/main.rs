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



define_wrapper!(
    Wrapper,
    [One, Two],
    fn add(&self, one: i32, two: i32) -> i32 {},
    fn say_hello(&self, name: String) {},
    fn call_to_get_code(&self, url: String) -> i32 {},
);


#[tokio::main]
async fn main() {
    let one = One;

    let wrapper = Wrapper::One(one);
    let outcome: i32 = wrapper.call_to_get_code("some url".to_string()).await;
    println!("here is the outcome: {}", outcome);
    wrapper.say_hello("test name".to_string()).await;
    println!("Hello, world!");
}
