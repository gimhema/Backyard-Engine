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

}
