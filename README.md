# A basic client/server chat program

## TODO
- [ ] Phase one server
  - [x] Print a single line
  - [x] Accept an input, and print that
  - [x] Accept an input from a file
- [ ] Phase one client
  - [ ] Write to the file that the server is looking at
- [ ] Expand so that the server is some kind of REST server that the client connects to

## Phases
1. A simple server that will accept a single client connection and display everything the client says on the screen. If the client user types ".bye", the client and the server will both quit.
2. A server as before, but this time it will remain 'open' for additional connection once a client has quit. The server can handle at most one connection at a time.
3. A server as before, but this time it can handle multiple clients simultaneously. The output from all connected clients will appear on the server's screen.
4. A server as before, but this time it sends all text received from any of the connected clients to all clients. This means that the server has to receive and send, and the client has to send as well as receive.