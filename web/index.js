
import { cool } from './test'
import './main.css'

// let socket_open = false
// const socket = new WebSocket('wss://echo.websocket.org')

// socket.addEventListener('open', event => { socket_open = true })

// socket.addEventListener('message', event => {
//   document.cookie = `last_response=${event.data};SameSite=Strict`
// })

const send_message = message => {
  // if (socket_open) {
  //   socket.send(message)
  // }
}

window.onload = () => {
  cool()
  const { pathname, ...rest } = window.location
  const room = pathname.slice(1)
  console.log('window onload', rest, room)
  const button = document.getElementById('submit_name')
  const input = document.getElementById('name_input')
  button.addEventListener('click', e => {
    console.log('Button pressed')
    send_message(input.value)
  })

  input.addEventListener('input', e => {
    console.log('text changed', input.value)
  })

  //onclick = neat
  //fetch('http://127.0.0.1:7878')
  // .then(response => cool())
  // .catch(e => console.log(e))
}