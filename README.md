# multi-game
it is a project that combines multiple projects together
we also have a website 
<a href="https://multi-game.b82.dev"> click to view our website</a>
## run options
### from source
simply run `cargo run --release`
to run the program but this relies on these dependenceys
- rust
- python
- pygame
- java
### from releases (recommend)
go to the latest release and download it for your specific os we support the following operating systems
- windows 10/11 on x86_64
- windows 10/11 on arm64 coming soon
- linux appimage on x86_64
- linux appimage on arm64
- Macos on apple silicon
- Macos on intels x86_64 bit
### Docker
- to use docker first build the image with
- `docker build -t multi-game .`
- then run it with
- `docker run -it multi-game`
- It is worth noting that Pong won't work properly inside Docker, even though all the other applications will.
### shell install (coming soon) *(only for the real ones on linux, bsd or the lucky ones on macos)*
```sh
curl -fsSL https://raw.githubusercontent.com/Ben8282/multi-game/main/install.sh | sh
```
- once it is done installing you can just run in your terminal to run it 
```sh
multi-game
```
- ***this is only coming soon and is not implented yet***
