use std::{fmt::Debug, ops::Add};

use chrono::{Duration, Utc};
use serde_json::{json, Value};
use validator::Validate;
use yew::{
    format::Json,
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        ConsoleService, FetchService,
    },
    web_sys::window as get_window,
};
use yewtil::NeqAssign;

#[derive(Debug, Validate)]
pub struct SignInForm {
    link: ComponentLink<Self>,
    props: Props,
    fetch_task: Option<FetchTask>,
    #[validate(
        length(min = 1, message = "Email should be longer"),
        email(message = "Email has incorrect format")
    )]
    email: String,
    #[validate(length(min = 1, message = "Password should be longer"))]
    password: String,
    email_error: Option<String>,
    password_error: Option<String>,
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    pub on_signin: Callback<()>,
}

#[derive(Debug)]
pub enum Msg {
    Submit(FocusEvent),
    UpdateEmail(String),
    UpdatePassword(String),
    ReceiveSignInResponse(Result<Value, anyhow::Error>),
}

impl SignInForm {
    fn fetch_sign_in(&mut self) {
        let req_body = json!({"email": self.email,"password": self.password});
        let request = match Request::post("/api/auth/login")
            .header("Content-Type", "application/json")
            .body(Json(&req_body))
        {
            Err(err) => {
                ConsoleService::error(&format!("Error: {}", err));
                return;
            }
            Ok(req) => req,
        };
        let callback =
            self.link
                .callback(|response: Response<Json<Result<Value, anyhow::Error>>>| {
                    let Json(data) = response.into_body();
                    Msg::ReceiveSignInResponse(data)
                });

        match FetchService::fetch(request, callback) {
            Ok(task) => self.fetch_task = Some(task),
            Err(err) => ConsoleService::error(&format!("Error: {}", err)),
        }
    }

    fn email_error_view(&self) -> Html {
        if let Some(err) = &self.email_error {
            html! { <p>{err}</p> }
        } else {
            html! {}
        }
    }

    fn password_error_view(&self) -> Html {
        if let Some(err) = &self.password_error {
            html! { <p>{err}</p> }
        } else {
            html! {}
        }
    }
}

impl Component for SignInForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            fetch_task: None,
            email: "".to_string(),
            password: "".to_string(),
            email_error: None,
            password_error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(e) => {
                e.prevent_default();
                match self.validate() {
                    Ok(()) => {
                        self.fetch_sign_in();
                    }
                    Err(validation_erors) => {
                        let field_errors = validation_erors.field_errors();
                        if let Some(errs) = field_errors.get("email") {
                            if let Some(err) = errs.get(0) {
                                match &err.message {
                                    Some(message) => {
                                        self.email_error = Some(message.to_string());
                                    }
                                    None => {
                                        self.email_error = Some(format!("{}", err));
                                    }
                                };
                            }
                        }
                        if let Some(errs) = field_errors.get("password") {
                            if let Some(err) = errs.get(0) {
                                match &err.message {
                                    Some(message) => {
                                        self.password_error = Some(message.to_string());
                                    }
                                    None => {
                                        self.password_error = Some(format!("{}", err));
                                    }
                                };
                            }
                        }
                    }
                };
            }
            Msg::UpdateEmail(val) => {
                self.email = val;
            }
            Msg::UpdatePassword(val) => {
                self.password = val;
            }
            Msg::ReceiveSignInResponse(response) => {
                let res_body = match response {
                    Err(err) => {
                        ConsoleService::error(&format!("Failed to sign in, reason: {}", err));
                        return false;
                    }
                    Ok(val) => val,
                };
                if let Some(errors) = res_body["errors"].as_str() {
                    ConsoleService::error(&format!("Failed to sign in, reasons: {}", errors))
                };
                let access_token = match res_body["data"]["access_token"].as_str() {
                    Some(val) => val,
                    None => {
                        ConsoleService::error(
                            "Failed to sign in, reasons: Missing access_token in response data",
                        );
                        return false;
                    }
                };
                let token_type = match res_body["data"]["token_type"].as_str() {
                    Some(val) => val,
                    None => {
                        ConsoleService::error(
                            "Failed to sign in, reasons: Missing token_type in response data",
                        );
                        return false;
                    }
                };
                let expires_in = match res_body["data"]["expires_in"].as_i64() {
                    Some(val) => val,
                    None => {
                        ConsoleService::error(
                            "Failed to sign in, reasons: Missing expires_in in response data",
                        );
                        return false;
                    }
                };
                let window = match get_window() {
                    Some(val) => val,
                    None => {
                        ConsoleService::error("Failed to get window object");
                        return false;
                    }
                };
                let local_storage = match window.local_storage() {
                    Ok(Some(val)) => val,
                    _ => {
                        ConsoleService::error("Failed to get localStorage object");
                        return false;
                    }
                };
                if let Err(err) = local_storage.set_item("access_token", access_token) {
                    ConsoleService::error(&format!(
                        "Failed to set access_token to localStorage: {:?}",
                        err
                    ));
                    return false;
                };
                if let Err(err) = local_storage.set_item("token_type", token_type) {
                    ConsoleService::error(&format!(
                        "Failed to set token_type to localStorage: {:?}",
                        err
                    ));
                    return false;
                };
                let expire_time = Utc::now()
                    .add(Duration::seconds(expires_in.into()))
                    .timestamp()
                    .to_string();
                if let Err(err) = local_storage.set_item("expires_in", &expire_time) {
                    ConsoleService::error(&format!(
                        "Failed to set token_type to localStorage: {:?}",
                        err
                    ));
                    return false;
                };
                self.props.on_signin.emit(());
            }
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <form class="signInForm" novalidate=true onsubmit=self.link.callback(|e: FocusEvent| Msg::Submit(e)) >
                <label for="email">{"Email: "}</label>
                <input type="email" id="email" name="email" value=self.email.clone() oninput=self.link.callback(|e: InputData| Msg::UpdateEmail(e.value)) />
                {self.email_error_view()}
                <label for="password">{"Password: "}</label>
                <input type="password" id="password" name="password" value=self.password.clone() oninput=self.link.callback(|e: InputData| Msg::UpdatePassword(e.value)) />
                {self.password_error_view()}
                <input type="submit" value="Submit" />
            </form>
        }
    }
}
