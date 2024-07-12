package game

import (
	backyard_player "platformer/ClientInterface"
	"sync"
)

var gPlayerNetworkManagerInstance *backyard_player.PlayerNetworkManager
var gIOManagerInstance *IOManager

var once sync.Once
var ioOnce sync.Once

func GetPlayerNetworkManager() *backyard_player.PlayerNetworkManager {
	once.Do(func() {
		gPlayerNetworkManagerInstance = &backyard_player.PlayerNetworkManager{}
	})

	return gPlayerNetworkManagerInstance
}

func GetIOManager() *IOManager {
	once.Do(func() {
		gIOManagerInstance = &IOManager{}
	})

	return gIOManagerInstance
}

func GameLogicMain() {
	println("Game Start . . .")
	// go GetPlayerNetworkManager().Init()
	GetIOManager().Run()

}
