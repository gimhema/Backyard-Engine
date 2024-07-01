package backyard_player

import (
	"fmt"
	"net"
)

type NetworkProtocol int

const (
	DEFAULT NetworkProtocol = iota //0
	TCP     NetworkProtocol = iota //1
	UDP     NetworkProtocol = iota //2
)

type BackyardSocket struct {
	protocol  NetworkProtocol
	socket    interface{}
	port      string
	ipAddress string
}

func (bSocket *BackyardSocket) MakeSocket(_protocol NetworkProtocol, _port string, _ipAddress string) {

	bSocket.protocol = _protocol

	switch _protocol {
	case TCP:
		bSocket.ipAddress = _ipAddress
		bSocket.port = _port
		bSocket.BuildSocketTCP()
	case UDP:
		bSocket.ipAddress = _ipAddress
		bSocket.port = _port
		bSocket.BuildSocketUDP()
	default:
		fmt.Println("Unknown socket type")
	}

}

func (bSocket *BackyardSocket) BuildSocketTCP() {
	connet_info := bSocket.ipAddress + ":" + bSocket.port

	_socket, err := net.Dial("tcp", connet_info)

	if err != nil {
		fmt.Println("Create Socket Error")
	} else {
		bSocket.socket = _socket
	}

}

func (bSocket *BackyardSocket) BuildSocketUDP() {
	connet_info := bSocket.ipAddress + ":" + bSocket.port

	_socket, err := net.ResolveUDPAddr("udp", connet_info)

	if err != nil {
		fmt.Println("Create Socket Error")
	} else {
		bSocket.socket = _socket
	}
}

type SocketManager struct {
	socket_container []BackyardSocket
}
