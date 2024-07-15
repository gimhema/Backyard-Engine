package backyard_player

type PlayerNetworkManager struct {
	socket_manager SocketManager
}

func (pnManager *PlayerNetworkManager) Init() {

}

func (pnManager PlayerNetworkManager) Run() {
	pnManager.Init()
}
