package game

import (
	quickshotmessage "platformer/QuickShotPackage"
)

func EventCall(data string) {
	quickshotmessage.TEST()
}

func EventListen() {
	for {
		message := GetPlayerNetworkManager().Read()
		EventCall(message)
	}
}
