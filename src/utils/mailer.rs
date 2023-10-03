use hyper::StatusCode;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use super::{constants, api_error::APIError};


pub async fn mailer(
    from_name: &str,
    to_name: &str, 
    to_email: &str, 
    subject: &str,
    content_type: ContentType,
    body: &str
) -> Result<(),APIError> {

    let host: String = (*constants::MAILER_HOST).clone();
    let port: u16 = (*constants::MAILER_PORT).clone().parse::<u16>().unwrap_or(465);

    let email: String = (*constants::MAILER_EMAIL).clone();
    let password: String = (*constants::MAILER_PASSWORD).clone();

    let from = format!("{from_name} <{email}>");
    let to = format!("{to_name} <{to_email}>");

    let mail = Message::builder()
        .from(from.parse().unwrap())
        // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .header(content_type)
        .body(String::from(body))
        .unwrap();
    
    let creds = Credentials::new(email.to_owned(), password.to_owned(),);
    print!("{} {}",email,password);
    let mailer = SmtpTransport::relay(&host)
        .unwrap()
        .credentials(creds)
        .port(port)
        .build();
    
    mailer.send(&mail).map_err(|_| APIError{ message: "Failed to send mail".to_owned(), status_code: StatusCode::INTERNAL_SERVER_ERROR, error_code: None })?;

    Ok(())
}
