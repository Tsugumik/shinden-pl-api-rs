use crate::client::ShindenHttpClient;
use crate::error::ShindenError;
use crate::headers::RequestType;

pub async fn login(
    client: &ShindenHttpClient,
    email: &str,
    password: &str
) -> Result<(), ShindenError> {
    let url = "https://shinden.pl/main/login";

    client.get_html(url, RequestType::Frontend).await?;

    let form = [
        ("username".to_string(), email.to_string()),
        ("password".to_string(), password.to_string()),
        ("remember".to_string(), "on".to_string()),
    ];

    client.post_form(url, &form, RequestType::Login).await?;

    Ok(())
}

pub async fn logout(client: &ShindenHttpClient) -> Result<(), ShindenError> {
    let url = "https://shinden.pl/main/logout";
    client.get_html(url, RequestType::Frontend).await?;
    Ok(())
}
