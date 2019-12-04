module Main where

low :: Int
low = 130254

high :: Int
high = 678275

possiblePasswords :: [Int]
possiblePasswords = [n | n <-[130254..678275], rising n, twoAdjacent n]


-- removeBlobs [1,2,3,4,4,4,5,6,7] = [1,2,3,5,6,7]
removeBlobs :: [Int] -> [Int]
removeBlobs [] = []
removeBlobs (x:xs) | length blob > 2 = removeBlobs rest
                   | otherwise       = blob ++ removeBlobs rest
                      where blob :: [Int]
                            blob = x : (takeWhile (==x) xs)
                            rest :: [Int]
                            rest = dropWhile (==x) xs

twoAdjacent :: Int -> Bool
twoAdjacent n = helper . removeBlobs $ digits n
  where
        helper :: Eq a => [a] -> Bool
        helper [] = False
        helper [x] = False 
        helper (x:y:ys) = x == y || helper (y:ys)

rising :: Int -> Bool
rising n = sorted (digits n) 
  where sorted :: Ord a => [a] -> Bool
        sorted [] = True
        sorted [x] = True
        sorted (x:y:ys) = x <= y && sorted (y:ys)

digits :: Int -> [Int]
digits 0 = []
digits n = digits (n `div` 10) ++ [n `mod` 10]

main :: IO()
main = print $ length possiblePasswords
