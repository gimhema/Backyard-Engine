package game

func EventListen() {
	for {
		message := GetPlayerNetworkManager().Read()
		EventCall(message)
	}
}
