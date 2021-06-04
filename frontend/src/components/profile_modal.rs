use crate::components::{CloseButton, ProfileIcon, SignInForm};
use anyhow::{anyhow, Result};
use chrono::{TimeZone, Utc};
use yew::{prelude::*, services::ConsoleService, web_sys::window as get_window};
use yewtil::NeqAssign;

#[derive(Debug)]
pub struct ProfileModal {
    props: Props,
    link: ComponentLink<Self>,
    is_signed_in: bool,
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    pub is_open: bool,
    pub on_close: Callback<()>,
}

pub enum Msg {
    Close,
    UpdateIsSignedIn,
}

impl ProfileModal {
    fn update_is_signed_in(&mut self) -> Result<()> {
        let window = match get_window() {
            Some(val) => val,
            None => {
                return Err(anyhow!("Failed to get window object"));
            }
        };
        let local_storage = match window.local_storage() {
            Ok(Some(val)) => val,
            _ => {
                return Err(anyhow!("Failed to get localStorage object"));
            }
        };
        if let Ok(None) | Err(_) = local_storage.get_item("access_token") {
            self.is_signed_in = false;
            return Ok(());
        };
        let expires_in_not_parsed: String = match local_storage.get_item("expires_in") {
            Ok(Some(val)) => val,
            _ => {
                self.is_signed_in = false;
                return Ok(());
            }
        };
        let expires_in: i64 = match expires_in_not_parsed.parse() {
            Ok(val) => val,
            Err(err) => {
                self.is_signed_in = false;
                return Err(anyhow!("Failed to parse localStorage.expires_in: {}", err));
            }
        };
        if Utc::now() > Utc.timestamp(expires_in, 0) {
            self.is_signed_in = false;
        } else {
            self.is_signed_in = true;
        }
        Ok(())
    }

    fn username_display(&self) -> String {
        if self.is_signed_in {
            "You are logged in".to_string()
        } else {
            "Guest".to_string()
        }
    }

    fn under_username_display(&self) -> String {
        if self.is_signed_in {
            "Have fun scrolling!".to_string()
        } else {
            "Please sign in".to_string()
        }
    }

    fn signin_form_view(&self) -> Html {
        if !self.is_signed_in {
            html! {
                <SignInForm on_signin=self.link.callback(|_| Msg::UpdateIsSignedIn)/>
            }
        } else {
            html! {}
        }
    }
}

impl Component for ProfileModal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            is_signed_in: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Close => {
                self.props.on_close.emit(());
            }
            Msg::UpdateIsSignedIn => {
                if let Err(err) = self.update_is_signed_in() {
                    ConsoleService::info(&format!("{}", err));
                }
            }
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if let Err(err) = self.update_is_signed_in() {
            ConsoleService::info(&format!("{}", err));
        }
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        if self.props.is_open {
            html! {
                <div class="profile">
                    <CloseButton on_click=self.link.callback(|_| Msg::Close)/>
                    <div class="profileDataRow">
                        <div class="profileIcon">
                            <ProfileIcon />
                        </div>
                        <div class="profileTextWrapper">
                            <p class="username">{self.username_display()}</p>
                            <p class="signIn">{self.under_username_display()}</p>
                        </div>
                    </div>
                    {self.signin_form_view()}
                </div>
            }
        } else {
            html! {}
        }
    }
}
