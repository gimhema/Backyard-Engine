package backyard_player

import (
	"fmt"
	"net"
)

func (bSocket *BackyardSocket) BuildSocketUDP() {
	connet_info := bSocket.ipAddress + ":" + bSocket.port

	_socket, err := net.ResolveUDPAddr("udp", connet_info)

	if err != nil {
		fmt.Println("Create Socket Error")
	} else {
		bSocket.socket = _socket
	}
}

func (bSocket *BackyardSocket) LisetnSocketUDP() {
	for {
		n, addr, err := bSocket.socket.(*net.UDPConn).ReadFromUDP(bSocket.buffer)
		if err != nil {
			fmt.Println("Error reading from server:", err)
			return
		}

		fmt.Printf("Received from %v: %s\n", addr, string(bSocket.buffer[:n]))
	}
}

func (bSocket *BackyardSocket) SendMessageUDP(_msg string) {

	_, err := bSocket.socket.(*net.UDPConn).Write([]byte(_msg))
	if err != nil {
		fmt.Println("Error sending message:", err)
	}

}
