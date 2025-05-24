local player = CS.RPG.GameCore.AdventureStatic.GetLocalPlayer().UnityGO

local pos_x = (9.131624)
local pos_y = (831.7098)
local pos_z = (816.63)

local rot_x = 0
local rot_y = 0
local rot_z = 0

local targetPosition = CS.UnityEngine.Vector3(pos_x,pos_y,pos_z)
local targetRotation = CS.UnityEngine.Quaternion.Euler(rot_x, rot_y, rot_z)

player.transform.position = targetPosition
player.transform.rotation = targetRotation
