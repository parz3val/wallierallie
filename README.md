# wallierallie
Wallpaper application that helps to randomly download application from unsplash and set it to Desktop wallpaper using feh

# How to use
1. clone to repo locally
2. cd into the repo
3. Create unsplash developer account and get client_id(api_key) and client_secret(api_secret).
4. Create env.rs file and add your environment variables like below

```rust
pub const client_id: &str = "your client id";
pub const client_secret: &str = "your client secret";
pub const oauth_url: &str = "https://unsplash.com/oauth/token/";
```

# How to create a shortcut for the sxhkd
1. Copy binary to /usr/bin
2. Set sxhkd to fire wallierallie

