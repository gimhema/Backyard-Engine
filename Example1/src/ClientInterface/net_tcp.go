package backyard_player

import (
	"bufio"
	"fmt"
	"net"
)

func (bSocket *BackyardSocket) BuildSocketTCP() {
	// connectInfo := bSocket.ipAddress + ":8080"

	_socket, err := net.Dial("tcp", "127.0.0.1:8080")
	if err != nil {
		fmt.Printf("Create Socket Error: %v\n", err)
		return
	}

	bSocket.socket = _socket
	bSocket.streamReader = bufio.NewReader(_socket.(net.Conn))
	fmt.Println("Socket successfully created and connected")
}

func (bSocket *BackyardSocket) ReadTCPSocketBuffer() {

	for {

		fmt.Printf("Read buffer . . .")
		response, err := bSocket.streamReader.ReadString('\n')
		if err != nil {
			fmt.Println("Error reading from server:", err)
		}
	
		fmt.Printf("Received: %s", response)

		// return response
	}
	
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
