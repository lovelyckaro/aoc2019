module Main where

type Mass = Integer
type Fuel = Integer

calcFuel :: Mass -> Fuel
calcFuel moduleMass = moduleFuel + fuelFuel moduleFuel
  where 
        moduleFuel :: Fuel
        moduleFuel = massToFuel moduleMass

massToFuel :: Mass -> Fuel
massToFuel m = m `div` 3 - 2

fuelFuel :: Fuel -> Fuel
fuelFuel m | fuelMass <= 0 = 0
           | otherwise    = fuelFuel fuelMass + fuelMass
              where fuelMass = massToFuel m


getInput :: FilePath -> IO [Mass]
getInput fp = do
  file <- readFile fp
  return (map read $ filter (/= "") $ lines file)

main :: IO()
main = do
  input <- getInput "input1.txt"
  let fuel = sum . (map calcFuel) $ input
  print fuel

