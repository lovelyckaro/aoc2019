module Main where

type Memory = [Int]

getInput :: IO Memory
getInput = do
  file <- readFile "input2.txt"
  return . read $ file

main :: IO ()
main = undefined

