function sleep(d){
  for(var t = Date.now();Date.now() - t <= d;);
}