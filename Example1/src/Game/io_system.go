package game

import (
	"fmt"
	"log"

	"github.com/eiannone/keyboard"
	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/inpututil"
)

type IOManager struct {
	keys []ebiten.Key
}

func (ioManager *IOManager) InputWait() error {
	ioManager.keys = inpututil.AppendPressedKeys(ioManager.keys[:0])
	return nil
}

func (ioManager *IOManager) Run() {

	if err := keyboard.Open(); err != nil {
		log.Fatal(err)
	}
	defer keyboard.Close()

	for {
		char, key, err := keyboard.GetKey()
		if err != nil {
			log.Fatal(err)
		}

		if key == keyboard.KeyEsc {
			fmt.Println("Program Exit.")
			break
		}

		fmt.Printf("Key : %c\n", char)
	}
}
