
import { cool } from './test'
import './main.css'

console.log('BOOM FAST')
let socket_open = false
const socket = new WebSocket('wss://echo.websocket.org')

socket.addEventListener('open', event => { socket_open = true })

socket.addEventListener('message', event => { 
  console.log('Message from server ', event.data) 
  document.cookie = "last_response=" + event.data
})

const send_message = message => {
  if (socket_open) {
    console.log('Sending message:', message)
    socket.send(message)
  }
}

window.onload = () => {
  cool()
  console.log('DOM load')
  const button = document.getElementById('submit_name')
  const input = document.getElementById('name_input')
  input.addEventListener('input', e => send_message(input.value))

  //onclick = neat
  //fetch('http://127.0.0.1:7878')
  // .then(response => cool())
  // .catch(e => console.log(e))
}