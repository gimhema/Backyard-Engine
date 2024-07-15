package backyard_player

type PlayerNetworkManager struct {
	socket_manager SocketManager
}

func (pnManager *PlayerNetworkManager) Init(_protocol NetworkProtocol, _port string, _ipAddress string) {
	pnManager.socket_manager.socket.MakeSocket(_protocol, _port, _ipAddress)
}

func (pnManager PlayerNetworkManager) Run() {
	pnManager.socket_manager.socket.Listen()
}
