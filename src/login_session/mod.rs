use crate::enums::{
    AuthTokenPlatformType,
    AuthSessionGuardType,
};
use crate::interfaces::{
    StartSessionResponseValidAction,
    LoginSessionOptions,
    StartAuthSessionResponse,
    AuthenticationClientConstructorOptions,
};
use crate::types::DateTime;
use crate::authentication_client::AuthenticationClient;
use crate::helpers::{USER_AGENT, decode_jwt, self};
use chrono::Duration;
use reqwest::Client;
use steamid_ng::SteamID;
use tokio::task::JoinHandle;

use crate::transports::WebSocketCMTransport;

#[derive(Debug)]
pub struct LoginSession {
    pub login_timeout: Duration,
    account_name: Option<String>,
    access_token: Option<String>,
    refresh_token: Option<String>,
    platform_type: AuthTokenPlatformType,
    client: Client,
    handler: AuthenticationClient,
    steam_guard_code: Option<String>,
    steam_guard_machine_token: Option<String>,
    start_session_response: Option<StartAuthSessionResponse>,
    had_remote_interaction: Option<bool>,
    polling_started_time: Option<DateTime>,
    // use tokio::task::JoinHandle
    poll_timer: Option<JoinHandle<()>>,
    polling_canceled: Option<bool>,
    access_token_set_at: Option<DateTime>,
}

impl LoginSession {
    pub async fn connect(
        platform_type: AuthTokenPlatformType,
        options: LoginSessionOptions,
    ) -> Result<Self, LoginSessionError> {
        // probably reqwest client
        // let agent:HTTPS.Agent = options.agent || new HTTPS.Agent({keepAlive: true});

		// if (options.httpProxy) {
		// 	agent = StdLib.HTTP.getProxyAgent(true, options.httpProxy) as HTTPS.Agent;
		// } else if (options.socksProxy) {
		// 	agent = new SocksProxyAgent(options.socksProxy);
		// }

        let client = Client::new();
        let transport = WebSocketCMTransport::connect().await?;
        
        Ok(Self {
            login_timeout: Duration::seconds(30),
            account_name: None,
            access_token: None,
            refresh_token: None,
            platform_type,
            client: client.clone(),
            handler: AuthenticationClient::new(AuthenticationClientConstructorOptions {
                platform_type,
                client,
                machine_id: options.machine_id,
                transport,
                user_agent: options.user_agent.unwrap_or_else(|| USER_AGENT),
            }),
            steam_guard_code: None,
            steam_guard_machine_token: None,
            start_session_response: None,
            had_remote_interaction: None,
            polling_started_time: None,
            poll_timer: None,
            polling_canceled: None,
            access_token_set_at: None,
        })
    }

    pub fn steamid(&self) -> Option<SteamID> {
        if let Some(start_session_response) = &self.start_session_response {
            if let Some(steamid) = start_session_response.steamid {
                return Some(steamid);
            }
        }

        let token = if let Some(access_token) = &self.access_token {
            // let decodedToken = decodeJwt(this.accessToken);
			// return new SteamID(decodedToken.sub);
            Some(access_token)
        } else if let Some(refresh_token) = &self.refresh_token {
            Some(refresh_token)
        } else {
            None
        };

        if let Some(token) = token {
            // let token = this.accessToken || this.refreshToken;
			// let decodedToken = decodeJwt(token);
			// return new SteamID(decodedToken.sub);
        }

        None
    }

    pub fn get_account_name(&self) -> &Option<String> {
        &self.account_name
    }

    pub fn get_access_token(&self) -> &Option<String> {
        &self.access_token
    }

    pub fn set_access_token(&mut self, access_token: String) -> Result<(), LoginSessionError> {
        if access_token.is_empty() {
            self.access_token = None;
            return Ok(());
        }
        
        let decoded = decode_jwt(&access_token)?;
        
        if decoded.audience.iter().any(|a| a == "derive") {
            return Err(LoginSessionError::ExpectedAccessToken);
        }
        
        if let Some(start_session_response) = &self.start_session_response {
            if let Some(steamid) = start_session_response.steamid {
                if steamid != decoded.steamid {
                    return Err(LoginSessionError::TokenIsForDifferentAccount);
                }
            }
        }
        
        if let Some(refresh_token) = &self.refresh_token {
            let decoded_refresh_token = decode_jwt(refresh_token)?;
            
            if decoded_refresh_token.steamid != decoded.steamid {
                return Err(LoginSessionError::AccessTokenBelongsToOtherAccount);
            }
        }
        
        self.access_token = Some(access_token);
        self.access_token_set_at = Some(chrono::Utc::now());
        
        Ok(())
    }
    
    async fn process_start_session_response(&mut self) {
        self.polling_canceled = Some(false);

        let mut valid_actions: Vec<StartSessionResponseValidAction> = Vec::new();

        if let Some(start_session_response) = &self.start_session_response {
            for allowed_confirmation in &start_session_response.allowed_confirmations {
                match allowed_confirmation.r#type {
                    AuthSessionGuardType::None => {

                    },
                    AuthSessionGuardType::EmailCode |
                    AuthSessionGuardType::DeviceCode => {
                        let code_type: String = if allowed_confirmation.r#type == AuthSessionGuardType::EmailCode {
                            "email"
                        } else {
                            "device"
                        }.into();
                        
                    },
                    AuthSessionGuardType::DeviceConfirmation |
                    AuthSessionGuardType::EmailConfirmation => {
                        valid_actions.push(StartSessionResponseValidAction {
                            r#type: allowed_confirmation.r#type,
                            detail: None,
                        })
                    },
                    AuthSessionGuardType::MachineToken => {
                        // Do nothing here since this is handled by _attemptEmailCodeAuth
                    },
                    r#type => {
                        // error unknown guard type
                    },
                }
            }
        }
    }

    async fn attempt_email_code_auth(&self) {
        todo!()
    }

    async fn attempt_totp_code_auth(&mut self) -> bool {
        if let Some(steam_guard_code) = &self.steam_guard_code {

        }

        false
    }

    async fn submit_steam_guard_code(&mut self) -> Result<(), LoginSessionError> {
        self.verify_started(true)?;

        if let Some(start_session_response) = &self.start_session_response {
            let needs_email_code = start_session_response.allowed_confirmations
                .iter()
                .any(|allow_confirmation| allow_confirmation.r#type == AuthSessionGuardType::EmailCode);
            let needs_totp_code = start_session_response.allowed_confirmations
                .iter()
                .any(|allow_confirmation| allow_confirmation.r#type == AuthSessionGuardType::DeviceCode);

            if !needs_email_code && !needs_totp_code {
                return Err(LoginSessionError::LoginAttemptSteamGuardNotRequired);
            }

            Ok(())
        } else {
            return Err(LoginSessionError::LoginSessionHasNotStarted);
        }
    }

    fn verify_started(&self, must_have_steamid: bool) -> Result<(), LoginSessionError> {
        if self.start_session_response.is_none() {
            return Err(LoginSessionError::LoginSessionHasNotStarted);
        }

        if self.polling_canceled.unwrap_or(false) {
            return Err(LoginSessionError::LoginAttemptCancelled);
        }

        if must_have_steamid && self.steamid().is_none() {
            return Err(LoginSessionError::LoginCannotUseMethodWithScheme);
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LoginSessionError {
    #[error("Login session has not been started yet")]
    LoginSessionHasNotStarted,
    #[error("Login attempt has been canceled")]
    LoginAttemptCancelled,
    #[error("Cannot use this method with this login scheme")]
    LoginCannotUseMethodWithScheme,
    #[error("No Steam Guard code is needed for this login attempt")]
    LoginAttemptSteamGuardNotRequired,
    #[error("Websocket CM: {}", .0)]
    WebSocketCM(#[from] crate::transports::websocket::Error),
    #[error("Decode error: {}", .0)]
    Decode(#[from] crate::helpers::DecodeError),
    #[error("The provided token is a refresh token, not an access token'")]
    ExpectedAccessToken,
    #[error("Token is for a different account. To work with a different account, create a new LoginSession.")]
    TokenIsForDifferentAccount,
    #[error("This access token belongs to a different account from the set refresh token.")]
    AccessTokenBelongsToOtherAccount,
}