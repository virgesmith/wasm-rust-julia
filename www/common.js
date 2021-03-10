'use strict';

function colour(r, g, b) {
  var s = (r*65536+g*256+b).toString(16).toUpperCase();
  while(s.length < 6) { s = "0" + s; }
  return "#" + s;
}

export function getGreyscale() {
  var colours = [];
  for (var i=0; i < 256; ++i) {
    var x = Math.round((16 - Math.sqrt(i))*16) - 1;
    colours.push(colour(x,x,x));
    //console.log(colours[i], colour(x,x,x));
  }
  return colours;
};

export function getColours(n) {
  var colours = [];
  const u = n - 1;
  const s = u / 2;
  const t = Math.PI/u;
  for (var i=0; i < n; ++i) {
    var r = 255 - Math.round(127.5 * (1.0 - Math.cos(i *     t)));
    var g = 255 - Math.round(127.5 * (1.0 - Math.cos(i * 3 * t)));
    var b = 255 - Math.round(127.5 * (1.0 - Math.cos(i * 5 * t)));

    colours.push(colour(r, g, b));
  }
  return colours;
};
