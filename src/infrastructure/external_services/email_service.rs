use anyhow::{Context, Result};
use lettre::{
    message::{header::ContentType, Mailbox, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

#[derive(Debug, Clone)]
pub struct EmailService {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
    from_name: String,
}

impl EmailService {
    pub fn new(
        smtp_host: String,
        smtp_port: u16,
        smtp_username: String,
        smtp_password: String,
        from_email: String,
        from_name: String,
    ) -> Result<Self> {
        let creds = Credentials::new(smtp_username.clone(), smtp_password);

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)
            .context("Failed to create SMTP relay")?
            .port(smtp_port)
            .credentials(creds)
            .build();

        Ok(Self {
            mailer,
            from_email,
            from_name,
        })
    }

    pub async fn send_welcome_email(&self, email: &str, name: &str) -> Result<()> {
        let subject = "Welcome to Rust Learning Platform!";
        let html_body = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Welcome to Rust Learning Platform</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background-color: #f8f9fa; padding: 20px; text-align: center; border-radius: 8px; }}
        .content {{ padding: 20px; }}
        .button {{ display: inline-block; background-color: #007bff; color: white; padding: 12px 24px; text-decoration: none; border-radius: 4px; margin: 20px 0; }}
        .footer {{ text-align: center; color: #666; font-size: 14px; margin-top: 30px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🦀 Welcome to Rust Learning Platform!</h1>
        </div>
        <div class="content">
            <h2>Hello {name}!</h2>
            <p>Welcome to the Rust Learning Platform! We're excited to have you join our community of Rust developers.</p>

            <p>Here's what you can do on our platform:</p>
            <ul>
                <li>📚 Learn Rust through interactive lessons</li>
                <li>💻 Practice coding with hands-on exercises</li>
                <li>🧠 Test your knowledge with quizzes</li>
                <li>🏆 Compete on the leaderboard</li>
                <li>📈 Track your learning progress</li>
            </ul>

            <p>Ready to start your Rust journey?</p>
            <a href="http://localhost:3000/dashboard" class="button">Get Started</a>

            <p>If you have any questions, feel free to reach out to our support team.</p>

            <p>Happy coding!</p>
            <p><strong>The Rust Learning Team</strong></p>
        </div>
        <div class="footer">
            <p>This email was sent to {email}. If you didn't create an account, please ignore this email.</p>
            <p>&copy; 2024 Rust Learning Platform. All rights reserved.</p>
        </div>
    </div>
</body>
</html>"#,
        );

        let text_body = format!(
            r#"Welcome to Rust Learning Platform!

Hello {name}!

Welcome to the Rust Learning Platform! We're excited to have you join our community of Rust developers.

Here's what you can do on our platform:
- Learn Rust through interactive lessons
- Practice coding with hands-on exercises
- Test your knowledge with quizzes
- Compete on the leaderboard
- Track your learning progress

Ready to start your Rust journey? Visit: http://localhost:3000/dashboard

If you have any questions, feel free to reach out to our support team.

Happy coding!
The Rust Learning Team

---
This email was sent to {email}. If you didn't create an account, please ignore this email.
© 2024 Rust Learning Platform. All rights reserved."#,
        );

        self.send_email(email, subject, &html_body, &text_body)
            .await
    }

    pub async fn send_password_reset_email(&self, email: &str, reset_token: &str) -> Result<()> {
        let subject = "Password Reset - Rust Learning Platform";
        let reset_url = format!("http://localhost:3000/reset-password?token={reset_token}");

        let html_body = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Password Reset</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background-color: #f8f9fa; padding: 20px; text-align: center; border-radius: 8px; }}
        .content {{ padding: 20px; }}
        .button {{ display: inline-block; background-color: #dc3545; color: white; padding: 12px 24px; text-decoration: none; border-radius: 4px; margin: 20px 0; }}
        .warning {{ background-color: #fff3cd; border: 1px solid #ffeaa7; padding: 15px; border-radius: 4px; margin: 20px 0; }}
        .footer {{ text-align: center; color: #666; font-size: 14px; margin-top: 30px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔐 Password Reset Request</h1>
        </div>
        <div class="content">
            <p>We received a request to reset your password for your Rust Learning Platform account.</p>

            <p>Click the button below to reset your password:</p>
            <a href="{reset_url}" class="button">Reset Password</a>

            <div class="warning">
                <strong>⚠️ Important:</strong>
                <ul>
                    <li>This link will expire in 1 hour</li>
                    <li>If you didn't request this reset, please ignore this email</li>
                    <li>Your password will remain unchanged until you create a new one</li>
                </ul>
            </div>

            <p>If the button doesn't work, copy and paste this link into your browser:</p>
            <p style="word-break: break-all; background-color: #f8f9fa; padding: 10px; border-radius: 4px;">{reset_url}</p>

            <p>If you have any questions, feel free to contact our support team.</p>

            <p>Best regards,<br><strong>The Rust Learning Team</strong></p>
        </div>
        <div class="footer">
            <p>This email was sent to {email}. If you didn't request a password reset, please ignore this email.</p>
            <p>&copy; 2024 Rust Learning Platform. All rights reserved.</p>
        </div>
    </div>
</body>
</html>"#,
        );

        let text_body = format!(
            r#"Password Reset Request - Rust Learning Platform

We received a request to reset your password for your Rust Learning Platform account.

Click the link below to reset your password:
{reset_url}

IMPORTANT:
- This link will expire in 1 hour
- If you didn't request this reset, please ignore this email
- Your password will remain unchanged until you create a new one

If you have any questions, feel free to contact our support team.

Best regards,
The Rust Learning Team

---
This email was sent to {email}. If you didn't request a password reset, please ignore this email.
© 2024 Rust Learning Platform. All rights reserved."#,
        );

        self.send_email(email, subject, &html_body, &text_body)
            .await
    }

    pub async fn send_achievement_email(
        &self,
        email: &str,
        name: &str,
        achievement: &str,
    ) -> Result<()> {
        let subject = format!("🏆 Achievement Unlocked: {achievement}");
        let html_body = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Achievement Unlocked</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background-color: #d4edda; padding: 20px; text-align: center; border-radius: 8px; }}
        .content {{ padding: 20px; }}
        .achievement {{ background-color: #fff3cd; border: 1px solid #ffeaa7; padding: 20px; border-radius: 8px; text-align: center; margin: 20px 0; }}
        .button {{ display: inline-block; background-color: #28a745; color: white; padding: 12px 24px; text-decoration: none; border-radius: 4px; margin: 20px 0; }}
        .footer {{ text-align: center; color: #666; font-size: 14px; margin-top: 30px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🏆 Achievement Unlocked!</h1>
        </div>
        <div class="content">
            <h2>Congratulations {name}!</h2>
            <p>You've just unlocked a new achievement on the Rust Learning Platform!</p>

            <div class="achievement">
                <h3>🎖️ {achievement}</h3>
                <p>Keep up the great work! Your dedication to learning Rust is paying off.</p>
            </div>

            <p>Continue your learning journey and unlock even more achievements!</p>
            <a href="http://localhost:3000/dashboard" class="button">View Progress</a>

            <p>Best regards,<br><strong>The Rust Learning Team</strong></p>
        </div>
        <div class="footer">
            <p>This email was sent to {email}. If you have any questions, contact our support team.</p>
            <p>&copy; 2024 Rust Learning Platform. All rights reserved.</p>
        </div>
    </div>
</body>
</html>"#,
        );

        let text_body = format!(
            r#"Achievement Unlocked - Rust Learning Platform

Congratulations {name}!

You've just unlocked a new achievement on the Rust Learning Platform!

🎖️ {achievement}

Keep up the great work! Your dedication to learning Rust is paying off.

Continue your learning journey and unlock even more achievements!
Visit: http://localhost:3000/dashboard

Best regards,
The Rust Learning Team

---
This email was sent to {email}. If you have any questions, contact our support team.
© 2024 Rust Learning Platform. All rights reserved."#,
        );

        self.send_email(email, &subject, &html_body, &text_body)
            .await
    }

    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        html_body: &str,
        text_body: &str,
    ) -> Result<()> {
        let from: Mailbox = format!("{} <{}>", self.from_name, self.from_email)
            .parse()
            .context("Failed to parse from email")?;

        let to: Mailbox = to.parse().context("Failed to parse to email")?;

        let email = Message::builder()
            .from(from)
            .to(to)
            .subject(subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_PLAIN)
                            .body(text_body.to_string()),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_HTML)
                            .body(html_body.to_string()),
                    ),
            )
            .context("Failed to build email message")?;

        self.mailer
            .send(email)
            .await
            .context("Failed to send email")?;

        Ok(())
    }
}
