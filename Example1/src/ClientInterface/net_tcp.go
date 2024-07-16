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
		bSocket.streamReader = bufio.NewReader(bSocket.socket.(net.Conn))
	}

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
