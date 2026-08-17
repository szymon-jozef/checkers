# Game of checkers online

Currently WIP

# Architecture
## Game logic
Game logic is handled through board and game-master structs. It's self-contained

## Network
This project has network server and client written with tokio. 

## Example clients
In `src/bin/client.rs` there is an example client in cli. It works for now, but in the end there will be also a gui client. Anyone can write their own clients, due to the way it's written, tho.

# Thoughts
Server is self-contained, so frontend can be written in:
- ratatui?
- raylib bindings for rust
- egui
- macroquad
- cli lol (already kinda happening)
