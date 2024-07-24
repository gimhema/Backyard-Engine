package backyard_player

type PlayerNetworkManager struct {
	socket_manager SocketManager
}

func (pnManager *PlayerNetworkManager) Init(_protocol NetworkProtocol, _port string, _ipAddress string) {
	pnManager.socket_manager.Init(_protocol, _port, _ipAddress)
}

func (pnManager PlayerNetworkManager) Run() {
	// pnManager.socket_manager.StartListen()
}

func (pnManager PlayerNetworkManager) Read() string {
	return pnManager.socket_manager.ReadBuffer()
}

func (pnManager PlayerNetworkManager) Send(msg string) {
	pnManager.socket_manager.socket.Send(msg)
}

func (pnManager PlayerNetworkManager) DisConnect() {
	pnManager.socket_manager.socket.DisConnectTCP()
	pnManager.socket_manager.socket.DisConnectUDP()
}
