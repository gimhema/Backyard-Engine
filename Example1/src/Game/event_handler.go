package game

func EventCall(data string) {

}

func EventListen() {
	for {
		message := GetPlayerNetworkManager().Read()
		EventCall(message)
	}
}
