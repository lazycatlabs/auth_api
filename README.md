
![Auth API](https://github.com/lazycatlabs/auth_api/assets/1531684/4911962d-e8a2-49e3-a615-17edfa8ea913)

Auth API written in Rust 🦀 using Actix-Web framework

This is a simple API that allows you to create users and authenticate them using JWT tokens.

## Installation ⚙️

- Install Rust from [here](https://www.rust-lang.org/tools/install)
- Install Postgres via Docker
```bash
docker compose up -d
```
- Install Diesel CLI from [here](https://diesel.rs/guides/getting-started/)
```bash
cargo install diesel_cli --no-default-features --features postgres
```

- Run the migration

```bash
diesel migration run
```

### Generate RSA Key

- Generate RSA Key in [here](https://travistidwell.com/jsencrypt/demo/) and select a key size to 4096 bits.
- Then click `Generate New Keys`
- Encode the `PRIVATE KEY` and `PUBLIC KEY` to [base64](https://www.base64encode.org/)
- Copy the `PRIVATE KEY` and `PUBLIC KEY` to `.env` file as `ACCESS_TOKEN_PRIVATE_KEY` and `ACCESS_TOKEN_PUBLIC_KEY`

### Run the project
- Install cargo watch and run the project on your local machine

```bash
cargo install cargo-watch
cargo watch -q -c -w src/ -x run
```

### For macOS users
If you face an error like this

```bash
note: ld: library 'pq' not found
```
you can fix it by running this command
```bash
brew link --force libpq
cargo clean
cargo build
```

## API Collection 📚
[![Run in Postman](https://run.pstmn.io/button.svg)](https://documenter.getpostman.com/view/27178159/2s9YXe8jwf)

## TODO 📝

- [x] Health Check
- [x] General Token
- [x] Auth Token
- [x] Register
- [x] Login
- [ ] Login with Social (Google, Apple)
- [x] Profile
    - [x] Get
    - [x] Update
    - [x] Delete
- [x] User session
- [x] Logout
- [ ] Refresh token
- [ ] Forgot password
- [x] Update password
- [x] Send Email via [MailJet](https://app.mailjet.com)
- [ ] Email verification
- [ ] Create Tests
- [x] Create docker-compose to build the project
- [x] List of user with pagination 

---
<h3 align="center">❤️ Buy me coffee if you love my works ☕️</h3>
<p align="center">
  <a href="https://www.buymeacoffee.com/Lzyct" target="_blank">
    <img src="https://www.buymeacoffee.com/assets/img/guidelines/download-assets-sm-2.svg" alt="buymeacoffe" style="vertical-align:top; margin:8px" height="36">
  </a>&nbsp;&nbsp;&nbsp;&nbsp;
   <a href="https://ko-fi.com/Lzyct" target="_blank">
    <img src="https://help.ko-fi.com/system/photos/3604/0095/9793/logo_circle.png" alt="ko-fi" style="vertical-align:top; margin:8px" height="36">
  </a>&nbsp;&nbsp;&nbsp;&nbsp;
  <a href="https://paypal.me/ukieTux" target="_blank">
    <img src="https://blog.zoom.us/wp-content/uploads/2019/08/paypal.png" alt="paypal" style="vertical-align:top; margin:8px" height="36">
  </a>
  <a href="https://saweria.co/Lzyct" target="_blank">
   <img src="https://1.bp.blogspot.com/-7OuHSxaNk6A/X92QPg8L9kI/AAAAAAAAG0E/lUzKf_uuVP8jCqvXpA7juh_l-TfK2jnbwCLcBGAsYHQ/s16000/SAWERIA.webp" style="vertical-align:top; margin:8px" height="36">
  </a>
</p>
<br><br>
