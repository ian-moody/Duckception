
let socket

const game = {
  send_message: () => console.error('Socket has not been opened yet')
}

game.open_game_socket = (ws_url, callback) => {

  socket = new WebSocket(ws_url)

  socket.addEventListener('open', event => {
    game.send_message = message => socket.send(message)
    callback()
  })

  socket.addEventListener('message', event => {
    const message = event.data
    console.log('Got message from socket', message)
  })

}

export default game




