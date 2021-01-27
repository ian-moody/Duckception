
export const addPlayer = (text, imageSource = 'https://picsum.photos/300/300?grayscale') => {
  const b = document.createElement('button')
  b.className = 'card profile'
  b.innerHTML = `${text} <img src="${imageSource}" />`
  document.getElementById('players').appendChild(b)
}