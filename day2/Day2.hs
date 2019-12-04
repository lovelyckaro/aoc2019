module Day2 where

type Memory = [Int]
type MemoryState = State Memory

getInput :: IO Memory
getInput = do
  file <- readFile "input2.txt"
  return . read $ file

eval :: Memory -> MemoryState Int


main :: IO ()
main = undefined

