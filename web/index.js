import { addPlayer } from './players'

let socket
let send_message = () => console.error('Socket has not been opened yet')

const openGameSocket = ws_url => {

  socket = new WebSocket(ws_url)

  socket.addEventListener('open', event => {
    send_message = message => socket.send(message)
  })

  socket.addEventListener('message', event => {
    const message = event.data
    console.log('Got message from socket', message)
    addPlayer(message, 'prof1')
  })

}


let roleCard, callCount = 0
const roleCardMouseListener = e => {
  const rect = e.target.getBoundingClientRect()
  const x_center = rect.width / 2, y_center = rect.height / 2
  // console.log(rect)
  const x = e.clientX - rect.left - x_center
  const y = e.clientY - rect.top - y_center
  callCount++
  // console.log(callCount, x, y)
  const hyp = Math.sqrt(x * x + y * y)
  roleCard.setAttribute('style', `transform: rotate3d(${y}, ${-x}, 0, ${hyp * .1}deg); transition: 0s`)
}

// Might not need , cannot read cookie for a different path

// const getCookie = cookie_name => {
//   const name = cookie_name + '='
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
  'Wake up'
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

  addPlayer('Daniel', 'prof2')
  addPlayer('Jackie', 'prof1')
  addPlayer('Samueal', 'prof4')
  addPlayer('Sarrah', 'prof3')

  if (process.env.NODE_ENV !== 'production') {
    document.getElementById('debug').style.display = 'block'
    // const debug = document.createElement('div')
    // debug.id = 'debug'
    // debug.innerHTML = ``
    // document.body.appendChild(debug)
  }
  const { pathname, host } = window.location

  openGameSocket(`ws://${host}/ws`)

  roleCard = document.getElementById('role-card')
  roleCard.onmousemove = roleCardMouseListener
  roleCard.onmouseleave = () => roleCard.removeAttribute('style')

  transition = document.getElementById('transition')
  const button = document.getElementById('submit_name')
  const input = document.getElementById('name_input')
  const version = document.getElementById('version')
  const info = document.getElementById('info')
  const theme_toggle = document.getElementById('theme_toggle')

  theme_toggle.addEventListener('change', function () { dayTransition(this.checked) })
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