package game

import (
	"fmt"
	quickshotmessage "platformer/QuickShotPackage"
)

func EventCall(msg string) {
	// quickshotmessage.TEST()

	id, _, data := quickshotmessage.Deserialize(msg)

	switch id {
	case 3:
		fmt.Println("Raw Message : ", msg)
		fmt.Println("Echo Recv Data : ", data)
	default:
		fmt.Println("Unknown Type")
	}

}
