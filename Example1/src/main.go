package main

import (
	backyard_player "platformer/ClientInterface"
	quickshotmessage "platformer/QuickShotPackage"
	"sync"
)

var GplayerNetworkManagerInstance *backyard_player.PlayerNetworkManager
var once sync.Once

func GetPlayerNetworkManager() *backyard_player.PlayerNetworkManager {
	once.Do(func() {
		GplayerNetworkManagerInstance = &backyard_player.PlayerNetworkManager{}
	})

	return GplayerNetworkManagerInstance
}

func main() {
	// Testing deserialization
	quickshotmessage.TEST()

	GetPlayerNetworkManager().Init()
}
