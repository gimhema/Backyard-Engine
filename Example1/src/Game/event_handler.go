package game

func EventListen() {
	defer GetPlayerNetworkManager().DisConnect()
	for {
		message := GetPlayerNetworkManager().Read()
		EventCall(message)
	}
}
