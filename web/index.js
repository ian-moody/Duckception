
import { cool } from './test'
import './main.css'

let socket_open = false
const socket = new WebSocket('ws://127.0.0.1:7878')

socket.addEventListener('open', event => { socket_open = true })

socket.addEventListener('message', event => {
  document.cookie = `last_response=${event.data};SameSite=Strict`
})

const send_message = message => {
  // if (socket_open) {
  //   socket.send(message)
  // }
}

// Might not need , cannot read cookie for a different path
const getCookie = cookie_name => {
  const name = cookie_name + "="
  console.log(document.cookie)
  const ca = decodeURIComponent(document.cookie).split(';')
  console.log(ca)
  for (let i = 0; i < ca.length; i++) {
    const c = ca[i]
    while (c.charAt(0) == ' ') c = c.substring(1)
    if (c.indexOf(name) == 0) return c.substring(name.length, c.length)
  }
  return ''
}

window.onload = () => {
  cool()
  const { pathname, ...rest } = window.location

  document.getElementById('info').innerHTML = `
  version: ${process.env.npm_package_version} <br> 
  path: ${pathname} <br> 
  room: ${pathname.split('/')[2]} <br> 
  user id: ${getCookie('user_id')}
  `
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