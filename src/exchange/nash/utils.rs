use nash_native_client::Client;
use super::NashParameters;
use super::shared::Result;

pub async fn client_from_params_failable(params: NashParameters) -> Result<Client> {
    let client = match params.credentials {
        Some(credentials) => {
            Client::from_keys(
                &credentials.secret,
                &credentials.session,
                params.affiliate_code,
                false,
                params.client_id,
                params.environment,
                params.timeout,
            )
            .await?
        }
        None => {
            Client::from_keys_path(
                None,
                None,
                false,
                params.client_id,
                params.environment,
                params.timeout,
            )
            .await?
        }
    };

    if let Some(interval) = params.sign_states_loop_interval {
        client.start_background_sign_states_loop(interval);
    }

    Ok(client)
}
