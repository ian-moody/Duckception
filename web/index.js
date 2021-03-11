import { add_player, create_player } from './players'
import { init_landing_page } from './landing'
import { set_role_card } from './role'
import { dayTransition, init_transitioner } from './transitioner'
import game from './game'


const get_room_name = () => decodeURIComponent(window.location.hash).substring(1)

const pages = {}

const init_pages = () => {
  const html_pages = [...document.getElementsByClassName('page')]
  html_pages.forEach(page => {
    document.body.removeChild(page)
    page.style.display = 'flex'
    pages[page.id] = page
  })
}

let current_page_name
const show_page = name => {
  if (!(name in pages)) throw new Error(`Trying to show page '${name}' by is not there`)
  console.log('current_page_name', current_page_name)
  if (current_page_name) document.body.removeChild(pages[current_page_name])
  document.body.prepend(pages[name])
  current_page_name = name
}

let game_room_initailized = false
let inc = 1

const test_players = [
  create_player('Daniel', 'prof2'),
  create_player('Jackie', 'prof1'),
  create_player('Samueal', 'prof4'),
  create_player('Sarrah', 'prof3')
]

const init_game_room_page = () => {
  if (!game_room_initailized) {
    const leave_room = document.getElementById('leave-room')
    leave_room.onclick = () => window.location.hash = ''

    const button = document.getElementById('submit_name')
    const name_input = document.getElementById('name_input')

    button.addEventListener('click', e => {
      game.send_message(name_input.value + ' increment ' + inc++)
    })

    // use player card???
    // const you = create_player('Your name!', 'prof1')
    // document.getElementById('yourself').replaceWith(you)

    test_players.forEach(node => add_player(node))

    set_role_card()
    init_transitioner()

    const roleCard = document.getElementById('role-card')
    roleCard.onclick = () => roleCard.classList.add('flipped')

    if (process.env.NODE_ENV !== 'production') document.getElementById('debug').style.display = 'block'

    const theme_toggle = document.getElementById('theme_toggle')
    theme_toggle.addEventListener('change', function () { dayTransition(this.checked) })
    const info = document.getElementById('info')
    info.innerHTML = `room: ${get_room_name()} <br>`

    game_room_initailized = true
  }
}

const enter_room = room_name => {
  console.log('Entering Room', room_name)
  game.send_message(`JOIN:${room_name}`)
  show_page('game-room')
  init_game_room_page()
}

const enter_landing = () => {
  console.log('Entering Landing')
  show_page('landing')
  init_landing_page()
}

const navigate = () => {
  const room_name = get_room_name()
  if (room_name) enter_room(room_name)
  else enter_landing()
}

window.onhashchange = navigate

window.addEventListener('DOMContentLoaded', () => {
  init_pages()
  const version = document.getElementById('version')
  version.innerHTML = `Version ${process.env.npm_package_version}`
  game.open_game_socket(`ws://${window.location.host}/ws`, navigate)
})