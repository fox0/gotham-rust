use gotham::hyper::{Body, Response, StatusCode};
use gotham::hyper::http::header::WARNING;
use gotham::state::State;
use gotham::router::builder::{build_simple_router, DrawRoutes, DefineSingleRoute};
use gotham::router::response::extender::ResponseExtender;
use gotham::helpers::http::response::create_empty_response;


fn index(state: State) -> (State, Response<Body>) {
    let mut result = create_empty_response(&state, StatusCode::OK);

    result.headers_mut().insert("x-gotham", "Hello World!".parse().unwrap());
    panic!("111");

    (state, result)
}

struct MyExtender;

impl ResponseExtender<Body> for MyExtender {
    fn extend(&self, state: &mut State, response: &mut Response<Body>) {
        // let _ = state;
        println!("11111111111111111111");
        response.headers_mut().insert("x-fox", "299 example.com Deprecated".parse().unwrap());
    }
}


fn main() {
    const ADDR: &str = "127.0.0.1:7878";
    println!("Listening http://{}", ADDR);
    gotham::start(ADDR, build_simple_router(|route| {
        route.add_response_extender(StatusCode::INTERNAL_SERVER_ERROR, MyExtender);
        route.get("/").to(index);
    }))
}
