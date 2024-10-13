import { log } from 'rune:runtime/debug'

export const guest = {
  init() {
    log('init')
  },
  update(time, deltaTime) {
    log(`update ${time}`)
  },
  render(time, deltaTime) {
    log(`render ${time}`)
  }
}
