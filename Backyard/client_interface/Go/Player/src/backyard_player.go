package backyard_player

type NetworkProtocol int

const (
	DEFAULT NetworkProtocol = iota //0
	TCP     NetworkProtocol = iota //1
	UDP     NetworkProtocol = iota //2
)

type NetworkSettingInfo struct {
	protocol  NetworkProtocol
	port      int
	ipAddress string
}

type PlayerNetworkManager struct {
	setting NetworkSettingInfo
}

func (pnManager *PlayerNetworkManager) Init(_setting NetworkSettingInfo) {
	pnManager.setting = _setting
}

func (pnManager PlayerNetworkManager) Run() {

}
