package backyard_player

import (
	"bufio"
	"fmt"
	"net"
)

func (bSocket *BackyardSocket) BuildSocketTCP() {
	connectInfo := bSocket.ipAddress + ":" + bSocket.port

	_socket, err := net.Dial("tcp", connectInfo)
	if err != nil {
		fmt.Printf("Create Socket Error: %v\n", err)
		return
	}

	bSocket.socket = _socket
	bSocket.streamReader = bufio.NewReader(_socket.(net.Conn))
	fmt.Println("Socket successfully created and connected")
}

func (bSocket *BackyardSocket) ReadTCPSocketBuffer() string {

	//for {
	//	response, err := bSocket.streamReader.ReadString('\n')
	//	if err != nil {
	//		fmt.Println("Error reading from server:", err)
	//	}
	//
	//	fmt.Printf("Received: %s", response)
	//}
	response, err := bSocket.streamReader.ReadString('\n')
	if err != nil {
		fmt.Println("Error reading from server:", err)
	}
	return response
}

func (bSocket *BackyardSocket) SendMessageTCP(_msg string) {
	_, err := bSocket.socket.(net.Conn).Write([]byte(_msg))
	if err != nil {
		fmt.Println("Error sending message:", err)
		return
	}
}

func (bSocket *BackyardSocket) DisConnectTCP() {
	bSocket.socket.(net.Conn).Close()
}
