![](https://cdn.nexusfn.net/file/2023/06/Rustmaker.png)

**A simple but  working matchmaker for Fortnite written in Rust with spaghetti code.**

Maybe I will rewrite it in the future, but for now it works, but there will probably be some small updates.

## How to use (With Momentum)

### Windows

1. Build the project with `cargo build --release`
2. Run the exe file in `target/release/matchmaker.exe`
3. Set the MATCHMAKER_IP to your public ip adress + the port you set in the .env file for this project

### Docker

1. Pull the docker image with `docker pull ghcr.io/nexus-fn/matchmaker:main`
2. Run the docker image with `docker run -p yourport:yourport -e PORT=yourport ghcr.io/nexus-fn/matchmaker:main`

Now it should work, if not, please open an issue or message me on discord: `Zetax#1234`