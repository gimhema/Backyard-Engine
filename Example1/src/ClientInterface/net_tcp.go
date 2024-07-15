package backyard_player

import (
	"bufio"
	"fmt"
	"net"
)

func (bSocket *BackyardSocket) BuildSocketTCP() {
	connet_info := bSocket.ipAddress + ":" + bSocket.port

	_socket, err := net.Dial("tcp", connet_info)

	if err != nil {
		fmt.Println("Create Socket Error")
	} else {
		bSocket.socket = _socket
	}

}

func (bSocket *BackyardSocket) LisetnSocketTCP() {
	reader := bufio.NewReader(bSocket.socket.(net.Conn))
	for {
		response, err := reader.ReadString('\n')
		if err != nil {
			fmt.Println("Error reading from server:", err)
			return
		}

		fmt.Printf("Received: %s", response)
	}
}

func (bSocket *BackyardSocket) SendMessageTCP(_msg string) {
	_, err := bSocket.socket.(net.Conn).Write([]byte(_msg))
	if err != nil {
		fmt.Println("Error sending message:", err)
		return
	}
}
