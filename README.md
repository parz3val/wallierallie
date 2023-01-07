# wallierallie
Wallpaper application that helps to randomly download application from unsplash and set it to Desktop wallpaper using feh

# How to use
1. clone to repo locally
2. cd into the repo
3. Create unsplash developer account and get client_id(api_key) and client_secret(api_secret).
4. Replace .example.env with .env and replace client_id and client_secret
5. Run cargo build --release or cargo Run
6. Use the binary accordingly


# How to create a shortcut for the sxhkd
1. Copy binary to /usr/bin and put .env in your $HOME
2. Set sxhkd to fire wallierallie

