// SDK上报网络质量（Web端示例）
navigator.connection.addEventListener('change', ()=> {
  cdnSdk.reportMetrics({
    rtt: navigator.connection.rtt,
    loss: calculatePacketLoss()
  });
});