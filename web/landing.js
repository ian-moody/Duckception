
import generate from 'project-name-generator'

const get_random_room_name = () => generate().dashed

export const init_landing_page = () => {

  const randomize_room_name = document.getElementById('randomize_room_name')
  randomize_room_name.onclick = () => {
    const room_input = document.getElementById('room_input')
    room_input.value = get_random_room_name()
  }

  const join_or_create = document.getElementById('join_or_create')
  const room_input = document.getElementById('room_input')
  join_or_create.onclick = () => window.location.hash = room_input.value

}



