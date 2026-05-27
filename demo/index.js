// -- RESIZE CANVAS
const canvas = document.querySelector('#jarsdraw-canvas')
const main = canvas.closest('main')
function resizeCanvas() {
    canvas.width = main.clientWidth
    canvas.height = main.clientHeight
}
resizeCanvas()
new ResizeObserver(resizeCanvas).observe(main)


document
    .querySelector('#jarsdraw-clear')
    .addEventListener('click', (event) => {
        event.stopPropagation()
    })
document
    .querySelector('#jarsdraw-canvas')
    .addEventListener('click', (event) => {
        event.stopPropagation()
    })