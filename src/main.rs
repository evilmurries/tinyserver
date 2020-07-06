use hyper;

fn main() {

    let addr = ([127, 0, 0, 1], 8080).into();

    let builder = hyper::Server::bind(&addr);

    let server = builder.serve(|| {
       hyper::service::service_fn_ok(|_| {
           hyper::Response::new(hyper::Body::from("Almost microservice..."))
       })
    });

}
