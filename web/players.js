
import prof1 from "url:./assets/prof1.jpg?as=webp&width=256&quality=65"
import prof2 from "url:./assets/prof2.jpg?as=webp&width=256&quality=65"
import prof3 from "url:./assets/prof3.jpg?as=webp&width=256&quality=65"
import prof4 from "url:./assets/prof4.jpg?as=webp&width=256&quality=65"

const profile_pictures = { prof1, prof2, prof3, prof4 }

const picsum = 'https://picsum.photos/300/300?grayscale'

export const create_player = (text, pictureName) => {
  const node = document.createElement('button')
  node.className = 'card profile'
  node.innerHTML = `${text} <img class="avatar" src="${profile_pictures[pictureName] || picsum}" />`
  return node
}

export const add_player = player_node => document.getElementById('players').appendChild(player_node)