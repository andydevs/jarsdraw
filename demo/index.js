/**
 * Demo entry point. Initialises a JarsdrawDemo on the page canvas and
 * wires up click and clear event handlers. Also handles resizing canvas
 */
import { JarsdrawDemo } from 'jarsdraw-demo'

// -- RESIZE CANVAS
const canvas = document.querySelector('#jarsdraw-canvas')
const main = canvas.closest('main')
let resizeCanvas = () => {
    canvas.width = main.clientWidth
    canvas.height = main.clientHeight
}
resizeCanvas()

// Create demo
let demo = new JarsdrawDemo('#jarsdraw-canvas')
console.log(`Demo size ${demo.width} x ${demo.height}`)

// Hook event handlers
document.querySelector('#jarsdraw-clear').addEventListener('click', (event) => {
    event.stopPropagation()
    demo.clear()
})
document.querySelector('#jarsdraw-canvas').addEventListener('click', (event) => {
    event.stopPropagation()
    let { offsetX: x, offsetY: y } = event
    demo.click(x, y)
})
