## Run the project

### Install bun
```
curl -fsSL https://bun.sh/install | bash
```

### Install dependencies
```
bun install
```

### Compiles and runs the project for development
```
bun run dev
```

### Compiles and optimizes the project for production
```
bun run build
```

## Roadmap
#### v0.1.0
- [x] Play/Pause song
  - [x] Control with Spacebar
- [x] Display song duration/position
- [x] Display album cover
- [x] Song queue
  - [x] Repeat one song
  - [x] Skip to previous/next song
- [x] Seek backward/forward
  - [x] Control with arrow keys
- [x] Volume controls
  - [x] Control with arrow keys
- [x] Playlists
  - [x] Shuffle
  - [x] Repeat (by default)
  - [x] Based on metadata
- [x] Custom CSS

### v0.1.1
- [x] Mini player indicator in library
- [ ] Playlists
  - [ ] Based on directories
  - [ ] Custom
- [ ] MPRIS support
- [ ] Youtube download/search support
- [ ] Fetch metadata from internet
   
- Took 69.5 hours to build

| Format | Seeking Support | Cover Art Support |
| ------ | --------------- | ----------------- |
| MP3    | Yes             | Yes               |
| OGG    | Yes             | Yes               |
| FLAC   | No              | Yes               |
| M4A    | Yes             | Yes               |
