module Day3 where
import Data.List (intersect)
import qualified Data.Map.Strict as M

-- Point   = (x, y)
type Point = (Int, Int)
-- Path   = map from Points to distances going there
type Path = M.Map Point Int

parsePath :: String -> Path
parsePath str = parse ((0,0),0) (words str)
  where parse :: (Point, Int) -> [String] -> Path
        parse _             []                 = M.empty
        parse startPoint (instruction:rest) = let path = walk startPoint instruction in
          M.unionWith min (M.fromList path) (parse (last path) rest)

-- Using a starting point, walk the distance from String
walk :: (Point, Int) -> String -> [(Point, Int)]
walk ((x,y),len) ('U':dist) = [((x, y + distance), len + distance) | distance <- [1..(read dist)]]
walk ((x,y),len) ('D':dist) = [((x, y - distance), len + distance) | distance <- [1..(read dist)]]
walk ((x,y),len) ('L':dist) = [((x - distance, y), len + distance) | distance <- [1..(read dist)]] 
walk ((x,y),len) ('R':dist) = [((x + distance, y), len + distance) | distance <- [1..(read dist)]]
 

intersectingPoints :: Path -> Path -> Path
intersectingPoints points1 points2 = M.intersectionWith (+) points1 points2

-- Manhattan distance from origin for point
distance :: Point -> Int
distance (x,y) = abs x + abs y 

getInput :: IO [Path]
getInput = do
  file <- readFile "input3.txt"
  return . (map parsePath) . lines $ file

main :: IO()
main = do
  inp <- getInput
  let inter = intersectingPoints (inp !! 0) (inp !! 1)
  let minDistance = minimum . M.elems $ inter
  print minDistance
