const WebSocket = require('ws')

console.log('Autobots roll out!')

const procstat = new WebSocket('ws://localhost:3012')

procstat.onopen = () => {
    console.log("We're live!")
    procstat.send("Hello, Jim!")
}

procstat.onerror = err => console.log('onError:', err)

procstat.onmessage = msg => {
    console.log('onMessage:', msg)
    procstat.send("I got your message!")
}

/*
$(function () {
  // if user is running mozilla then use it's built-in WebSocket
  window.WebSocket = window.WebSocket || window.MozWebSocket;

  var connection = new WebSocket('ws://127.0.0.1:1337');

  connection.onopen = function () {
    // connection is opened and ready to use
  };

  connection.onerror = function (error) {
    // an error occurred when sending/receiving data
  };

  connection.onmessage = function (message) {
    // try to decode json (I assume that each message
    // from server is json)
    try {
      var json = JSON.parse(message.data);
    } catch (e) {
      console.log('This doesn\'t look like a valid JSON: ',
          message.data);
      return;
    }
    // handle incoming message
  };
});
*/
