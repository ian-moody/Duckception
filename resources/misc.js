// Might not need , cannot read cookie for a different path
const getCookie = cookie_name => {
  const name = cookie_name + '='
  const ca = decodeURIComponent(document.cookie).split(';')
  console.log(document.cookie, ca)
  for (let i = 0; i < ca.length; i++) {
    const c = ca[i]
    while (c.charAt(0) == ' ') c = c.substring(1)
    if (c.indexOf(name) == 0) return c.substring(name.length, c.length)
  }
  return ''
}


// 3d hover animation for divs
let roleCard, callCount = 0
const init_role_card = () => {
  roleCard = document.getElementById('role-card')
  if (roleCard) {
    roleCard.onmousemove = e => {
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
    roleCard.onmouseleave = () => roleCard.removeAttribute('style')
  }
  else console.warn('role-card not found')
}