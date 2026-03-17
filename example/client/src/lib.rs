pub mod hello {
    include!("hello.rs");
}

use hello::{HelloRequest, HelloReply};
use leptos::*;
use grpc_web_rust::{Client, GrpcWebContentType};
use prost::Message;

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = create_signal("World".to_string());
    let (response, set_response) = create_signal("".to_string());
    let (error, set_error) = create_signal("".to_string());

    let greet = move |_| {
        let name = name.get();
        let set_response = set_response.clone();
        let set_error = set_error.clone();

        wasm_bindgen_futures::spawn_local(async move {
            set_response.set("Calling server...".to_string());
            set_error.set("".to_string());

            let result = call_greeter(&name).await;

            match result {
                Ok(msg) => set_response.set(msg),
                Err(e) => set_error.set(e),
            }
        });
    };

    view! {
        <div>
            <h1>"gRPC-Web + Leptos Example"</h1>
            <input
                type="text"
                value=name.get()
                on:input=move |ev| set_name.set(event_target_value(&ev))
            />
            <button on:click=greet>"Say Hello"</button>

            <h2>"Response:"</h2>
            <p>{response.get()}</p>

            <h2>"Error:"</h2>
            <p style:color="red">{error.get()}</p>
        </div>
    }
}

async fn call_greeter(name: &str) -> Result<String, String> {
    let client = Client::new("http://127.0.0.1:50051")
        .with_content_type(GrpcWebContentType::Binary);

    let req = HelloRequest::new(name);

    let response: Vec<u8> = client
        .unary("hello.Greeter", "SayHello", req)
        .await
        .map_err(|e| e.to_string())?;

    let reply = HelloReply::decode(response.as_slice())
        .map_err(|e| e.to_string())?;

    Ok(reply.message)
}

fn main() {
    mount_to_body(App);
}
