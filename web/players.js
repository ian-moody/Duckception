
import prof1 from "url:./assets/prof1.jpg?as=webp&width=256&quality=65"
import prof2 from "url:./assets/prof2.jpg?as=webp&width=256&quality=65"
import prof3 from "url:./assets/prof3.jpg?as=webp&width=256&quality=65"
import prof4 from "url:./assets/prof4.jpg?as=webp&width=256&quality=65"

const profile_pictures = { prof1, prof2, prof3, prof4 }

const picsum = 'https://picsum.photos/300/300?grayscale'

export const addPlayer = (text, pictureName) => {
  const b = document.createElement('button')
  b.className = 'card profile'
  b.innerHTML = `${text} <img src="${profile_pictures[pictureName] || picsum}" />`
  document.getElementById('players').appendChild(b)
}