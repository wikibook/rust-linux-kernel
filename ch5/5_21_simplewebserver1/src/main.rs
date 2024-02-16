use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

async fn response_examples(req: Request<Body>) -> Result<Response<Body>> {
    let index_html = String::from("<h1>Hello World!</h>");
    let notfound_html = String::from("<h1>404 not found</h>");

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(index_html.into())),
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(notfound_html.into())
                .unwrap())
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8080".parse().unwrap();

    let new_service = make_service_fn(move |_| {
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                response_examples(req)
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}