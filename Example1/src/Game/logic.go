package game

import (
	backyard_player "platformer/ClientInterface"
	quickshotmessage "platformer/QuickShotPackage"
	"sync"
)

var GplayerNetworkManagerInstance *backyard_player.PlayerNetworkManager
var GIOManagerInstance *IOManager

var once sync.Once
var ioOnce sync.Once

func GetPlayerNetworkManager() *backyard_player.PlayerNetworkManager {
	once.Do(func() {
		GplayerNetworkManagerInstance = &backyard_player.PlayerNetworkManager{}
	})

	return GplayerNetworkManagerInstance
}

func GetIOManager() *IOManager {
	once.Do(func() {
		GIOManagerInstance = &IOManager{}
	})

	return GIOManagerInstance
}

func GameLogicMain() {

	quickshotmessage.TEST()
	GetPlayerNetworkManager().Init()
	go GetIOManager().Wait()
}
