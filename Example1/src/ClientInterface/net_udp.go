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

func (bSocket *BackyardSocket) ReadUDPSocketBuffer() string {
	//for {
	//	n, addr, err := bSocket.socket.(*net.UDPConn).ReadFromUDP(bSocket.datagramBuffer)
	//	if err != nil {
	//		fmt.Println("Error reading from server:", err)
	//		return
	//	}
	//	// game.EventListen(string(bSocket.buffer[:n]))
	//	fmt.Printf("Received from %v: %s\n", addr, string(bSocket.datagramBuffer[:n]))
	//}

	data, _, _ := bSocket.socket.(*net.UDPConn).ReadFromUDP(bSocket.datagramBuffer)

	return string(bSocket.datagramBuffer[:data])
}

func (bSocket *BackyardSocket) SendMessageUDP(_msg string) {

	_, err := bSocket.socket.(*net.UDPConn).Write([]byte(_msg))
	if err != nil {
		fmt.Println("Error sending message:", err)
	}

}
