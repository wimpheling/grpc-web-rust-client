use example_types::{HelloRequest, HelloReply};
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
    use prost::Message;
    
    let client = Client::new("http://localhost:8081")
        .with_content_type(GrpcWebContentType::Binary);

    let req = HelloRequest { name: name.to_string() };

    let response = client
        .unary::<HelloRequest, HelloReply>("hello.Greeter", "SayHello", req)
        .await
        .map_err(|e: grpc_web_rust::Error| e.to_string())?;

    Ok(response.message)
}

fn main() {
    mount_to_body(App);
}
