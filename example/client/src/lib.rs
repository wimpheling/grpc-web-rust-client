use example_types::{HelloRequest, HelloReply, CountRequest};
use grpc_web_rust::{Client, GrpcWebContentType};
use leptos::*;
use leptos_router::{Router, Routes, Route};
use wasm_bindgen::prelude::*;
use futures::{StreamExt, Stream};
use std::pin::Pin;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <a href="/">"Hello World"</a>
                <a href="/count">"Count Arithmetic"</a>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=HelloPage />
                    <Route path="/count" view=CountPage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn HelloPage() -> impl IntoView {
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
                Ok(msg) => {
                    set_response.set(msg.clone());
                    // Write response to DOM directly for test verification
                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            if let Some(body) = document.body() {
                                let div = document.create_element("div").unwrap();
                                div.set_id("grpc-result");
                                div.set_text_content(Some(&msg));
                                let _ = body.append_child(&div);
                            }
                        }
                    }
                }
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

#[component]
fn LogEntry(log: String) -> impl IntoView {
    view! { <p>{log}</p> }
}

#[component]
pub fn CountPage() -> impl IntoView {
    let (start, set_start) = create_signal(1);
    let (logs, set_logs) = create_signal(Vec::<String>::new());
    let (error, set_error) = create_signal("".to_string());

    let count = move |_| {
        let start_val = start.get();
        let set_logs = set_logs.clone();
        let set_error = set_error.clone();

        wasm_bindgen_futures::spawn_local(async move {
            set_error.set("".to_string());
            set_logs.set(vec!["Starting...".to_string()]);
            web_sys::console::log_1(&"CountArithmetic started".into());

            let client = Client::new("http://localhost:8081")
                .with_content_type(GrpcWebContentType::Binary);
            let req = CountRequest { start: start_val };
            
            let mut stream: Pin<Box<dyn Stream<Item = Result<example_types::CountResponse, grpc_web_rust::Error>>>> = client.server_streaming("arithmetic_progression_streaming.ArithmeticProgressionStreaming", "CountArithmeticProgression", req);
            web_sys::console::log_1(&"Stream created".into());

            while let Some(result) = stream.next().await {
                web_sys::console::log_1(&"Loop iteration".into());
                match result {
                    Ok(resp) => {
                        let msg = format!("Received: {}", resp.value);
                        web_sys::console::log_1(&msg.clone().into());
                        set_logs.update(|logs| logs.push(msg));
                    }
                    Err(e) => {
                        let err_msg = format!("Stream error: {}", e);
                        web_sys::console::log_1(&err_msg.clone().into());
                        set_error.set(err_msg);
                        break;
                    }
                }
            }
            web_sys::console::log_1(&"Loop finished".into());
        });
    };

    view! {
        <div>
            <h1>"Count Arithmetic Progression"</h1>
            <p>"Start value: " {move || start.get()}</p>
            <input
                type="number"
                value=start.get()
                on:input=move |ev| {
                    if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                        set_start.set(val);
                    }
                }
            />
            <button on:click=count>"Start Counting"</button>

            <h2>"Log:"</h2>
            <div id="stream-log">
                <p>{move || logs.get().join(", ")}</p>
            </div>

            <h2>"Error:"</h2>
            <p style:color="red">{error.get()}</p>
        </div>
    }
}

async fn call_greeter(name: &str) -> Result<String, String> {
    let client = Client::new("http://localhost:8081")
        .with_content_type(GrpcWebContentType::Binary);

    let req = HelloRequest { name: name.to_string() };

    let response = client
        .unary::<HelloRequest, HelloReply>("hello.Greeter", "SayHello", req)
        .await
        .map_err(|e: grpc_web_rust::Error| e.to_string())?;

    Ok(response.message)
}


