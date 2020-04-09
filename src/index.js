const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

ctx.fillStyle = 'green';
ctx.fillRect(10, 10, 150, 100);

let state = {
    "key_down_queue":[]
}
window.addEventListener('keydown', function(event) {
    console.log(event.key);
    state.key_down_queue.push(event.key);
});
function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
  
const frame_time_ms = (1.0/60.0)*1000.0;
function game_loop(){
    console.log(state.key_down_queue);
    state.key_down_queue=[]
}
async function main(){
    while(0==0){
        game_loop();
        await sleep(frame_time_ms)
    }
}
main();