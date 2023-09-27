Auth API written in Rust 🦀 using Actix-Web framework

This is a simple API that allows you to create users and authenticate them using JWT tokens.



## Installation

### Generate RSA Key
- Generate RSA Key in [here](https://travistidwell.com/jsencrypt/demo/) and select a key size to 4096 bits.
- Then click `Generate New Keys`
- Encode the `PRIVATE KEY` and `PUBLIC KEY` to [base64](https://www.base64encode.org/)
- Copy the `PRIVATE KEY` and `PUBLIC KEY` to `.env` file as `ACCESS_TOKEN_PRIVATE_KEY` and `ACCESS_TOKEN_PUBLIC_KEY`


## TODO

- [ ] General Token
- [ ] Auth Token
- [ ] Register
- [ ] Login
- [ ] Login with Social
- [ ] Get Account
- [ ] Logout
- [ ] Refresh token
- [ ] Forgot password
- [ ] Reset password
- [ ] Email verification
- [ ] Create Tests


