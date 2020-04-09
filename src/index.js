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
function draw(x,y,width,height){
    ctx.fillStyle = 'green';
    ctx.fillRect(x, y, width, height);
}
function clear(){
    ctx.clearRect(0, 0, 300, 300);
}
function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
  
const frame_time_ms = (1.0/60.0)*1000.0;
var x=0.0;
var y = 0.0;
const move_speed = 0.6;
function game_loop(){
    clear();
    if(state.key_down_queue.length>=1){
        console.log(state.key_down_queue);
    }
    for(let i in state.key_down_queue){
        if(state.key_down_queue[i]=="w"){
            y+=move_speed;
        }
        if(state.key_down_queue[i]=="s"){
            y-=move_speed;
        }
        if(state.key_down_queue[i]=="d"){
            x+=move_speed;
        }
        if(state.key_down_queue[i]=="a"){
            x-=move_speed;
        }
    }
    draw(x,y,10,10);
    state.key_down_queue=[]
    
}
async function main(){
    while(0==0){
        game_loop();
        await sleep(frame_time_ms)
    }
}
main();