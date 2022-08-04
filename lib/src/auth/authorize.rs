use tiny_http::{Server, Response};

#[derive(Debug)]
pub enum AuthorizeError {
    WebBrowserCouldNotOpen(std::io::Error),
    CallbackServerStartup(Box<dyn std::error::Error + Send + Sync>),
    CallbackListenFailure(std::io::Error),
    UnexpectedResponse
}

pub fn authorize_interactively(client_id: &str, port: u16) -> Result<String, AuthorizeError> {
    let callback_location = format!("localhost:{}", port);

    webbrowser::open(&format!(
        "https://ticktick.com/oauth/authorize\
         ?client_id={}\
         &scope=tasks:write%20tasks:read\
         &redirect_uri=http://{}&state={{}}\
         &response_type=code\
        ",
        client_id, callback_location
    )).map_err(AuthorizeError::WebBrowserCouldNotOpen)?;

    let callback_server = Server::http(callback_location)
        .map_err(AuthorizeError::CallbackServerStartup)?;

    let received = callback_server.recv().map_err(AuthorizeError::CallbackListenFailure)?;

    let code = received
        .url()
        .split(&['?', '&'])
        .skip(1)
        .map(|q| q.splitn(2, '=').collect::<Vec<_>>())
        .find(|v| v[0] == "code")
        .map(|v| v[1].to_string())
        .ok_or(AuthorizeError::UnexpectedResponse);

    received.respond(Response::from_string("Okay! Please back to the application to continue.")).ok();

    code
}
