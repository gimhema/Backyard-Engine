package game

import (
	"fmt"

	quickshotmessage "platformer/QuickShotPackage"
)

func (ioManager *IOManager) InputAction(key rune) {
	fmt.Printf("Key : %c\n", key)

	switch key {
	case 'A', 'a':
		fmt.Println("Request Echo")
		RequestEcho()
	case 'B', 'b':
		fmt.Println("Action for B key")
	case 'C', 'c':
		fmt.Println("Action for C key")
	case 'D', 'd':
		fmt.Println("Action for D key")
	default:
		fmt.Println("Unsupported Key")
	}
}

func RequestEcho() {
	sendMsg := quickshotmessage.NewQMessage(1, 0, []string{"[3:0:HelloServer]"})
	serializedData := quickshotmessage.Serialize(sendMsg)
	GetPlayerNetworkManager().Send(serializedData)
}
