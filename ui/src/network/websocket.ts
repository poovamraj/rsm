export default function connectSocket() {
    const socket = new WebSocket('ws://localhost:8080');
    socket.addEventListener('open', function (event) {
        socket.send(JSON.stringify({
            type: "ConnectUi"
        }));
    });

    // Listen for messages
    socket.addEventListener('message', function (event) {
        console.log('Message from server ', event.data);
    });
}