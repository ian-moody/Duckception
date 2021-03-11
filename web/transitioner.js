
const sleepComments = [
  'Good night',
  'Sleep well',
  'Sweet dreams',
  'Lights out',
  'Go to sleep'
]

const wakeComments = [
  'Good morning',
  'Rise and shine',
  'Wake up'
]

const randArrayIndex = arr => Math.floor(Math.random() * arr.length)
const choose = arr => arr[randArrayIndex(arr)]

let transition

export const init_transitioner = () => transition = document.getElementById('transition')

export const dayTransition = checked => {
  transition.innerHTML = 'Day 1<br/>' + choose(checked ? sleepComments : wakeComments)
  transition.classList.toggle('on')
  transition.classList.add(checked ? 'sleep' : 'wake')
  transition.classList.remove(checked ? 'wake' : 'sleep')
  setTimeout(() => {
    document.body.classList.toggle('dark', checked)
    setTimeout(() => transition.classList.toggle('on'), 500)
  }, 1100)
}