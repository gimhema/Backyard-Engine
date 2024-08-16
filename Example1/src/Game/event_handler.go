package game

func EventListen() {
	defer GetPlayerNetworkManager().DisConnect()
	GetPlayerNetworkManager().Read()
	//for {
	//	GetPlayerNetworkManager().Read()
	//	// EventCall(message)
	//}
}
