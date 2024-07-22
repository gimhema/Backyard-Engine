package game

import "fmt"

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

	GetPlayerNetworkManager().Send("Hello Server ! !")
}
