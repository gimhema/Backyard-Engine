package game

import (
	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/inpututil"
)

type IOManager struct {
	keys []ebiten.Key
}

func (ioManager *IOManager) Update() error {
	ioManager.keys = inpututil.AppendPressedKeys(ioManager.keys[:0])
	return nil
}

func (ioManager *IOManager) Wait() {

}
