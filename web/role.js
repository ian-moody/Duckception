

{/*

Icon attribs

crystal ball

<div>Icons made by <a href="https://www.flaticon.com/authors/wanicon" title="wanicon">wanicon</a> from <a href="https://www.flaticon.com/" title="Flaticon">www.flaticon.com</a></div> 

wolf

<div>Icons made by <a href="https://www.freepik.com" title="Freepik">Freepik</a> from <a href="https://www.flaticon.com/" title="Flaticon">www.flaticon.com</a></div>

villager / grain

<div>Icons made by <a href="https://www.freepik.com" title="Freepik">Freepik</a> from <a href="https://www.flaticon.com/" title="Flaticon">www.flaticon.com</a></div>

*/ }

const roles = {
  villager: { title: 'Villager', info: 'You are a villager. It is your job to lynch all of the wolves.' },
  wolf: { title: 'Wolf', info: 'You are a wolf! It is your job to kill all the villagers. Wolves must collectively agree on a target to kill each night' },
  seer: { title: 'Seer', info: 'You are a seer. It is your job to detect the wolves. you may have one vision per night to determine the role of another player' },
  oracle: {},
  village_drunk: {},
  harlot: {},
  guardian_angel: {},
  bodyguard: {},
  priest: {},
  detective: {},
  investigator: {},
  prophet: {},
  augur: {},
  mystic: {},
  time_lord: {},
  matchmaker: {},
  mad_scientist: {},
  hunter: {},
  vigilante: {},
  shaman: {},
}

const get_role = role_name => {
  const role = roles[role_name]
  return role
}

export const set_role_card = (role_name = 'wolf') => {
  const card_content = document.getElementById('card-content')
  const { title, info } = get_role(role_name)
  card_content.innerHTML = `
  ${title}
  <div class="role-icon icon-${role_name}"></div>
  ${info}
  `
}

