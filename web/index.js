
import { cool } from './test'

let socket_open = false
const socket = new WebSocket('wss://echo.websocket.org')//'ws://127.0.0.1:7878')

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
// const getCookie = cookie_name => {
//   const name = cookie_name + "="
//   const ca = decodeURIComponent(document.cookie).split(';')
//   console.log(document.cookie, ca)
//   for (let i = 0; i < ca.length; i++) {
//     const c = ca[i]
//     while (c.charAt(0) == ' ') c = c.substring(1)
//     if (c.indexOf(name) == 0) return c.substring(name.length, c.length)
//   }
//   return ''
// }

const sleepComments = [
  'Good night',
  'Sleep well',
  'Sweet dreams',
  'Lights out',
  'Go to sleep'
]

const wakeComments = [
  'Good morning',
]

const randArrayIndex = arr => Math.floor(Math.random() * arr.length)
const choose = arr => arr[randArrayIndex(arr)]

let transition
const dayTransition = checked => {
  transition.innerHTML = 'Day 1<br/>' + choose(checked ? sleepComments : wakeComments)
  transition.classList.toggle('on')
  transition.classList.add(checked ? 'sleep' : 'wake')
  transition.classList.remove(checked ? 'wake' : 'sleep')
  setTimeout(() => {
    document.body.classList.toggle('dark', checked)
    setTimeout(() => transition.classList.toggle('on'), 500)
  }, 1100)
}

// window.onload = () => {
window.addEventListener('DOMContentLoaded', () => {
  cool()
  const { pathname, ...rest } = window.location

  transition = document.getElementById("transition")
  const button = document.getElementById('submit_name')
  const input = document.getElementById('name_input')
  const version = document.getElementById('version')
  const info = document.getElementById('info')
  const theme_toggle = document.getElementById("theme_toggle")

  theme_toggle.addEventListener('change', function () { dayTransition(this.checked) });
  version.innerHTML = `Version ${process.env.npm_package_version}`
  info.innerHTML = `
  path: ${pathname} <br>
  room: ${pathname.split('/')[2]} <br>
  `

  button.addEventListener('click', e => {
    console.log('Button pressed')
    send_message(input.value)
  })

  input.addEventListener('input', e => {
    console.log('text changed', input.value)
  })

})