package backyard_player

type NetworkProtocol int

const (
	DEFAULT NetworkProtocol = iota //0
	TCP     NetworkProtocol = iota //1
	UDP     NetworkProtocol = iota //2
)

type BackyardSocket struct {
	socket_protocol NetworkProtocol
	some_socket     interface{}
	port            int
	ipAddress       string
}

type SocketManager struct {
	socket_container []BackyardSocket
}
