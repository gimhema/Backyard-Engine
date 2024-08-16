package backyard_player

import (
	"bufio"
	"fmt"
)

type NetworkProtocol int

const (
	DEFAULT NetworkProtocol = iota //0
	TCP     NetworkProtocol = iota //1
	UDP     NetworkProtocol = iota //2
)

type BackyardSocket struct {
	protocol       NetworkProtocol
	socket         interface{}
	port           string
	ipAddress      string
	datagramBuffer []byte        // UDP Buffer
	streamReader   *bufio.Reader //  TCP Buffer
}

type SocketManager struct {
	socket BackyardSocket
}

func (socketManager *SocketManager) Init(_protocol NetworkProtocol, _port string, _ipAddress string) {
	socketManager.socket.MakeSocket(_protocol, _port, _ipAddress)
}

func (socketManager *SocketManager) ReadBuffer() {
	socketManager.socket.ReadSocketBuffer()
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

func (bSocket *BackyardSocket) ReadSocketBuffer() {

	switch bSocket.protocol {
	case TCP:
		bSocket.ReadTCPSocketBuffer()
	// case UDP:
	// 	return bSocket.ReadUDPSocketBuffer()
	}

}

func (bSocket *BackyardSocket) Send(_msg string) {

	switch bSocket.protocol {
	case TCP:
		bSocket.SendMessageTCP(_msg)
	case UDP:
		bSocket.SendMessageUDP(_msg)
	default:
		fmt.Println("Unknown socket type")
	}

}
