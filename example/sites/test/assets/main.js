const UPDATE_DELAY = 1000
const SECONDS_PER_UPDATE = 1

const SECONDS_IN_MINUTE = 60
const MINUTES_IN_HOUR = 60

const timer = document.querySelector('#timer')
const audio = new Audio('assets/audio.mp3')
let totalTime = 0

const numberToTimeElement = (time) => {
    return time.toString().padStart(2, '0')
}

setInterval(() => {
    totalTime += SECONDS_PER_UPDATE
    if(!timer) return

    let seconds = numberToTimeElement(totalTime % SECONDS_IN_MINUTE)
    let minutes = numberToTimeElement(Math.floor(totalTime / SECONDS_IN_MINUTE))
    let hours = numberToTimeElement(Math.floor(totalTime / SECONDS_IN_MINUTE / MINUTES_IN_HOUR))

    timer.textContent = `${hours}:${minutes}:${seconds}`
}, UPDATE_DELAY)