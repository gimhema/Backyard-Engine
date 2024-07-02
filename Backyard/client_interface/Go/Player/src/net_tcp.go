package backyard_player

import (
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

}

func (bSocket *BackyardSocket) SendMessageTCP(_msg string) {

}
