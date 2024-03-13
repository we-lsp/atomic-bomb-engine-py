<template>
  <!-- <div id="control-panel">
    <p>Click and drag to make more!</p>
    <label>Trails: </label>
    <input type="checkbox" id="trail" name="trail" checked />
    <button id="clear">Clear</button>
    <a href="https://codepen.io/jackrugile/pen/aCzHs" target="_blank"
      >View Version 2</a
    >
  </div> -->
  <div class="loading">
    <canvas ref="myCanvas" class="dark"></canvas>
  </div>
</template>

<script setup>
import { onMounted, ref } from "vue";
const myCanvas = ref(null); // 创建一个引用
let c = null;
onMounted(() => {
  c = myCanvas.value;
  window.requestAnimFrame = (function () {
    return (
      window.requestAnimationFrame ||
      window.webkitRequestAnimationFrame ||
      window.mozRequestAnimationFrame ||
      window.oRequestAnimationFrame ||
      window.msRequestAnimationFrame ||
      function (a) {
        window.setTimeout(a, 1e3 / 60);
      }
    );
  })();

  document.onselectstart = function () {
    return false;
  };
  var ctx = c.getContext("2d");
  var dpr = window.devicePixelRatio;
  var cw = window.innerWidth;
  var ch = window.innerHeight;
  c.width = cw * dpr;
  c.height = ch * dpr;
  ctx.scale(dpr, dpr);
  var rand = function (rMi, rMa) {
    return ~~(Math.random() * (rMa - rMi + 1) + rMi);
  };
  ctx.lineCap = "round";
  var orbs = [];
  var orbCount = 30;
  var radius;

  // var trailCB = document.getElementById("trail");
  var trail = true;

  var clearer = document.getElementById("clear");

  function createOrb(mx, my) {
    var dx = cw / 2 - mx;
    var dy = ch / 2 - my;
    var dist = Math.sqrt(dx * dx + dy * dy);
    var angle = Math.atan2(dy, dx);
    orbs.push({
      x: mx,
      y: my,
      lastX: mx,
      lastY: my,
      hue: 0,
      colorAngle: 0,
      angle: angle + Math.PI / 2,
      //size: .5+dist/250,
      size: rand(1, 3) / 2,
      centerX: cw / 2,
      centerY: ch / 2,
      radius: dist,
      speed: (rand(5, 10) / 1000) * (dist / 750) + 0.015,
      alpha: 1 - Math.abs(dist) / cw,
      draw: function () {
        ctx.strokeStyle = "hsla(" + this.colorAngle + ",100%,50%,1)";
        ctx.lineWidth = this.size;
        ctx.beginPath();
        ctx.moveTo(this.lastX, this.lastY);
        ctx.lineTo(this.x, this.y);
        ctx.stroke();
      },
      update: function () {
        var mx = this.x;
        var my = this.y;
        this.lastX = this.x;
        this.lastY = this.y;
        var x1 = cw / 2;
        var y1 = ch / 2;
        var x2 = mx;
        var y2 = my;
        var rise = y1 - y2;
        var run = x1 - x2;
        var slope = -(rise / run);
        var radian = Math.atan(slope);
        var angleH = Math.floor(radian * (180 / Math.PI));
        if (x2 < x1 && y2 < y1) {
          angleH += 180;
        }
        if (x2 < x1 && y2 > y1) {
          angleH += 180;
        }
        if (x2 > x1 && y2 > y1) {
          angleH += 360;
        }
        if (y2 < y1 && slope == "-Infinity") {
          angleH = 90;
        }
        if (y2 > y1 && slope == "Infinity") {
          angleH = 270;
        }
        if (x2 < x1 && slope == "0") {
          angleH = 180;
        }
        if (isNaN(angleH)) {
          angleH = 0;
        }

        this.colorAngle = angleH;
        this.x = this.centerX + Math.sin(this.angle * -1) * this.radius;
        this.y = this.centerY + Math.cos(this.angle * -1) * this.radius;
        this.angle += this.speed;
      },
    });
  }

  function orbGo(e) {
    var mx = e.pageX - c.offsetLeft;
    var my = e.pageY - c.offsetTop;
    createOrb(mx, my);
  }

  function turnOnMove() {
    c.addEventListener("mousemove", orbGo, false);
  }

  function turnOffMove() {
    c.removeEventListener("mousemove", orbGo, false);
  }

  function toggleTrails() {
    trail = true;
  }

  function clear() {
    orbs = [];
  }

  c.addEventListener("mousedown", orbGo, false);
  c.addEventListener("mousedown", turnOnMove, false);
  c.addEventListener("mouseup", turnOffMove, false);
  // trailCB.addEventListener("change", toggleTrails, false);
  // clearer.addEventListener("click", clear, false);

  var count = 100;
  while (count--) {
    createOrb(cw / 2, ch / 2 + count * 2);
  }

  var loop = function () {
    window.requestAnimFrame(loop);
    if (trail) {
      // ctx.fillStyle = "rgba(0,0,0,.1)";
      ctx.fillRect(0, 0, cw, ch);
    } else {
      ctx.clearRect(0, 0, cw, ch);
    }
    var i = orbs.length;
    while (i--) {
      var orb = orbs[i];
      var updateCount = 3;
      while (updateCount--) {
        orb.update();
        orb.draw(ctx);
      }
    }
  };

  loop();
});
const chart = ref(null);

onMounted(() => {});
</script>

<style scoped>
.loading {
  position: absolute;
  top: 50px;
  right: 4rem;
  height: 100px;
  width: 100px;
}
canvas {
  height: 100%;
  left: 0;
  margin: 0;
  padding: 0;
  position: absolute;
  top: 0;
  width: 100%;
  background: none;
}

p {
  font-size: 12px;
  margin: 0 0 5px;
}

label {
  font-size: 12px;
  font-weight: bold;
}

button {
  display: block;
  margin: 10px 0 5px;
}

a {
  border-bottom: 1px dotted #444;
  color: #fff;
  font-size: 12px;
  text-decoration: none;
}
</style>
